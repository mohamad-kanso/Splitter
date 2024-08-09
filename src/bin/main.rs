use std::fs;

use serde_json::Value;
use splitter_wasm::*;


fn main() {
    let data = fs::read_to_string("payload.json").unwrap();

    let json: Value = serde_json::from_str(&data).unwrap();
    // let start = chrono::Utc::now();
    let test = test(json,"$.payload.logs".to_string());
    println!("\nfull time: {:?}ms",test);
    // let end = chrono::Utc::now();

    // let execution = end - start;
    // println!("execution time in rust is: {}ms",test);
    
}