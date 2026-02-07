use regex::Regex;
use crate::models::FnolData;

pub fn extract_fields(text: &str) -> FnolData {
    let mut data = FnolData::new();
    
    data.policy_number = extract_policy_number(text);
    data.policyholder_name = extract_policyholder_name(text);
    data.insured_address = extract_address(text);
    data.incident_date = extract_incident_date(text);
    data.incident_time = extract_incident_time(text);
    data.location = extract_location(text);
    data.description = extract_description(text);
    data.estimated_damage = extract_damage(text);
    data.claim_type = extract_claim_type(text);
    data.vehicle_vin = extract_vin(text);
    data.vehicle_year = extract_vehicle_year(text);
    data.vehicle_make = extract_vehicle_make(text);
    data.vehicle_model = extract_vehicle_model(text);
    data.driver_name = extract_driver_name(text);
    data.owner_name = extract_owner_name(text);
    data.police_report_number = extract_police_report(text);
    
    data
}

fn extract_policy_number(text: &str) -> Option<String> {
    let re = Regex::new(r"POLICY NUMBER:\s*([A-Z0-9-]+)").ok()?;
    re.captures(text)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().trim().to_string())
}

fn extract_policyholder_name(text: &str) -> Option<String> {
    let re = Regex::new(r"NAME OF INSURED \([^)]+\):\s*([A-Za-z\s]+)").ok()?;
    re.captures(text)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().trim().to_string())
}

fn extract_address(text: &str) -> Option<String> {
    let re = Regex::new(r"INSURED'S MAILING ADDRESS:\s*([^\n]+)").ok()?;
    re.captures(text)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().trim().to_string())
}

fn extract_incident_date(text: &str) -> Option<String> {
    let re = Regex::new(r"DATE OF LOSS AND TIME:\s*(\d{2}/\d{2}/\d{4})").ok()?;
    re.captures(text)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().trim().to_string())
}

fn extract_incident_time(text: &str) -> Option<String> {
    let re = Regex::new(r"DATE OF LOSS AND TIME:\s*\d{2}/\d{2}/\d{4}\s+(\d{1,2}:\d{2}\s*[AP]M)").ok()?;
    re.captures(text)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().trim().to_string())
}

fn extract_location(text: &str) -> Option<String> {
    let re = Regex::new(r"STREET:\s*([^\n]+)").ok()?;
    re.captures(text)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().trim().to_string())
}

fn extract_description(text: &str) -> Option<String> {
    // Look for "DESCRIPTION OF ACCIDENT:" followed by content on next lines
    if let Some(start_idx) = text.find("DESCRIPTION OF ACCIDENT:") {
        let after_label = &text[start_idx + "DESCRIPTION OF ACCIDENT:".len()..];
        
        // Find where the next all-caps label starts
        let end_patterns = [
            "\nPOLICE OR FIRE",
            "\nINSURED VEHICLE",
            "\nDRIVER'S NAME",
            "\nOWNER'S NAME",
            "\nREPORT NUMBER",
        ];
        
        let mut end_idx = after_label.len();
        for pattern in &end_patterns {
            if let Some(idx) = after_label.find(pattern) {
                if idx < end_idx {
                    end_idx = idx;
                }
            }
        }
        
        let description = after_label[..end_idx].trim();
        
        if description.len() > 10 {
            return Some(description.to_string());
        }
    }
    
    None
}

fn extract_damage(text: &str) -> Option<f64> {
    let re = Regex::new(r"ESTIMATE AMOUNT:\s*\$?([\d,]+\.?\d*)").ok()?;
    re.captures(text)
        .and_then(|cap| cap.get(1))
        .and_then(|m| {
            let amount_str = m.as_str().replace(",", "");
            amount_str.parse::<f64>().ok()
        })
}

fn extract_claim_type(text: &str) -> Option<String> {
    // Check for explicit claim type first
    if let Ok(re) = Regex::new(r"CLAIM TYPE:\s*([^\n]+)") {
        if let Some(cap) = re.captures(text) {
            if let Some(m) = cap.get(1) {
                return Some(m.as_str().trim().to_string());
            }
        }
    }
    
    // Otherwise infer from description
    let text_lower = text.to_lowercase();
    if text_lower.contains("injury") || text_lower.contains("injured") || text_lower.contains("hospital") {
        return Some("Injury".to_string());
    }
    if text_lower.contains("collision") {
        return Some("Collision".to_string());
    }
    
    Some("Auto".to_string())
}

fn extract_vin(text: &str) -> Option<String> {
    let re = Regex::new(r"V\.I\.N\.:\s*([A-HJ-NPR-Z0-9]{17})").ok()?;
    re.captures(text)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().trim().to_string())
}

fn extract_vehicle_year(text: &str) -> Option<String> {
    let re = Regex::new(r"VEH #\s*YEAR:\s*(\d{4})").ok()?;
    re.captures(text)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().trim().to_string())
}

fn extract_vehicle_make(text: &str) -> Option<String> {
    let re = Regex::new(r"MAKE:\s*([A-Za-z]+)").ok()?;
    re.captures(text)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().trim().to_string())
}

fn extract_vehicle_model(text: &str) -> Option<String> {
    let re = Regex::new(r"MODEL:\s*([A-Za-z0-9\s]+)").ok()?;
    re.captures(text)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().trim().to_string())
}

fn extract_driver_name(text: &str) -> Option<String> {
    let re = Regex::new(r"DRIVER'S NAME AND ADDRESS:\s*([A-Za-z\s]+)").ok()?;
    re.captures(text)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().trim().to_string())
}

fn extract_owner_name(text: &str) -> Option<String> {
    let re = Regex::new(r"OWNER'S NAME AND ADDRESS:\s*([A-Za-z\s]+)").ok()?;
    re.captures(text)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().trim().to_string())
}

fn extract_police_report(text: &str) -> Option<String> {
    let re = Regex::new(r"REPORT NUMBER:\s*([A-Z0-9-]+)").ok()?;
    re.captures(text)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().trim().to_string())
}