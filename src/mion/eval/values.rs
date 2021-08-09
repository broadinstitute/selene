use std::fmt::{Display, Formatter};
use crate::util::iter_util::fmt_vec;
use crate::mion::eval::expressions::Function;
use crate::util::error::Error;
use std::sync::Arc;

pub(crate) enum Value {
    Unit,
    String(Arc<String>),
    Int(i64),
    Float(f64),
    Array(Arc<Vec<Value>>),
    Function(Arc<Box<dyn Function + Send + Sync>>),
}

impl Value {
    pub(crate) fn as_string(&self) -> Result<String, Error> {
        if let Value::String(string_rc) = self {
            Ok(string_rc.as_ref().clone())
        } else {
            Err(Error::from(format!("Value {} is not a string value.", self)))
        }
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Value::String(string_rc) => { Value::String(string_rc.clone()) }
            Value::Int(int) => { Value::Int(*int) }
            Value::Float(float) => { Value::Float(*float) }
            Value::Array(array_rc) => { Value::Array(array_rc.clone()) }
            Value::Function(function_rc) => { Value::Function(function_rc.clone())}
            Value::Unit => { Value::Unit }
        }
    }
}

impl From<&String> for Value {
    fn from(string: &String) -> Self {
        Value::String(Arc::new(string.clone()))
    }
}

impl From<&i64> for Value {
    fn from(int: &i64) -> Self {
        Value::Int(*int)
    }
}

impl From<&f64> for Value {
    fn from(float: &f64) -> Self {
        Value::Float(*float)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(string) => { string.fmt(f) }
            Value::Int(int) => { int.fmt(f) }
            Value::Float(float) => { float.fmt(f) }
            Value::Array(values) => { fmt_vec("[", values, "]", f) }
            Value::Function(function) => { function.id().fmt(f) }
            Value::Unit => { "unit".fmt(f) }
        }
    }
}

