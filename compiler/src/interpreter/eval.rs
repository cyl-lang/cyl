use crate::ast::*;
use std::collections::HashMap;
use super::{Value, value_to_string, StdLibWrapper};
use std::io::Write;

pub struct Interpreter {
    pub variables: HashMap<String, Value>,
    pub stdlib: StdLibWrapper,
    pub output_buffer: Vec<String>, // Captures printed output for tests
}

impl Interpreter {

    fn eval_expression(&mut self, expr: &Expression) -> Value {
        use std::fs::OpenOptions;
        use std::io::Write as IoWrite;
        let cyl_debug_enabled = std::env::var("CYL_DEBUG_LOG").is_ok();
        if cyl_debug_enabled {
            if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("./cyl_debug.log") {
                writeln!(f, "[debug] eval_expression: expr = {:?}", expr).ok();
            }
        }
        match expr {
            Expression::IntLiteral(i) => Value::Int(*i),
            Expression::FloatLiteral(f) => Value::Float(*f),
            Expression::StringLiteral(s) => Value::String(s.clone()),
            Expression::BoolLiteral(b) => Value::Bool(*b),
            Expression::Identifier(name) => {
                let val = self.variables.get(name).cloned().unwrap_or(Value::Void);
                if cyl_debug_enabled {
                    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("./cyl_debug.log") {
                        writeln!(f, "[debug] Identifier lookup: {} => {:?}", name, val).ok();
                    }
                }
                if cyl_debug_enabled {
                    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("./cyl_debug.log") {
                        writeln!(f, "[debug] Identifier: {} => {:?}", name, val).ok();
                    }
                }
                val
            },
            Expression::ArrayLiteral(items) => {
                let elements = items.iter().map(|item| self.eval_expression(item)).collect();
                Value::Array(elements)
            }
            Expression::IndexAccess { object, index } => {
                if cyl_debug_enabled {
                    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("./cyl_debug.log") {
                        writeln!(f, "[debug] IndexAccess: evaluating object").ok();
                    }
                }
                let arr_val = self.eval_expression(object);
                if cyl_debug_enabled {
                    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("./cyl_debug.log") {
                        writeln!(f, "[debug] IndexAccess: object evaluated to {:?}", arr_val).ok();
                        writeln!(f, "[debug] IndexAccess: evaluating index").ok();
                    }
                }
                let idx_val = self.eval_expression(index);
                if cyl_debug_enabled {
                    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("./cyl_debug.log") {
                        writeln!(f, "[debug] IndexAccess: index evaluated to {:?}", idx_val).ok();
                    }
                }
                match (&arr_val, &idx_val) {
                    (Value::Array(elements), Value::Int(idx)) if *idx >= 0 => {
                        let result = elements.get(*idx as usize).cloned().unwrap_or(Value::Void);
                        if cyl_debug_enabled {
                            if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("./cyl_debug.log") {
                                writeln!(f, "[debug] IndexAccess: performing array lookup at index {}, result = {:?}", idx, result).ok();
                            }
                        }
                        result
                    }
                    _ => {
                        if cyl_debug_enabled {
                            if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("./cyl_debug.log") {
                                writeln!(f, "[debug] IndexAccess: arr_val={:?}, idx_val={:?}", arr_val, idx_val).ok();
                            }
                        }
                        Value::Void
                    }
                }
            }
            Expression::BinaryOp { left, operator, right } => {
                let l = self.eval_expression(left);
                let r = self.eval_expression(right);
                match operator {
                    BinaryOperator::Add => match (l, r) {
                        (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
                        (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
                        (Value::String(a), Value::String(b)) => Value::String(a + &b),
                        (Value::String(a), b) => Value::String(a + &value_to_string(&b)),
                        (a, Value::String(b)) => Value::String(value_to_string(&a) + &b),
                        _ => Value::Void,
                    },
                    BinaryOperator::Subtract => match (l, r) {
                        (Value::Int(a), Value::Int(b)) => Value::Int(a - b),
                        (Value::Float(a), Value::Float(b)) => Value::Float(a - b),
                        _ => Value::Void,
                    },
                    BinaryOperator::Multiply => match (l, r) {
                        (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
                        (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
                        _ => Value::Void,
                    },
                    BinaryOperator::Divide => match (l, r) {
                        (Value::Int(a), Value::Int(b)) if b != 0 => Value::Int(a / b),
                        (Value::Float(a), Value::Float(b)) if b != 0.0 => Value::Float(a / b),
                        _ => Value::Void,
                    },
                    BinaryOperator::Equal => match (l, r) {
                        (Value::Int(a), Value::Int(b)) => Value::Bool(a == b),
                        (Value::Float(a), Value::Float(b)) => Value::Bool((a - b).abs() < f64::EPSILON),
                        (Value::String(a), Value::String(b)) => Value::Bool(a == b),
                        (Value::Bool(a), Value::Bool(b)) => Value::Bool(a == b),
                        _ => Value::Bool(false),
                    },
                    BinaryOperator::NotEqual => match (l, r) {
                        (Value::Int(a), Value::Int(b)) => Value::Bool(a != b),
                        (Value::Float(a), Value::Float(b)) => Value::Bool((a - b).abs() >= f64::EPSILON),
                        (Value::String(a), Value::String(b)) => Value::Bool(a != b),
                        (Value::Bool(a), Value::Bool(b)) => Value::Bool(a != b),
                        _ => Value::Bool(true),
                    },
                    BinaryOperator::Less => match (l, r) {
                        (Value::Int(a), Value::Int(b)) => Value::Bool(a < b),
                        (Value::Float(a), Value::Float(b)) => Value::Bool(a < b),
                        _ => Value::Bool(false),
                    },
                    BinaryOperator::Greater => match (l, r) {
                        (Value::Int(a), Value::Int(b)) => Value::Bool(a > b),
                        (Value::Float(a), Value::Float(b)) => Value::Bool(a > b),
                        _ => Value::Bool(false),
                    },
                    BinaryOperator::LessEqual => match (l, r) {
                        (Value::Int(a), Value::Int(b)) => Value::Bool(a <= b),
                        (Value::Float(a), Value::Float(b)) => Value::Bool(a <= b),
                        _ => Value::Bool(false),
                    },
                    BinaryOperator::GreaterEqual => match (l, r) {
                        (Value::Int(a), Value::Int(b)) => Value::Bool(a >= b),
                        (Value::Float(a), Value::Float(b)) => Value::Bool(a >= b),
                        _ => Value::Bool(false),
                    },
                    _ => Value::Void,
                }
            }
            Expression::Assignment { target, value } => {
                let val = self.eval_expression(value);
                if let Expression::Identifier(var_name) = target.as_ref() {
                    if cyl_debug_enabled {
                        if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("./cyl_debug.log") {
                            writeln!(f, "[debug] Assignment: {} = {:?}", var_name, val).ok();
                        }
                    }
                    self.variables.insert(var_name.clone(), val.clone());
                    val
                } else {
                    Value::Void
                }
            }
            Expression::Call { callee, arguments } => {
                if let Expression::Identifier(func_name) = &**callee {
                    let args: Vec<Value> = arguments.iter().map(|a| self.eval_expression(a)).collect();
                    match func_name.as_str() {
                        "print" | "println" => {
                            if let Some(val) = args.first() {
                                match val {
                                    Value::Array(elements) => {
                                        let mut first = true;
                                        for elem in elements {
                                            if !first {
                                                self.output_buffer.push("".to_string());
                                            }
                                            let s = value_to_string(&elem);
                                            self.output_buffer.push(s);
                                            first = false;
                                        }
                                    }
                                    _ => {
                                        let s = value_to_string(val);
                                        self.output_buffer.push(s);
                                    }
                                }
                            } else if func_name == "println" {
                                self.output_buffer.push(String::new());
                            }
                            return Value::Void;
                        }
                        "print_int" => {
                            if let Some(val) = args.first() {
                                let s = match val {
                                    Value::Int(i) => i.to_string(),
                                    _ => value_to_string(val),
                                };
                                self.output_buffer.push(s);
                            }
                            return Value::Void;
                        }
                        _ => {}
                    }
                }
                Value::Void
            }
            Expression::ObjectLiteral(map) => {
                let mut fields = HashMap::new();
                for (k, v) in map.iter() {
                    fields.insert(k.clone(), self.eval_expression(v));
                }
                Value::Struct("<anon>".to_string(), fields)
            }
            _ => Value::Void,
        }
    }

    fn pattern_matches(&self, _pattern: &Pattern, _val: &Value) -> bool {
        true
    }
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            stdlib: StdLibWrapper::new(),
            output_buffer: Vec::new(),
        }
    }
    // ...existing run and run_main...


    fn eval_block(&mut self, block: &BlockStatement) -> Result<(), String> {
        if std::env::var("CYL_DEBUG_LOG").is_ok() {
            if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("./cyl_debug.log") {
                writeln!(f, "[debug] eval_block: Block statements: {:?}", block.statements).ok();
            }
        }
        for stmt in &block.statements {
            if std::env::var("CYL_DEBUG_LOG").is_ok() {
                if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("./cyl_debug.log") {
                    writeln!(f, "[debug] eval_block: Executing statement: {:?}", stmt).ok();
                }
            }
            match stmt {
                Statement::Return(ret) => {
                    if let Some(expr) = &ret.value {
                        self.eval_expression(expr);
                    }
                    return Ok(());
                }
                Statement::Declare(_) | Statement::Expression(_) | Statement::Match(_) | Statement::If(_) | Statement::While(_) | Statement::For(_) | Statement::Block(_) => {
                    self.eval_statement(stmt);
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn eval_statement_with_diagnostics(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::Declare(decl) => {
                // Evaluate the right-hand side expression and assign to variable
                let val = self.eval_expression(&decl.value);
                if std::env::var("CYL_DEBUG_LOG").is_ok() {
                    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("./cyl_debug.log") {
                        writeln!(f, "[debug] Declare: {} = {:?}", decl.name, val).ok();
                    }
                }
                self.variables.insert(decl.name.clone(), val.clone());
                Ok(())
            }
            Statement::Expression(expr) => {
                self.eval_expression(expr);
                Ok(())
            }
            Statement::If(if_stmt) => {
                let condition = self.eval_expression(&if_stmt.condition);
                let is_true = match condition {
                    Value::Bool(b) => b,
                    Value::Int(i) => i != 0,
                    Value::Float(f) => f != 0.0,
                    Value::String(s) => !s.is_empty(),
                    Value::Void => false,
                    _ => true,
                };
                if is_true {
                    self.eval_block(&if_stmt.then_block)?;
                } else if let Some(else_block) = &if_stmt.else_block {
                    self.eval_statement_with_diagnostics(else_block)?;
                }
                Ok(())
            }
            Statement::While(while_stmt) => {
                loop {
                    let condition = self.eval_expression(&while_stmt.condition);
                    let is_true = match condition {
                        Value::Bool(b) => b,
                        Value::Int(i) => i != 0,
                        Value::Float(f) => f != 0.0,
                        Value::String(s) => !s.is_empty(),
                        Value::Void => false,
                        _ => true,
                    };
                    if !is_true {
                        break;
                    }
                    self.eval_block(&while_stmt.body)?;
                }
                Ok(())
            }
            Statement::Match(m) => {
                let val = self.eval_expression(&m.expression);
                for arm in &m.arms {
                    if self.pattern_matches(&arm.pattern, &val) {
                        self.eval_block(&arm.body).ok();
                        break;
                    }
                }
                Ok(())
            }
            Statement::For(for_stmt) => {
                let cyl_debug_enabled = std::env::var("CYL_DEBUG_LOG").is_ok();
                use std::fs::OpenOptions;
                use std::io::Write as IoWrite;
                if cyl_debug_enabled {
                    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("./cyl_debug.log") {
                        writeln!(f, "[debug] Entering for-loop: variable = {}, iterable = {:?}", for_stmt.variable, for_stmt.iterable).ok();
                    }
                }
                let iterable = self.eval_expression(&for_stmt.iterable);
                if cyl_debug_enabled {
                    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("./cyl_debug.log") {
                        writeln!(f, "[debug] For-loop iterable evaluated to: {:?}", iterable).ok();
                    }
                }
                match iterable {
                    Value::Int(n) if n > 0 => {
                        for i in 0..n {
                            if cyl_debug_enabled {
                                if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("./cyl_debug.log") {
                                    writeln!(f, "[debug] For-loop iteration: {} = {}", for_stmt.variable, i).ok();
                                }
                            }
                            self.variables.insert(for_stmt.variable.clone(), Value::Int(i));
                            self.eval_block(&for_stmt.body)?;
                        }
                    }
                    Value::Array(arr) => {
                        for (i, elem) in arr.iter().enumerate() {
                            if cyl_debug_enabled {
                                if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("./cyl_debug.log") {
                                    writeln!(f, "[debug] For-loop array iteration: {} = {}, {}_value = {:?}", for_stmt.variable, i, for_stmt.variable, elem).ok();
                                }
                            }
                            self.variables.insert(for_stmt.variable.clone(), Value::Int(i as i64));
                            self.variables.insert(format!("{}_value", for_stmt.variable), elem.clone());
                            self.eval_block(&for_stmt.body)?;
                        }
                    }
                    _ => {}
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn eval_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Declare(decl) => {
                // Evaluate the right-hand side expression and assign to variable
                let val = self.eval_expression(&decl.value);
                if std::env::var("CYL_DEBUG_LOG").is_ok() {
                    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("./cyl_debug.log") {
                        writeln!(f, "[debug] Declare: {} = {:?}", decl.name, val).ok();
                    }
                }
                self.variables.insert(decl.name.clone(), val.clone());
            }
            Statement::Expression(expr) => {
                self.eval_expression(expr);
            }
            Statement::If(if_stmt) => {
                let condition = self.eval_expression(&if_stmt.condition);
                let is_true = match condition {
                    Value::Bool(b) => b,
                    Value::Int(i) => i != 0,
                    Value::Float(f) => f != 0.0,
                    Value::String(s) => !s.is_empty(),
                    Value::Void => false,
                    _ => true,
                };
                if is_true {
                    self.eval_block(&if_stmt.then_block).ok();
                } else if let Some(else_block) = &if_stmt.else_block {
                    self.eval_statement(else_block);
                }
            }
            Statement::While(while_stmt) => {
                loop {
                    let condition = self.eval_expression(&while_stmt.condition);
                    let is_true = match condition {
                        Value::Bool(b) => b,
                        Value::Int(i) => i != 0,
                        Value::Float(f) => f != 0.0,
                        Value::String(s) => !s.is_empty(),
                        Value::Void => false,
                        _ => true,
                    };
                    if !is_true {
                        break;
                    }
                    self.eval_block(&while_stmt.body).ok();
                }
            }
            Statement::Match(m) => {
                let val = self.eval_expression(&m.expression);
                for arm in &m.arms {
                    if self.pattern_matches(&arm.pattern, &val) {
                        self.eval_block(&arm.body).ok();
                        break;
                    }
                }
            }
            Statement::For(for_stmt) => {
                let cyl_debug_enabled = std::env::var("CYL_DEBUG_LOG").is_ok();
                use std::fs::OpenOptions;
                use std::io::Write as IoWrite;
                if cyl_debug_enabled {
                    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("./cyl_debug.log") {
                        writeln!(f, "[debug] Entering for-loop: variable = {}, iterable = {:?}", for_stmt.variable, for_stmt.iterable).ok();
                    }
                }
                let iterable = self.eval_expression(&for_stmt.iterable);
                if cyl_debug_enabled {
                    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("./cyl_debug.log") {
                        writeln!(f, "[debug] For-loop iterable evaluated to: {:?}", iterable).ok();
                    }
                }
                match iterable {
                    Value::Int(n) if n > 0 => {
                        for i in 0..n {
                            if cyl_debug_enabled {
                                if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("./cyl_debug.log") {
                                    writeln!(f, "[debug] For-loop iteration: {} = {}", for_stmt.variable, i).ok();
                                }
                            }
                            self.variables.insert(for_stmt.variable.clone(), Value::Int(i));
                            self.eval_block(&for_stmt.body).ok();
                        }
                    }
                    Value::Array(arr) => {
                        for (i, elem) in arr.iter().enumerate() {
                            if cyl_debug_enabled {
                                if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("./cyl_debug.log") {
                                    writeln!(f, "[debug] For-loop array iteration: {} = {}, {}_value = {:?}", for_stmt.variable, i, for_stmt.variable, elem).ok();
                                }
                            }
                            self.variables.insert(for_stmt.variable.clone(), Value::Int(i as i64));
                            self.variables.insert(format!("{}_value", for_stmt.variable), elem.clone());
                            self.eval_block(&for_stmt.body).ok();
                        }
                    }
                    _ => {}
                }
            }
            Statement::Function(_) => {}
            Statement::Return(_) => {}
            _ => {}
        }
    }

    fn infer_parameter_types(&mut self, _func: &FunctionDeclaration) -> Result<(), String> {
        Ok(())
    }

    pub fn run(&mut self, program: &Program) -> Result<(), String> {
        let mut functions = std::collections::HashMap::new();
        for stmt in &program.statements {
            if let Statement::Function(func) = stmt {
                if func.is_async {
                    return Err("Async functions are not yet implemented".to_string());
                }
                if let Err(e) = self.infer_parameter_types(func) {
                    eprintln!("[error] {e}");
                }
                functions.insert(func.name.clone(), func);
            } else {
                if let Err(e) = self.eval_statement_with_diagnostics(stmt) {
                    eprintln!("[error] {e}");
                }
            }
        }
        if let Some(main_func) = functions.get("main") {
            self.eval_block(&main_func.body).ok();
        }
        Ok(())
    }

    pub fn run_main(&mut self, program: &Program) -> Result<(), String> {
        let main_fn = program.statements.iter().find_map(|stmt| {
            if let Statement::Function(func) = stmt {
                if func.name == "main" {
                    Some(func)
                } else {
                    None
                }
            } else {
                None
            }
        });
        let main_fn = match main_fn {
            Some(f) => f,
            None => return Err("No 'main' function found".to_string()),
        };
        self.eval_block(&main_fn.body)
    }

    // ... Place all methods from the previous impl Interpreter here, updating stdlib usage ...
    // For brevity, only the struct and new() are shown. All methods should be moved here and updated to use separated modules.
}
