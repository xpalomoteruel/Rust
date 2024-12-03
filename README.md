# Stock Market Data Scraper in Rust

## Project Description

This project aims to create a Stock Market Data Scraper using Rust. The scraper will connect to public APIs to fetch stock market data, analyze the data, and provide fundamental analysis using key financial ratios such as Price-to-Earnings (P/E), Free Cash Flow (FCF) yield, Enterprise Value to Free Cash Flow (EV/FCF), EBITDA, and debt ratios. This project is designed to help familiarize yourself with Rust's web capabilities, including HTTP requests, JSON parsing, and basic data handling. By the end of the project, we will have a command-line tool that can pull data, perform fundamental analysis, and present meaningful insights.

## Features
- Fetch stock market data using public APIs (e.g., Yahoo Finance or Alpha Vantage).
- Parse and store the financial data in a structured format.
- Analyze data to provide fundamental metrics such as P/E, FCF yield, EV/FCF, EBITDA, and debt ratios.
- Command-line interface to interact with the tool.

## Folder Structure
The project will have the following structure:

```
stock-market-scraper/
├── src/                 # Main source code of the project
│   ├── main.rs          # Entry point of the application
│   ├── api/             # Module for API calls to fetch data
│   ├── parser/          # Module for parsing and structuring API responses
│   ├── analyzer/        # Module for analyzing stock data and calculating key financial ratios
│   └── utils/           # Utility functions for formatting and other helper tasks
├── tests/               # Unit and integration tests
│   ├── api_tests.rs     # Tests for the API module
│   ├── parser_tests.rs  # Tests for the parser module
│   ├── analyzer_tests.rs # Tests for data analysis functions
├── Cargo.toml           # Rust package manifest, managing dependencies
└── README.md            # Project documentation
```

### Detailed Folder and File Descriptions
- **src/**: Contains the Rust source code.
  - **main.rs**: The main entry point for the application where the overall logic is managed.
  - **api/**: Contains code related to fetching data from external APIs. This could include different modules for each API or shared logic to make HTTP requests using `reqwest`.
  - **parser/**: Responsible for parsing JSON responses from the APIs into Rust data structures that can be used by other parts of the application.
  - **analyzer/**: Includes functions for analyzing data, such as calculating key financial ratios like P/E, FCF yield, EV/FCF, EBITDA, and debt ratios, and generating any actionable insights.
  - **utils/**: Utility functions that are shared across modules, such as date formatters or general-purpose calculations.
- **tests/**: Contains unit and integration tests for each part of the project.
  - **api_tests.rs**: Tests to validate the API interaction, ensuring that data is fetched correctly.
  - **parser_tests.rs**: Tests to verify that JSON responses are parsed as expected.
  - **analyzer_tests.rs**: Tests for validating the calculations and logic inside the analyzer module, particularly focusing on financial ratios.
- **Cargo.toml**: Manages Rust dependencies, such as `reqwest` for HTTP requests, `serde` for JSON parsing, etc.
- **README.md**: This documentation file providing an overview, setup instructions, and usage examples.

## Installation and Setup
1. **Clone the Repository**:
   ```sh
   git clone <repository-url>
   cd stock-market-scraper
   ```

2. **Install Dependencies**:
   Ensure that you have Rust installed. Install necessary dependencies by running:
   ```sh
   cargo build
   ```

3. **Run the Application**:
   To run the application and start fetching stock data:
   ```sh
   cargo run
   ```

## Usage
- After running the application, you will be prompted to enter a stock symbol (e.g., AAPL for Apple).
- The application will then fetch the data, perform fundamental analysis, and present the results on the command line, including key metrics such as P/E, FCF yield, EV/FCF, EBITDA, and debt ratios.

## Future Improvements
- Add more detailed fundamental analysis options, such as growth rates or profitability metrics.
- Implement a simple GUI to make it easier for end-users.
- Add options for saving historical data to a database for long-term trend analysis.

## Contributing
Contributions are welcome! Please fork the repository and create a pull request with your feature additions or bug fixes.

## License
This project is licensed under the MIT License.
