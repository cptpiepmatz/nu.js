use chrono::{DateTime, Utc};
use nu_protocol::Record;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase", tag = "type", content = "value")]
pub enum Value {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Filesize(i64),
    Duration(i64),
    #[serde(with = "serde_wasm_bindgen::preserve")]
    Date(js_sys::Date),
    #[serde(with = "serde_wasm_bindgen::preserve")]
    Record(js_sys::Object),
    List(Vec<Value>),
    #[serde(with = "serde_wasm_bindgen::preserve")]
    Binary(js_sys::Uint8Array),
    Nothing,
    Unsupported, // Glob, Range, Closure, Error, CellPath, Custom
}

impl From<nu_protocol::Value> for Value {
    fn from(value: nu_protocol::Value) -> Self {
        use nu_protocol::Value as NuValue;
        match value {
            NuValue::Bool { val, .. } => Value::Bool(val),
            NuValue::Int { val, .. } => Value::Int(val),
            NuValue::Float { val, .. } => Value::Float(val),
            NuValue::String { val, .. } => Value::String(val),
            NuValue::Glob { .. } => Value::Unsupported,
            NuValue::Filesize { val, .. } => Value::Filesize(val.get()),
            NuValue::Duration { val, .. } => Value::Duration(val),
            NuValue::Date { val, .. } => Value::Date(DateTime::from(val).into()),
            NuValue::Range { .. } => Value::Unsupported,
            NuValue::Record { val, .. } => Value::from_record(val.into_owned()),
            NuValue::List { vals, .. } => Value::List(vals.into_iter().map(Into::into).collect()),
            NuValue::Closure { .. } => Value::Unsupported,
            NuValue::Error { .. } => Value::Unsupported,
            NuValue::Binary { val, .. } => Value::Binary(val.as_slice().into()),
            NuValue::CellPath { .. } => Value::Unsupported,
            NuValue::Custom { .. } => Value::Unsupported,
            NuValue::Nothing { .. } => Value::Nothing,
        }
    }
}

impl Value {
    fn from_record(record: Record) -> Value {
        let entries: Vec<_> = record.into_iter().map(|(key, value)| JsValue::from(vec![
            JsValue::from(key),
            JsValue::from(Value::from(value)),
        ])).collect();
        let entries = JsValue::from(entries);
        let record = js_sys::Object::from_entries(&entries).unwrap();
        Value::Record(record)
    }
}

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(extends = js_sys::Error, js_name = TryFromValueError)]
//     pub type TryFromValueError;

//     #[wasm_bindgen(constructor)]
//     fn new(message: &str) -> TryFromValueError;
// }
