use crate::models::FnolData;

pub fn determine_route(data: &FnolData, missing_fields: &[String]) -> (String, String, f32) {
    if !missing_fields.is_empty() {
        let confidence = 60.0;
        return (
            "Manual Review".to_string(),
            format!("Missing {} mandatory field(s): {}", 
                    missing_fields.len(),
                    missing_fields.join(", ")),
            confidence
        );
    }
    
    if let Some(desc) = &data.description {
        let fraud_keywords = ["fraud", "staged", "inconsistent", "suspicious", "fake"];
        for keyword in &fraud_keywords {
            if desc.to_lowercase().contains(keyword) {
                let confidence = 95.0;
                return (
                    "Investigation Queue".to_string(),
                    format!("Description contains potential fraud indicator: '{}'", keyword),
                    confidence
                );
            }
        }
    }
    
    if let Some(claim_type) = &data.claim_type {
        if claim_type.to_lowercase().contains("injury") {
            let confidence = 90.0;
            return (
                "Specialist Queue".to_string(),
                "Injury claim requires specialist medical review".to_string(),
                confidence
            );
        }
    }
    
    if let Some(damage) = data.estimated_damage {
        if damage < 25000.0 {
            let confidence = 85.0;
            return (
                "Fast-track".to_string(),
                format!("Estimated damage ${:.2} is below $25,000 threshold", damage),
                confidence
            );
        } else {
            let confidence = 75.0;
            return (
                "Manual Review".to_string(),
                format!("Estimated damage ${:.2} exceeds threshold", damage),
                confidence
            );
        }
    }
    
    let confidence = 70.0;
    ("Manual Review".to_string(), "Standard processing".to_string(), confidence)
}

pub fn find_missing_fields(data: &FnolData) -> Vec<String> {
    let mut missing = Vec::new();
    
    if data.policy_number.is_none() {
        missing.push("Policy Number".to_string());
    }
    if data.policyholder_name.is_none() {
        missing.push("Policyholder Name".to_string());
    }
    if data.incident_date.is_none() {
        missing.push("Incident Date".to_string());
    }
    if data.location.is_none() {
        missing.push("Location".to_string());
    }
    if data.description.is_none() {
        missing.push("Description".to_string());
    }
    if data.estimated_damage.is_none() {
        missing.push("Estimated Damage".to_string());
    }
    if data.claim_type.is_none() {
        missing.push("Claim Type".to_string());
    }
    
    missing
}