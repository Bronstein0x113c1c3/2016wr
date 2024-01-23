use std::vec;

use ureq;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
struct Data<'a>{
    TradingDate: &'a str,
    StockCode: &'a str,
    ClosePrice: f64,
    Change: f64,
    PerChange: f64,
    TotalVol: f64,
    Row: i64,
}


fn main() {
    let resp = ureq::get("https://api.vietstock.vn//finance/toptrading?type=7&catID=1")
        .set("Host", "api.vietstock.vn")
        .set("Referer", "https://finance.vietstock.vn/")
        .set("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36 Edg/121.0.0.0")
        .set("Connection", "close")
        .call()
        .unwrap();
    let mut res = resp.into_string().unwrap();
    // println!("{}",res.as_str());
    // let res = serde_json::from_str(res.as_str());
    let mut p:Vec<Data> = serde_json::from_str(res.as_str()).unwrap();
    for x in p{
        println!("{:?}", x);
    }
    }
