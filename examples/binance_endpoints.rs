use binance::api::*;
use binance::savings::*;
use binance::config::*;
use binance::general::*;
use binance::account::*;
use binance::market::*;
use binance::model::KlineSummary;
use binance::errors::ErrorKind as BinanceLibErrorKind;

fn main() {
    // The general spot API endpoints; shown with
    // testnet=false and testnet=true
    general(false);
    general(true);

    // The market data API endpoint
    market_data();

    // The account data API and savings API endpoint examples need an API key. Change those lines locally
    // and uncomment the line below (and do not commit your api key :)).
    //account();
    //savings();
}

const apiKey: &str = "X4BHNSimXOK6RKs2FcKqExquJtHjMxz5hWqF0BBeVnfa5bKFMk7X0wtkfEz0cPrJ";
const secret: &str = "x8gLihunpNq0d46F2q0TWJmeCDahX5LMXSlv3lSFNbMI3rujSOpTDKdhbcmPSf2i";

fn general(use_testnet: bool) {
    let mut client: Account = Binance::new(Some(apiKey.into()), Some(secret.into()));
    client.set_testnet(true);

    let order = client
}

#[allow(dead_code)]
fn savings() {
    let api_key = Some("YOUR_API_KEY".into());
    let api_secret = Some("YOUR_SECRET_KEY".into());

    let savings: Savings = Binance::new(api_key, api_secret);

    match savings.get_all_coins() {
        Ok(answer) => println!("{:#?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match savings.asset_detail(None) {
        Ok(answer) => println!("{:#?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match savings.deposit_address("BTC", None) {
        Ok(answer) => println!("{:#?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }
}

#[allow(dead_code)]
fn market_data() {
    let market: Market = Binance::new(None, None);

    // Order book at default depth
    match market.get_depth("BNBETH") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }
    // Order book at depth 500
    match market.get_custom_depth("BNBETH", 500) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Latest price for ALL symbols
    match market.get_all_prices() {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Latest price for ONE symbol
    match market.get_price("KNCETH") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Current average price for ONE symbol
    match market.get_average_price("KNCETH") {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Best price/qty on the order book for ALL symbols
    match market.get_all_book_tickers() {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Best price/qty on the order book for ONE symbol
    match market.get_book_ticker("BNBETH") {
        Ok(answer) => println!(
            "Bid Price: {}, Ask Price: {}",
            answer.bid_price, answer.ask_price
        ),
        Err(e) => println!("Error: {}", e),
    }

    // 24hr ticker price change statistics
    match market.get_24h_price_stats("BNBETH") {
        Ok(answer) => println!(
            "Open Price: {}, Higher Price: {}, Lower Price: {:?}",
            answer.open_price, answer.high_price, answer.low_price
        ),
        Err(e) => println!("Error: {}", e),
    }

    // 10 latest (aggregated) trades
    match market.get_agg_trades("BNBETH", None, None, None, Some(10)) {
        Ok(trades) => {
            let trade = &trades[0]; // You need to iterate over them
            println!(
                "{} BNB Qty: {}, Price: {}",
                if trade.maker { "SELL" } else { "BUY" },
                trade.qty,
                trade.price
            );
        }
        Err(e) => println!("Error: {}", e),
    }

    // last 10 5min klines (candlesticks) for a symbol:
    match market.get_klines("BNBETH", "5m", 10, None, None) {
        Ok(klines) => {
            match klines {
                binance::model::KlineSummaries::AllKlineSummaries(klines) => {
                    let kline: KlineSummary = klines[0].clone(); // You need to iterate over the klines
                    println!(
                        "Open: {}, High: {}, Low: {}",
                        kline.open, kline.high, kline.low
                    );
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
