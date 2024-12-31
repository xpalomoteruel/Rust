use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use dotenv::dotenv;
use std::env;
use std::fs::File;
use std::io::Write;

// Function to save JSON to a file for debugging
pub fn save_to_file(data: &serde_json::Value, file_name: &str) -> std::io::Result<()> {
    let mut file = File::create(file_name)?;
    let json_data = serde_json::to_string_pretty(data)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}

// Struct to hold selected financial metrics
#[derive(Serialize, Deserialize, Debug)]
pub struct FinancialMetrics {
    // Overview metrics
    pub symbol: String,
    pub price: f64,
    pub eps: f64,
    pub pe_ratio: f64,
    pub peg_ratio: f64,
    pub dividend_yield: f64,
    pub beta: f64,
    pub book_value: f64,
    pub market_cap: f64,

    // Balance sheet metrics
    pub total_debt: f64,
    pub total_assets: f64,
    pub total_shareholder_equity: f64,
    pub current_debt: f64,
    pub current_long_term_debt: f64,
    pub goodwill: Option<f64>,

    // Income statement metrics
    pub net_income: f64,
    pub ebit: f64,
    pub ebitda: f64,
    pub cost_of_revenue: f64,

    // Derived metrics
    pub roe: f64,
    pub debt_to_equity: f64,
    pub revenue_growth: f64,
    pub fcf: f64,
    pub fcf_yield: f64,
}

pub async fn fetch_full_stock_data(symbol: &str) -> Result<FinancialMetrics, Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("ALPHA_VANTAGE_API_KEY")?;

    // Fetch the GLOBAL_QUOTE for price
    let quote_url = format!(
        "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol={}&apikey={}",
        symbol, api_key
    );
    let quote_response = reqwest::get(&quote_url).await?;
    let quote_json: serde_json::Value = quote_response.json().await?;
    save_to_file(&quote_json, "out/quote.json")?;
    let price: f64 = quote_json
        .get("Global Quote")
        .and_then(|q| q.get("05. price"))
        .and_then(|p| p.as_str())
        .ok_or("Price not found")?
        .parse()?;

    // Fetch the BALANCE_SHEET
    let balance_url = format!(
        "https://www.alphavantage.co/query?function=BALANCE_SHEET&symbol={}&apikey={}",
        symbol, api_key
    );
    let balance_response = reqwest::get(&balance_url).await?;
    let balance_json: serde_json::Value = balance_response.json().await?;
    save_to_file(&balance_json, "out/balance.json")?;
    let balance_reports = balance_json
        .get("annualReports")
        .and_then(|r| r.as_array())
        .ok_or("Failed to get annual reports")?;

    let total_debt: f64 = balance_reports
        .get(0)
        .and_then(|report| report.get("totalLiabilities"))
        .and_then(|td| td.as_str())
        .ok_or("Total Debt not found")?
        .parse()?;
    let ppe_str = balance_reports
        .get(0)
        .and_then(|report| report.get("propertyPlantEquipment"))
        .and_then(|ppe| ppe.as_str())
        .ok_or("PPE not found")?;
    let ppe: f64 = ppe_str.parse()?;

    // Fetch the INCOME_STATEMENT
    let income_url = format!(
        "https://www.alphavantage.co/query?function=INCOME_STATEMENT&symbol={}&apikey={}",
        symbol, api_key
    );
    let income_response = reqwest::get(&income_url).await?;
    let income_json: serde_json::Value = income_response.json().await?;
    save_to_file(&income_json, "out/income_statement.json")?;
    let annual_reports = income_json
        .get("annualReports")
        .and_then(|r| r.as_array())
        .ok_or("Failed to get annual reports")?;
    let net_income: f64 = annual_reports
        .get(0)
        .and_then(|report| report.get("netIncome"))
        .and_then(|ni| ni.as_str())
        .ok_or("Net Income not found")?
        .parse()?;
    let depreciation: f64 = annual_reports
        .get(0)
        .and_then(|report| report.get("depreciationAndAmortization"))
        .and_then(|d| d.as_str())
        .ok_or("Depreciation not found")?
        .parse()?;
    let ebit: f64 = annual_reports
        .get(0)
        .and_then(|report| report.get("ebit"))
        .and_then(|e| e.as_str())
        .ok_or("EBIT not found")?
        .parse()?;

    // Calculate Free Cash Flow (FCF)
    let free_cash_flow = net_income + depreciation - ppe;

    // Fetch the OVERVIEW
    let overview_url = format!(
        "https://www.alphavantage.co/query?function=OVERVIEW&symbol={}&apikey={}",
        symbol, api_key
    );
    let overview_response = reqwest::get(&overview_url).await?;
    let overview_json: serde_json::Value = overview_response.json().await?;
    save_to_file(&overview_json, "out/overview.json")?;
    let eps: f64 = overview_json
        .get("EPS")
        .and_then(|e| e.as_str())
        .ok_or("EPS not found")?
        .parse()?;
    let pe_ratio = if eps != 0.0 { price / eps } else { 0.0 };

    // Fetch Revenue Growth Rate for PEG Ratio
    let current_revenue: f64 = annual_reports
        .get(0)
        .and_then(|report| report.get("totalRevenue"))
        .and_then(|tr| tr.as_str())
        .ok_or("Total Revenue not found")?
        .parse()?;
    let previous_revenue: f64 = annual_reports
        .get(1)
        .and_then(|report| report.get("totalRevenue"))
        .and_then(|tr| tr.as_str())
        .ok_or("Previous Year Revenue not found")?
        .parse()?;
    let revenue_growth_rate = if previous_revenue != 0.0 {
        (current_revenue - previous_revenue) / previous_revenue
    } else {
        0.0
    };
    let peg_ratio = if revenue_growth_rate != 0.0 {
        pe_ratio / (revenue_growth_rate * 100.0)
    } else {
        f64::INFINITY
    };

    // Create FinancialMetrics struct
    Ok(FinancialMetrics {
        symbol: symbol.to_string(),
        price,
        eps,
        pe_ratio,
        peg_ratio,
        dividend_yield: 0.0, 
        beta: 0.0,           
        book_value: 0.0,     
        market_cap: 0.0,     
        total_debt,
        total_assets: 0.0,          
        total_shareholder_equity: 0.0, 
        current_debt: 0.0,           
        current_long_term_debt: 0.0, 
        goodwill: None,              
        net_income,
        ebit,
        ebitda: 0.0, 
        cost_of_revenue: 0.0, 
        roe: 0.0,             
        debt_to_equity: 0.0,  
        revenue_growth: revenue_growth_rate,
        fcf: free_cash_flow,
        fcf_yield: 0.0, 
    })
}
