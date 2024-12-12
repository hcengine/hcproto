extern crate serde;

use std::collections::HashMap;
use std::fs::File;

use hcproto::Value;

fn iter_json_to_value(v: serde_json::Value) -> Value {
    match v {
        serde_json::Value::Null => Value::Nil,
        serde_json::Value::Bool(b) => Value::Bool(b),
        serde_json::Value::Number(f) => {
            if f.is_u64() {
                Value::U64(f.as_u64().unwrap())
            } else if f.is_i64() {
                Value::I64(f.as_i64().unwrap())
            } else if f.is_f64() {
                Value::F64(f.as_f64().unwrap())
            } else {
                panic!("unreach!!!");
            }
        }
        serde_json::Value::String(s) => Value::Str(s),
        serde_json::Value::Array(vec) => {
            let mut ret = Vec::<Value>::new();
            for it in vec {
                ret.push(iter_json_to_value(it));
            }
            return Value::Arr(ret);
        }
        serde_json::Value::Object(map) => {
            let mut ret = HashMap::<Value, Value>::new();
            for (key, value) in map {
                ret.insert(Value::Str(key), iter_json_to_value(value));
            }
            return Value::Map(ret);
        }
    }
}

pub fn get_json() -> Value {
    let f = File::open("./examples/level4-full.json").unwrap();
    let v: serde_json::Value = serde_json::from_reader(f).unwrap();
    return iter_json_to_value(v);
}

fn main() {}
