# FNOL Processor - Autonomous Insurance Claims Routing Agent

An intelligent agent that automatically processes First Notice of Loss (FNOL) documents, extracts key information, and routes claims to the appropriate workflow.

## 🎯 Overview

This system:
- **Extracts** critical fields from insurance claim PDFs (ACORD forms)
- **Validates** mandatory information and identifies missing data
- **Classifies** claims based on severity, type, and risk indicators
- **Routes** claims to appropriate queues (Fast-track, Manual Review, Investigation, Specialist)
- **Outputs** structured JSON with routing decisions and reasoning

## 🏗️ Architecture
```
┌─────────────┐
│  PDF Input  │
└──────┬──────┘
       │
       ▼
┌─────────────────┐
│ Text Extraction │ (pdf-extract)
└──────┬──────────┘
       │
       ▼
┌─────────────────┐
│ Field Extractor │ (regex patterns)
└──────┬──────────┘
       │
       ▼
┌─────────────────┐
│ Validation      │ (missing field detection)
└──────┬──────────┘
       │
       ▼
┌─────────────────┐
│ Routing Logic   │ (rules engine)
└──────┬──────────┘
       │
       ▼
┌─────────────────┐
│  JSON Output    │
└─────────────────┘
```

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))

### Installation
```bash
# Clone repository
git clone <your-repo-url>
cd fnol-processor

# Build project
cargo build --release

# Run
cargo run
```

### Usage

1. Place FNOL PDF documents in the `sample_docs/` folder
2. Run the processor: `cargo run`
3. Check results in `output/` folder

## 📋 Extracted Fields

**Policy Information:**
- Policy Number
- Policyholder Name
- Insured Address

**Incident Information:**
- Date & Time
- Location
- Description

**Asset Details:**
- Vehicle VIN, Year, Make, Model
- Estimated Damage

**Involved Parties:**
- Driver Name
- Owner Name
- Police Report Number

## 🔀 Routing Rules

| Condition | Route | Priority |
|-----------|-------|----------|
| Missing mandatory fields | **Manual Review** | High |
| Fraud keywords detected | **Investigation Queue** | Critical |
| Injury claim | **Specialist Queue** | High |
| Damage < $25,000 | **Fast-track** | Standard |
| Default | **Manual Review** | Standard |

### Fraud Detection Keywords
- "fraud", "staged", "inconsistent", "suspicious", "fake"

## 📤 Output Format
```json
{
  "extracted_fields": {
    "policy_number": "ABC123456",
    "policyholder_name": "John Doe",
    "incident_date": "01/15/2024",
    "estimated_damage": 15000.0,
    ...
  },
  "missing_fields": [],
  "recommended_route": "Fast-track",
  "reasoning": "Estimated damage $15,000.00 is below $25,000 threshold",
  "confidence_score": 85.0
}
```

## 🛠️ Technical Approach

### Extraction Strategy
- **PDF Parsing**: Uses `pdf-extract` crate for text extraction
- **Pattern Matching**: Regex patterns tailored for ACORD form structure
- **Field Validation**: Checks for presence and format of mandatory fields

### Routing Logic
- **Rule-based engine** with priority hierarchy
- **Confidence scoring** based on field completeness
- **Fraud detection** using keyword analysis

### Error Handling
- Graceful degradation when fields are missing
- Detailed logging of extraction failures
- Confidence scores reflect data quality

## 🧪 Testing
```bash
# Run with sample data
cargo run

# Run tests (if implemented)
cargo test
```

## 📁 Project Structure
```
fnol-processor/
├── Cargo.toml          # Dependencies
├── README.md           # This file
├── src/
│   ├── main.rs         # Entry point & orchestration
│   ├── models.rs       # Data structures
│   ├── extractor.rs    # PDF parsing & field extraction
│   └── router.rs       # Routing logic & validation
├── sample_docs/        # Input PDFs
└── output/             # Generated JSON results
```

## 🔧 Future Enhancements

- [ ] ML-based field extraction for non-standard formats
- [ ] Support for scanned/image-based PDFs (OCR)
- [ ] REST API for integration with claims systems
- [ ] Database persistence for audit trails
- [ ] Dashboard for monitoring routing decisions


## 👤 Author

[chaitanya kumar yelisetti]
