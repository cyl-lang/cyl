use std::collections::HashMap;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Array(Vec<Value>),
    Struct(String, HashMap<String, Value>),
    Enum(String, Vec<Value>),       // Enum(variant, fields)
    Result(Box<Value>, Box<Value>), // Ok(val), Err(val)
    #[allow(dead_code)]
    Future(Box<Value>), // For async/await, treat as sync for now
    Void,
}
