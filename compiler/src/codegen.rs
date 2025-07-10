use crate::ast::{BinaryOperator, Expression, FunctionDeclaration, Program, Statement, Type};
use crate::error::CylError;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicMetadataTypeEnum, BasicTypeEnum};
use inkwell::values::{
    BasicMetadataValueEnum, BasicValueEnum, FunctionValue, IntValue, PointerValue,
};
use inkwell::{AddressSpace, IntPredicate};
use std::collections::HashMap;

pub struct LLVMCodegen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,

    // Symbol tables
    variables: HashMap<String, (PointerValue<'ctx>, Type)>, // Store type info with variables
    functions: HashMap<String, FunctionValue<'ctx>>,
    function_signatures: HashMap<String, (Vec<Type>, Option<Type>)>, // (params, return_type)
}

impl<'ctx> LLVMCodegen<'ctx> {
    pub fn new(context: &'ctx Context) -> Result<Self, CylError> {
        let module = context.create_module("cyl_module");

        Ok(Self {
            context,
            module,
            builder: context.create_builder(),
            variables: HashMap::new(),
            functions: HashMap::new(),
            function_signatures: HashMap::new(),
        })
    }

    pub fn compile_program(&mut self, program: &Program) -> Result<(), CylError> {
        // First pass: declare all functions
        for statement in &program.statements {
            if let Statement::Function(function) = statement {
                self.declare_function(function)?;
            }
        }

        // Second pass: compile function bodies and other statements
        for statement in &program.statements {
            match statement {
                Statement::Function(function) => {
                    self.compile_function(function)?;
                }
                _ => {
                    // For now, skip non-function statements at the top level
                    // TODO: Handle global statements
                }
            }
        }

        Ok(())
    }

    fn declare_function(&mut self, function: &FunctionDeclaration) -> Result<(), CylError> {
        let param_types: Vec<BasicMetadataTypeEnum> = function
            .parameters
            .iter()
            .map(|param| self.cyl_type_to_llvm(&param.param_type).map(|t| t.into()))
            .collect::<Result<Vec<_>, _>>()?;

        let fn_type = if let Some(ref rt) = function.return_type {
            if matches!(rt, Type::Void) {
                // Void return type - use LLVM void type
                self.context.void_type().fn_type(&param_types, false)
            } else {
                // Non-void return type
                let return_type = self.cyl_type_to_llvm(rt)?;
                match return_type {
                    BasicTypeEnum::IntType(int_type) => int_type.fn_type(&param_types, false),
                    BasicTypeEnum::FloatType(float_type) => float_type.fn_type(&param_types, false),
                    BasicTypeEnum::PointerType(ptr_type) => ptr_type.fn_type(&param_types, false),
                    BasicTypeEnum::ArrayType(array_type) => array_type.fn_type(&param_types, false),
                    BasicTypeEnum::StructType(struct_type) => {
                        struct_type.fn_type(&param_types, false)
                    }
                    BasicTypeEnum::VectorType(vector_type) => {
                        vector_type.fn_type(&param_types, false)
                    }
                }
            }
        } else {
            // No return type specified - assume void
            self.context.void_type().fn_type(&param_types, false)
        };

        let fn_value = self.module.add_function(&function.name, fn_type, None);
        self.functions.insert(function.name.clone(), fn_value);

        // Store function signature for type inference
        let param_types_cyl: Vec<Type> = function
            .parameters
            .iter()
            .map(|p| p.param_type.clone())
            .collect();
        self.function_signatures.insert(
            function.name.clone(),
            (param_types_cyl, function.return_type.clone()),
        );

        Ok(())
    }

    fn compile_function(&mut self, function: &FunctionDeclaration) -> Result<(), CylError> {
        let fn_value = self.functions[&function.name];
        let entry_block = self.context.append_basic_block(fn_value, "entry");
        self.builder.position_at_end(entry_block);

        // Clear local variables for new function
        self.variables.clear();

        // Add parameters to symbol table
        for (i, param) in function.parameters.iter().enumerate() {
            let param_value = fn_value.get_nth_param(i as u32).unwrap();
            let alloca = self.create_entry_block_alloca(&param.name, &param.param_type)?;
            self.builder.build_store(alloca, param_value).unwrap();
            self.variables
                .insert(param.name.clone(), (alloca, param.param_type.clone()));
        }

        // Compile function body
        for statement in &function.body.statements {
            self.compile_statement(statement)?;
        }

        // If no explicit return, add a default return
        if function.return_type.is_none() {
            self.builder.build_return(None).unwrap();
        }

        Ok(())
    }

    fn compile_statement(&mut self, statement: &Statement) -> Result<(), CylError> {
        match statement {
            Statement::Expression(expr) => {
                // For expression statements, we compile the expression but don't use the result
                // This allows function calls that return void
                match expr {
                    Expression::Call { callee, arguments } => {
                        if let Expression::Identifier(function_name) = callee.as_ref() {
                            if let Some(fn_value) = self.functions.get(function_name).copied() {
                                let args: Vec<BasicMetadataValueEnum> = arguments
                                    .iter()
                                    .map(|arg| self.compile_expression(arg).map(|v| v.into()))
                                    .collect::<Result<Vec<_>, _>>()?;

                                self.builder.build_call(fn_value, &args, "calltmp").unwrap();
                                // Don't care about the return value for statement-level calls
                            } else {
                                return Err(CylError::CodeGenError {
                                    message: format!("Unknown function: {}", function_name),
                                });
                            }
                        } else {
                            return Err(CylError::CodeGenError {
                                message: "Complex function calls not yet supported".to_string(),
                            });
                        }
                    }
                    _ => {
                        self.compile_expression(expr)?;
                    }
                }
            }
            Statement::Declare(declare_stmt) => {
                // Use the explicitly declared type, or infer from the expression
                let var_type = if let Some(ref explicit_type) = declare_stmt.var_type {
                    explicit_type.clone()
                } else {
                    // Try to infer type from the expression
                    match &declare_stmt.value {
                        Expression::Call { callee, .. } => {
                            if let Expression::Identifier(function_name) = callee.as_ref() {
                                if let Some((_, return_type)) =
                                    self.function_signatures.get(function_name)
                                {
                                    return_type.clone().unwrap_or(Type::Int) // Default to int if function is void
                                } else {
                                    Type::Int // Unknown function, default to int
                                }
                            } else {
                                Type::Int // Complex call, default to int
                            }
                        }
                        Expression::IntLiteral(_) => Type::Int,
                        Expression::FloatLiteral(_) => Type::Float,
                        Expression::BoolLiteral(_) => Type::Bool,
                        Expression::StringLiteral(_) => Type::String,
                        _ => Type::Int, // Default fallback
                    }
                };

                // Skip void variables (they don't make sense)
                if matches!(var_type, Type::Void) {
                    return Err(CylError::CodeGenError {
                        message: "Cannot declare void variable".to_string(),
                    });
                }

                // First compile the expression to get its value
                let init_value = self.compile_expression(&declare_stmt.value)?;
                let alloca = self.create_entry_block_alloca(&declare_stmt.name, &var_type)?;
                self.builder.build_store(alloca, init_value).unwrap();
                self.variables
                    .insert(declare_stmt.name.clone(), (alloca, var_type));
            }
            Statement::Return(return_stmt) => {
                if let Some(ref return_expr) = return_stmt.value {
                    let return_value = self.compile_expression(return_expr)?;
                    self.builder.build_return(Some(&return_value)).unwrap();
                } else {
                    self.builder.build_return(None).unwrap();
                }
            }
            Statement::If(if_stmt) => {
                let condition_value = self.compile_expression(&if_stmt.condition)?;
                let condition_bool = self.value_to_bool(condition_value)?;

                let function = self
                    .builder
                    .get_insert_block()
                    .unwrap()
                    .get_parent()
                    .unwrap();
                let then_bb = self.context.append_basic_block(function, "then");
                let else_bb = self.context.append_basic_block(function, "else");
                let merge_bb = self.context.append_basic_block(function, "ifcont");

                self.builder
                    .build_conditional_branch(condition_bool, then_bb, else_bb)
                    .unwrap();

                // Then block
                self.builder.position_at_end(then_bb);
                for stmt in &if_stmt.then_block.statements {
                    self.compile_statement(stmt)?;
                }
                self.builder.build_unconditional_branch(merge_bb).unwrap();

                // Else block
                self.builder.position_at_end(else_bb);
                if let Some(ref else_stmt) = if_stmt.else_block {
                    self.compile_statement(else_stmt)?;
                }
                self.builder.build_unconditional_branch(merge_bb).unwrap();

                // Merge block
                self.builder.position_at_end(merge_bb);
            }
            Statement::While(while_stmt) => {
                let function = self
                    .builder
                    .get_insert_block()
                    .unwrap()
                    .get_parent()
                    .unwrap();
                let loop_bb = self.context.append_basic_block(function, "loop");
                let body_bb = self.context.append_basic_block(function, "loop_body");
                let after_bb = self.context.append_basic_block(function, "after_loop");

                // Jump to loop condition check
                self.builder.build_unconditional_branch(loop_bb).unwrap();

                // Loop condition check
                self.builder.position_at_end(loop_bb);
                let condition_value = self.compile_expression(&while_stmt.condition)?;
                let condition_bool = self.value_to_bool(condition_value)?;
                self.builder
                    .build_conditional_branch(condition_bool, body_bb, after_bb)
                    .unwrap();

                // Loop body
                self.builder.position_at_end(body_bb);
                for stmt in &while_stmt.body.statements {
                    self.compile_statement(stmt)?;
                }
                self.builder.build_unconditional_branch(loop_bb).unwrap();

                // After loop
                self.builder.position_at_end(after_bb);
            }
            Statement::Block(block) => {
                for stmt in &block.statements {
                    self.compile_statement(stmt)?;
                }
            }
            _ => {
                return Err(CylError::CodeGenError {
                    message: format!("Statement type not yet implemented: {:?}", statement),
                });
            }
        }
        Ok(())
    }

    fn compile_expression(
        &mut self,
        expression: &Expression,
    ) -> Result<BasicValueEnum<'ctx>, CylError> {
        match expression {
            Expression::IntLiteral(value) => Ok(self
                .context
                .i64_type()
                .const_int(*value as u64, false)
                .into()),
            Expression::FloatLiteral(value) => {
                Ok(self.context.f64_type().const_float(*value).into())
            }
            Expression::StringLiteral(value) => {
                let string_type = self.context.i8_type().array_type(value.len() as u32 + 1);
                let string_global =
                    self.module
                        .add_global(string_type, Some(AddressSpace::default()), "str");
                let string_value = self.context.const_string(value.as_bytes(), true);
                string_global.set_initializer(&string_value);
                Ok(string_global.as_pointer_value().into())
            }
            Expression::BoolLiteral(value) => Ok(self
                .context
                .bool_type()
                .const_int(*value as u64, false)
                .into()),
            Expression::Identifier(name) => {
                if let Some((variable, var_type)) = self.variables.get(name) {
                    // Don't try to load void types
                    if matches!(var_type, Type::Void) {
                        return Err(CylError::CodeGenError {
                            message: "Cannot use void variable".to_string(),
                        });
                    }
                    let llvm_type = self.cyl_type_to_llvm(var_type)?;
                    let loaded = self.builder.build_load(llvm_type, *variable, name).unwrap();
                    Ok(loaded)
                } else {
                    Err(CylError::CodeGenError {
                        message: format!("Undefined variable: {}", name),
                    })
                }
            }
            Expression::BinaryOp {
                left,
                operator,
                right,
            } => {
                let left_val = self.compile_expression(left)?;
                let right_val = self.compile_expression(right)?;

                match operator {
                    BinaryOperator::Add => {
                        if left_val.is_int_value() && right_val.is_int_value() {
                            Ok(self
                                .builder
                                .build_int_add(
                                    left_val.into_int_value(),
                                    right_val.into_int_value(),
                                    "addtmp",
                                )
                                .unwrap()
                                .into())
                        } else if left_val.is_float_value() || right_val.is_float_value() {
                            let left_float = if left_val.is_float_value() {
                                left_val.into_float_value()
                            } else {
                                self.builder
                                    .build_signed_int_to_float(
                                        left_val.into_int_value(),
                                        self.context.f64_type(),
                                        "tmp",
                                    )
                                    .unwrap()
                            };
                            let right_float = if right_val.is_float_value() {
                                right_val.into_float_value()
                            } else {
                                self.builder
                                    .build_signed_int_to_float(
                                        right_val.into_int_value(),
                                        self.context.f64_type(),
                                        "tmp",
                                    )
                                    .unwrap()
                            };
                            Ok(self
                                .builder
                                .build_float_add(left_float, right_float, "addtmp")
                                .unwrap()
                                .into())
                        } else {
                            Err(CylError::CodeGenError {
                                message: "Invalid types for addition".to_string(),
                            })
                        }
                    }
                    BinaryOperator::Subtract => {
                        if left_val.is_int_value() && right_val.is_int_value() {
                            Ok(self
                                .builder
                                .build_int_sub(
                                    left_val.into_int_value(),
                                    right_val.into_int_value(),
                                    "subtmp",
                                )
                                .unwrap()
                                .into())
                        } else {
                            Err(CylError::CodeGenError {
                                message: "Subtraction not implemented for non-integers yet"
                                    .to_string(),
                            })
                        }
                    }
                    BinaryOperator::Multiply => {
                        if left_val.is_int_value() && right_val.is_int_value() {
                            Ok(self
                                .builder
                                .build_int_mul(
                                    left_val.into_int_value(),
                                    right_val.into_int_value(),
                                    "multmp",
                                )
                                .unwrap()
                                .into())
                        } else {
                            Err(CylError::CodeGenError {
                                message: "Multiplication not implemented for non-integers yet"
                                    .to_string(),
                            })
                        }
                    }
                    BinaryOperator::Divide => {
                        if left_val.is_int_value() && right_val.is_int_value() {
                            Ok(self
                                .builder
                                .build_int_signed_div(
                                    left_val.into_int_value(),
                                    right_val.into_int_value(),
                                    "divtmp",
                                )
                                .unwrap()
                                .into())
                        } else {
                            Err(CylError::CodeGenError {
                                message: "Division not implemented for non-integers yet"
                                    .to_string(),
                            })
                        }
                    }
                    BinaryOperator::Equal => {
                        if left_val.is_int_value() && right_val.is_int_value() {
                            Ok(self
                                .builder
                                .build_int_compare(
                                    IntPredicate::EQ,
                                    left_val.into_int_value(),
                                    right_val.into_int_value(),
                                    "cmptmp",
                                )
                                .unwrap()
                                .into())
                        } else {
                            Err(CylError::CodeGenError {
                                message: "Equality comparison not implemented for non-integers yet"
                                    .to_string(),
                            })
                        }
                    }
                    BinaryOperator::NotEqual => {
                        if left_val.is_int_value() && right_val.is_int_value() {
                            Ok(self
                                .builder
                                .build_int_compare(
                                    IntPredicate::NE,
                                    left_val.into_int_value(),
                                    right_val.into_int_value(),
                                    "cmptmp",
                                )
                                .unwrap()
                                .into())
                        } else {
                            Err(CylError::CodeGenError {
                                message:
                                    "Not-equal comparison not implemented for non-integers yet"
                                        .to_string(),
                            })
                        }
                    }
                    BinaryOperator::Less => {
                        if left_val.is_int_value() && right_val.is_int_value() {
                            Ok(self
                                .builder
                                .build_int_compare(
                                    IntPredicate::SLT,
                                    left_val.into_int_value(),
                                    right_val.into_int_value(),
                                    "cmptmp",
                                )
                                .unwrap()
                                .into())
                        } else {
                            Err(CylError::CodeGenError {
                                message:
                                    "Less-than comparison not implemented for non-integers yet"
                                        .to_string(),
                            })
                        }
                    }
                    BinaryOperator::LessEqual => {
                        if left_val.is_int_value() && right_val.is_int_value() {
                            Ok(self
                                .builder
                                .build_int_compare(
                                    IntPredicate::SLE,
                                    left_val.into_int_value(),
                                    right_val.into_int_value(),
                                    "cmptmp",
                                )
                                .unwrap()
                                .into())
                        } else {
                            Err(CylError::CodeGenError {
                                message:
                                    "Less-equal comparison not implemented for non-integers yet"
                                        .to_string(),
                            })
                        }
                    }
                    BinaryOperator::Greater => {
                        if left_val.is_int_value() && right_val.is_int_value() {
                            Ok(self
                                .builder
                                .build_int_compare(
                                    IntPredicate::SGT,
                                    left_val.into_int_value(),
                                    right_val.into_int_value(),
                                    "cmptmp",
                                )
                                .unwrap()
                                .into())
                        } else {
                            Err(CylError::CodeGenError {
                                message:
                                    "Greater-than comparison not implemented for non-integers yet"
                                        .to_string(),
                            })
                        }
                    }
                    BinaryOperator::GreaterEqual => {
                        if left_val.is_int_value() && right_val.is_int_value() {
                            Ok(self
                                .builder
                                .build_int_compare(
                                    IntPredicate::SGE,
                                    left_val.into_int_value(),
                                    right_val.into_int_value(),
                                    "cmptmp",
                                )
                                .unwrap()
                                .into())
                        } else {
                            Err(CylError::CodeGenError {
                                message:
                                    "Greater-equal comparison not implemented for non-integers yet"
                                        .to_string(),
                            })
                        }
                    }
                    _ => Err(CylError::CodeGenError {
                        message: format!("Binary operator not implemented: {:?}", operator),
                    }),
                }
            }
            Expression::Call { callee, arguments } => {
                // For now, assume callee is an identifier representing a function name
                if let Expression::Identifier(function_name) = callee.as_ref() {
                    if let Some(fn_value) = self.functions.get(function_name).copied() {
                        let args: Vec<BasicMetadataValueEnum> = arguments
                            .iter()
                            .map(|arg| self.compile_expression(arg).map(|v| v.into()))
                            .collect::<Result<Vec<_>, _>>()?;

                        let call_result =
                            self.builder.build_call(fn_value, &args, "calltmp").unwrap();
                        if let Some(result) = call_result.try_as_basic_value().left() {
                            Ok(result)
                        } else {
                            // Void function - this should not be used in expressions that need a value
                            Err(CylError::CodeGenError {
                                message: format!(
                                    "Function '{}' returns void and cannot be used in expressions",
                                    function_name
                                ),
                            })
                        }
                    } else {
                        Err(CylError::CodeGenError {
                            message: format!("Unknown function: {}", function_name),
                        })
                    }
                } else {
                    Err(CylError::CodeGenError {
                        message: "Complex function calls not yet supported".to_string(),
                    })
                }
            }
            Expression::Assignment { target, value } => {
                // For now, only support identifier assignments
                if let Expression::Identifier(var_name) = target.as_ref() {
                    let var_ptr = if let Some((var_ptr, _)) = self.variables.get(var_name) {
                        *var_ptr
                    } else {
                        return Err(CylError::CodeGenError {
                            message: format!("Undefined variable in assignment: {}", var_name),
                        });
                    };

                    let new_value = self.compile_expression(value)?;
                    self.builder.build_store(var_ptr, new_value).unwrap();
                    Ok(new_value) // Return the assigned value
                } else {
                    Err(CylError::CodeGenError {
                        message: "Only simple variable assignment supported currently".to_string(),
                    })
                }
            }
            _ => Err(CylError::CodeGenError {
                message: format!("Expression type not implemented: {:?}", expression),
            }),
        }
    }

    fn value_to_bool(&self, value: BasicValueEnum<'ctx>) -> Result<IntValue<'ctx>, CylError> {
        if value.is_int_value() {
            let int_val = value.into_int_value();
            // For integers, check if non-zero
            Ok(self
                .builder
                .build_int_compare(
                    IntPredicate::NE,
                    int_val,
                    int_val.get_type().const_zero(),
                    "tobool",
                )
                .unwrap())
        } else if value.is_float_value() {
            let float_val = value.into_float_value();
            // For floats, check if non-zero
            Ok(self
                .builder
                .build_float_compare(
                    inkwell::FloatPredicate::ONE, // Ordered and not equal
                    float_val,
                    float_val.get_type().const_zero(),
                    "tobool",
                )
                .unwrap())
        } else {
            // For other types, assume true for now
            Ok(self.context.bool_type().const_int(1, false))
        }
    }

    fn cyl_type_to_llvm(&self, cyl_type: &Type) -> Result<BasicTypeEnum<'ctx>, CylError> {
        match cyl_type {
            Type::Int => Ok(self.context.i64_type().into()),
            Type::Float => Ok(self.context.f64_type().into()),
            Type::String => Ok(self
                .context
                .i8_type()
                .ptr_type(AddressSpace::default())
                .into()),
            Type::Bool => Ok(self.context.bool_type().into()),
            Type::Char => Ok(self.context.i8_type().into()),
            Type::Void => Err(CylError::CodeGenError {
                message: "Void type cannot be used as a basic type".to_string(),
            }),
            Type::Custom(name) => {
                // Map common type names
                match name.as_str() {
                    "i32" | "int32" => Ok(self.context.i32_type().into()),
                    "i64" | "int64" => Ok(self.context.i64_type().into()),
                    "i16" | "int16" => Ok(self.context.i16_type().into()),
                    "i8" | "int8" => Ok(self.context.i8_type().into()),
                    "u32" | "uint32" => Ok(self.context.i32_type().into()), // Unsigned treated as signed for now
                    "u64" | "uint64" => Ok(self.context.i64_type().into()),
                    "u16" | "uint16" => Ok(self.context.i16_type().into()),
                    "u8" | "uint8" => Ok(self.context.i8_type().into()),
                    "f32" | "float32" => Ok(self.context.f32_type().into()),
                    "f64" | "float64" => Ok(self.context.f64_type().into()),
                    _ => Err(CylError::CodeGenError {
                        message: format!("Unknown type: {}", name),
                    }),
                }
            }
            _ => Err(CylError::CodeGenError {
                message: format!("Type not implemented: {:?}", cyl_type),
            }),
        }
    }

    fn create_entry_block_alloca(
        &self,
        name: &str,
        cyl_type: &Type,
    ) -> Result<PointerValue<'ctx>, CylError> {
        let builder = self.context.create_builder();
        let entry_block = self
            .builder
            .get_insert_block()
            .unwrap()
            .get_parent()
            .unwrap()
            .get_first_basic_block()
            .unwrap();

        match entry_block.get_first_instruction() {
            Some(first_instr) => builder.position_before(&first_instr),
            None => builder.position_at_end(entry_block),
        }

        let llvm_type = self.cyl_type_to_llvm(cyl_type)?;
        Ok(builder.build_alloca(llvm_type, name).unwrap())
    }

    pub fn print_ir(&self) {
        println!("{}", self.module.print_to_string().to_string());
    }
}
