# CSV Cleanup Automation

A small Rust library/CLI that reads CSV files, cleans specific columns, and writes cleaned CSV output. The code uses Polars' lazy API to build a single pipeline and execute it once for better performance.

## How it works (high level)

- Entry points:
  - Library: `process_csv_file(input: &Path) -> Result<DataFrame>` (and `process_csv_file_with_options` for custom output/schema).
  - CLI (optional `main.rs`) parses input/output paths and an optional JSON schema.
- Reading:
  - Uses `LazyCsvReader` to avoid eager materialization. The pipeline selects only the required columns up front.
- Cleaning:
  - Applies string operations (example: `replace_all` on the `Name` column with regex `[\[\]]` to remove square brackets) inside the lazy plan.
- Casting (optional):
  - If a schema map is provided (column -> `DataType`), casts are added to the lazy plan before `collect()`.
- Writing:
  - After `lazy.collect()` returns a `DataFrame`, the result is written to the specified output CSV (creates parent directory if needed).
- Errors:
  - The crate defines a `CsvError` enum that wraps `std::io::Error` and `PolarsError`, returned via a `Result<T, CsvError>` alias.

## CLI usage

Build and run (release recommended):

```bash
cargo build --release
cargo run --release -- <input.csv> -o ./data/analyzed_output.csv --schema schema.json
```

- `input.csv` — path to the CSV to process.
- `-o/--output` — optional output path (defaults to `./data/analyzed_output.csv`).
- `--schema` — optional JSON file mapping column names to types (e.g. `{"Name":"utf8","Total Prints":"u64"}`).

## Example schema JSON

```json
{
  "Name": "utf8",
  "Total Prints": "u64",
  "Black & WhiteTotal(Printer)": "u64"
}
```

## Notes and tips

- Use release builds for realistic performance (`cargo run --release`).
- For very large files, consider streaming/chunked processing instead of collecting the entire DataFrame.
- Prefer passing explicit output paths (library API) instead of modifying constants.
- Ensure the input path is correct; library functions return `CsvError::NotFound` if missing.

## License

MIT
