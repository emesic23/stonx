use yata::prelude::*;
use yata::methods::EMA;


pub struct TradingConfig {
    pub ticker: String,
    pub last_operation_price: f64,
    pub dip_threshold: f64,
    pub upward_trend_threshold: f64,
    pub next_operation: State,
    pub ema: EMA
}

pub enum State {
    BUY,
    SELL
}

impl TradingConfig {
    pub fn new(ticker: String, last_operation_price: f64, dip_threshold: f64, upward_trend_threshold: f64, next_operation: State) -> Self{
        let mut ema = EMA::new(10, &last_operation_price).unwrap();
        TradingConfig {ticker, last_operation_price: last_operation_price, dip_threshold: dip_threshold, upward_trend_threshold: upward_trend_threshold, next_operation: next_operation, ema: ema}
    }
}


