use crate::lexer::Token;
use crate::error::CylError;
use crate::ast::*;
use super::helpers::*;

// Function, struct, enum parsing logic will be moved here.

impl Parser {
    // Function parsing logic
    // TODO: Move the real implementation from the old parser.rs
    // pub fn parse_function(&mut self) -> Result<Statement, CylError> { ... }
    // pub fn parse_struct(&mut self) -> Result<Statement, CylError> { ... }
    // pub fn parse_enum(&mut self) -> Result<Statement, CylError> { ... }

    pub fn parse_function(&mut self) -> Result<Statement, CylError> {
        let is_async = self.match_token(&Token::Async);
        self.consume(Token::Fn, "Expected 'fn'")?;
        let name = match &self.peek().token {
            Token::Identifier(n) => { let n = n.clone(); self.advance(); n },
            _ => return Err(CylError::ParseError { message: "Expected function name".to_string(), line: self.peek().line, column: self.peek().column })
        };
        // Always check for generics after the name
        let type_parameters = if self.check(&Token::Less) {
            let params = self.parse_generics()?;
            params
        } else {
            Vec::new()
        };
        self.consume(Token::LeftParen, "Expected '(' after function name and generics")?;
        let mut parameters = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                let param_name = match &self.peek().token {
                    Token::Identifier(n) => { let n = n.clone(); self.advance(); n },
                    _ => return Err(CylError::ParseError { message: "Expected parameter name".to_string(), line: self.peek().line, column: self.peek().column })
                };
                // Accept <type> or :type or nothing (type inference)
                let param_type = if self.check(&Token::Less) {
                    self.advance();
                    let mut type_names = Vec::new();
                    let mut saw_identifier = false;
                    loop {
                        match &self.peek().token {
                            Token::Identifier(tn) => {
                                type_names.push(tn.clone());
                                saw_identifier = true;
                                self.advance();
                            },
                            Token::Greater => {
                                self.advance();
                                break;
                            },
                            _ => {
                                self.advance();
                            }
                        }
                    }
                    if saw_identifier {
                        if type_names.len() == 1 {
                            Type::Custom(type_names.remove(0))
                        } else {
                            Type::Generic("_anon".to_string(), type_names)
                        }
                    } else {
                        Type::Infer
                    }
                } else if self.check(&Token::Colon) {
                    self.advance();
                    self.parse_type()?
                } else {
                    Type::Infer
                };
                let default_value = if self.check(&Token::Assign) {
                    self.advance();
                    Some(self.parse_expression()?)
                } else {
                    None
                };
                parameters.push(Parameter {
                    name: param_name,
                    param_type,
                    is_mutable: false,
                    default_value,
                });
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }
        self.consume(Token::RightParen, "Expected ')' after parameters")?;
        // Return type (optional, supports tuple types)
        let return_type = if self.check(&Token::Arrow) {
            self.advance();
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
            type_parameters,
        }))
    }

    pub fn parse_struct(&mut self) -> Result<Statement, CylError> {
        self.consume(Token::Struct, "Expected 'struct'")?;
        let name = match &self.peek().token {
            Token::Identifier(n) => { let n = n.clone(); self.advance(); n },
            _ => return Err(CylError::ParseError { message: "Expected struct name".to_string(), line: self.peek().line, column: self.peek().column })
        };
        // Always check for generics after the name
        let type_parameters = if self.check(&Token::Less) {
            let params = self.parse_generics()?;
            params
        } else {
            Vec::new()
        };
        self.consume(Token::LeftBrace, "Expected '{' after struct name and generics")?;
        let mut fields = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            if let Token::Identifier(field_name) = &self.peek().token {
                let field_name = field_name.clone();
                self.advance();
                self.consume(Token::Colon, "Expected ':' after field name")?;
                let field_type = self.parse_type()?;
                let is_public = false; // TODO: support pub fields
                fields.push(StructField { name: field_name, field_type, is_public });
                if self.match_token(&Token::Comma) || self.check(&Token::RightBrace) {
                    // continue
                } else {
                    return Err(CylError::ParseError { message: "Expected ',' or '}' after struct field".to_string(), line: self.peek().line, column: self.peek().column });
                }
            } else {
                break;
            }
        }
        self.consume(Token::RightBrace, "Expected '}' after struct fields")?;
        Ok(Statement::Struct(StructDeclaration { name, fields, type_parameters }))
    }

    pub fn parse_enum(&mut self) -> Result<Statement, CylError> {
        self.consume(Token::Enum, "Expected 'enum'")?;
        let name = match &self.peek().token {
            Token::Identifier(n) => { let n = n.clone(); self.advance(); n },
            _ => return Err(CylError::ParseError { message: "Expected enum name".to_string(), line: self.peek().line, column: self.peek().column })
        };
        // Always check for generics after the name
        let type_parameters = if self.check(&Token::Less) {
            let params = self.parse_generics()?;
            params
        } else {
            Vec::new()
        };
        self.consume(Token::LeftBrace, "Expected '{' after enum name and generics")?;
        let mut variants = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            if let Token::Identifier(variant_name) = &self.peek().token {
                let variant_name = variant_name.clone();
                self.advance();
                let fields = if self.check(&Token::LeftParen) {
                    self.advance();
                    let mut types = Vec::new();
                    if !self.check(&Token::RightParen) {
                        loop {
                            types.push(self.parse_type()?);
                            if !self.match_token(&Token::Comma) {
                                break;
                            }
                        }
                    }
                    self.consume(Token::RightParen, "Expected ')' after enum variant fields")?;
                    Some(types)
                } else {
                    None
                };
                variants.push(EnumVariant { name: variant_name, fields });
                if self.match_token(&Token::Comma) || self.check(&Token::RightBrace) {
                    // continue
                } else {
                    return Err(CylError::ParseError { message: "Expected ',' or '}' after enum variant".to_string(), line: self.peek().line, column: self.peek().column });
                }
            } else {
                break;
            }
        }
        self.consume(Token::RightBrace, "Expected '}' after enum variants")?;
        Ok(Statement::Enum(EnumDeclaration { name, variants, type_parameters }))
    }
}
