use crate::ast::{
    BinaryOperator, Expression, FunctionDeclaration, Program, Statement, Type,
};
use crate::error::CylError;
use cranelift_codegen::{
    ir::{AbiParam, Value, types, InstBuilder, condcodes::IntCC},
    Context, settings::Flags
};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_module::{Linkage, Module, FuncId};
use cranelift_object::{ObjectBuilder, ObjectModule};
use std::collections::HashMap;

pub struct CraneliftCodegen {
    module: ObjectModule,
    ctx: Context,
    
    // Symbol tables
    variables: HashMap<String, Variable>,
    var_types: HashMap<String, Type>, // Use string keys instead of Variable
    functions: HashMap<String, FuncId>,
    function_signatures: HashMap<String, (Vec<Type>, Option<Type>)>,
    
    // Variable counter for unique variables
    var_counter: usize,
}

impl CraneliftCodegen {
    pub fn new() -> Result<Self, CylError> {
        let isa_builder = cranelift_native::builder()
            .map_err(|e| CylError::CodeGenError { 
                message: format!("Failed to create ISA builder: {}", e) 
            })?;
        let isa = isa_builder
            .finish(Flags::new(cranelift_codegen::settings::builder()))
            .map_err(|e| CylError::CodeGenError { 
                message: format!("Failed to create ISA: {}", e) 
            })?;

        let builder = ObjectBuilder::new(
            isa,
            "cyl_module".to_string(),
            cranelift_module::default_libcall_names(),
        )
        .map_err(|e| CylError::CodeGenError { 
            message: format!("Failed to create object builder: {}", e) 
        })?;

        let module = ObjectModule::new(builder);

        Ok(Self {
            module,
            ctx: Context::new(),
            variables: HashMap::new(),
            var_types: HashMap::new(),
            functions: HashMap::new(),
            function_signatures: HashMap::new(),
            var_counter: 0,
        })
    }

    pub fn compile_program(&mut self, program: &Program) -> Result<(), CylError> {
        // First pass: declare all functions
        for stmt in &program.statements {
            if let Statement::Function(func) = stmt {
                self.declare_function(func)?;
            }
        }

        // Second pass: compile function bodies
        for stmt in &program.statements {
            match stmt {
                Statement::Function(func) => {
                    self.compile_function(func)?;
                }
                _ => {
                    return Err(CylError::CodeGenError {
                        message: "Top-level statements other than function declarations not supported yet"
                            .to_string(),
                    });
                }
            }
        }

        Ok(())
    }

    fn declare_function(&mut self, func: &FunctionDeclaration) -> Result<(), CylError> {
        let mut sig = self.module.make_signature();

        // Set return type (only if not void)
        if let Some(ref return_type) = func.return_type {
            if return_type != &Type::Void {
                sig.returns.push(AbiParam::new(self.type_to_cranelift(return_type)?));
            }
        }

        // Set parameter types
        let mut param_types = Vec::new();
        for param in &func.parameters {
            let cl_type = self.type_to_cranelift(&param.param_type)?;
            sig.params.push(AbiParam::new(cl_type));
            param_types.push(param.param_type.clone());
        }

        let func_id = self
            .module
            .declare_function(&func.name, Linkage::Export, &sig)
            .map_err(|e| CylError::CodeGenError { 
                message: format!("Failed to declare function: {}", e) 
            })?;

        self.functions.insert(func.name.clone(), func_id);
        self.function_signatures
            .insert(func.name.clone(), (param_types, func.return_type.clone()));

        Ok(())
    }

    fn compile_function(&mut self, func: &FunctionDeclaration) -> Result<(), CylError> {
        let func_id = self.functions[&func.name];
        
        // Clear context for new function
        self.ctx.clear();
        self.variables.clear();
        self.var_types.clear();
        self.var_counter = 0;

        // Create function builder context
        let mut fn_builder_ctx = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut fn_builder_ctx);

        // Create entry block
        let entry_block = builder.create_block();
        builder.append_block_params_for_function_params(entry_block);
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);

        // Declare function parameters as variables
        for (i, param) in func.parameters.iter().enumerate() {
            let var = Variable::from_u32(self.var_counter as u32);
            self.var_counter += 1;
            
            let param_value = builder.block_params(entry_block)[i];
            
            // Convert type separately to avoid borrow issues
            let cranelift_type = match param.param_type {
                Type::Int => types::I64,
                Type::Float => types::F64,
                Type::Bool => types::I8,
                Type::String => types::I64,
                _ => return Err(CylError::CodeGenError { 
                    message: format!("Parameter type not supported: {:?}", param.param_type) 
                }),
            };
            
            builder.declare_var(var, cranelift_type);
            builder.def_var(var, param_value);
            
            self.variables.insert(param.name.clone(), var);
            self.var_types.insert(param.name.clone(), param.param_type.clone());
        }

        // Compile function body statements one by one, handling each statement type directly
        for stmt in &func.body.statements {
            match stmt {
                Statement::Declare(decl) => {
                    let var = Variable::from_u32(self.var_counter as u32);
                    self.var_counter += 1;

                    // Compile the initial value
                    let val = Self::compile_expr_static(&decl.value, &mut builder, &self.variables)?;
                    let var_type = Self::infer_expression_type_static(&decl.value)?;
                    
                    // Convert type inline to avoid borrow issues
                    let cranelift_type = match var_type {
                        Type::Int => types::I64,
                        Type::Float => types::F64,
                        Type::Bool => types::I8,
                        Type::String => types::I64,
                        _ => return Err(CylError::CodeGenError { 
                            message: format!("Variable type not supported: {:?}", var_type) 
                        }),
                    };
                    
                    builder.declare_var(var, cranelift_type);
                    builder.def_var(var, val);
                    
                    self.variables.insert(decl.name.clone(), var);
                    self.var_types.insert(decl.name.clone(), var_type);
                }
                Statement::Return(ret_stmt) => {
                    if let Some(ref expr) = ret_stmt.value {
                        let val = Self::compile_expr_static(expr, &mut builder, &self.variables)?;
                        builder.ins().return_(&[val]);
                    } else {
                        builder.ins().return_(&[]);
                    }
                }
                Statement::Expression(expr) => {
                    Self::compile_expr_static(expr, &mut builder, &self.variables)?;
                }
                _ => {
                    return Err(CylError::CodeGenError { 
                        message: format!("Statement type not implemented: {:?}", stmt) 
                    });
                }
            }
        }

        // Ensure we have a proper terminator
        // Check if we need to add a return statement
        let mut has_return = false;
        for stmt in &func.body.statements {
            if matches!(stmt, Statement::Return(_)) {
                has_return = true;
                break;
            }
        }

        if !has_return {
            builder.ins().return_(&[]);
        }

        // Finalize function
        builder.finalize();

        // Define the function in the module
        self.module
            .define_function(func_id, &mut self.ctx)
            .map_err(|e| CylError::CodeGenError { 
                message: format!("Failed to define function: {}", e) 
            })?;

        Ok(())
    }

    fn compile_expr_static(
        expr: &Expression,
        builder: &mut FunctionBuilder,
        variables: &HashMap<String, Variable>,
    ) -> Result<Value, CylError> {
        match expr {
            Expression::IntLiteral(val) => {
                Ok(builder.ins().iconst(types::I64, *val))
            }
            Expression::FloatLiteral(val) => {
                Ok(builder.ins().f64const(*val))
            }
            Expression::StringLiteral(_) => {
                // TODO: Implement string literals
                Err(CylError::CodeGenError { 
                    message: "String literals not implemented yet".to_string() 
                })
            }
            Expression::BoolLiteral(val) => {
                Ok(builder.ins().iconst(types::I8, if *val { 1 } else { 0 }))
            }
            Expression::Identifier(name) => {
                if let Some(&var) = variables.get(name) {
                    Ok(builder.use_var(var))
                } else {
                    Err(CylError::CodeGenError { 
                        message: format!("Undefined variable: {}", name) 
                    })
                }
            }
            Expression::BinaryOp { left, operator, right } => {
                let left_val = Self::compile_expr_static(left, builder, variables)?;
                let right_val = Self::compile_expr_static(right, builder, variables)?;

                match operator {
                    BinaryOperator::Add => Ok(builder.ins().iadd(left_val, right_val)),
                    BinaryOperator::Subtract => Ok(builder.ins().isub(left_val, right_val)),
                    BinaryOperator::Multiply => Ok(builder.ins().imul(left_val, right_val)),
                    BinaryOperator::Divide => Ok(builder.ins().sdiv(left_val, right_val)),
                    BinaryOperator::Equal => Ok(builder.ins().icmp(IntCC::Equal, left_val, right_val)),
                    BinaryOperator::NotEqual => Ok(builder.ins().icmp(IntCC::NotEqual, left_val, right_val)),
                    _ => Err(CylError::CodeGenError { 
                        message: format!("Binary operator not implemented: {:?}", operator) 
                    }),
                }
            }
            Expression::Call { callee, arguments: _ } => {
                // Extract function name from callee
                let function_name = match callee.as_ref() {
                    Expression::Identifier(name) => name,
                    _ => return Err(CylError::CodeGenError { 
                        message: "Complex function calls not supported yet".to_string() 
                    }),
                };

                // Handle builtin functions
                if function_name == "print" || function_name == "println" {
                    // For now, just return a dummy value since we can't easily call printf
                    // TODO: Implement proper print function calling
                    Ok(builder.ins().iconst(types::I32, 0))
                } else {
                    // User-defined function call (simple case - return dummy for now)
                    Ok(builder.ins().iconst(types::I32, 0))
                }
            }
            _ => Err(CylError::CodeGenError { 
                message: format!("Expression type not implemented: {:?}", expr) 
            }),
        }
    }

    fn infer_expression_type_static(expr: &Expression) -> Result<Type, CylError> {
        match expr {
            Expression::IntLiteral(_) => Ok(Type::Int),
            Expression::FloatLiteral(_) => Ok(Type::Float),
            Expression::BoolLiteral(_) => Ok(Type::Bool),
            Expression::StringLiteral(_) => Ok(Type::String),
            Expression::Identifier(_) => {
                // For simplicity, assume int for now
                Ok(Type::Int)
            }
            Expression::BinaryOp { left, operator, .. } => {
                match operator {
                    BinaryOperator::Equal | BinaryOperator::NotEqual => Ok(Type::Bool),
                    _ => Self::infer_expression_type_static(left), // Assume same type as left operand
                }
            }
            _ => Err(CylError::CodeGenError { 
                message: format!("Type inference not implemented for: {:?}", expr) 
            }),
        }
    }

    fn type_to_cranelift(&self, ty: &Type) -> Result<cranelift_codegen::ir::Type, CylError> {
        match ty {
            Type::Int => Ok(types::I64),
            Type::Float => Ok(types::F64),
            Type::Bool => Ok(types::I8),
            Type::String => Ok(types::I64), // Pointer to string data
            Type::Void => Err(CylError::CodeGenError { 
                message: "Cannot convert void type".to_string() 
            }),
            _ => Err(CylError::CodeGenError { 
                message: format!("Type conversion not implemented: {:?}", ty) 
            }),
        }
    }

    pub fn write_object_file(&mut self, path: &str) -> Result<(), CylError> {
        // We need to consume the module to finish it
        let module = std::mem::replace(&mut self.module, {
            // Create a dummy module for replacement
            let isa_builder = cranelift_native::builder()
                .map_err(|e| CylError::CodeGenError { 
                    message: format!("Failed to create ISA builder: {}", e) 
                })?;
            let isa = isa_builder
                .finish(Flags::new(cranelift_codegen::settings::builder()))
                .map_err(|e| CylError::CodeGenError { 
                    message: format!("Failed to create ISA: {}", e) 
                })?;
            let builder = ObjectBuilder::new(
                isa,
                "dummy".to_string(),
                cranelift_module::default_libcall_names(),
            )
            .map_err(|e| CylError::CodeGenError { 
                message: format!("Failed to create object builder: {}", e) 
            })?;
            ObjectModule::new(builder)
        });
        
        let product = module.finish();
        
        std::fs::write(path, product.emit().unwrap())
            .map_err(|e| CylError::CodeGenError { 
                message: format!("Failed to write object file: {}", e) 
            })?;

        println!("Object file written to: {}", path);
        Ok(())
    }

    pub fn print_ir(&self) {
        println!("Cranelift IR generation complete!");
        // TODO: Add IR printing functionality if needed
    }
}
