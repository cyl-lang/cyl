use crate::ast::*;
use crate::stdlib;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Struct(String, HashMap<String, Value>),
    Enum(String, Vec<Value>),       // Enum(variant, fields)
    Result(Box<Value>, Box<Value>), // Ok(val), Err(val)
    #[allow(dead_code)]
    Future(Box<Value>), // For async/await, treat as sync for now
    Void,
}

pub struct Interpreter {
    variables: HashMap<String, Value>,
    stdlib: stdlib::StdLib,
}

#[allow(dead_code)]
impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            stdlib: stdlib::StdLib::new(),
        }
    }

    pub fn run(&mut self, program: &Program) -> Result<(), String> {
        // Store function definitions
        let mut functions = HashMap::new();
        
        for stmt in &program.statements {
            if let Statement::Function(func) = stmt {
                // Check for unsupported features
                if func.is_async {
                    return Err("Async functions are not yet implemented".to_string());
                }
                
                if let Err(e) = self.infer_parameter_types(func) {
                    eprintln!("[error] {e}");
                }
                if let Err(e) = self.check_function_return_type(func) {
                    eprintln!("[error] {e}");
                }
                functions.insert(func.name.clone(), func);
            } else {
                if let Err(e) = self.eval_statement_with_diagnostics(stmt) {
                    eprintln!("[error] {e}");
                }
            }
        }
        
        // If there's a main function, execute it
        if let Some(main_func) = functions.get("main") {
            self.eval_block(&main_func.body).ok();
        }
        
        Ok(())
    }

    fn infer_parameter_types(&mut self, func: &FunctionDeclaration) -> Result<(), String> {
        // For each parameter with Type::Infer, try to infer from usage in the function body
        for param in func.parameters.iter() {
            if let Type::Infer = param.param_type {
                let mut inferred_types = vec![];
                self.collect_param_usages(&func.body, &param.name, &mut inferred_types);
                inferred_types.sort();
                inferred_types.dedup();
                if inferred_types.is_empty() {
                    return Err(format!(
                        "Could not infer type for parameter '{}' in function '{}' (no usage found)",
                        param.name, func.name
                    ));
                } else if inferred_types.len() > 1 {
                    return Err(format!(
                        "Ambiguous type for parameter '{}' in function '{}': inferred types: {:?}",
                        param.name, func.name, inferred_types
                    ));
                } else {
                    // Would update the parameter type here if mutable; for now, just check
                }
            }
        }
        Ok(())
    }

    fn collect_param_usages(
        &mut self,
        block: &BlockStatement,
        param: &str,
        found: &mut Vec<String>,
    ) {
        for stmt in &block.statements {
            match stmt {
                Statement::Expression(expr) => {
                    self.infer_type_from_expr(expr, param, found);
                }
                Statement::Declare(decl) => {
                    self.infer_type_from_expr(&decl.value, param, found);
                }
                Statement::Return(ret) => {
                    if let Some(expr) = &ret.value {
                        self.infer_type_from_expr(expr, param, found);
                    }
                }
                Statement::If(ifstmt) => {
                    self.infer_type_from_expr(&ifstmt.condition, param, found);
                    self.collect_param_usages(&ifstmt.then_block, param, found);
                    if let Some(else_block) = &ifstmt.else_block {
                        self.collect_param_usages_from_stmt(else_block, param, found);
                    }
                }
                _ => {}
            }
        }
    }

    fn collect_param_usages_from_stmt(
        &mut self,
        stmt: &Statement,
        param: &str,
        found: &mut Vec<String>,
    ) {
        if let Statement::If(ifstmt) = stmt {
            self.infer_type_from_expr(&ifstmt.condition, param, found);
            self.collect_param_usages(&ifstmt.then_block, param, found);
            if let Some(else_block) = &ifstmt.else_block {
                self.collect_param_usages_from_stmt(else_block, param, found);
            }
        }
    }

    fn infer_type_from_expr(&mut self, expr: &Expression, param: &str, found: &mut Vec<String>) {
        match expr {
            Expression::Identifier(name) if name == param => {
                // If used as a bare identifier, type is ambiguous (could be anything)
            }
            Expression::BinaryOp {
                left,
                operator: _,
                right,
            } => {
                if let Expression::Identifier(name) = &**left {
                    if name == param {
                        // Infer from right
                        let ty = self.infer_value_type(right);
                        if ty != "unknown" {
                            found.push(ty);
                        }
                    }
                }
                if let Expression::Identifier(name) = &**right {
                    if name == param {
                        // Infer from left
                        let ty = self.infer_value_type(left);
                        if ty != "unknown" {
                            found.push(ty);
                        }
                    }
                }
                self.infer_type_from_expr(left, param, found);
                self.infer_type_from_expr(right, param, found);
            }
            Expression::Call { callee, arguments } => {
                for arg in arguments {
                    self.infer_type_from_expr(arg, param, found);
                }
                self.infer_type_from_expr(callee, param, found);
            }
            Expression::Assignment { target, value } => {
                if let Expression::Identifier(name) = &**target {
                    if name == param {
                        let ty = self.infer_value_type(value);
                        if ty != "unknown" {
                            found.push(ty);
                        }
                    }
                }
                self.infer_type_from_expr(target, param, found);
                self.infer_type_from_expr(value, param, found);
            }
            _ => {}
        }
    }

    fn infer_value_type(&mut self, expr: &Expression) -> String {
        match self.eval_expression(expr) {
            Value::Int(_) => "int".to_string(),
            Value::Float(_) => "float".to_string(),
            Value::String(_) => "string".to_string(),
            Value::Bool(_) => "bool".to_string(),
            Value::Void => "void".to_string(),
            Value::Struct(_, _) => "struct".to_string(),
            Value::Enum(_, _) => "enum".to_string(),
            _ => "unknown".to_string(),
        }
    }

    pub fn run_main(&mut self, program: &Program) -> Result<(), String> {
        // Find the main function
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
        // Execute the main function body
        self.eval_block(&main_fn.body)
    }

    /// Evaluate a block statement (list of statements)
    fn eval_block(&mut self, block: &BlockStatement) -> Result<(), String> {
        for stmt in &block.statements {
            match stmt {
                Statement::Return(ret) => {
                    // For now, just stop execution on return
                    if let Some(expr) = &ret.value {
                        self.eval_expression(expr);
                    }
                    return Ok(());
                }
                Statement::Declare(_) | Statement::Expression(_) | Statement::Match(_) | Statement::If(_) | Statement::While(_) => {
                    self.eval_statement(stmt);
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn eval_statement_with_diagnostics(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::Declare(decl) => {
                // Type inference: if var_type is None, try to infer from value
                let val = self.eval_expression(&decl.value);
                if decl.var_type.is_none() {
                    let inferred = match &val {
                        Value::Int(_) => "int",
                        Value::Float(_) => "float",
                        Value::String(_) => "string",
                        Value::Bool(_) => "bool",
                        Value::Struct(_, _) => "struct",
                        Value::Enum(_, _) => "enum",
                        Value::Void => "void",
                        _ => "unknown",
                    };
                    if inferred == "unknown" {
                        return Err(format!(
                            "Type inference failed for variable '{}' (initializer could not be inferred)",
                            decl.name
                        ));
                    }
                }
                self.variables.insert(decl.name.clone(), val);
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
            _ => Ok(()),
        }
    }

    fn eval_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Declare(decl) => {
                let val = self.eval_expression(&decl.value);
                self.variables.insert(decl.name.clone(), val);
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
            Statement::Function(_) => {}
            Statement::Return(_) => {}
            _ => {}
        }
    }

    fn eval_expression(&mut self, expr: &Expression) -> Value {
        match expr {
            Expression::IntLiteral(i) => Value::Int(*i),
            Expression::FloatLiteral(f) => Value::Float(*f),
            Expression::StringLiteral(s) => Value::String(s.clone()),
            Expression::BoolLiteral(b) => Value::Bool(*b),
            Expression::MemberAccess { object, property } => {
                let obj_val = self.eval_expression(object);
                match obj_val {
                    Value::Struct(_, ref fields) => {
                        fields.get(property).cloned().unwrap_or(Value::Void)
                    }
                    Value::Enum(ref variant, ref vals)
                        if property == "status" && variant == "Response" =>
                    {
                        vals.first().cloned().unwrap_or(Value::Void)
                    }
                    Value::Enum(ref variant, ref vals)
                        if property == "body" && variant == "Response" =>
                    {
                        vals.get(1).cloned().unwrap_or(Value::Void)
                    }
                    _ => Value::Void,
                }
            }
            Expression::Call { callee, arguments } => {
                // Handle direct function calls like print(), println(), etc.
                if let Expression::Identifier(func_name) = &**callee {
                    let args: Vec<Value> = arguments.iter().map(|a| self.eval_expression(a)).collect();
                    
                    match func_name.as_str() {
                        "print" => {
                            if let Some(val) = args.first() {
                                println!("{}", self.value_to_string(val));
                            }
                            return Value::Void;
                        }
                        "println" => {
                            if let Some(val) = args.first() {
                                println!("{}", self.value_to_string(val));
                            } else {
                                println!();
                            }
                            return Value::Void;
                        }
                        "print_int" => {
                            if let Some(val) = args.first() {
                                println!("{}", self.value_to_string(val));
                            }
                            return Value::Void;
                        }
                        _ => {
                            // Fall through to existing logic for user-defined functions
                        }
                    }
                }
                
                // Dispatch to stdlib modules
                if let Expression::MemberAccess { object, property } = &**callee {
                    if let Expression::Identifier(obj_name) = &**object {
                        let args: Vec<Value> =
                            arguments.iter().map(|a| self.eval_expression(a)).collect();
                        if let Some(module) = self.stdlib.modules.get(obj_name) {
                            if let Some(_func) = module.functions().get(property) {
                                // Here, you would call the actual implementation for the function.
                                // For now, just mock a few common ones:
                                match (obj_name.as_str(), property.as_str()) {
                                    ("os", "print") => {
                                        if let Some(val) = args.first() {
                                            println!("{}", self.value_to_string(val));
                                        }
                                        return Value::Void;
                                    }
                                    ("os", "exit") => {
                                        std::process::exit(
                                            args.first()
                                                .and_then(|v| match v {
                                                    Value::Int(i) => Some(*i as i32),
                                                    _ => None,
                                                })
                                                .unwrap_or(1),
                                        );
                                    }
                                    ("math", "sqrt") => {
                                        if let Some(Value::Float(x)) = args.first() {
                                            return Value::Float(x.sqrt());
                                        }
                                        return Value::Void;
                                    }
                                    ("math", "abs") => {
                                        if let Some(Value::Float(x)) = args.first() {
                                            return Value::Float(x.abs());
                                        }
                                        return Value::Void;
                                    }
                                    ("math", "pow") => {
                                        if let (Some(Value::Float(x)), Some(Value::Float(y))) =
                                            (args.first(), args.get(1))
                                        {
                                            return Value::Float(x.powf(*y));
                                        }
                                        return Value::Void;
                                    }
                                    ("string", "len") => {
                                        if let Some(Value::String(s)) = args.first() {
                                            return Value::Int(s.len() as i64);
                                        }
                                        return Value::Void;
                                    }
                                    ("string", "contains") => {
                                        if let (
                                            Some(Value::String(s)),
                                            Some(Value::String(substr)),
                                        ) = (args.first(), args.get(1))
                                        {
                                            return Value::Bool(s.contains(substr));
                                        }
                                        return Value::Void;
                                    }
                                    // Add more stdlib function mocks as needed
                                    _ => {}
                                }
                            }
                        }
                        // Fallback: old hardcoded mocks for net/json
                        match (obj_name.as_str(), property.as_str()) {
                            ("net", "get") => {
                                let url = args
                                    .first()
                                    .and_then(|v| match v {
                                        Value::String(s) => Some(s.clone()),
                                        _ => None,
                                    })
                                    .unwrap_or_default();
                                if url.contains("/1") {
                                    Value::Enum("Response".to_string(), vec![Value::Int(200), Value::String("{\"name\":\"Alice\",\"email\":\"alice@example.com\"}".to_string())])
                                } else {
                                    Value::Enum(
                                        "Response".to_string(),
                                        vec![
                                            Value::Int(404),
                                            Value::String("Not found".to_string()),
                                        ],
                                    )
                                }
                            }
                            ("json", "parse") => {
                                let body = args
                                    .first()
                                    .and_then(|v| match v {
                                        Value::String(s) => Some(s.clone()),
                                        _ => None,
                                    })
                                    .unwrap_or_default();
                                if body.contains("Alice") {
                                    let mut data = HashMap::new();
                                    data.insert(
                                        "name".to_string(),
                                        Value::String("Alice".to_string()),
                                    );
                                    data.insert(
                                        "email".to_string(),
                                        Value::String("alice@example.com".to_string()),
                                    );
                                    Value::Result(
                                        Box::new(Value::Enum(
                                            "Ok".to_string(),
                                            vec![Value::Enum(
                                                "Object".to_string(),
                                                vec![Value::Struct("data".to_string(), data)],
                                            )],
                                        )),
                                        Box::new(Value::Void),
                                    )
                                } else {
                                    Value::Result(
                                        Box::new(Value::Void),
                                        Box::new(Value::Enum(
                                            "Err".to_string(),
                                            vec![Value::String("Invalid JSON".to_string())],
                                        )),
                                    )
                                }
                            }
                            _ => Value::Void,
                        }
                    } else {
                        Value::Void
                    }
                } else {
                    Value::Void
                }
            }
            Expression::Identifier(name) => {
                self.variables.get(name).cloned().unwrap_or(Value::Void)
            }
            Expression::ObjectLiteral(map) => {
                let mut fields = HashMap::new();
                for (k, v) in map.iter() {
                    fields.insert(k.clone(), self.eval_expression(v));
                }
                Value::Struct("<anon>".to_string(), fields)
            }
            Expression::BinaryOp {
                left,
                operator,
                right,
            } => {
                let l = self.eval_expression(left);
                let r = self.eval_expression(right);
                match operator {
                    BinaryOperator::Add => match (l, r) {
                        (Value::String(a), Value::String(b)) => Value::String(a + &b),
                        (Value::String(a), b) => Value::String(a + &self.value_to_string(&b)),
                        (a, Value::String(b)) => Value::String(self.value_to_string(&a) + &b),
                        (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
                        (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
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
            Expression::Await(inner) => {
                // For now, treat as sync
                let val = self.eval_expression(inner);
                match val {
                    Value::Future(b) => *b,
                    v => v,
                }
            }
            Expression::Assignment { target, value } => {
                // Evaluate the right-hand side first
                let val = self.eval_expression(value);
                
                // For now, only support identifier assignment
                if let Expression::Identifier(var_name) = target.as_ref() {
                    self.variables.insert(var_name.clone(), val.clone());
                    val
                } else {
                    // Complex assignment targets not supported yet
                    Value::Void
                }
            }
            _ => Value::Void,
        }
    }

    fn check_function_return_type(&mut self, func: &FunctionDeclaration) -> Result<(), String> {
        let mut found_returns = vec![];
        self.collect_return_values(&func.body, &mut found_returns);
        let declared = &func.return_type;
        if let Some(declared_type) = declared {
            for (ret_val, line) in &found_returns {
                if !self.value_type_matches(declared_type, ret_val) {
                    return Err(format!(
                        "Function '{}' return type mismatch at line {}: expected '{:?}', got '{:?}'",
                        func.name, line, declared_type, ret_val
                    ));
                }
            }
        } else {
            // Try to infer from returns
            if found_returns.is_empty() {
                // No return, treat as void
                return Ok(());
            }
            let first = &found_returns[0].0;
            for (ret_val, line) in &found_returns {
                if !self.value_type_matches(&self.type_of_value(first), ret_val) {
                    return Err(format!(
                        "Function '{}' has ambiguous return types (e.g. at line {}: '{:?}', elsewhere: '{:?}')",
                        func.name, line, first, ret_val
                    ));
                }
            }
        }
        Ok(())
    }

    fn collect_return_values(&mut self, block: &BlockStatement, found: &mut Vec<(Value, usize)>) {
        for (i, stmt) in block.statements.iter().enumerate() {
            match stmt {
                Statement::Return(ret) => {
                    if let Some(expr) = &ret.value {
                        let val = self.eval_expression(expr);
                        found.push((val, i + 1));
                    } else {
                        found.push((Value::Void, i + 1));
                    }
                }
                Statement::If(ifstmt) => {
                    self.collect_return_values(&ifstmt.then_block, found);
                    if let Some(else_block) = &ifstmt.else_block {
                        self.collect_return_values_from_stmt(else_block, found);
                    }
                }
                _ => {}
            }
        }
    }

    fn collect_return_values_from_stmt(
        &mut self,
        stmt: &Statement,
        found: &mut Vec<(Value, usize)>,
    ) {
        if let Statement::If(ifstmt) = stmt {
            self.collect_return_values(&ifstmt.then_block, found);
            if let Some(else_block) = &ifstmt.else_block {
                self.collect_return_values_from_stmt(else_block, found);
            }
        }
    }

    fn type_of_value(&self, value: &Value) -> Type {
        match value {
            Value::Int(_) => Type::Int,
            Value::Float(_) => Type::Float,
            Value::String(_) => Type::String,
            Value::Bool(_) => Type::Bool,
            Value::Void => Type::Void,
            Value::Struct(name, _) => Type::Custom(name.clone()),
            Value::Enum(name, _) => Type::Custom(name.clone()),
            _ => Type::Infer,
        }
    }

    fn value_type_matches(&self, expected: &Type, value: &Value) -> bool {
        match (expected, value) {
            (Type::Int, Value::Int(_)) => true,
            (Type::Float, Value::Float(_)) => true,
            (Type::String, Value::String(_)) => true,
            (Type::Bool, Value::Bool(_)) => true,
            (Type::Void, Value::Void) => true,
            (Type::Custom(name), Value::Struct(value_name, _)) => name == value_name,
            (Type::Custom(name), Value::Enum(value_name, _)) => name == value_name,
            _ => false,
        }
    }

    fn collect_param_names(&self, block: &BlockStatement, found: &mut Vec<String>) {
        for stmt in &block.statements {
            match stmt {
                Statement::Return(ret) => {
                    if let Some(expr) = &ret.value {
                        self.collect_param_names_from_expr(expr, found);
                    }
                }
                Statement::If(ifstmt) => {
                    self.collect_param_names_from_expr(&ifstmt.condition, found);
                    self.collect_param_names(&ifstmt.then_block, found);
                    if let Some(else_block) = &ifstmt.else_block {
                        self.collect_param_names_from_stmt(else_block, found);
                    }
                }
                _ => {}
            }
        }
    }

    fn collect_param_names_from_stmt(&self, stmt: &Statement, found: &mut Vec<String>) {
        if let Statement::If(ifstmt) = stmt {
            self.collect_param_names_from_expr(&ifstmt.condition, found);
            self.collect_param_names(&ifstmt.then_block, found);
            if let Some(else_block) = &ifstmt.else_block {
                self.collect_param_names_from_stmt(else_block, found);
            }
        }
    }

    #[allow(clippy::only_used_in_recursion)]
    fn collect_param_names_from_expr(&self, expr: &Expression, found: &mut Vec<String>) {
        match expr {
            Expression::Identifier(name) => {
                if !found.contains(name) {
                    found.push(name.clone());
                }
            }
            Expression::BinaryOp { left, right, .. } => {
                self.collect_param_names_from_expr(left, found);
                self.collect_param_names_from_expr(right, found);
            }
            Expression::Call { callee, arguments } => {
                for arg in arguments {
                    self.collect_param_names_from_expr(arg, found);
                }
                self.collect_param_names_from_expr(callee, found);
            }
            Expression::Assignment { target, value } => {
                self.collect_param_names_from_expr(target, found);
                self.collect_param_names_from_expr(value, found);
            }
            _ => {}
        }
    }

    fn pattern_matches(&self, pat: &Pattern, val: &Value) -> bool {
        match (pat, val) {
            (Pattern::Identifier(_), _) => true,
            (Pattern::TupleOrEnum(variant, _), Value::Enum(v, _)) => variant == v,
            (Pattern::Enum { variant, .. }, Value::Enum(v, _)) => variant == v,
            (Pattern::Literal(Expression::StringLiteral(s)), Value::String(v)) => s == v,
            (Pattern::Literal(Expression::IntLiteral(i)), Value::Int(v)) => i == v,
            _ => false,
        }
    }

    #[allow(clippy::only_used_in_recursion)]
    fn value_to_string(&self, val: &Value) -> String {
        match val {
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::String(s) => {
                // Remove surrounding quotes if they exist
                if s.starts_with('"') && s.ends_with('"') && s.len() > 1 {
                    s[1..s.len()-1].to_string()
                } else {
                    s.clone()
                }
            },
            Value::Bool(b) => b.to_string(),
            Value::Struct(name, fields) => {
                let mut s = format!("{name} {{ ");
                for (k, v) in fields.iter() {
                    s.push_str(&format!("{}: {}, ", k, self.value_to_string(v)));
                }
                s.push('}');
                s
            }
            Value::Enum(variant, vals) => format!("{variant}({vals:?})"),
            Value::Result(ok, err) => {
                if let Value::Void = **err {
                    self.value_to_string(ok)
                } else {
                    self.value_to_string(err)
                }
            }
            Value::Future(inner) => self.value_to_string(inner),
            Value::Void => "<void>".to_string(),
        }
    }
}
