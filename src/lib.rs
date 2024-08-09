use std::str::FromStr;
use std::time::Instant;
use chrono;
use jsonpath_rust::JsonPath;
use serde_json::{json, Value};
use wasm_bindgen::prelude::*;

#[derive(Debug, Default)]
pub struct IP {
    pub payload: Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metadata: Option<Value>,
    pub channel_id: Option<Value>,
}

impl IP {
    pub fn new() -> IP {
        IP {
            payload: Value::Null,
            timestamp: chrono::Utc::now(),
            metadata: None,
            channel_id: None,
        }
    }
}

#[derive(Default)]
pub struct IPContext {
    pub flow_id: String,
    pub execution_id: String,
    pub execution_start_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Default)]
pub struct ExecutionOutput {
    pub index: i32,
    pub ip: IP,
}

pub trait Node {
    fn get_id(&self) -> String;
    fn get_type(&self) -> String;
    fn get_pin_data(&self) -> Vec<(i32, Value)>;
    fn is_pinned(&self) -> bool;
    fn execute(&self, input: IP, context: Option<IPContext>) -> Option<Vec<ExecutionOutput>> {
        let values = self.n_execute(input, context);
        let mut mappedvalues: Vec<ExecutionOutput> = vec![];
        match values {
            None => return None,
            Some(v) => {
                for item in v {
                    mappedvalues.push(ExecutionOutput {
                        index: item.0,
                        ip: IP {
                            payload: item.1,
                            timestamp: chrono::Utc::now(),
                            metadata: None,
                            channel_id: None
                        }
                    });
                }
                if mappedvalues.is_empty() {
                    None
                } else {
                    Some(mappedvalues)
                }
            }
        }
    }
    fn n_execute(&self,ip: IP, context: Option<IPContext>)->Option<Vec<(i32, Value)>>;
}


#[allow(dead_code)]
pub struct SplitterNode {
    id: String,
    name: String,
    type_name: String,
    pin_data: Vec<(i32, Value)>,
    is_pinned: bool,
    path: String,
}

impl SplitterNode {
    pub fn new(config: String) -> SplitterNode {
        SplitterNode {
            id: "0".to_string(),
            name: "SplitterNode".to_string(),
            type_name: "splitter".to_string(),
            pin_data: vec![(0, json!(""))],
            is_pinned: false,
            path: config,
        }
    }
    
}

// #[wasm_bindgen]
pub fn splitter(mut json_object: Value, query: String) -> Vec<Value>{
    let start = Instant::now();
    let mut final_object: Vec<Value> = Vec::new();
    // let mut json: Value = serde_wasm_bindgen::from_value(json_object).unwrap();
    let path = JsonPath::from_str(&query).unwrap();
    
    let slice_start = Instant::now();
    let slice = path.find(&json_object);
    let slice_time = slice_start.elapsed();
    println!("getting slice {slice_time:?}");

    let query_slice: Vec<&str> = query.split('.').collect();
    let f_query = query_slice[query_slice.len() -1];
    
    if slice.as_array().is_some_and(|x| x.is_empty()) {
        panic!("Invalid JSONPath");
    }

    
    
    // let slice_obj = slice.as_array().unwrap()[0];
    match &slice[0] {
        Value::Array(arr) => {
            // if arr.len() == 0 {
            //     return JsValue::from_str("");
            // }
            // if arr.len() == 1 {
            //     return serde_wasm_bindgen::to_value(&json).unwrap();
            // }
            let loop_start = Instant::now();
            let obj = json_object.get_mut("payload").unwrap();
            obj.as_object_mut().unwrap().remove(f_query);
            for item in arr {
                let insert = Instant::now();
                obj.as_object_mut().unwrap().insert(f_query.to_string(), item.clone());
                println!("inserting took: {:?}",insert.elapsed());
                let pushing = Instant::now();
                let mut new = serde_json::Map::new();
                new.insert("payload".to_string(), obj.clone());
                final_object.push(serde_json::Value::Object(new));
                println!("pushing took: {:?}",pushing.elapsed());
            }
            let loop_end = loop_start.elapsed();
            println!("looping: {:?}",loop_end);
            let end = start.elapsed();
            println!("execution_time: {:?}",end);
            // for item in &final_object{println!("{}", item)}
            return final_object;
        }

        _ => panic!("No array to split")
    }
}

// #[wasm_bindgen]
pub fn test (json_object: Value, query: String) -> i64 {
    let start = chrono::Utc::now();
    for _ in 0..1000 {
        splitter(json_object.clone(), query.clone());
    }
    let end = chrono::Utc::now();
    let execution = end - start;
    execution.num_milliseconds()
}

impl Node for SplitterNode {
    fn get_id(&self) -> String {
        self.id.clone()
    }
    fn get_type(&self) -> String {
        self.type_name.clone()
    }
    fn get_pin_data(&self) -> Vec<(i32, Value)> {
        self.pin_data.clone()
    }
    fn is_pinned(&self) -> bool {
        self.is_pinned
    }
    fn n_execute(&self,ip: IP, _context: Option<IPContext>)->Option<Vec<(i32, Value)>> {
        
        let result = splitter(ip.payload, self.path.clone());
        
        if result.len() == 0 { return None };
        let mut result_map: Vec<(i32, Value)> = Vec::new();
        let mut i = 0;
        for item in result {
            result_map.push((i,item));
            i += 1
        }
        return Some(result_map);
    }
}