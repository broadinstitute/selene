use std::fmt::{Display, Formatter};
use crate::util::iter_util::fmt_vec;

pub(crate) enum Value {
    String(String),
    Int(i64),
    Float(f64),
    Array(Vec<Value>)
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(string) => { string.fmt(f) }
            Value::Int(int) => { int.fmt(f) }
            Value::Float(float) => { float.fmt(f) }
            Value::Array(values) => { fmt_vec("[", values, "]", f) }
        }
    }
}

