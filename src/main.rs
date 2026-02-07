mod models;
mod extractor;
mod router;

use pdf_extract::extract_text;
use models::*;
use std::fs;
use std::path::Path;

fn main() {
    println!("═══════════════════════════════════════════════════");
    println!("🚀 FNOL Processor - Autonomous Claims Routing Agent");
    println!("═══════════════════════════════════════════════════\n");
    
    let sample_dir = "sample_docs";
    
    if !Path::new(sample_dir).exists() {
        eprintln!("❌ Error: '{}' directory not found!", sample_dir);
        eprintln!("   Please create it and add documents.");
        return;
    }
    
    // Get all PDF and TXT files
    let files = match fs::read_dir(sample_dir) {
        Ok(entries) => {
            entries
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path()
                        .extension()
                        .and_then(|s| s.to_str())
                        .map(|s| {
                            let ext = s.to_lowercase();
                            ext == "pdf" || ext == "txt"
                        })
                        .unwrap_or(false)
                })
                .collect::<Vec<_>>()
        }
        Err(e) => {
            eprintln!("❌ Error reading directory: {}", e);
            return;
        }
    };
    
    if files.is_empty() {
        eprintln!("⚠️  No PDF or TXT files found in '{}'", sample_dir);
        eprintln!("   Please add FNOL documents to process.");
        return;
    }
    
    println!("📄 Found {} document(s) to process\n", files.len());
    
    // Create output directory
    fs::create_dir_all("output").expect("Failed to create output directory");
    
    // Process each file
    for (idx, entry) in files.iter().enumerate() {
        let file_path = entry.path();
        let filename = file_path.file_name().unwrap().to_str().unwrap();
        
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📋 Processing: {}", filename);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        
        // Extract text based on file type
        let text = if file_path.extension().and_then(|s| s.to_str()) == Some("pdf") {
            match extract_text(&file_path) {
                Ok(t) => {
                    println!("✓ PDF text extracted successfully");
                    t
                }
                Err(e) => {
                    eprintln!("✗ Error reading PDF: {}", e);
                    continue;
                }
            }
        } else {
            // Read TXT file
            match fs::read_to_string(&file_path) {
                Ok(t) => {
                    println!("✓ TXT file read successfully");
                    t
                }
                Err(e) => {
                    eprintln!("✗ Error reading TXT file: {}", e);
                    continue;
                }
            }
        };
        
        // Extract fields
        println!("✓ Extracting fields...");
        let extracted = extractor::extract_fields(&text);
        
        // Find missing fields
        let missing = router::find_missing_fields(&extracted);
        
        // Determine route
        let (route, reasoning, confidence) = router::determine_route(&extracted, &missing);
        
        // Create result
        let result = ProcessingResult {
            extracted_fields: extracted,
            missing_fields: missing,
            recommended_route: route.clone(),
            reasoning: reasoning.clone(),
            confidence_score: confidence,
        };
        
        // Display summary
        println!("\n📊 RESULTS:");
        println!("   Route: {}", route);
        println!("   Confidence: {:.1}%", confidence);
        println!("   Reasoning: {}", reasoning);
        
        if !result.missing_fields.is_empty() {
            println!("   ⚠️  Missing Fields: {}", result.missing_fields.join(", "));
        }
        
        // Save JSON output
        let output_filename = format!("output/result_{}.json", idx + 1);
        let json = serde_json::to_string_pretty(&result)
            .expect("Failed to serialize result");
        
        fs::write(&output_filename, &json)
            .expect("Failed to write output file");
        
        println!("   ✅ Saved to: {}\n", output_filename);
    }
    
    println!("═══════════════════════════════════════════════════");
    println!("✅ Processing complete!");
    println!("═══════════════════════════════════════════════════");
}