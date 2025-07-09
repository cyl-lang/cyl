use super::{Function, ModuleTrait, Type};
use std::collections::HashMap;

pub struct NetModule {
    pub functions: HashMap<String, Function>,
    pub types: HashMap<String, Type>,
}

impl NetModule {
    pub fn new() -> Self {
        let mut functions = HashMap::new();
        functions.insert(
            "get".to_string(),
            Function {
                name: "get".to_string(),
                signature: "fn get(url: string) -> HttpResponse".to_string(),
                description: "Perform an HTTP GET request".to_string(),
                is_builtin: true,
            },
        );
        functions.insert(
            "post".to_string(),
            Function {
                name: "post".to_string(),
                signature: "fn post(url: string, data: string) -> HttpResponse".to_string(),
                description: "Perform an HTTP POST request".to_string(),
                is_builtin: true,
            },
        );
        let mut types = HashMap::new();
        types.insert(
            "HttpResponse".to_string(),
            Type {
                name: "HttpResponse".to_string(),
                description: "HTTP response with status, headers, and body".to_string(),
            },
        );
        NetModule { functions, types }
    }
}

impl Default for NetModule {
    fn default() -> Self {
        Self::new()
    }
}

impl ModuleTrait for NetModule {
    fn name(&self) -> &str {
        "net"
    }
    fn functions(&self) -> &HashMap<String, Function> {
        &self.functions
    }
    fn types(&self) -> &HashMap<String, Type> {
        &self.types
    }
}
