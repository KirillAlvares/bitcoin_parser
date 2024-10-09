use reqwest::{blocking::*, header::USER_AGENT};
use scraper::{Html, Selector};
use csv::Writer;
use std::{fs::File, time::SystemTime, time::UNIX_EPOCH};


const CUSTOM_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36";

fn main() {
    let url = &format!("http://finance.yahoo.com/quote/BTC-USD/history/?period1=1410912000&period2={}&frequency=1d",
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis());

    let responce = Client::new()
        .get(url)
        .header(USER_AGENT, CUSTOM_USER_AGENT)
        .send()
        .unwrap()
        .text()
        .unwrap();

    let mut row_data = Html::parse_document(&responce)
        .select(&Selector::parse("tr.yf-ewueuo").unwrap())
        .map(|el| {
            el
                .text()
                .filter_map(|s|{
                    let s = s.trim().replace(",", "");
                    if !s.is_empty(){ Some(s) } else { None }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let rows_data_first = row_data.get_mut(0).unwrap();
    rows_data_first.remove(5);
    rows_data_first.remove(7);

    let mut writer = Writer::from_writer(File::create("data.csv").unwrap());
    for row in &row_data {
        writer.write_record(row).unwrap();
    }
    writer.flush().unwrap();
}
