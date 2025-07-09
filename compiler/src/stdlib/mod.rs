pub mod fs;
pub mod math;
pub mod net;
pub mod os;
pub mod string;
// Add more modules as needed

use std::collections::HashMap;

pub struct StdLib {
    pub modules: HashMap<String, Box<dyn ModuleTrait>>,
}

pub trait ModuleTrait {
    #[allow(dead_code)]
    fn name(&self) -> &str;
    fn functions(&self) -> &HashMap<String, Function>;
    #[allow(dead_code)]
    fn types(&self) -> &HashMap<String, Type>;
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct Function {
    pub name: String,
    pub signature: String,
    pub description: String,
    pub is_builtin: bool,
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct Type {
    pub name: String,
    pub description: String,
}

impl StdLib {
    pub fn new() -> Self {
        let mut modules: HashMap<String, Box<dyn ModuleTrait>> = HashMap::new();
        modules.insert("os".to_string(), Box::new(os::OsModule::new()));
        modules.insert("net".to_string(), Box::new(net::NetModule::new()));
        modules.insert("fs".to_string(), Box::new(fs::FsModule::new()));
        modules.insert("math".to_string(), Box::new(math::MathModule::new()));
        modules.insert("string".to_string(), Box::new(string::StringModule::new()));
        // Add more modules as needed
        StdLib { modules }
    }
}

impl Default for StdLib {
    fn default() -> Self {
        Self::new()
    }
}
