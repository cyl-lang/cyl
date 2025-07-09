use super::{Function, ModuleTrait, Type};
use std::collections::HashMap;

pub struct FsModule {
    pub functions: HashMap<String, Function>,
    pub types: HashMap<String, Type>,
}

impl FsModule {
    pub fn new() -> Self {
        let mut functions = HashMap::new();
        functions.insert(
            "read".to_string(),
            Function {
                name: "read".to_string(),
                signature: "fn read(path: string) -> string".to_string(),
                description: "Read file contents as string".to_string(),
                is_builtin: true,
            },
        );
        functions.insert(
            "write".to_string(),
            Function {
                name: "write".to_string(),
                signature: "fn write(path: string, content: string) -> void".to_string(),
                description: "Write string to file".to_string(),
                is_builtin: true,
            },
        );
        functions.insert(
            "exists".to_string(),
            Function {
                name: "exists".to_string(),
                signature: "fn exists(path: string) -> bool".to_string(),
                description: "Check if file or directory exists".to_string(),
                is_builtin: true,
            },
        );
        FsModule {
            functions,
            types: HashMap::new(),
        }
    }
}

impl Default for FsModule {
    fn default() -> Self {
        Self::new()
    }
}

impl ModuleTrait for FsModule {
    fn name(&self) -> &str {
        "fs"
    }
    fn functions(&self) -> &HashMap<String, Function> {
        &self.functions
    }
    fn types(&self) -> &HashMap<String, Type> {
        &self.types
    }
}
