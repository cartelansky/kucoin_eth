use reqwest;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://api.kucoin.com/api/v1/symbols";
    let response = reqwest::get(url).await?.text().await?;
    let data: Value = serde_json::from_str(&response)?;

    let mut eth_markets: Vec<String> = Vec::new();

    if let Some(symbols) = data["data"].as_array() {
        for symbol in symbols {
            if let (Some(base_currency), Some(quote_currency)) = (
                symbol["baseCurrency"].as_str(),
                symbol["quoteCurrency"].as_str(),
            ) {
                if quote_currency == "ETH" {
                    eth_markets.push(format!("KUCOIN:{}ETH", base_currency));
                }
            }
        }
    }

    eth_markets.sort_by(|a, b| {
        let a_parts: Vec<&str> = a.split(":").collect();
        let b_parts: Vec<&str> = b.split(":").collect();
        let a_name = a_parts[1].trim_end_matches("ETH");
        let b_name = b_parts[1].trim_end_matches("ETH");

        if a_name.chars().next().unwrap().is_numeric()
            && b_name.chars().next().unwrap().is_numeric()
        {
            b_name.cmp(a_name)
        } else if a_name.chars().next().unwrap().is_numeric() {
            std::cmp::Ordering::Less
        } else if b_name.chars().next().unwrap().is_numeric() {
            std::cmp::Ordering::Greater
        } else {
            a_name.cmp(b_name)
        }
    });

    let mut file = File::create("kucoin_eth_markets.txt")?;
    for market in eth_markets {
        writeln!(file, "{}", market)?;
    }

    println!("Veriler başarıyla 'kucoin_eth_markets.txt' dosyasına yazıldı.");
    Ok(())
}
