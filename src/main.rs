use tokio; // Required for async main
mod api;

#[tokio::main]
async fn main() {
    let stock_symbol = "AAPL"; // Example: Apple stock

    match api::fetch_full_stock_data(stock_symbol).await {
        Ok(stock_data) => {
            println!("Fetched complete stock data:");
            println!("Symbol: {}", stock_data.symbol);
            println!("Price: {:.2}", stock_data.price);
            println!("Net Income: {:.2}", stock_data.net_income);
            println!("Outstanding Shares: (printed internally during API call)");
            println!("EPS (Earnings Per Share): {:.2}", stock_data.eps);
            println!("P/E Ratio: {:.2}", stock_data.pe_ratio);
            println!("Total Debt: {:.2}", stock_data.total_debt);
            // Print other placeholder values or metrics if desired
        }
        Err(e) => eprintln!("Error fetching stock data: {}", e),
    }
}