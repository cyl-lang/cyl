use super::{Function, ModuleTrait, Type};
use std::collections::HashMap;

pub struct OsModule {
    pub functions: HashMap<String, Function>,
    pub types: HashMap<String, Type>,
}

impl OsModule {
    pub fn new() -> Self {
        let mut functions = HashMap::new();
        functions.insert(
            "print".to_string(),
            Function {
                name: "print".to_string(),
                signature: "fn print(message: string) -> void".to_string(),
                description: "Print a message to stdout".to_string(),
                is_builtin: true,
            },
        );
        functions.insert(
            "exit".to_string(),
            Function {
                name: "exit".to_string(),
                signature: "fn exit(code: int) -> void".to_string(),
                description: "Exit the program with a status code".to_string(),
                is_builtin: true,
            },
        );
        OsModule {
            functions,
            types: HashMap::new(),
        }
    }
}

impl Default for OsModule {
    fn default() -> Self {
        Self::new()
    }
}

impl ModuleTrait for OsModule {
    fn name(&self) -> &str {
        "os"
    }
    fn functions(&self) -> &HashMap<String, Function> {
        &self.functions
    }
    fn types(&self) -> &HashMap<String, Type> {
        &self.types
    }
}
