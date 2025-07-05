/// Standard library modules and functions for the Cyl programming language
use std::collections::HashMap;

pub struct StdLib {
    modules: HashMap<String, Module>,
}

#[allow(dead_code)] // Fields will be used in future development
pub struct Module {
    pub name: String,
    pub functions: HashMap<String, Function>,
    pub types: HashMap<String, Type>,
}

#[allow(dead_code)] // Fields will be used in future development
pub struct Function {
    pub name: String,
    pub signature: String,
    pub description: String,
    pub is_builtin: bool,
}

#[allow(dead_code)] // Fields will be used in future development
pub struct Type {
    pub name: String,
    pub description: String,
}

impl StdLib {
    pub fn new() -> Self {
        let mut stdlib = Self {
            modules: HashMap::new(),
        };

        stdlib.add_core_modules();
        stdlib
    }

    fn add_core_modules(&mut self) {
        // OS module
        let mut os_module = Module {
            name: "os".to_string(),
            functions: HashMap::new(),
            types: HashMap::new(),
        };

        os_module.functions.insert(
            "print".to_string(),
            Function {
                name: "print".to_string(),
                signature: "fn print(message: string) -> void".to_string(),
                description: "Print a message to stdout".to_string(),
                is_builtin: true,
            },
        );

        os_module.functions.insert(
            "exit".to_string(),
            Function {
                name: "exit".to_string(),
                signature: "fn exit(code: int) -> void".to_string(),
                description: "Exit the program with a status code".to_string(),
                is_builtin: true,
            },
        );

        self.modules.insert("os".to_string(), os_module);

        // Network module
        let mut net_module = Module {
            name: "net".to_string(),
            functions: HashMap::new(),
            types: HashMap::new(),
        };

        net_module.functions.insert(
            "get".to_string(),
            Function {
                name: "get".to_string(),
                signature: "fn get(url: string) -> HttpResponse".to_string(),
                description: "Perform an HTTP GET request".to_string(),
                is_builtin: true,
            },
        );

        net_module.functions.insert(
            "post".to_string(),
            Function {
                name: "post".to_string(),
                signature: "fn post(url: string, data: string) -> HttpResponse".to_string(),
                description: "Perform an HTTP POST request".to_string(),
                is_builtin: true,
            },
        );

        // HTTP Response type
        net_module.types.insert(
            "HttpResponse".to_string(),
            Type {
                name: "HttpResponse".to_string(),
                description: "HTTP response with status, headers, and body".to_string(),
            },
        );

        self.modules.insert("net".to_string(), net_module);

        // File system module
        let mut fs_module = Module {
            name: "fs".to_string(),
            functions: HashMap::new(),
            types: HashMap::new(),
        };

        fs_module.functions.insert(
            "read".to_string(),
            Function {
                name: "read".to_string(),
                signature: "fn read(path: string) -> string".to_string(),
                description: "Read file contents as string".to_string(),
                is_builtin: true,
            },
        );

        fs_module.functions.insert(
            "write".to_string(),
            Function {
                name: "write".to_string(),
                signature: "fn write(path: string, content: string) -> void".to_string(),
                description: "Write string to file".to_string(),
                is_builtin: true,
            },
        );

        fs_module.functions.insert(
            "exists".to_string(),
            Function {
                name: "exists".to_string(),
                signature: "fn exists(path: string) -> bool".to_string(),
                description: "Check if file or directory exists".to_string(),
                is_builtin: true,
            },
        );

        self.modules.insert("fs".to_string(), fs_module);
    }

    #[allow(dead_code)] // Methods will be used in future development
    pub fn get_module(&self, name: &str) -> Option<&Module> {
        self.modules.get(name)
    }

    #[allow(dead_code)] // Methods will be used in future development
    pub fn get_function(&self, module: &str, function: &str) -> Option<&Function> {
        self.modules.get(module)?.functions.get(function)
    }

    #[allow(dead_code)] // Methods will be used in future development
    pub fn list_modules(&self) -> Vec<&str> {
        self.modules.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for StdLib {
    fn default() -> Self {
        Self::new()
    }
}
