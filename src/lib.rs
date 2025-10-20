use polars::prelude::DataFrame;
use polars::prelude::*;
use std::fs::File;
use std::path::Path;

//csv column names
pub const COL_NAME: &str = "Name";
pub const COL_TOTAL_PRINTS: &str = "Total Prints";
pub const COL_BW_PRINTER: &str = "Black & WhiteTotal(Printer)";
pub const COL_BW_COPIER: &str = "Black & WhiteTotal(Copier/Document Server)";
pub const COL_BW_LARGE: &str = "Black & White(Large size)(Copier/Document Server)";

// CSV file paths under the ./data folder.
pub const INPUT_CSV_DIR: &str = "./data/IPAK_NRB_PROGRAMS_.csv";
pub const OUTPUT_CSV_FILE: &str = "./data/analyzed_output.csv";

pub fn process_csv_file(input_path: &Path) -> Result<DataFrame, PolarsError> {
    // Check if the input file exists
    if !input_path.exists() {
        return Err(PolarsError::ComputeError(
            format!("Input file does not exist: {:?}", input_path).into(),
        ));
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
        COL_NAME,
        COL_TOTAL_PRINTS,
        COL_BW_PRINTER,
        COL_BW_COPIER,
        COL_BW_LARGE,
    ])?;

    // Clean the "Name" column by removing square brackets REGEX (r#"[\[\]]"#))
    let cleaned_dataframe = selected_dataframe
        .lazy()
        .with_column(
            col(COL_NAME)
                .str()
                .replace_all(lit(r#"[\[\]]"#), lit(""), false)
                .alias(COL_NAME),
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

    println!("\n Output file created at: {}", OUTPUT_CSV_FILE);

    Ok(cleaned_dataframe)
}
