use super::{Function, ModuleTrait, Type};
use std::collections::HashMap;

pub struct StringModule {
    pub functions: HashMap<String, Function>,
    pub types: HashMap<String, Type>,
}

impl StringModule {
    pub fn new() -> Self {
        let mut functions = HashMap::new();
        functions.insert(
            "len".to_string(),
            Function {
                name: "len".to_string(),
                signature: "fn len(s: string) -> int".to_string(),
                description: "Length of a string".to_string(),
                is_builtin: true,
            },
        );
        functions.insert(
            "contains".to_string(),
            Function {
                name: "contains".to_string(),
                signature: "fn contains(s: string, substr: string) -> bool".to_string(),
                description: "Check if a string contains a substring".to_string(),
                is_builtin: true,
            },
        );
        StringModule {
            functions,
            types: HashMap::new(),
        }
    }
}

impl ModuleTrait for StringModule {
    fn name(&self) -> &str {
        "string"
    }
    fn functions(&self) -> &HashMap<String, Function> {
        &self.functions
    }
    fn types(&self) -> &HashMap<String, Type> {
        &self.types
    }
}
