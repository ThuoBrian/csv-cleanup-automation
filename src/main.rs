use csv_processing_automation::*;
use std::path::Path;

fn main() {
    match process_csv_file(Path::new(INPUT_CSV_DIR)) {
        Ok(df) => {
            println!("Processed DataFrame:\n{}", df);
        }
        Err(e) => {
            eprintln!("Failed to analyze CSV: {:?}", e);
        }
    }
}
