use chrono::DateTime;
use nu_protocol::{Record, Value as NuValue};
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

use crate::UnsupportedValueError;

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

impl From<NuValue> for Value {
    fn from(value: NuValue) -> Self {
        match value {
            NuValue::Bool { val, .. } => Value::Bool(val),
            NuValue::Int { val, .. } => Value::Int(val),
            NuValue::Float { val, .. } => Value::Float(val),
            NuValue::String { val, .. } => Value::String(val),
            NuValue::Glob { .. } => Value::Unsupported,
            NuValue::Filesize { val, .. } => Value::Filesize(val.get()),
            NuValue::Duration { val, .. } => Value::Duration(val),
            NuValue::Date { val, .. } => todo!(),
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
        let entries: Vec<_> = record
            .into_iter()
            .map(|(key, value)| {
                JsValue::from(vec![JsValue::from(key), JsValue::from(Value::from(value))])
            })
            .collect();
        let entries = JsValue::from(entries);
        let record = js_sys::Object::from_entries(&entries).unwrap();
        Value::Record(record)
    }
}

impl TryFrom<Value> for NuValue {
    type Error = UnsupportedValueError;

    fn try_from(value: Value) -> Result<Self, UnsupportedValueError> {
        let span = nu_protocol::Span::unknown();
        Ok(match value {
            Value::Bool(val) => NuValue::bool(val, span),
            Value::Int(val) => NuValue::int(val, span),
            Value::Float(val) => NuValue::float(val, span),
            Value::String(val) => NuValue::string(val, span),
            Value::Filesize(val) => NuValue::filesize(val, span),
            Value::Duration(val) => NuValue::duration(val, span),
            Value::Date(val) => todo!(),
            Value::Record(_) => todo!(),
            Value::List(vals) => NuValue::list(
                vals.into_iter()
                    .map(|val| NuValue::try_from(val))
                    .collect::<Result<Vec<_>, _>>()?,
                span,
            ),
            Value::Binary(_) => todo!(),
            Value::Nothing => NuValue::nothing(span),
            Value::Unsupported => return Err(UnsupportedValueError::new("stuff".to_string())),
        })
    }
}
