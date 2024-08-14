use std::{fs, str::FromStr};
use chrono;
use jsonpath_rust::JsonPath;
use serde_json::{json, Value};
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
        let result = splitter(data, &self.string_path, self.path.clone());
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
pub fn splitter(json_object: Value, query: &str, path: JsonPath) -> Value {

//getting the object found at the path as json value
    let slice = path.find(&json_object);
    
//checking if the slice is valid
    if slice.as_array().unwrap().is_empty() {
        panic!("Invalid JSONPath: no matching elements found");
    }

//splitting the query in order to be used in the insertion
    let mut depths: Vec<&str> = query.split('.').collect();
    depths.remove(0);
    let f_query = query.split('.').last().expect("Invalid query: empty string");

//getting the slice in the form of an array of json object elements
    let arr = slice[0].as_array()
        .expect("JSONPath result is not an array");

//removing the old queried part and initating the looping on the elements of the sliced array        
    let final_obj: Vec<Value> = arr.iter().map(|item| {
        let mut json_copy = json_object.clone();
        let mut map_copy = json_copy
            .as_object_mut().unwrap();
        for &depth in &depths[..depths.len()-1] {
            map_copy = map_copy.get_mut(depth)
                .and_then(Value::as_object_mut)
                .unwrap();
        }
        map_copy.remove(f_query);
        map_copy.insert(f_query.to_string(), item.clone());
        json_copy
    }).collect();

//finally return the final object in the correct format
    serde_json::Value::Array(final_obj)
}