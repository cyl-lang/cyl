
//! LanguagePlugin trait for plugin-based extensibility
use std::collections::HashMap;

// Rust-side trait for native plugins (still supported)
pub trait LanguagePlugin {
    fn register_syntax(&self) -> Vec<String> { Vec::new() }
    fn register_types(&self) -> Vec<String> { Vec::new() }
    fn register_functions(&self) -> HashMap<String, Box<dyn Fn(Vec<String>) -> String>> { HashMap::new() }
    fn eval_hook(&self, _expr: &str) -> Option<String> { None }
}

// Python plugin loader using pyo3
use pyo3::prelude::*;
use pyo3::types::{PyModule, PyAny};

pub struct PythonPlugin {
    pub instance: PyObject,
}

impl PythonPlugin {
    pub fn new(py: Python, module_path: &str) -> PyResult<Self> {
        let plugin_module = PyModule::import(py, module_path)?;
        let instance = plugin_module.getattr("LanguagePlugin")?.call0()?;
        Ok(PythonPlugin { instance: instance.into() })
    }

    pub fn register_syntax(&self, py: Python) -> PyResult<Vec<String>> {
        let result = self.instance.call_method0(py, "register_syntax")?;
        result.extract(py)
    }

    pub fn register_types(&self, py: Python) -> PyResult<Vec<String>> {
        let result = self.instance.call_method0(py, "register_types")?;
        result.extract(py)
    }

    pub fn register_functions(&self, py: Python) -> PyResult<HashMap<String, PyObject>> {
        let result = self.instance.call_method0(py, "register_functions")?;
        result.extract(py)
    }

    pub fn eval_hook(&self, py: Python, expr: &str) -> PyResult<Option<String>> {
        let result = self.instance.call_method1(py, "eval_hook", (expr,))?;
        if result.is_none(py) {
            Ok(None)
        } else {
            Ok(Some(result.extract(py)?))
        }
    }
}

// Example usage:
// Python::with_gil(|py| {
//     let plugin = PythonPlugin::new(py, "plugins.example_plugin").unwrap();
//     let syntax = plugin.register_syntax(py).unwrap();
//     println!("Python plugin syntax: {:?}", syntax);
// });
