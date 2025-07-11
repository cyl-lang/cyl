use crate::ast::{
    BinaryOperator, Expression, FunctionDeclaration, Program, Statement, StructDeclaration, Type,
};
use crate::error::CylError;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassManager;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine,
};
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, StructType};
use inkwell::values::{
    BasicMetadataValueEnum, BasicValueEnum, FunctionValue, IntValue, PointerValue,
};
use inkwell::{AddressSpace, IntPredicate, OptimizationLevel};
use std::collections::HashMap;
use std::path::Path;

pub struct LLVMCodegen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,

    // Symbol tables
    variables: HashMap<String, (PointerValue<'ctx>, Type)>, // Store type info with variables
    functions: HashMap<String, FunctionValue<'ctx>>,
    function_signatures: HashMap<String, (Vec<Type>, Option<Type>)>, // (params, return_type)
    struct_types: HashMap<String, (StructType<'ctx>, Vec<(String, Type)>)>, // (LLVM type, field info)
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
            struct_types: HashMap::new(),
        })
    }

    /// Declare builtin/standard library functions  
    fn declare_builtin_functions(&mut self) -> Result<(), CylError> {
        // Declare printf for print functionality
        let printf_type = self.context.i32_type().fn_type(
            &[self
                .context
                .i8_type()
                .ptr_type(AddressSpace::default())
                .into()],
            true, // variadic
        );
        let printf_fn = self.module.add_function("printf", printf_type, None);

        // Declare puts for simple string printing
        let puts_type = self.context.i32_type().fn_type(
            &[self
                .context
                .i8_type()
                .ptr_type(AddressSpace::default())
                .into()],
            false,
        );
        let puts_fn = self.module.add_function("puts", puts_type, None);

        // Register Cyl builtin functions that will map to these
        self.functions.insert("print".to_string(), puts_fn);
        self.functions.insert("printf".to_string(), printf_fn);

        // For print_int, we'll use printf with a format string
        self.functions.insert("print_int".to_string(), printf_fn);

        // Add function signatures for type checking
        self.function_signatures.insert(
            "print".to_string(),
            (vec![Type::String], Some(Type::Int)), // puts returns int
        );
        self.function_signatures
            .insert("printf".to_string(), (vec![Type::String], Some(Type::Int)));
        self.function_signatures
            .insert("print_int".to_string(), (vec![Type::Int], Some(Type::Int)));

        Ok(())
    }

    pub fn compile_program(&mut self, program: &Program) -> Result<(), CylError> {
        // Declare builtin functions first
        self.declare_builtin_functions()?;

        // First pass: declare all structs and functions
        for statement in &program.statements {
            match statement {
                Statement::Struct(struct_decl) => {
                    self.declare_struct(struct_decl)?;
                }
                Statement::Function(function) => {
                    self.declare_function(function)?;
                }
                _ => {}
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

        // Handle main function specially for executable generation
        let (fn_name, fn_type) = if function.name == "main" {
            // Create C-style main function: int main()
            let main_type = self.context.i32_type().fn_type(&[], false);
            ("main".to_string(), main_type)
        } else {
            // Regular function handling
            let fn_type = if let Some(ref rt) = function.return_type {
                if matches!(rt, Type::Void) {
                    // Void return type - use LLVM void type
                    self.context.void_type().fn_type(&param_types, false)
                } else {
                    // Non-void return type
                    let return_type = self.cyl_type_to_llvm(rt)?;
                    match return_type {
                        BasicTypeEnum::IntType(int_type) => int_type.fn_type(&param_types, false),
                        BasicTypeEnum::FloatType(float_type) => {
                            float_type.fn_type(&param_types, false)
                        }
                        BasicTypeEnum::PointerType(ptr_type) => {
                            ptr_type.fn_type(&param_types, false)
                        }
                        BasicTypeEnum::ArrayType(array_type) => {
                            array_type.fn_type(&param_types, false)
                        }
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
            (function.name.clone(), fn_type)
        };

        let fn_value = self.module.add_function(&fn_name, fn_type, None);
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

    fn declare_struct(&mut self, struct_decl: &StructDeclaration) -> Result<(), CylError> {
        // Create LLVM struct type
        let struct_type = self.context.opaque_struct_type(&struct_decl.name);

        // Convert field types to LLVM types
        let mut field_types: Vec<BasicTypeEnum> = Vec::new();
        let mut field_info: Vec<(String, Type)> = Vec::new();

        for field in &struct_decl.fields {
            let llvm_type = self.cyl_type_to_llvm(&field.field_type)?;
            field_types.push(llvm_type);
            field_info.push((field.name.clone(), field.field_type.clone()));
        }

        // Set the body of the struct type
        struct_type.set_body(&field_types, false);

        // Store the struct type and field information
        self.struct_types
            .insert(struct_decl.name.clone(), (struct_type, field_info));

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
        if function.name == "main" {
            // Main function should return 0 (success)
            let zero = self.context.i32_type().const_int(0, false);
            self.builder.build_return(Some(&zero)).unwrap();
        } else if function.return_type.is_none() {
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
                            // Handle builtin functions specially
                            if function_name == "print_int" {
                                self.compile_print_int_call(arguments)?;
                                return Ok(());
                            }

                            if let Some(fn_value) = self.functions.get(function_name).copied() {
                                let args: Vec<BasicMetadataValueEnum> = arguments
                                    .iter()
                                    .map(|arg| self.compile_expression(arg).map(|v| v.into()))
                                    .collect::<Result<Vec<_>, _>>()?;

                                self.builder.build_call(fn_value, &args, "calltmp").unwrap();
                                // Don't care about the return value for statement-level calls
                            } else {
                                return Err(CylError::CodeGenError {
                                    message: format!("Unknown function: {function_name}"),
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
                        Expression::ObjectLiteral(fields) => {
                            // Infer struct type from object literal
                            if let Some(Expression::StringLiteral(struct_name)) =
                                fields.get("__struct_name__")
                            {
                                Type::Custom(struct_name.clone())
                            } else {
                                Type::Int // Fallback if no struct name
                            }
                        }
                        Expression::MemberAccess { object, property } => {
                            // For member access, try to infer the field type
                            if let Expression::Identifier(obj_name) = object.as_ref() {
                                // Try to find the object in variables to get its type
                                if let Some((_, Type::Custom(struct_name))) =
                                    self.variables.get(obj_name)
                                {
                                    // Look up the field type in the struct definition
                                    if let Some((_, field_info)) =
                                        self.struct_types.get(struct_name)
                                    {
                                        if let Some((_, field_type)) =
                                            field_info.iter().find(|(name, _)| name == property)
                                        {
                                            field_type.clone()
                                        } else {
                                            Type::Int // Field not found
                                        }
                                    } else {
                                        Type::Int // Struct not found
                                    }
                                } else {
                                    Type::Int // Variable not found or not a struct
                                }
                            } else {
                                Type::Int // Complex object expression
                            }
                        }
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

                // Handle array literals specially
                if matches!(&declare_stmt.value, Expression::ArrayLiteral(_)) {
                    // For array literals, the init_value is already a pointer to the array
                    // We can directly use it as the variable
                    if let Expression::ArrayLiteral(elements) = &declare_stmt.value {
                        // Determine array type from first element
                        let first_val = self.compile_expression(&elements[0])?;
                        let element_type = first_val.get_type();
                        let array_size = elements.len() as u32;
                        let _array_type = element_type.array_type(array_size);
                        let inferred_type = Type::Array(Box::new(Type::Int)); // For now, assume int arrays

                        self.variables.insert(
                            declare_stmt.name.clone(),
                            (init_value.into_pointer_value(), inferred_type),
                        );
                    }
                } else if matches!(&declare_stmt.value, Expression::ObjectLiteral(_)) {
                    // For struct literals, the init_value is already a pointer to the struct
                    // We can directly use it as the variable
                    self.variables.insert(
                        declare_stmt.name.clone(),
                        (init_value.into_pointer_value(), var_type),
                    );
                } else if matches!(&declare_stmt.value, Expression::MemberAccess { .. })
                    && matches!(var_type, Type::Custom(_))
                {
                    // For struct field access that returns a struct pointer, use it directly
                    self.variables.insert(
                        declare_stmt.name.clone(),
                        (init_value.into_pointer_value(), var_type),
                    );
                } else {
                    // Regular variable allocation and storage
                    let alloca = self.create_entry_block_alloca(&declare_stmt.name, &var_type)?;
                    self.builder.build_store(alloca, init_value).unwrap();
                    self.variables
                        .insert(declare_stmt.name.clone(), (alloca, var_type));
                }
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
            Statement::For(for_stmt) => {
                // Compile for loop: for variable in iterable { body }
                // Currently supports: for i in N (iterate from 0 to N-1)

                // Get the current function
                let current_fn = self
                    .builder
                    .get_insert_block()
                    .unwrap()
                    .get_parent()
                    .unwrap();

                // Create basic blocks for the loop
                let loop_bb = self.context.append_basic_block(current_fn, "loop");
                let body_bb = self.context.append_basic_block(current_fn, "loopbody");
                let after_bb = self.context.append_basic_block(current_fn, "afterloop");

                // Initialize loop variable (i = 0)
                let loop_var_type = self.context.i32_type(); // Changed from i64 to i32
                let loop_var_ptr = self
                    .builder
                    .build_alloca(loop_var_type, &for_stmt.variable)
                    .unwrap();
                let zero = loop_var_type.const_int(0, false);
                self.builder.build_store(loop_var_ptr, zero).unwrap();

                // Store the loop variable in our variables map
                self.variables
                    .insert(for_stmt.variable.clone(), (loop_var_ptr, Type::Int));

                // Jump to loop condition check
                self.builder.build_unconditional_branch(loop_bb).unwrap();

                // Loop condition: i < limit
                self.builder.position_at_end(loop_bb);
                let current_val = self
                    .builder
                    .build_load(loop_var_type, loop_var_ptr, "loopvar")
                    .unwrap();

                // Compile the iterable expression (e.g., the number 5 in "for i in 5")
                let limit_val = self.compile_expression(&for_stmt.iterable)?;
                let limit_int = limit_val.into_int_value();

                // Compare: current_val < limit_val
                let condition = self
                    .builder
                    .build_int_compare(
                        IntPredicate::ULT,
                        current_val.into_int_value(),
                        limit_int,
                        "loopcond",
                    )
                    .unwrap();

                self.builder
                    .build_conditional_branch(condition, body_bb, after_bb)
                    .unwrap();

                // Loop body
                self.builder.position_at_end(body_bb);
                for stmt in &for_stmt.body.statements {
                    self.compile_statement(stmt)?;
                }

                // Increment loop variable: i = i + 1
                let current_val = self
                    .builder
                    .build_load(loop_var_type, loop_var_ptr, "loopvar")
                    .unwrap();
                let one = loop_var_type.const_int(1, false);
                let next_val = self
                    .builder
                    .build_int_add(current_val.into_int_value(), one, "nextval")
                    .unwrap();
                self.builder.build_store(loop_var_ptr, next_val).unwrap();

                // Jump back to condition check
                self.builder.build_unconditional_branch(loop_bb).unwrap();

                // After loop
                self.builder.position_at_end(after_bb);

                // Remove the loop variable from scope (optional, but clean)
                self.variables.remove(&for_stmt.variable);
            }
            _ => {
                return Err(CylError::CodeGenError {
                    message: format!("Statement type not yet implemented: {statement:?}"),
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
                .i32_type() // Changed from i64 to i32
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

                    // For arrays, return the pointer directly (don't load)
                    if matches!(var_type, Type::Array(_)) {
                        Ok((*variable).into())
                    } else {
                        // For other types, load the value
                        let llvm_type = self.cyl_type_to_llvm(var_type)?;
                        let loaded = self.builder.build_load(llvm_type, *variable, name).unwrap();
                        Ok(loaded)
                    }
                } else {
                    Err(CylError::CodeGenError {
                        message: format!("Undefined variable: {name}"),
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
                        message: format!("Binary operator not implemented: {operator:?}"),
                    }),
                }
            }
            Expression::Call { callee, arguments } => {
                // For now, assume callee is an identifier representing a function name
                if let Expression::Identifier(function_name) = callee.as_ref() {
                    // Handle builtin functions specially
                    if function_name == "print_int" {
                        return self.compile_print_int_call(arguments);
                    }

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
                            message: format!("Unknown function: {function_name}"),
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
                            message: format!("Undefined variable in assignment: {var_name}"),
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
            Expression::ArrayLiteral(elements) => {
                if elements.is_empty() {
                    return Err(CylError::CodeGenError {
                        message: "Empty arrays not supported yet".to_string(),
                    });
                }

                // Compile the first element to determine the array type
                let first_val = self.compile_expression(&elements[0])?;
                let element_type = first_val.get_type();
                let array_size = elements.len() as u32;

                // Create array type
                let array_type = element_type.array_type(array_size);

                // Allocate array on stack
                let array_ptr = self.builder.build_alloca(array_type, "array").unwrap();

                // Store each element in the array
                for (i, element) in elements.iter().enumerate() {
                    let element_val = self.compile_expression(element)?;
                    let index_val = self.context.i32_type().const_int(i as u64, false);
                    let element_ptr = unsafe {
                        self.builder
                            .build_gep(
                                array_type,
                                array_ptr,
                                &[self.context.i32_type().const_zero(), index_val],
                                &format!("arr_elem_{i}"),
                            )
                            .unwrap()
                    };
                    self.builder.build_store(element_ptr, element_val).unwrap();
                }

                Ok(array_ptr.into())
            }
            Expression::IndexAccess { object, index } => {
                let array_val = self.compile_expression(object)?;
                let index_val = self.compile_expression(index)?;

                if !array_val.is_pointer_value() {
                    return Err(CylError::CodeGenError {
                        message: "Can only index into arrays".to_string(),
                    });
                }

                let array_ptr = array_val.into_pointer_value();
                let index_int = if index_val.is_int_value() {
                    index_val.into_int_value()
                } else {
                    return Err(CylError::CodeGenError {
                        message: "Array index must be an integer".to_string(),
                    });
                };

                // Build GEP to access array element
                // We need to cast index to i32 first for GEP
                let index_i32 = if index_int.get_type() == self.context.i32_type() {
                    index_int
                } else {
                    self.builder
                        .build_int_cast(index_int, self.context.i32_type(), "cast_index")
                        .unwrap()
                };

                let element_ptr = unsafe {
                    self.builder
                        .build_gep(
                            self.context.i32_type(), // Changed from i64 to i32
                            array_ptr,
                            &[self.context.i32_type().const_zero(), index_i32],
                            "arr_access",
                        )
                        .unwrap()
                };

                // Load the value from the array element
                let loaded_val = self
                    .builder
                    .build_load(
                        self.context.i32_type(), // Changed from i64 to i32
                        element_ptr,
                        "arr_val",
                    )
                    .unwrap();

                Ok(loaded_val)
            }
            Expression::ObjectLiteral(fields) => {
                // Struct literal compilation
                // Look for __struct_name__ to identify the struct type
                if let Some(Expression::StringLiteral(struct_name)) = fields.get("__struct_name__")
                {
                    if let Some((struct_type, field_info)) =
                        self.struct_types.get(struct_name).cloned()
                    {
                        // Allocate memory for the struct
                        let struct_ptr = self
                            .builder
                            .build_alloca(struct_type, "struct_alloc")
                            .unwrap();

                        // Initialize each field
                        for (field_index, (field_name, _field_type)) in
                            field_info.iter().enumerate()
                        {
                            if let Some(field_expr) = fields.get(field_name) {
                                let field_value = self.compile_expression(field_expr)?;

                                // Get pointer to the field
                                let field_ptr = self
                                    .builder
                                    .build_struct_gep(
                                        struct_type,
                                        struct_ptr,
                                        field_index as u32,
                                        &format!("field_{field_name}"),
                                    )
                                    .unwrap();

                                // Store the value in the field
                                self.builder.build_store(field_ptr, field_value).unwrap();
                            }
                        }

                        Ok(struct_ptr.into())
                    } else {
                        Err(CylError::CodeGenError {
                            message: format!("Unknown struct type: {struct_name}"),
                        })
                    }
                } else {
                    Err(CylError::CodeGenError {
                        message: "Object literal without struct type information".to_string(),
                    })
                }
            }
            Expression::MemberAccess { object, property } => {
                // Struct field access compilation
                let struct_ptr = if let Expression::Identifier(var_name) = object.as_ref() {
                    // For identifiers, get the pointer directly from variables without loading
                    if let Some((variable, var_type)) = self.variables.get(var_name) {
                        if matches!(var_type, Type::Custom(_)) {
                            *variable
                        } else {
                            return Err(CylError::CodeGenError {
                                message: format!("Variable '{}' is not a struct", var_name),
                            });
                        }
                    } else {
                        return Err(CylError::CodeGenError {
                            message: format!("Unknown variable '{}'", var_name),
                        });
                    }
                } else {
                    // For other expressions, compile and expect a pointer
                    let struct_val = self.compile_expression(object)?;
                    if !struct_val.is_pointer_value() {
                        return Err(CylError::CodeGenError {
                            message: "Can only access fields on struct pointers".to_string(),
                        });
                    }
                    struct_val.into_pointer_value()
                };

                // Find the field index - iterate through all struct types to find matching field
                let struct_types_clone = self.struct_types.clone();
                for (struct_type, field_info) in struct_types_clone.values() {
                    if let Some(field_index) =
                        field_info.iter().position(|(name, _)| name == property)
                    {
                        // Get pointer to the field
                        let field_ptr = self
                            .builder
                            .build_struct_gep(
                                *struct_type,
                                struct_ptr,
                                field_index as u32,
                                &format!("field_{property}"),
                            )
                            .unwrap();

                        // Load the value from the field
                        let field_type = &field_info[field_index].1;

                        // For struct fields, return the pointer instead of loading the value
                        if matches!(field_type, Type::Custom(_)) {
                            return Ok(field_ptr.into());
                        } else {
                            // For primitive fields, load the value
                            let llvm_type = self.cyl_type_to_llvm(field_type)?;
                            let loaded_val = self
                                .builder
                                .build_load(llvm_type, field_ptr, &format!("load_{property}"))
                                .unwrap();
                            return Ok(loaded_val);
                        }
                    }
                }

                Err(CylError::CodeGenError {
                    message: format!("Field '{}' not found in any struct type", property),
                })
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
            Type::Int => Ok(self.context.i32_type().into()), // Changed from i64 to i32
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
                    _ => {
                        // Check if it's a struct type
                        if let Some((struct_type, _)) = self.struct_types.get(name) {
                            Ok((*struct_type).into())
                        } else {
                            Err(CylError::CodeGenError {
                                message: format!("Unknown type: {name}"),
                            })
                        }
                    }
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
        self.module.print_to_stderr();
    }

    /// Apply optimization passes to the module
    pub fn optimize(&self, opt_level: u8) -> Result<(), CylError> {
        // For now, skip optimization passes to avoid segfaults
        // TODO: Investigate and fix optimization pass issues
        if opt_level == 0 {
            return Ok(());
        }

        let fpm = PassManager::create(&self.module);

        match opt_level {
            1 => {
                // Basic optimizations
                fpm.add_instruction_combining_pass();
                fpm.add_cfg_simplification_pass();
            }
            2 => {
                // Standard optimizations
                fpm.add_instruction_combining_pass();
                fpm.add_cfg_simplification_pass();
                fpm.add_promote_memory_to_register_pass();
            }
            3 => {
                // Conservative aggressive optimizations
                fpm.add_instruction_combining_pass();
                fpm.add_cfg_simplification_pass();
                fpm.add_promote_memory_to_register_pass();
            }
            _ => {
                return Err(CylError::CodeGenError {
                    message: format!("Invalid optimization level: {opt_level}"),
                });
            }
        }

        fpm.initialize();

        // Run function passes on all functions
        for function in self.module.get_functions() {
            fpm.run_on(&function);
        }

        Ok(())
    }

    /// Generate object file from LLVM IR
    pub fn compile_to_object(&self, output_path: &Path, opt_level: u8) -> Result<(), CylError> {
        // Initialize LLVM targets
        Target::initialize_all(&InitializationConfig::default());

        // Get the default target triple for this machine
        let target_triple = TargetMachine::get_default_triple();
        let target = Target::from_triple(&target_triple).map_err(|e| CylError::CodeGenError {
            message: format!("Failed to create target: {e}"),
        })?;

        // Create target machine
        let optimization_level = match opt_level {
            0 => OptimizationLevel::None,
            1 => OptimizationLevel::Less,
            2 => OptimizationLevel::Default,
            3 => OptimizationLevel::Aggressive,
            _ => OptimizationLevel::Default,
        };

        let target_machine = target
            .create_target_machine(
                &target_triple,
                "generic",
                "",
                optimization_level,
                RelocMode::PIC, // Use Position Independent Code for PIE compatibility
                CodeModel::Default,
            )
            .ok_or_else(|| CylError::CodeGenError {
                message: "Failed to create target machine".to_string(),
            })?;

        // Generate object file
        target_machine
            .write_to_file(&self.module, FileType::Object, output_path)
            .map_err(|e| CylError::CodeGenError {
                message: format!("Failed to write object file: {e}"),
            })?;

        Ok(())
    }

    /// Generate executable from LLVM IR
    pub fn compile_to_executable(&self, output_path: &Path, opt_level: u8) -> Result<(), CylError> {
        // Apply optimizations
        self.optimize(opt_level)?;

        // Create temporary object file
        let obj_path = output_path.with_extension("o");
        self.compile_to_object(&obj_path, opt_level)?;

        // Link object file to create executable
        self.link_executable(&obj_path, output_path)?;

        // Clean up temporary object file
        if obj_path.exists() {
            std::fs::remove_file(&obj_path).map_err(|e| CylError::CodeGenError {
                message: format!("Failed to remove temporary object file: {e}"),
            })?;
        }

        Ok(())
    }

    /// Link object file to create executable
    fn link_executable(&self, obj_path: &Path, output_path: &Path) -> Result<(), CylError> {
        use std::process::Command;

        // Determine the appropriate linker and system libraries based on the target platform
        #[cfg(target_os = "macos")]
        let mut cmd = {
            // Use cc for easier linking on macOS
            let mut c = Command::new("cc");
            c.arg("-o").arg(output_path).arg(obj_path);
            c
        };

        #[cfg(target_os = "linux")]
        let mut cmd = {
            // Use cc with PIE flags for Linux to ensure compatibility with modern security settings
            let mut c = Command::new("cc");
            c.arg("-pie")  // Create Position Independent Executable
                .arg("-o")
                .arg(output_path)
                .arg(obj_path);
            c
        };

        #[cfg(target_os = "windows")]
        let mut cmd = {
            let mut c = Command::new("link");
            c.arg("/OUT:").arg(output_path).arg(obj_path);
            c
        };

        // For other platforms, try using cc as a fallback linker
        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
        let mut cmd = {
            let mut c = Command::new("cc");
            c.arg("-o").arg(output_path).arg(obj_path);
            c
        };

        let output = cmd.output().map_err(|e| CylError::CodeGenError {
            message: format!("Failed to run linker: {e}"),
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CylError::CodeGenError {
                message: format!("Linker failed: {stderr}"),
            });
        }

        Ok(())
    }

    /// Compile print_int function call which uses printf with a format string
    fn compile_print_int_call(
        &mut self,
        arguments: &[Expression],
    ) -> Result<BasicValueEnum<'ctx>, CylError> {
        if arguments.len() != 1 {
            return Err(CylError::CodeGenError {
                message: "print_int expects exactly one argument".to_string(),
            });
        }

        // Compile the integer argument
        let int_val = self.compile_expression(&arguments[0])?;

        // Create the format string "%d\n" for printf
        let format_str = "%d\n\0";
        let format_global = self
            .builder
            .build_global_string_ptr(format_str, "format_int")
            .unwrap();

        // Get the printf function
        let printf_fn = self.functions.get("printf").copied().unwrap();

        // Call printf with format string and integer
        let call_result = self
            .builder
            .build_call(
                printf_fn,
                &[format_global.as_pointer_value().into(), int_val.into()],
                "print_int_call",
            )
            .unwrap();

        if let Some(result) = call_result.try_as_basic_value().left() {
            Ok(result)
        } else {
            // This shouldn't happen with printf but handle gracefully
            Ok(self.context.i32_type().const_zero().into())
        }
    }
}
