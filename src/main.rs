use csv_processing_automation::*;
use std::{path::Path, process};

fn main() {
    println!("Processing CSV file: {}", INPUT_CSV_FILE);

    match process_csv_file(Path::new(INPUT_CSV_FILE)) {
        Ok(df) => {
            println!("\nProcessing completed successfully!");
            println!("Number of rows processed: {}", df.height());
            println!("Output written to: {}", OUTPUT_CSV_FILE);

            // Display the first few rows as a sample
            println!("\nFirst 5 rows of processed data:");
            println!("{}", df.head(Some(5)));
        }
        Err(e) => {
            eprintln!("Error processing CSV: {}", e);
            process::exit(1);
        }
    }
}
