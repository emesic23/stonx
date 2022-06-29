use yahoo::Quote;
use yahoo_finance_api as yahoo;
use std::time::{Duration, UNIX_EPOCH};
use chrono::prelude::*;
use tokio_test;
use plotters::prelude::*;

fn get_history(ticker: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<Quote>{
    let provider = yahoo::YahooConnector::new();
    
    // returns historic quotes with daily interval
    let resp = tokio_test::block_on(provider.get_quote_history(ticker, start, end)).unwrap();
    let quotes = resp.quotes().unwrap();
    return quotes;
}

fn main() {
    let start = Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0);
    let end = Utc.ymd(2020, 1, 31).and_hms_milli(23, 59, 59, 999);
    let ticker: &str = "AAPL";
    let quotes: Vec<Quote> = get_history(ticker, start, end);
    let mut time: Vec<f64> = Vec::new();
    let mut open: Vec<f64> = Vec::new();
    for entry in quotes{
        time.push(entry.timestamp as f64);
        open.push(entry.open);
    }
    let time_open: Vec<(f64, f64)> = time.iter().cloned().zip(open.iter().cloned()).collect();
    
    let root = BitMapBackend::new("/Users/esmir/stonx/output/test.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Timestamp vs open price", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(1577845800.0..1580524200.0, 0.0..300.0)
        .unwrap();

    chart.draw_series(
            time_open.iter().map(|point| Circle::new(*point, 4.0_f64, &BLUE)),).unwrap();

    println!("Test: {:?}", time_open);
}