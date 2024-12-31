use tokio; // Required for async main
mod api;

#[tokio::main]
async fn main() {
    let stock_symbol = "AAPL";

    match api::fetch_full_stock_data(stock_symbol).await {
        Ok(stock_data) => {
            println!("Fetched complete stock data:\n");

            // Format and print core metrics
            println!("Symbol: {}", stock_data.symbol);
            println!("Price: {:.2}", stock_data.price);
            println!("Net Income (M): {:.0}", stock_data.net_income / 1_000_000.0);
            println!("Total Debt (M): {:.0}", stock_data.total_debt / 1_000_000.0);
            println!("EPS (Earnings Per Share): {:.2}", stock_data.eps);
            println!("P/E Ratio: {:.2}", stock_data.pe_ratio);

            // Compute Free Cash Flow (FCF) and related metrics
            let free_cash_flow = stock_data.ebitda - stock_data.cost_of_revenue;
            let fcf_yield = if stock_data.market_cap != 0.0 {
                free_cash_flow / stock_data.market_cap * 100.0
            } else {
                0.0
            };


            // Compute EV/EBIT and EV/EBITDA
            let enterprise_value = stock_data.market_cap + stock_data.total_debt - stock_data.price;
            let ev_ebit = if stock_data.ebit != 0.0 {
                enterprise_value / stock_data.ebit
            } else {
                0.0
            };
            let ev_ebitda = if stock_data.ebitda != 0.0 {
                enterprise_value / stock_data.ebitda
            } else {
                0.0
            };

            // Print calculated metrics
            println!("Free Cash Flow (M): {:.0}", free_cash_flow / 1_000_000.0);
            println!("FCF Yield (%): {:.2}", (free_cash_flow / stock_data.market_cap) * 100.0);
            println!("EV/EBIT: {:.2}", ev_ebit);
            println!("EV/EBITDA: {:.2}", ev_ebitda);

            // Other relevant ratios
            println!("Dividend Yield (%): {:.2}", stock_data.dividend_yield * 100.0);
            println!("Beta: {:.2}", stock_data.beta);
            println!("ROE (%): {:.2}", stock_data.roe * 100.0);
            println!("PEG Ratio: {:.2}", stock_data.peg_ratio);
        }
        Err(e) => eprintln!("Error fetching stock data: {}", e),
    }
}