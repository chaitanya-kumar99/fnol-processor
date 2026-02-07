use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FnolData {
    pub policy_number: Option<String>,
    pub policyholder_name: Option<String>,
    pub insured_address: Option<String>,
    pub incident_date: Option<String>,
    pub incident_time: Option<String>,
    pub location: Option<String>,
    pub description: Option<String>,
    pub estimated_damage: Option<f64>,
    pub claim_type: Option<String>,
    pub vehicle_vin: Option<String>,
    pub vehicle_year: Option<String>,
    pub vehicle_make: Option<String>,
    pub vehicle_model: Option<String>,
    pub driver_name: Option<String>,
    pub owner_name: Option<String>,
    pub police_report_number: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProcessingResult {
    pub extracted_fields: FnolData,
    pub missing_fields: Vec<String>,
    pub recommended_route: String,
    pub reasoning: String,
    pub confidence_score: f32,
}

impl FnolData {
    pub fn new() -> Self {
        FnolData {
            policy_number: None,
            policyholder_name: None,
            insured_address: None,
            incident_date: None,
            incident_time: None,
            location: None,
            description: None,
            estimated_damage: None,
            claim_type: None,
            vehicle_vin: None,
            vehicle_year: None,
            vehicle_make: None,
            vehicle_model: None,
            driver_name: None,
            owner_name: None,
            police_report_number: None,
        }
    }
}