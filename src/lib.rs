use std::{fs, str::FromStr};
use chrono;
use jsonpath_rust::JsonPath;
use serde_json::{json, Map, Value};
use serde_wasm_bindgen::from_value;
use serde_wasm_bindgen::Error;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
// use wasm_bindgen_test::console_log;

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

#[allow(dead_code)]
#[wasm_bindgen]
pub struct SplitterNode {
    id: String,
    name: String,
    type_name: String,
    pin_data: Vec<(i32, Value)>,
    is_pinned: bool,
    string_path: String,
    path: JsonPath,
}

#[wasm_bindgen]
impl SplitterNode {
    #[wasm_bindgen(constructor)]
    pub fn new(config: &str) -> SplitterNode {
        SplitterNode {
            id: "0".to_string(),
            name: "SplitterNode".to_string(),
            type_name: "splitter".to_string(),
            pin_data: vec![(0, json!(""))],
            is_pinned: false,
            string_path: config.to_string(), 
            path: JsonPath::from_str(config).expect("failed in getting jsonpath"),
        }
    }

    fn _execute(&self, data: Value) -> Option<Vec<(i32,Value)>> {
        let result = splitter2(data, &self.string_path, self.path.clone());
        if result.as_array().unwrap().is_empty() {
            return None;
        }
        let mut result_map = Vec::new();
        for item in result.as_array().unwrap().clone() {
            result_map.push((0 as i32,item))
        }
        Some(result_map)
    }

    #[wasm_bindgen]
    pub fn execute(&self, data: JsValue) -> Result<JsValue,Error> {
        let value: Value = from_value(data).map_err(|err| JsValue::from_str(&err.to_string()))?;
        let result = self._execute(value);
        match result {
            Some(result) => return serde_wasm_bindgen::to_value(&result),
            None => return serde_wasm_bindgen::to_value("")
        }
    }
    
}

pub fn get_inputs() -> (Value,String) {
    let data = fs::read_to_string("payload.json").unwrap();
    let json: Value = serde_json::from_str(&data).unwrap();
    let query = "$.payload.logs".to_string();
    (json, query)
}

// #[wasm_bindgen]
pub fn splitter2(mut json_object: Value, query: &str, path: JsonPath) -> Value {
    // let start = chrono::Utc::now();

//getting the object found at the path as json value
    let slice = path.find(&json_object);
    // console_log!("getting slice {:?}", (chrono::Utc::now() - start).to_std().unwrap());
    
//checking if the slice is valid
    if slice.as_array().unwrap().is_empty() {
        panic!("Invalid JSONPath: no matching elements found");
    }
    // console_log!("checking slice: {:?}",(chrono::Utc::now()-start).to_std().unwrap());

//splitting the query in order to be used in the insertion
    let f_query = query.split('.').last().expect("Invalid query: empty string");
    // console_log!("splitting: {:?}",(chrono::Utc::now() - start).to_std().unwrap());
    
//getting the payload which is going to be modified
    let payload = json_object.get_mut("payload")
        .and_then(Value::as_object_mut)
        .expect("Invalid JSON structure: 'payload' not found or not an object");
    // console_log!("getting payload {:?}", (chrono::Utc::now() - start).to_std().unwrap());

//getting the slice in the form of an array of json object elements
    let arr = slice[0].as_array()
        .expect("JSONPath result is not an array");

//removing the old queried part and initating the looping on the elements of the sliced array        
    payload.remove(f_query);
    let final_obj: Vec<Value> = arr.iter().map(|item| {
        let mut new_map = payload.clone();
        new_map.insert(f_query.to_string(), item.clone());
        // console_log!("inserting took {:?}", (chrono::Utc::now() - start).to_std().unwrap());

        Value::Object(Map::from_iter([
            ("payload".to_string(), Value::Object(new_map))
        ]))
    }).collect();
    // console_log!("looping: {:?}",(chrono::Utc::now() - start).to_std().unwrap());
    // console_log!("wasm_bindgen: {:?}", (chrono::Utc::now() - start).to_std().unwrap());
    // console_log!("execution: {:?}", (chrono::Utc::now() - start).to_std().unwrap());
    serde_json::Value::Array(final_obj)
}