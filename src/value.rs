use chrono::{DateTime, Utc};
use wasm_bindgen::prelude::*;
use nu_protocol::Value as NuValue;

#[wasm_bindgen(inspectable, getter_with_clone)]
#[derive(Debug, Clone)]
pub struct Value {
    pub kind: ValueKind,
    pub value: JsValue,
    pub span: Span,
}

#[wasm_bindgen(js_class = Value)]
impl Value {
    #[wasm_bindgen]
    pub fn some_string() -> Value {
        Value {
            kind: ValueKind::String,
            value: "lol".into(),
            span: Span {
                start: 0,
                end: 3,
            }
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueKind {
    Bool = "bool",
    Int = "int",
    Float = "float",
    String = "string",
    Glob = "glob",
    Filesize = "filesize",
    Duration = "duration",
    Date = "date",
    Range = "range",
    Record = "record",
    List = "list",
    Closure = "closure",
    Error = "error",
    Binary = "binary",
    CellPath = "cell path",
    Custom = "custom",
    Nothing = "nothing",
}

#[wasm_bindgen(inspectable)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl From<nu_protocol::Span> for Span {
    fn from(value: nu_protocol::Span) -> Self {
        Span {
            start: value.start,
            end: value.end,
        }
    }
}

impl From<NuValue> for Value {
    fn from(value: NuValue) -> Self {
        match value {
            NuValue::Bool { val, internal_span } => 
                Value::from_value_bool(val, internal_span),

            NuValue::Int { val, internal_span } => 
                Value::from_value_int(val, internal_span),

            NuValue::Float { val, internal_span } => 
                Value::from_value_float(val, internal_span),

            NuValue::String { val, internal_span } => 
                Value::from_value_string(val, internal_span),

            NuValue::Filesize { val, internal_span } => 
                Value::from_value_filesize(val, internal_span),

            NuValue::Duration { val, internal_span } => 
                Value::from_value_duration(val, internal_span),

            NuValue::Date { val, internal_span } => 
                Value::from_value_date(val, internal_span),

            NuValue::List { vals, internal_span } => 
                Value::from_value_list(vals.into_iter().map(Into::into).collect(), internal_span),

            _ => todo!("Handle other NuValue variants"),
        }
    }
}

impl Value {
    #[inline]
    fn from_value_bool(val: bool, span: nu_protocol::Span) -> Value {
        Value {
            kind: ValueKind::Bool,
            value: val.into(),
            span: span.into(),
        }
    }

    #[inline]
    fn from_value_int(val: i64, span: nu_protocol::Span) -> Value {
        Value {
            kind: ValueKind::Int,
            value: val.into(),
            span: span.into(),
        }
    }

    #[inline]
    fn from_value_float(val: f64, span: nu_protocol::Span) -> Value {
        Value {
            kind: ValueKind::Float,
            value: val.into(),
            span: span.into(),
        }
    }

    #[inline]
    fn from_value_string(val: String, span: nu_protocol::Span) -> Value {
        Value {
            kind: ValueKind::String,
            value: val.into(),
            span: span.into(),
        }
    }

    #[inline]
    fn from_value_filesize(val: nu_protocol::Filesize, span: nu_protocol::Span) -> Value {
        Value {
            kind: ValueKind::Filesize,
            value: val.get().into(),
            span: span.into(),
        }
    }

    #[inline]
    fn from_value_duration(val: i64, span: nu_protocol::Span) -> Value {
        Value {
            kind: ValueKind::Duration,
            value: val.into(),
            span: span.into(),
        }
    }

    #[inline]
    fn from_value_date(val: chrono::DateTime<chrono::FixedOffset>, span: nu_protocol::Span) -> Value {
        let date = DateTime::<Utc>::from(val);
        let date = js_sys::Date::from(date);
        let date = JsValue::from(date);
        Value {
            kind: ValueKind::Date,
            value: date,
            span: span.into(),
        }
    }

    #[inline]
    fn from_value_list(vals: Vec<Value>, span: nu_protocol::Span) -> Value {
        Value {
            kind: ValueKind::List,
            value: vals.into(),
            span: span.into(),
        }
    }
}
