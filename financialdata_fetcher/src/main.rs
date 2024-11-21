use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, time::Duration, thread};
use ureq;

const ALPHA_VANTAGE_API_KEY: &str = "P5UNQNXTCY87YLC6"; 

// Structs for each asset
#[derive(Serialize, Deserialize, Debug)]
struct Bitcoin {
    symbol: String,
    price: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Ethereum {
    symbol: String,
    price: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct SP500 {
    symbol: String,
    price: f64,
}

// Pricing trait definition
trait Pricing {
    fn fetch_price(&mut self);
    fn save_to_file(&self);
}

// Implementing Pricing trait for Bitcoin
impl Pricing for Bitcoin {
    fn fetch_price(&mut self) {
        let url = "https://api.coindesk.com/v1/bpi/currentprice/BTC.json";
        let response: String = ureq::get(url).call().unwrap().into_string().unwrap();
        let json: serde_json::Value = serde_json::from_str(&response).unwrap();
        self.price = json["bpi"]["USD"]["rate_float"].as_f64().unwrap();
        self.symbol = "BTC".to_string();
    }

    fn save_to_file(&self) {
        let mut file = File::create("bitcoin_price.txt").unwrap();
        writeln!(file, "Symbol: {}, Price: ${:.2}", self.symbol, self.price).unwrap();
    }
}

// Implementing Pricing trait for Ethereum
impl Pricing for Ethereum {
    fn fetch_price(&mut self) {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
        let response: String = ureq::get(url).call().unwrap().into_string().unwrap();
        let json: serde_json::Value = serde_json::from_str(&response).unwrap();
        self.price = json["ethereum"]["usd"].as_f64().unwrap();
        self.symbol = "ETH".to_string();
    }

    fn save_to_file(&self) {
        let mut file = File::create("ethereum_price.txt").unwrap();
        writeln!(file, "Symbol: {}, Price: ${:.2}", self.symbol, self.price).unwrap();
    }
}

// Implementing Pricing trait for SP500
impl Pricing for SP500 {
    fn fetch_price(&mut self) {
        let url = format!(
            "https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol=SPY&interval=1min&apikey={}",
            ALPHA_VANTAGE_API_KEY
        );
        let response: String = ureq::get(&url).call().unwrap().into_string().unwrap();
        let json: serde_json::Value = serde_json::from_str(&response).unwrap();
        self.price = json["Time Series (1min)"].as_object().unwrap().iter().nth(0).unwrap().1["1. open"].as_str().unwrap().parse().unwrap();
        self.symbol = "SP500".to_string();
    }

    fn save_to_file(&self) {
        let mut file = File::create("sp500_price.txt").unwrap();
        writeln!(file, "Symbol: {}, Price: ${:.2}", self.symbol, self.price).unwrap();
    }
}

// Main function
fn main() {
    let mut bitcoin = Bitcoin { symbol: String::new(), price: 0.0 };
    let mut ethereum = Ethereum { symbol: String::new(), price: 0.0 };
    let mut sp500 = SP500 { symbol: String::new(), price: 0.0 };

    let assets: &mut Vec<&mut dyn Pricing> = &mut vec![&mut bitcoin, &mut ethereum, &mut sp500];

    loop {
        for asset in assets.iter_mut() {
            asset.fetch_price();
            asset.save_to_file();
        }
        println!("Prices fetched and saved. Waiting for 10 seconds...");
        thread::sleep(Duration::from_secs(10));
    }
}
