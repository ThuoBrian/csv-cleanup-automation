use polars::prelude::*;
use std::fs::{File, create_dir_all};
use std::path::Path;
use std::{error::Error, fmt};

/// CSV-related error type for the crate (no external deps).
#[derive(Debug)]
pub enum CsvError {
    Io(std::io::Error),
    Polars(PolarsError),
    NotFound(String),
}

impl fmt::Display for CsvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CsvError::Io(e) => write!(f, "IO error: {}", e),
            CsvError::Polars(e) => write!(f, "Polars error: {}", e),
            CsvError::NotFound(s) => write!(f, "Not found: {}", s),
        }
    }
}
impl Error for CsvError {}

impl From<std::io::Error> for CsvError {
    fn from(e: std::io::Error) -> Self {
        CsvError::Io(e)
    }
}
impl From<PolarsError> for CsvError {
    fn from(e: PolarsError) -> Self {
        CsvError::Polars(e)
    }
}

pub type Result<T> = std::result::Result<T, CsvError>;

// CSV column names
pub const COL_NAME: &str = "Name";
pub const COL_TOTAL_PRINTS: &str = "Total Prints";
pub const COL_BW_PRINTER: &str = "Black & WhiteTotal(Printer)";
pub const COL_BW_COPIER: &str = "Black & WhiteTotal(Copier/Document Server)";
pub const COL_BW_LARGE: &str = "Black & White(Large size)(Copier/Document Server)";

// Defaults (library users should pass paths when possible)
pub const INPUT_CSV_FILE: &str = "./data/IPAK_NRB_PROGRAMS_.csv";
pub const OUTPUT_CSV_FILE: &str = "./data/analyzed_output.csv";

pub fn process_csv_file(input_path: &Path) -> Result<DataFrame> {
    if !input_path.exists() {
        return Err(CsvError::NotFound(format!("{:?}", input_path)));
    }

    let path_str = input_path
        .to_str()
        .ok_or_else(|| CsvError::NotFound("invalid input path".to_string()))?;

    // Build lazy pipeline directly from CSV file
    let lazy = LazyCsvReader::new(path_str)
        .has_header(true)
        .with_try_parse_dates(false) // optional: tweak depending on data
        .finish()
        .map_err(CsvError::Polars)?
        .select(&[
            col(COL_NAME),
            col(COL_TOTAL_PRINTS),
            col(COL_BW_PRINTER),
            col(COL_BW_COPIER),
            col(COL_BW_LARGE),
        ])
        // replace_all regex flag = true
        .with_column(
            col(COL_NAME)
                .str()
                .replace_all(lit(r#"[\[\]]"#), lit(""), true)
                .alias(COL_NAME),
        );

    let mut df = lazy.collect().map_err(CsvError::Polars)?;

    // Ensure output directory exists
    if let Some(parent) = Path::new(OUTPUT_CSV_FILE).parent() {
        create_dir_all(parent)?;
    }

    let mut out = File::create(OUTPUT_CSV_FILE)?;
    CsvWriter::new(&mut out)
        .has_header(true)
        .finish(&mut df)
        .map_err(CsvError::Polars)?;

    Ok(df)
}
