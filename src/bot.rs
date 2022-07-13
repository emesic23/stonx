use crate::trading_config::{TradingConfig, State};
use alpaca_finance::{Account, Alpaca, TimeInForce, OrderType, Order, Streamer, StreamMessage};
use std::error::Error;

use std::time::{UNIX_EPOCH};
use log::info;
use async_trait::async_trait;
// use alpaca_finance::{Account, Alpaca, TimeInForce, OrderType, Order, Streamer, StreamMessage};
use futures::{future, StreamExt};
use yahoo_finance_api as yahoo;
use yahoo::Quote;

pub struct TradingBot {
    pub trading_config: TradingConfig,
    pub alpaca: Alpaca
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


    pub fn new(trading_config: TradingConfig, alpaca: Alpaca) -> Self {
        TradingBot { trading_config: trading_config, alpaca: alpaca}
    }

    pub async fn try_to_buy(&mut self, diff: f64) -> Result<f64, Box<dyn Error>> {
        if diff >= self.trading_config.upward_trend_threshold || diff <= self.trading_config.dip_threshold{
            let current_balance = self.get_balances().await?;
            info!("current account balance {:?} & USD", current_balance);
            println!("current account balance {:?} & USD", current_balance);

            let amount = 5;
            let limit:f64 = 130.0;
            self.trading_config.last_operation_price = self.place_buy_order(self.trading_config.ticker.as_str(), amount, limit).await?;
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

            let amount = 5;
            let limit:f64 = 130.0;
            self.trading_config.last_operation_price = self.place_sell_order(self.trading_config.ticker.as_str(), amount, limit).await?;
            self.trading_config.next_operation = State::SELL;
            info!("Bought ? for {:?} & USD", self.trading_config.last_operation_price);
        }
        Ok(self.trading_config.last_operation_price)
    }
}


#[async_trait]
pub trait Market{
    async fn get_balances(&self) -> Result<f64, Box<dyn Error>>;
    async fn get_market_price(&self, ticker: &str) -> Result<f64, Box<dyn Error>>;
    async fn place_sell_order(&self, ticker: &str, amount: u32, price: f64) -> Result<f64, Box<dyn Error>>;
    async fn place_buy_order(&self, ticker: &str, amount: u32, price: f64) -> Result<f64, Box<dyn Error>>;
    async fn get_history(&self, ticker: &str, period: &str, duration: &str) -> Result<Vec<Quote>, Box<dyn Error>>;
}


#[async_trait]
impl Market for TradingBot {
    async fn get_balances(&self) -> Result<f64, Box<dyn Error>> {
        let account = Account::get(&self.alpaca).await.unwrap();
        println!("I have ${:} in my account.", account.cash);

        Ok(account.cash)
    }

    async fn get_market_price(&self, ticker: &str) -> Result<f64, Box<dyn Error>>{
        let provider = yahoo_finance_api::YahooConnector::new();
        let response = provider.get_latest_quotes(ticker, "1m").await.unwrap();
        let quote = response.last_quote().unwrap();
        println!("GOT QUOTE");
        quote.
        Ok(quote.close)
    }

    async fn place_sell_order(&self, ticker: &str, amount: u32, price: f64) -> Result<f64, Box<dyn Error>> {
        let order = Order::sell(ticker, amount, OrderType::Limit, TimeInForce::DAY).limit_price(price).place(&self.alpaca).await.unwrap();
        Ok(order.filled_avg_price.unwrap())
    }

    async fn place_buy_order(&self, ticker: &str, amount: u32, price: f64) -> Result<f64, Box<dyn Error>>{
        let order = Order::buy(ticker, amount, OrderType::Limit, TimeInForce::DAY).limit_price(price).place(&self.alpaca).await.unwrap();
        Ok(order.filled_avg_price.unwrap())
    }

    async fn get_history(&self, ticker: &str, period: &str, duration: &str) -> Result<Vec<Quote>, Box<dyn Error>> {
        let provider = yahoo::YahooConnector::new();
        let response = provider.get_quote_range(ticker, period, duration).await.unwrap();
        let quotes = response.quotes().unwrap();
        Ok(quotes)
    }
}