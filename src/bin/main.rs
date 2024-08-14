use std::{fs, str::FromStr};
use jsonpath_rust::JsonPath;
use serde_json::Value;
use splitter_wasm::*;

fn main() {
    let data = fs::read_to_string("payload.json").unwrap();
    let json: Value = serde_json::from_str(&data).unwrap();

    let start = chrono::Utc::now();
    let test = splitter(json,
        "$.payload.logs",
        JsonPath::from_str("$.payload.logs")
        .expect("failed in getting jsonpath"));
    println!("{}",test);

    println!("\nfull time: {:?}",(chrono::Utc::now()-start).to_std().unwrap());
    // println!("{}",test);
}