use crate::ast::*;
use crate::lexer::{Token, TokenWithLocation};
use crate::error::CylError;

pub struct Parser {
    tokens: Vec<TokenWithLocation>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenWithLocation>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, CylError> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        
        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, CylError> {
        match &self.peek().token {
            Token::Import => self.parse_import(),
            Token::Fn | Token::Async => self.parse_function(),
            Token::Struct => self.parse_struct(),
            Token::Enum => self.parse_enum(),
            Token::Let | Token::Const => self.parse_declare(),
            Token::Return => self.parse_return(),
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::For => self.parse_for(),
            Token::Match => self.parse_match(),
            Token::Try => self.parse_try(),
            Token::Break => {
                self.advance();
                self.consume(Token::Semicolon, "Expected ';' after break")?;
                Ok(Statement::Break)
            }
            Token::Continue => {
                self.advance();
                self.consume(Token::Semicolon, "Expected ';' after continue")?;
                Ok(Statement::Continue)
            }
            _ => {
                let expr = self.parse_expression()?;
                self.consume(Token::Semicolon, "Expected ';' after expression")?;
                Ok(Statement::Expression(expr))
            }
        }
    }

    fn parse_import(&mut self) -> Result<Statement, CylError> {
        self.consume(Token::Import, "Expected 'import'")?;
        
        let module = match &self.peek().token {
            Token::Identifier(name) => name.clone(),
            // Allow type keywords as module names
            Token::StringType => "string".to_string(),
            Token::IntType => "int".to_string(),
            Token::FloatType => "float".to_string(),
            Token::BoolType => "bool".to_string(),
            Token::CharType => "char".to_string(),
            Token::Void => "void".to_string(),
            _ => {
                return Err(CylError::ParseError {
                    message: "Expected module name".to_string(),
                    line: self.peek().line,
                    column: self.peek().column,
                });
            }
        };
        
        self.advance();
        
        // TODO: Handle selective imports
        let items = None;
        
        self.consume(Token::Semicolon, "Expected ';' after import")?;
        
        Ok(Statement::Import(ImportStatement { module, items }))
    }

    fn parse_function(&mut self) -> Result<Statement, CylError> {
        let is_async = matches!(self.peek().token, Token::Async);
        if is_async {
            self.advance();
        }
        
        self.consume(Token::Fn, "Expected 'fn'")?;
        
        let name = if let Token::Identifier(name) = &self.peek().token {
            let name = name.clone();
            self.advance();
            name
        } else {
            return Err(CylError::ParseError {
                message: "Expected function name".to_string(),
                line: self.peek().line,
                column: self.peek().column,
            });
        };

        self.consume(Token::LeftParen, "Expected '(' after function name")?;
        
        let mut parameters = Vec::new();
        while !self.check(&Token::RightParen) && !self.is_at_end() {
            let param_name = if let Token::Identifier(name) = &self.peek().token {
                let name = name.clone();
                self.advance();
                name
            } else {
                return Err(CylError::ParseError {
                    message: "Expected parameter name".to_string(),
                    line: self.peek().line,
                    column: self.peek().column,
                });
            };

            self.consume(Token::Colon, "Expected ':' after parameter name")?;
            let param_type = self.parse_type()?;
            
            parameters.push(Parameter {
                name: param_name,
                param_type,
                is_mutable: false, // TODO: Handle mut parameters
            });

            if !self.check(&Token::RightParen) {
                self.consume(Token::Comma, "Expected ',' between parameters")?;
            }
        }
        
        self.consume(Token::RightParen, "Expected ')' after parameters")?;
        
        let return_type = if self.match_token(&Token::Arrow) {
            Some(self.parse_type()?)
        } else {
            None
        };

        let body = self.parse_block()?;

        Ok(Statement::Function(FunctionDeclaration {
            name,
            parameters,
            return_type,
            body,
            is_async,
        }))
    }

    fn parse_block(&mut self) -> Result<BlockStatement, CylError> {
        self.consume(Token::LeftBrace, "Expected '{'")?;
        
        let mut statements = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        
        self.consume(Token::RightBrace, "Expected '}'")?;
        
        Ok(BlockStatement { statements })
    }

    fn parse_type(&mut self) -> Result<Type, CylError> {
        let base_type = match &self.peek().token {
            Token::IntType => {
                self.advance();
                Type::Int
            }
            Token::FloatType => {
                self.advance();
                Type::Float
            }
            Token::StringType => {
                self.advance();
                Type::String
            }
            Token::BoolType => {
                self.advance();
                Type::Bool
            }
            Token::CharType => {
                self.advance();
                Type::Char
            }
            Token::Void => {
                self.advance();
                Type::Void
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Type::Custom(name)
            }
            Token::LeftBracket => {
                self.advance();
                let element_type = self.parse_type()?;
                self.consume(Token::RightBracket, "Expected ']' after array element type")?;
                Type::Array(Box::new(element_type))
            }
            _ => return Err(CylError::ParseError {
                message: "Expected type".to_string(),
                line: self.peek().line,
                column: self.peek().column,
            }),
        };

        // Check for optional type modifier
        if self.match_token(&Token::Question) {
            Ok(Type::Optional(Box::new(base_type)))
        } else {
            Ok(base_type)
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, CylError> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expression, CylError> {
        let expr = self.parse_logical_or()?;

        if self.match_token(&Token::Assign) {
            let value = self.parse_assignment()?;
            return Ok(Expression::Assignment {
                target: Box::new(expr),
                value: Box::new(value),
            });
        }

        Ok(expr)
    }

    fn parse_logical_or(&mut self) -> Result<Expression, CylError> {
        let mut expr = self.parse_logical_and()?;

        while self.match_token(&Token::Or) {
            let right = self.parse_logical_and()?;
            expr = Expression::BinaryOp {
                left: Box::new(expr),
                operator: BinaryOperator::Or,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_logical_and(&mut self) -> Result<Expression, CylError> {
        let mut expr = self.parse_equality()?;

        while self.match_token(&Token::And) {
            let right = self.parse_equality()?;
            expr = Expression::BinaryOp {
                left: Box::new(expr),
                operator: BinaryOperator::And,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expression, CylError> {
        let mut expr = self.parse_comparison()?;

        while let Some(op) = self.match_binary_op(&[Token::Equal, Token::NotEqual]) {
            let right = self.parse_comparison()?;
            expr = Expression::BinaryOp {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expression, CylError> {
        let mut expr = self.parse_term()?;

        while let Some(op) = self.match_binary_op(&[
            Token::Less,
            Token::LessEqual,
            Token::Greater,
            Token::GreaterEqual,
        ]) {
            let right = self.parse_term()?;
            expr = Expression::BinaryOp {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expression, CylError> {
        let mut expr = self.parse_factor()?;

        while let Some(op) = self.match_binary_op(&[Token::Plus, Token::Minus]) {
            let right = self.parse_factor()?;
            expr = Expression::BinaryOp {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expression, CylError> {
        let mut expr = self.parse_unary()?;

        while let Some(op) = self.match_binary_op(&[Token::Multiply, Token::Divide, Token::Modulo]) {
            let right = self.parse_unary()?;
            expr = Expression::BinaryOp {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expression, CylError> {
        if let Some(op) = self.match_unary_op(&[Token::Not, Token::Minus]) {
            let expr = self.parse_unary()?;
            return Ok(Expression::UnaryOp {
                operator: op,
                operand: Box::new(expr),
            });
        }

        if self.match_token(&Token::Await) {
            let expr = self.parse_unary()?;
            return Ok(Expression::Await(Box::new(expr)));
        }

        self.parse_call()
    }

    fn parse_call(&mut self) -> Result<Expression, CylError> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.match_token(&Token::LeftParen) {
                expr = self.finish_call(expr)?;
            } else if self.match_token(&Token::Dot) {
                if let Token::Identifier(name) = &self.peek().token {
                    let property = name.clone();
                    self.advance();
                    expr = Expression::MemberAccess {
                        object: Box::new(expr),
                        property,
                    };
                } else {
                    return Err(CylError::ParseError {
                        message: "Expected property name after '.'".to_string(),
                        line: self.peek().line,
                        column: self.peek().column,
                    });
                }
            } else if self.match_token(&Token::LeftBracket) {
                let index = self.parse_expression()?;
                self.consume(Token::RightBracket, "Expected ']' after index")?;
                expr = Expression::IndexAccess {
                    object: Box::new(expr),
                    index: Box::new(index),
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expression) -> Result<Expression, CylError> {
        let mut arguments = Vec::new();

        if !self.check(&Token::RightParen) {
            loop {
                arguments.push(self.parse_expression()?);
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }

        self.consume(Token::RightParen, "Expected ')' after arguments")?;

        Ok(Expression::Call {
            callee: Box::new(callee),
            arguments,
        })
    }

    fn parse_primary(&mut self) -> Result<Expression, CylError> {
        match &self.peek().token {
            Token::IntLiteral(value) => {
                let value = *value;
                self.advance();
                Ok(Expression::IntLiteral(value))
            }
            Token::FloatLiteral(value) => {
                let value = *value;
                self.advance();
                Ok(Expression::FloatLiteral(value))
            }
            Token::StringLiteral(value) => {
                let value = value.clone();
                self.advance();
                Ok(Expression::StringLiteral(value))
            }
            Token::BoolLiteral(value) => {
                let value = *value;
                self.advance();
                Ok(Expression::BoolLiteral(value))
            }
            Token::CharLiteral(value) => {
                let value = *value;
                self.advance();
                Ok(Expression::CharLiteral(value))
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(Expression::Identifier(name))
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(Token::RightParen, "Expected ')' after expression")?;
                Ok(expr)
            }
            Token::LeftBracket => {
                self.advance();
                let mut elements = Vec::new();
                
                if !self.check(&Token::RightBracket) {
                    loop {
                        elements.push(self.parse_expression()?);
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                    }
                }
                
                self.consume(Token::RightBracket, "Expected ']' after array elements")?;
                Ok(Expression::ArrayLiteral(elements))
            }
            _ => Err(CylError::ParseError {
                message: "Expected expression".to_string(),
                line: self.peek().line,
                column: self.peek().column,
            }),
        }
    }

    // Implementation for other statement types
    fn parse_declare(&mut self) -> Result<Statement, CylError> {
        let is_mutable = matches!(self.peek().token, Token::Let);
        let is_const = matches!(self.peek().token, Token::Const);
        
        self.advance(); // consume 'let' or 'const'
        
        // Check for 'mut' keyword
        let is_mutable = if is_mutable && self.match_token(&Token::Mut) {
            true
        } else {
            is_mutable
        };

        let name = if let Token::Identifier(name) = &self.peek().token {
            let name = name.clone();
            self.advance();
            name
        } else {
            return Err(CylError::ParseError {
                message: "Expected variable name".to_string(),
                line: self.peek().line,
                column: self.peek().column,
            });
        };

        // Optional type annotation
        let var_type = if self.match_token(&Token::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };

        self.consume(Token::Assign, "Expected '=' after variable declaration")?;
        let value = self.parse_expression()?;
        self.consume(Token::Semicolon, "Expected ';' after variable declaration")?;

        Ok(Statement::Declare(DeclareStatement {
            name,
            value,
            var_type,
            is_mutable: is_mutable && !is_const,
        }))
    }

    fn parse_return(&mut self) -> Result<Statement, CylError> {
        self.consume(Token::Return, "Expected 'return'")?;
        
        let value = if self.check(&Token::Semicolon) {
            None
        } else {
            Some(self.parse_expression()?)
        };
        
        self.consume(Token::Semicolon, "Expected ';' after return statement")?;
        
        Ok(Statement::Return(ReturnStatement { value }))
    }

    fn parse_if(&mut self) -> Result<Statement, CylError> {
        self.consume(Token::If, "Expected 'if'")?;
        let condition = self.parse_expression()?;
        let then_block = self.parse_block()?;
        
        let else_block = if self.match_token(&Token::Else) {
            if self.check(&Token::If) {
                // else if
                Some(Box::new(self.parse_if()?))
            } else {
                // else block
                let _else_body = self.parse_block()?;
                Some(Box::new(Statement::Expression(Expression::Identifier("block_placeholder".to_string()))))
            }
        } else {
            None
        };

        Ok(Statement::If(IfStatement {
            condition,
            then_block,
            else_block,
        }))
    }

    fn parse_while(&mut self) -> Result<Statement, CylError> {
        self.consume(Token::While, "Expected 'while'")?;
        let condition = self.parse_expression()?;
        let body = self.parse_block()?;

        Ok(Statement::While(WhileStatement { condition, body }))
    }

    fn parse_for(&mut self) -> Result<Statement, CylError> {
        self.consume(Token::For, "Expected 'for'")?;
        
        let variable = if let Token::Identifier(name) = &self.peek().token {
            let name = name.clone();
            self.advance();
            name
        } else {
            return Err(CylError::ParseError {
                message: "Expected variable name in for loop".to_string(),
                line: self.peek().line,
                column: self.peek().column,
            });
        };

        // TODO: Handle 'in' keyword when it's added to lexer
        // For now, assume next token should be the iterable
        let iterable = self.parse_expression()?;
        let body = self.parse_block()?;

        Ok(Statement::For(ForStatement {
            variable,
            iterable,
            body,
        }))
    }

    fn parse_struct(&mut self) -> Result<Statement, CylError> {
        self.consume(Token::Struct, "Expected 'struct'")?;
        
        let name = if let Token::Identifier(name) = &self.peek().token {
            let name = name.clone();
            self.advance();
            name
        } else {
            return Err(CylError::ParseError {
                message: "Expected struct name".to_string(),
                line: self.peek().line,
                column: self.peek().column,
            });
        };

        // TODO: Handle type parameters <T, U>
        let type_parameters = Vec::new();

        self.consume(Token::LeftBrace, "Expected '{' after struct name")?;
        
        let mut fields = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            // TODO: Handle pub keyword
            let is_public = false;
            
            let field_name = if let Token::Identifier(name) = &self.peek().token {
                let name = name.clone();
                self.advance();
                name
            } else {
                return Err(CylError::ParseError {
                    message: "Expected field name".to_string(),
                    line: self.peek().line,
                    column: self.peek().column,
                });
            };

            self.consume(Token::Colon, "Expected ':' after field name")?;
            let field_type = self.parse_type()?;
            
            fields.push(StructField {
                name: field_name,
                field_type,
                is_public,
            });

            if !self.check(&Token::RightBrace) {
                self.consume(Token::Comma, "Expected ',' between struct fields")?;
            }
        }
        
        self.consume(Token::RightBrace, "Expected '}' after struct fields")?;

        Ok(Statement::Struct(StructDeclaration {
            name,
            fields,
            type_parameters,
        }))
    }

    fn parse_enum(&mut self) -> Result<Statement, CylError> {
        self.consume(Token::Enum, "Expected 'enum'")?;
        
        let name = if let Token::Identifier(name) = &self.peek().token {
            let name = name.clone();
            self.advance();
            name
        } else {
            return Err(CylError::ParseError {
                message: "Expected enum name".to_string(),
                line: self.peek().line,
                column: self.peek().column,
            });
        };

        self.consume(Token::LeftBrace, "Expected '{' after enum name")?;
        
        let mut variants = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            let variant_name = if let Token::Identifier(name) = &self.peek().token {
                let name = name.clone();
                self.advance();
                name
            } else {
                return Err(CylError::ParseError {
                    message: "Expected variant name".to_string(),
                    line: self.peek().line,
                    column: self.peek().column,
                });
            };

            // TODO: Handle variant fields (Variant(Type1, Type2))
            let fields = None;
            
            variants.push(EnumVariant {
                name: variant_name,
                fields,
            });

            if !self.check(&Token::RightBrace) {
                self.consume(Token::Comma, "Expected ',' between enum variants")?;
            }
        }
        
        self.consume(Token::RightBrace, "Expected '}' after enum variants")?;

        Ok(Statement::Enum(EnumDeclaration { name, variants }))
    }

    fn parse_match(&mut self) -> Result<Statement, CylError> {
        self.consume(Token::Match, "Expected 'match'")?;
        let expression = self.parse_expression()?;
        
        self.consume(Token::LeftBrace, "Expected '{' after match expression")?;
        
        let mut arms = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            let pattern = self.parse_pattern()?;
            
            // TODO: Handle guard expressions (pattern if condition)
            let guard = None;
            
            self.consume(Token::FatArrow, "Expected '=>' after match pattern")?;
            let body = self.parse_block()?;
            
            arms.push(MatchArm {
                pattern,
                guard,
                body,
            });

            if !self.check(&Token::RightBrace) {
                self.consume(Token::Comma, "Expected ',' between match arms")?;
            }
        }
        
        self.consume(Token::RightBrace, "Expected '}' after match arms")?;

        Ok(Statement::Match(MatchStatement { expression, arms }))
    }

    fn parse_try(&mut self) -> Result<Statement, CylError> {
        self.consume(Token::Try, "Expected 'try'")?;
        let body = self.parse_block()?;
        
        let mut catch_clauses = Vec::new();
        while self.match_token(&Token::Catch) {
            // TODO: Handle specific exception types
            let exception_type = None;
            let variable = None;
            
            let catch_body = self.parse_block()?;
            
            catch_clauses.push(CatchClause {
                exception_type,
                variable,
                body: catch_body,
            });
        }

        Ok(Statement::Try(TryStatement { body, catch_clauses }))
    }

    fn parse_pattern(&mut self) -> Result<Pattern, CylError> {
        match &self.peek().token {
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(Pattern::Identifier(name))
            }
            Token::IntLiteral(_) | Token::FloatLiteral(_) | Token::StringLiteral(_) | 
            Token::BoolLiteral(_) | Token::CharLiteral(_) => {
                let expr = self.parse_primary()?;
                Ok(Pattern::Literal(expr))
            }
            // TODO: Handle wildcard pattern (_)
            _ => {
                Ok(Pattern::Wildcard)
            }
        }
    }

    // Helper methods
    fn match_token(&mut self, token: &Token) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_binary_op(&mut self, tokens: &[Token]) -> Option<BinaryOperator> {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return Some(match token {
                    Token::Plus => BinaryOperator::Add,
                    Token::Minus => BinaryOperator::Subtract,
                    Token::Multiply => BinaryOperator::Multiply,
                    Token::Divide => BinaryOperator::Divide,
                    Token::Modulo => BinaryOperator::Modulo,
                    Token::Equal => BinaryOperator::Equal,
                    Token::NotEqual => BinaryOperator::NotEqual,
                    Token::Less => BinaryOperator::Less,
                    Token::LessEqual => BinaryOperator::LessEqual,
                    Token::Greater => BinaryOperator::Greater,
                    Token::GreaterEqual => BinaryOperator::GreaterEqual,
                    Token::And => BinaryOperator::And,
                    Token::Or => BinaryOperator::Or,
                    _ => unreachable!(),
                });
            }
        }
        None
    }

    fn match_unary_op(&mut self, tokens: &[Token]) -> Option<UnaryOperator> {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return Some(match token {
                    Token::Not => UnaryOperator::Not,
                    Token::Minus => UnaryOperator::Minus,
                    Token::Plus => UnaryOperator::Plus,
                    _ => unreachable!(),
                });
            }
        }
        None
    }

    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(&self.peek().token) == std::mem::discriminant(token)
        }
    }

    fn advance(&mut self) -> &TokenWithLocation {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token == Token::Eof
    }

    fn peek(&self) -> &TokenWithLocation {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &TokenWithLocation {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token: Token, message: &str) -> Result<&TokenWithLocation, CylError> {
        if self.check(&token) {
            Ok(self.advance())
        } else {
            Err(CylError::ParseError {
                message: message.to_string(),
                line: self.peek().line,
                column: self.peek().column,
            })
        }
    }
}
