use polars::prelude::DataFrame;
use polars::prelude::*;
use std::fs::File;
use std::path::Path;

/// CSV file paths.
pub const INPUT_CSV_FILE: &str = "./data/{raw-csv-doc}.csv";
pub const OUTPUT_CSV_FILE: &str = "./data/analyzed_output.csv";

pub fn process_csv_file(input_path: &Path) -> Result<DataFrame, PolarsError> {
    // Check if the input file exists
    if !input_path.exists() {
        eprintln!("Input file does not exist: {:?}", input_path);
    }
    // Open the input file
    let input_file = File::open(input_path).map_err(|error| {
        PolarsError::ComputeError(format!("Failed to open '{:?}': {}", input_path, error).into())
    })?;

    // Read the CSV file into a DataFrame
    let dataframe = CsvReader::new(input_file)
        .has_header(true)
        .finish()
        .map_err(|error| {
            PolarsError::ComputeError(
                format!("Failed to read '{:?}': {}", input_path, error).into(),
            )
        })?;

    // Select specific columns
    let selected_dataframe = dataframe.select([
        "Name",
        "Total Prints",
        "Black & WhiteTotal(Printer)",
        "Black & WhiteTotal(Copier/Document Server)",
        "Black & White(Large size)(Copier/Document Server)",
    ])?;

    // Clean the "Name" column by removing square brackets REGEX (r#"[\[\]]"#))
    let cleaned_dataframe = selected_dataframe
        .lazy()
        .with_column(
            col("Name")
                .str()
                .replace_all(lit(r#"[\[\]]"#), lit(""), false)
                .alias("Name"),
        )
        .collect()?;

    // Create the output file
    let mut output_file = File::create(OUTPUT_CSV_FILE).map_err(|error| {
        PolarsError::ComputeError(
            format!("Failed to create '{}': {}", OUTPUT_CSV_FILE, error).into(),
        )
    })?;

    // Write the cleaned DataFrame to the output file
    let mut cleaned_dataframe = cleaned_dataframe;
    CsvWriter::new(&mut output_file)
        .has_header(true)
        .finish(&mut cleaned_dataframe)?;

    println!("\nOutput file created at: {}", OUTPUT_CSV_FILE);

    Ok(cleaned_dataframe)
}
