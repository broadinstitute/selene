use std::fmt::{Display, Formatter};
use crate::util::iter_util::{fmt_vec, fmt_map};
use crate::mion::eval::expressions::Function;
use crate::util::error::Error;
use std::sync::Arc;
use std::collections::HashMap;
use crate::mion::eval::identifier::Identifier;

pub(crate) enum Value {
    Unit,
    String(Arc<String>),
    Int(i64),
    Float(f64),
    Array(Arc<Vec<Value>>),
    Object(HashMap<Identifier, Value>),
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
            Value::Function(function_rc) => {
                Value::Function(function_rc.clone())
            }
            Value::Unit => { Value::Unit }
            Value::Object(hash_map) => { Value::Object(hash_map.clone()) }
        }
    }
}

impl From<&String> for Value {
    fn from(string: &String) -> Self {
        Value::String(Arc::new(string.clone()))
    }
}

impl From<&str> for Value {
    fn from(string: &str) -> Self {
        Value::String(Arc::new(String::from(string)))
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
            Value::Object(hash_map) => {
                fmt_map("{ ", hash_map, " }", f)
            }
        }
    }
}

pub(crate) struct ObjectBuilder {
    map: HashMap<Identifier, Value>,
}

impl ObjectBuilder {
    pub(crate) fn new() -> ObjectBuilder {
        let map = HashMap::<Identifier, Value>::new();
        ObjectBuilder { map }
    }
    pub(crate) fn with_string(self, key: &str, value: &str) -> ObjectBuilder {
        let identifier = Identifier::new(String::from(key));
        let value = Value::from(value);
        let mut map = self.map;
        map.insert(identifier, value);
        ObjectBuilder { map }
    }
    pub(crate) fn into_object(self) -> Value {
        let map = self.map;
        Value::Object(map)
    }
}