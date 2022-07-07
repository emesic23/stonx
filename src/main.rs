use yahoo::Quote;
use yahoo_finance_api as yahoo;
use std::time::{UNIX_EPOCH};
use std::error::Error;
use chrono::prelude::*;
use tokio_test;
use plotters::prelude::*;
use log::info;
use tokio::time::{self, interval, Instant, Duration};
use async_trait::async_trait;

pub struct TradingBot {
    pub trading_config: TradingConfig
}

pub struct TradingConfig {
    pub last_operation_price: f64,
    pub dip_threshold: f64,
    pub upward_trend_threshold: f64,
    pub next_operation: State
}

pub enum State {
    BUY,
    SELL
}

impl TradingConfig {
    pub fn new(last_operation_price: f64, dip_threshold: f64, upward_trend_threshold: f64, next_operation: State) -> Self{
        TradingConfig {last_operation_price: last_operation_price, dip_threshold: dip_threshold, upward_trend_threshold: upward_trend_threshold, next_operation: next_operation}
    }
}
#[async_trait]
pub trait Market{
    async fn get_balances(&self) -> Result<f64, Box<dyn Error>>;
    async fn get_market_price(&self, ticker: &str) -> Result<f64, Box<dyn Error>>;
    async fn place_sell_order(&self, amount: f64) -> Result<f64, Box<dyn Error>>;
    async fn place_buy_order(&self, amount: f64) -> Result<f64, Box<dyn Error>>;
}

impl TradingBot{
    pub async fn start(&mut self) -> Result<(), Box<dyn Error>>{
        let curr_price = self.get_market_price("AAPL").await?;
        info!("current market prtice: {:?} &", curr_price);
        println!("current market prtice: {:?} &", curr_price);
        let pct_diff = (curr_price - self.trading_config.last_operation_price) / self.trading_config.last_operation_price * 100 as f64;
        info!("percentage_diff: {:?} &", pct_diff);
        println!("percentage_diff: {:?} &", pct_diff);
        match self.trading_config.next_operation {
            State::BUY => {
                self.trading_config.last_operation_price = self.try_to_buy(pct_diff).await?;
            }
            State::SELL => {
                self.trading_config.last_operation_price = self.try_to_sell(pct_diff).await?;
            }
        }

        Ok(())
    }


    pub fn new(trading_config: TradingConfig) -> Self {
        TradingBot { trading_config: trading_config}
    }

    pub async fn try_to_buy(&mut self, diff: f64) -> Result<f64, Box<dyn Error>> {
        if diff >= self.trading_config.upward_trend_threshold || diff <= self.trading_config.dip_threshold{
            let current_balance = self.get_balances().await?;
            info!("current account balance {:?} & USD", current_balance);
            println!("current account balance {:?} & USD", current_balance);
            self.trading_config.last_operation_price = self.place_buy_order(current_balance).await?;
            self.trading_config.next_operation = State::SELL;
            info!("Bought ? for {:?} & USD", self.trading_config.last_operation_price);
            println!("Bought ? for {:?} & USD", self.trading_config.last_operation_price)
        }
        Ok(self.trading_config.last_operation_price)
    }

    pub async fn try_to_sell(&mut self, diff: f64) -> Result<f64, Box<dyn Error>> {
        if diff >= self.trading_config.upward_trend_threshold || diff <= self.trading_config.dip_threshold{
            let current_balance = self.get_balances().await?;
            info!("current account balance {:?} & USD", current_balance);
            self.trading_config.last_operation_price = self.place_sell_order(current_balance).await?;
            self.trading_config.next_operation = State::SELL;
            info!("Bought ? for {:?} & USD", self.trading_config.last_operation_price);
        }
        Ok(self.trading_config.last_operation_price)
    }
}

#[async_trait]
impl Market for TradingBot {
    async fn get_balances(&self) -> Result<f64, Box<dyn Error>> {
        println!("Place Holder for balances!");
        Ok(500.0)
    }

    async fn get_market_price(&self, ticker: &str) -> Result<f64, Box<dyn Error>>{
        let provider = yahoo::YahooConnector::new();
        let response = provider.get_latest_quotes(ticker, "1m").await.unwrap();
        let quote = response.last_quote().unwrap();
        println!("GOT QUOTE");
        Ok(quote.close)
    }

    async fn place_sell_order(&self, amount: f64) -> Result<f64, Box<dyn Error>> {
        println!("Place Holder for Sell Order");
        Ok(amount)
    }

    async fn place_buy_order(&self, amount: f64) -> Result<f64, Box<dyn Error>>{
        println!("Place Holder for Buy Order");
        Ok(amount)
    }
}

// fn get_history(ticker: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<Quote>{
//     let provider = yahoo::YahooConnector::new();
    
//     // returns historic quotes with daily interval
//     let resp = tokio_test::block_on(provider.get_quote_history(ticker, start, end)).unwrap();
//     let quotes = resp.quotes().unwrap();
//     return quotes;
// }

// pub fn get_curr_price(ticker: &str) -> Quote{
//     let provider = yahoo::YahooConnector::new();
//     let response = tokio_test::block_on(provider.get_latest_quotes(ticker, "1m")).unwrap();
//     let quote = response.last_quote().unwrap();
//     return quote
// }

// fn main() {
//     let start = Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0);
//     let end = Utc.ymd(2020, 1, 31).and_hms_milli(23, 59, 59, 999);
//     let ticker: &str = "AAPL";
//     let quotes: Vec<Quote> = get_history(ticker, start, end);
//     let mut time: Vec<f64> = Vec::new();
//     let mut open: Vec<f64> = Vec::new();
//     for entry in quotes{
//         time.push(entry.timestamp as f64);
//         open.push(entry.open);
//     }
//     let time_open: Vec<(f64, f64)> = time.iter().cloned().zip(open.iter().cloned()).collect();
    
//     let root = BitMapBackend::new("/Users/esmir/stonx/output/test.png", (640, 480)).into_drawing_area();
//     root.fill(&WHITE).unwrap();

//     let mut chart = ChartBuilder::on(&root)
//         .caption("Timestamp vs open price", ("sans-serif", 50).into_font())
//         .margin(5)
//         .x_label_area_size(30)
//         .y_label_area_size(30)
//         .build_cartesian_2d(1577845800.0..1580524200.0, 0.0..300.0)
//         .unwrap();

//     chart.draw_series(
//             time_open.iter().map(|point| Circle::new(*point, 4.0_f64, &BLUE)),).unwrap();

//     println!("Test: {:?}", time_open);
// }
#[tokio::main]
async fn main() {
    let mut config = TradingConfig::new(100.0, 10.0, 15.0, State::BUY);
    let mut bot = TradingBot::new(config);
    let mut interval = interval(Duration::from_secs(10));
    loop {
        interval.tick().await;
        let start = Instant::now();
        let now: Date<Local> = Local::now().date();

        info!("[TRADE] Trade start at {:?}", now);
        println!("[TRADE] Trade start at {:?}", now);
        bot.start().await;
        let duration = start.elapsed();
        info!("[END]end elapsed : {:?}", duration);
        println!("[END]end elapsed : {:?}", duration);
        info!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
    }
}