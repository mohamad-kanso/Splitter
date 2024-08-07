use std::{fs, time::Instant};

use serde_json::Value;
use splitter_wasm::*;


fn main() {
    let start = Instant::now();
    let data = fs::read_to_string("payload.json").unwrap();

    let json: Value = serde_json::from_str(&data).unwrap();
    // let start = chrono::Utc::now();
    let _test = splitter(json,"$.payload.logs".to_string());
    let end = start.elapsed();
    println!("\nfull time: {:?}",end);
    // let end = chrono::Utc::now();

    // let execution = end - start;
    // println!("execution time in rust is: {}ms",test);
    
}