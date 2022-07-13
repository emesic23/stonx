
use std::time::{UNIX_EPOCH};
use tokio::time::{self, interval, Instant, Duration};
use alpaca_finance::{Account, Alpaca, TimeInForce, OrderType, Order, Streamer, StreamMessage};
use futures::{future, StreamExt};

mod bot;
mod trading_config;


#[tokio::main]
async fn main() {
    let config = trading_config::TradingConfig::new("AAPL".to_string(), 100.0, 10.0, 15.0, trading_config::State::BUY);
    let alpaca = Alpaca::paper("PKY2MI2RBM9DZTSQ4C4G", "1O9p52rxxkbuydmg30x58l8sMYX157d0ToTFSbIK").await.unwrap();
    let bot = bot::TradingBot::new(config, alpaca);

    let streamer = Streamer::new(&bot.alpaca);

    // TODO: Make macro for getting Order from stream
    streamer.start().await.for_each(|msg|{
        match msg{
            StreamMessage::Account(m) => println!("Got an account update!"),
            StreamMessage::Order(m) => println!("GOt Order update"),
            _ => println!("Got an unexpected msg")
        }
        future::ready(())
    }).await;
    // let mut interval = interval(Duration::from_secs(60));
    // loop {
    //     interval.tick().await;
    //     let start = Instant::now();
    //     let now: Date<Local> = Local::now().date();

    //     info!("[TRADE] Trade start at {:?}", now);
    //     println!("[TRADE] Trade start at {:?}", now);
    //     bot.start().await;
    //     let duration = start.elapsed();
    //     info!("[END]end elapsed : {:?}", duration);
    //     println!("[END]end elapsed : {:?}", duration);
    //     info!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
    // }

    
}