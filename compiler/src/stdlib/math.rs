use super::{Function, ModuleTrait, Type};
use std::collections::HashMap;

pub struct MathModule {
    pub functions: HashMap<String, Function>,
    pub types: HashMap<String, Type>,
}

impl MathModule {
    pub fn new() -> Self {
        let mut functions = HashMap::new();
        functions.insert(
            "sqrt".to_string(),
            Function {
                name: "sqrt".to_string(),
                signature: "fn sqrt(x: float) -> float".to_string(),
                description: "Square root of a number".to_string(),
                is_builtin: true,
            },
        );
        functions.insert(
            "abs".to_string(),
            Function {
                name: "abs".to_string(),
                signature: "fn abs(x: float) -> float".to_string(),
                description: "Absolute value".to_string(),
                is_builtin: true,
            },
        );
        functions.insert(
            "pow".to_string(),
            Function {
                name: "pow".to_string(),
                signature: "fn pow(x: float, y: float) -> float".to_string(),
                description: "Raise x to the power y".to_string(),
                is_builtin: true,
            },
        );
        MathModule {
            functions,
            types: HashMap::new(),
        }
    }
}

impl Default for MathModule {
    fn default() -> Self {
        Self::new()
    }
}

impl ModuleTrait for MathModule {
    fn name(&self) -> &str {
        "math"
    }
    fn functions(&self) -> &HashMap<String, Function> {
        &self.functions
    }
    fn types(&self) -> &HashMap<String, Type> {
        &self.types
    }
}
