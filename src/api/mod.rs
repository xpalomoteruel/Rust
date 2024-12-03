use reqwest;
use serde::Deserialize;
use serde_json;  // Import serde_json to work with JSON
use dotenv::dotenv;
use std::env;

#[derive(Deserialize, Debug)]
pub struct StockData {
    pub symbol: String,
    pub price: f64,
    pub eps: f64,
    pub pe_ratio: f64,
    pub fcf: f64,
    pub fcf_yield: f64,
    pub ev: f64,
    pub ebitda: f64,
    pub ev_ebitda: f64,
    pub debt_to_equity: f64,
    pub dividend_yield: f64,
    pub total_debt: f64,
    pub revenue_growth: f64,
    pub net_income: f64,
    pub roe: f64,
}

pub async fn fetch_full_stock_data(symbol: &str) -> Result<StockData, Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("ALPHA_VANTAGE_API_KEY")?;

    // Fetch the GLOBAL_QUOTE for current price
    let quote_url = format!(
        "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol={}&apikey={}",
        symbol, api_key
    );
    let quote_response = reqwest::get(&quote_url).await?;
    let quote_json: serde_json::Value = quote_response.json().await?;
    println!("Global Quote Response: {:#?}", quote_json); // Print full response

    let price: f64 = quote_json
        .get("Global Quote")
        .and_then(|q| q.get("05. price"))
        .and_then(|p| p.as_str())
        .ok_or("Price not found")?
        .parse()?;

    // Fetch the INCOME_STATEMENT for Net Income
    let income_url = format!(
        "https://www.alphavantage.co/query?function=INCOME_STATEMENT&symbol={}&apikey={}",
        symbol, api_key
    );
    let income_response = reqwest::get(&income_url).await?;
    let income_json: serde_json::Value = income_response.json().await?;
    println!("Income Statement Response: {:#?}", income_json); // Print full response

    let annual_reports = income_json
        .get("annualReports")
        .and_then(|r| r.as_array())
        .ok_or("Failed to get annual reports")?;

    let net_income_str = annual_reports
        .get(0)
        .and_then(|report| report.get("netIncome"))
        .and_then(|ni| ni.as_str())
        .ok_or("Net Income not found")?;
    let net_income: f64 = net_income_str.parse()?;

    // Fetch the BALANCE_SHEET for Total Debt
    let balance_url = format!(
        "https://www.alphavantage.co/query?function=BALANCE_SHEET&symbol={}&apikey={}",
        symbol, api_key
    );
    let balance_response = reqwest::get(&balance_url).await?;
    let balance_json: serde_json::Value = balance_response.json().await?;
    println!("Balance Sheet Response: {:#?}", balance_json); // Print full response

    let balance_reports = balance_json
        .get("annualReports")
        .and_then(|r| r.as_array())
        .ok_or("Failed to get annual reports")?;

    // Attempt to fetch total debt
    let total_debt_str = balance_reports
        .get(0)
        .and_then(|report| report.get("totalDebt"))
        .and_then(|td| td.as_str());
    
    if let Some(total_debt_val) = total_debt_str {
        let total_debt: f64 = total_debt_val.parse()?;
        println!("Total Debt: {}", total_debt);
    } else {
        println!("Total Debt not found in Balance Sheet response.");
    }

    // Fetch the OVERVIEW for Outstanding Shares
    let overview_url = format!(
        "https://www.alphavantage.co/query?function=OVERVIEW&symbol={}&apikey={}",
        symbol, api_key
    );
    let overview_response = reqwest::get(&overview_url).await?;
    let overview_json: serde_json::Value = overview_response.json().await?;
    println!("Overview Response: {:#?}", overview_json); // Print full response

    let outstanding_shares_str = overview_json
        .get("SharesOutstanding")
        .and_then(|os| os.as_str())
        .ok_or("Outstanding shares not found")?;
    let outstanding_shares: f64 = outstanding_shares_str.parse()?;

    // Calculate EPS using Net Income / Outstanding Shares
    let eps = if outstanding_shares != 0.0 {
        net_income / outstanding_shares
    } else {
        0.0 // Handle case where outstanding shares is zero
    };

    // Placeholder values for other metrics, replace with proper API calls
    let pe_ratio = if eps != 0.0 {
        price / eps
    } else {
        0.0 // Handle case where EPS is zero
    };
    let fcf = 0.0; // Needs Cash Flow Statement data
    let fcf_yield = 0.0; // To be calculated
    let ev = 0.0; // Enterprise Value, needs multiple inputs
    let ebitda = 0.0; // EBITDA value
    let ev_ebitda = 0.0; // To be calculated
    let debt_to_equity = 0.0; // Debt to Equity ratio
    let dividend_yield = 0.0; // Dividend per share to price ratio
    let revenue_growth = 0.0; // Needs year-over-year revenue data
    let roe = 0.0; // Needs shareholder equity

    let stock_data = StockData {
        symbol: symbol.to_string(),
        price,
        eps,
        pe_ratio,
        fcf,
        fcf_yield,
        ev,
        ebitda,
        ev_ebitda,
        debt_to_equity,
        dividend_yield,
        total_debt: 0.0, // Placeholder for now
        revenue_growth,
        net_income,
        roe,
    };

    Ok(stock_data)
}

