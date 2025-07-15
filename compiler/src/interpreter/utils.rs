use crate::ast::*;
use super::Value;

pub fn value_to_string(val: &Value) -> String {
    match val {
        Value::Int(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::String(s) => s.clone(),
        Value::Bool(b) => b.to_string(),
        Value::Array(elements) => {
            let inner = elements.iter().map(value_to_string).collect::<Vec<_>>().join(", ");
            format!("[{}]", inner)
        }
        Value::Struct(name, fields) => {
            let mut s = format!("{} {{ ", name);
            for (k, v) in fields {
                s.push_str(&format!("{}: {}, ", k, value_to_string(v)));
            }
            s.push('}');
            s
        }
        Value::Enum(name, fields) => {
            let mut s = format!("{}(", name);
            for v in fields {
                s.push_str(&format!("{}, ", value_to_string(v)));
            }
            s.push(')');
            s
        }
        Value::Result(ok, err) => format!("Ok({}), Err({})", value_to_string(ok), value_to_string(err)),
        Value::Future(inner) => format!("Future({})", value_to_string(inner)),
        Value::Void => "<void>".to_string(),
    }
}
