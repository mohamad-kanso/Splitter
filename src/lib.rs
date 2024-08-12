use std::{fs, str::FromStr};
// use std::time::Instant;
use chrono;
use jsonpath_rust::JsonPath;
use serde_json::{json, Map, Value};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::console_log;

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

#[wasm_bindgen]
pub fn splitter1(json_object: JsValue, query: String) -> JsValue {
    let start = chrono::Utc::now();
    let mut json_object = serde_wasm_bindgen::from_value(json_object).unwrap();
    let mut final_object: Vec<Value> = Vec::new();
    // let mut json: Value = serde_wasm_bindgen::from_value(json_object).unwrap();
    let path = JsonPath::from_str(&query).unwrap();

    let slice_start = chrono::Utc::now();
    let slice = path.find_slice(&json_object);
    let slice_time = chrono::Utc::now() - slice_start;
    console_log!("getting slice {:?}",slice_time.to_std().unwrap());

    let query_slice: Vec<&str> = query.split('.').collect();
    let f_query = query_slice[query_slice.len() - 1];

    if slice.len() == 0 {
        panic!("Invalid JSONPath");
    }

    let slice_obj = slice[0].clone().to_data();
    match slice_obj {
        Value::Array(arr) => {
            // if arr.len() == 0 {
            //     return JsValue::from_str("");
            // }
            // if arr.len() == 1 {
            //     return serde_wasm_bindgen::to_value(&json).unwrap();
            // }
            let loop_start = chrono::Utc::now();
            let obj = json_object.get_mut("payload").unwrap();
            obj.as_object_mut().unwrap().remove(f_query);
            let object_map = obj.as_object().unwrap();

            for item in arr {
                // let n = json_object.clone();
                let insert = chrono::Utc::now();
                let mut n = object_map.clone();
                n.insert(f_query.to_string(), vec![item].into());
                console_log!("inserting took: {:?}", (chrono::Utc::now() - insert).to_std().unwrap());
                // let pushing: Instant = Instant::now();

                final_object.push(serde_json::Value::Object(n));
                // println!("pushing took: {:?}", pushing.elapsed());
            }
            console_log!("looping: {:?}", (chrono::Utc::now() - loop_start).to_std().unwrap());
            // let end = start.elapsed();
            console_log!("execution_time: {:?}", (chrono::Utc::now() - start).to_std().unwrap());
            serde_wasm_bindgen::to_value(&final_object).unwrap()
        }

        _ => panic!("No array to split"),
    }
}

#[wasm_bindgen]
pub fn test1 (json_object: JsValue, query: String) -> i64 {
    let start = chrono::Utc::now();
    for _ in 0..1000 {
        splitter1(json_object.clone(), query.clone());
    }
    let end = chrono::Utc::now();
    let execution = end - start;
    execution.num_milliseconds()
}

#[wasm_bindgen]
pub fn test2 (json_object: JsValue, query: String) -> i64 {
    let start = chrono::Utc::now();
    // console_log!("this is test2 start time: {}",start);
    for _ in 0..1000 {
        splitter2(json_object.clone(), &query);
    }
    let end = chrono::Utc::now();
    let execution = end - start;
    console_log!("loop execution time took: {:?}",execution.to_std().unwrap());
    execution.num_milliseconds()
}

pub fn get_inputs() -> (Value,String) {
    let data = fs::read_to_string("payload.json").unwrap();
    let json: Value = serde_json::from_str(&data).unwrap();
    let query = "$.payload.logs".to_string();
    (json, query)
}

#[wasm_bindgen]
pub fn splitter2(json_object: JsValue, query: &str) -> JsValue {
    let start = chrono::Utc::now();
    let mut json_object = serde_wasm_bindgen::from_value(json_object).unwrap();
    console_log!("serializing: {:?}",(chrono::Utc::now() - start).to_std().unwrap());
    let path = JsonPath::from_str(query).expect("Invalid JSONPath");
    console_log!("path: {:?}",(chrono::Utc::now() - start).to_std().unwrap());
    let slice = path.find(&json_object);
    console_log!("getting slice {:?}", (chrono::Utc::now() - start).to_std().unwrap());
    
    if slice.as_array().unwrap().is_empty() {
        panic!("Invalid JSONPath: no matching elements found");
    }

    let f_query = query.split('.').last().expect("Invalid query: empty string");
    console_log!("splitting: {:?}",(chrono::Utc::now() - start).to_std().unwrap());
    
    let payload = json_object.get_mut("payload")
        .and_then(Value::as_object_mut)
        .expect("Invalid JSON structure: 'payload' not found or not an object");
    console_log!("getting payload {:?}", (chrono::Utc::now() - start).to_std().unwrap());

    let arr = slice[0].as_array()
        .expect("JSONPath result is not an array");

    payload.remove(f_query);
    let final_obj: Vec<Value> = arr.iter().map(|item| {
        let mut new_map = payload.clone();
        new_map.insert(f_query.to_string(), item.clone());
        console_log!("inserting took {:?}", (chrono::Utc::now() - start).to_std().unwrap());

        Value::Object(Map::from_iter([
            ("payload".to_string(), Value::Object(new_map))
        ]))
    }).collect();
    console_log!("looping: {:?}",(chrono::Utc::now() - start).to_std().unwrap());
    // console_log!("{:?}",final_obj);
    let final_obj = serde_wasm_bindgen::to_value(&final_obj).unwrap();
    console_log!("wasm_bindgen: {:?}", (chrono::Utc::now() - start).to_std().unwrap());
    console_log!("execution: {:?}", (chrono::Utc::now() - start).to_std().unwrap());
    final_obj
}
// impl Node for SplitterNode {
//     fn get_id(&self) -> String {
//         self.id.clone()
//     }
//     fn get_type(&self) -> String {
//         self.type_name.clone()
//     }
//     fn get_pin_data(&self) -> Vec<(i32, Value)> {
//         self.pin_data.clone()
//     }
//     fn is_pinned(&self) -> bool {
//         self.is_pinned
//     }
//     fn n_execute(&self,ip: IP, _context: Option<IPContext>)->Option<Vec<(i32, Value)>> {
        
//         let result = splitter(ip.payload, self.path.clone());
        
//         if result.len() == 0 { return None };
//         let mut result_map: Vec<(i32, Value)> = Vec::new();
//         let mut i = 0;
//         for item in result {
//             result_map.push((i,item));
//             i += 1
//         }
//         return Some(result_map);
//     }
// }