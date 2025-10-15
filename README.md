# CSV Cleanup Automation

## Overview

CSV Cleanup Automation is a Rust-based project designed to automate the process of cleaning and analyzing CSV files. This tool helps streamline data processing tasks by providing a simple interface for reading, cleaning, and writing CSV data.

## Features

- **Dynamic CSV Processing**: Automatically processes multiple CSV files from a specified directory.
- **Data Cleaning**: Cleans specific columns by removing unwanted characters and formatting issues.
- **Output Generation**: Generates cleaned CSV files for further analysis.

## Getting Started

### Prerequisites

- Rust (1.50 or later)
- Cargo (Rust package manager)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/csv-cleanup-automation.git
   cd csv-cleanup-automation
   ```

2. Build the project:
   ```bash
   cargo build
   ```

### Usage

To run the CSV cleanup automation, use the following command:

```bash
cargo run -- <input_csv_directory>
```

Replace `<input_csv_directory>` with the path to the directory containing your CSV files.

### Example

```bash
cargo run -- ./data
```

This command will process all CSV files in the `./data` directory and generate cleaned output files.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
