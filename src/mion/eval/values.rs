use std::fmt::{Display, Formatter};
use crate::util::iter_util::fmt_vec;
use std::rc::Rc;
use crate::mion::eval::expressions::Function;

pub(crate) enum Value {
    String(Rc<String>),
    Int(i64),
    Float(f64),
    Array(Rc<Vec<Value>>),
    Function(Rc<Box<dyn Function>>),
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Value::String(string_rc) => { Value::String(string_rc.clone()) }
            Value::Int(_) => { self.clone() }
            Value::Float(_) => { self.clone() }
            Value::Array(array_rc) => { Value::Array(array_rc.clone()) }
            Value::Function(function_rc) => { Value::Function(function_rc.clone())}
        }
    }
}

impl From<&String> for Value {
    fn from(string: &String) -> Self {
        Value::String(Rc::new(string.clone()))
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
        }
    }
}

