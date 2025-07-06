use crate::lexer::Token;
use crate::error::CylError;
use crate::ast::*;
use super::helpers::*;

// Statement and variable declaration parsing logic will be moved here.

impl Parser {
    pub fn parse_statement(&mut self) -> Result<Statement, CylError> {
        match &self.peek().token {
            Token::Import => self.parse_import(),
            Token::Fn | Token::Async => self.parse_function(),
            Token::Struct => self.parse_struct(),
            Token::Enum => self.parse_enum(),
            Token::Let | Token::Const => {
                let stmt = self.parse_declare()?;
                self.consume(Token::Semicolon, "Expected ';' after declaration")?;
                Ok(stmt)
            },
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
            Token::Identifier(_) => {
                // Always treat identifier = ... or identifier <...> = ... as declaration at statement level
                if self.tokens.get(self.current + 1).map_or(false, |t| {
                    matches!(t.token, Token::Colon | Token::LeftAngle | Token::Assign)
                }) {
                    let stmt = self.parse_declare()?;
                    self.consume(Token::Semicolon, "Expected ';' after declaration")?;
                    Ok(stmt)
                } else {
                    let expr = self.parse_expression()?;
                    self.consume(Token::Semicolon, "Expected ';' after expression")?;
                    Ok(Statement::Expression(expr))
                }
            }
            _ => {
                let expr = self.parse_expression()?;
                self.consume(Token::Semicolon, "Expected ';' after expression")?;
                Ok(Statement::Expression(expr))
            }
        }
    }

    // TODO: Move the real implementation from the old parser.rs for:
    // parse_import, parse_declare, parse_return, parse_if, parse_while, parse_for, parse_match, parse_try

    // Stub implementations for all required parser methods
    pub fn parse_import(&mut self) -> Result<Statement, CylError> {
        Err(CylError::ParseError { message: "parse_import not yet implemented".to_string(), line: 0, column: 0 })
    }
    pub fn parse_declare(&mut self) -> Result<Statement, CylError> {
        // Accept: let x = 42; const PI <float> = 3.14; x = 42; y <float> = 3.14;
        let (is_mutable, name) = match &self.peek().token {
            Token::Let => {
                self.advance();
                let name = match &self.peek().token {
                    Token::Identifier(n) => { let n = n.clone(); self.advance(); n },
                    _ => return Err(CylError::ParseError { message: "Expected identifier after 'let'".to_string(), line: self.peek().line, column: self.peek().column })
                };
                (true, name)
            },
            Token::Const => {
                self.advance();
                let name = match &self.peek().token {
                    Token::Identifier(n) => { let n = n.clone(); self.advance(); n },
                    _ => return Err(CylError::ParseError { message: "Expected identifier after 'const'".to_string(), line: self.peek().line, column: self.peek().column })
                };
                (false, name)
            },
            Token::Identifier(n) => {
                let name = n.clone();
                self.advance();
                (true, name)
            },
            _ => return Err(CylError::ParseError { message: "Expected 'let', 'const', or identifier for declaration".to_string(), line: self.peek().line, column: self.peek().column })
        };
        // Support y <float> = ... as a type annotation
        let var_type = if self.check(&Token::LeftAngle) {
            self.advance();
            let type_name = match &self.peek().token {
                Token::Identifier(tn) => { let tn = tn.clone(); self.advance(); tn },
                _ => return Err(CylError::ParseError { message: "Expected type name after '<'".to_string(), line: self.peek().line, column: self.peek().column })
            };
            self.consume(Token::RightAngle, "Expected '>' after type name")?;
            Some(Type::Custom(type_name))
        } else if self.check(&Token::Colon) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };
        self.consume(Token::Assign, "Expected '=' in declaration")?;
        let value = self.parse_expression()?;
        Ok(Statement::Declare(DeclareStatement {
            name,
            var_type,
            value,
            is_mutable,
        }))
    }
    pub fn parse_return(&mut self) -> Result<Statement, CylError> {
        Err(CylError::ParseError { message: "parse_return not yet implemented".to_string(), line: 0, column: 0 })
    }
    pub fn parse_if(&mut self) -> Result<Statement, CylError> {
        Err(CylError::ParseError { message: "parse_if not yet implemented".to_string(), line: 0, column: 0 })
    }
    pub fn parse_while(&mut self) -> Result<Statement, CylError> {
        Err(CylError::ParseError { message: "parse_while not yet implemented".to_string(), line: 0, column: 0 })
    }
    pub fn parse_for(&mut self) -> Result<Statement, CylError> {
        Err(CylError::ParseError { message: "parse_for not yet implemented".to_string(), line: 0, column: 0 })
    }
    pub fn parse_match(&mut self) -> Result<Statement, CylError> {
        self.consume(Token::Match, "Expected 'match'")?;
        let expr = self.parse_expression()?;
        self.consume(Token::LeftBrace, "Expected '{' after match expression")?;
        let mut arms = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            // Parse pattern (simple: Identifier or Identifier(T))
            let pattern = if let Token::Identifier(name) = &self.peek().token {
                let name = name.clone();
                self.advance();
                if self.check(&Token::LeftParen) {
                    self.advance();
                    let mut subpatterns = Vec::new();
                    if !self.check(&Token::RightParen) {
                        loop {
                            // Only identifiers for now
                            if let Token::Identifier(subname) = &self.peek().token {
                                subpatterns.push(Pattern::Identifier(subname.clone()));
                                self.advance();
                            } else {
                                return Err(CylError::ParseError { message: "Expected identifier in tuple pattern".to_string(), line: self.peek().line, column: self.peek().column });
                            }
                            if !self.match_token(&Token::Comma) {
                                break;
                            }
                        }
                    }
                    self.consume(Token::RightParen, "Expected ')' in pattern")?;
                    Pattern::TupleOrEnum(name, subpatterns)
                } else {
                    Pattern::Identifier(name)
                }
            } else {
                return Err(CylError::ParseError { message: "Expected pattern".to_string(), line: self.peek().line, column: self.peek().column });
            };
            self.consume(Token::FatArrow, "Expected '=>' after pattern")?;
            let value = self.parse_expression()?;
            let body = BlockStatement { statements: vec![Statement::Expression(value)] };
            arms.push(MatchArm { pattern, guard: None, body });
            if self.match_token(&Token::Comma) {
                // allow trailing comma
            }
        }
        self.consume(Token::RightBrace, "Expected '}' after match arms")?;
        Ok(Statement::Match(MatchStatement { expression: expr, arms }))
    }
    pub fn parse_try(&mut self) -> Result<Statement, CylError> {
        Err(CylError::ParseError { message: "parse_try not yet implemented".to_string(), line: 0, column: 0 })
    }
    pub fn parse_struct(&mut self) -> Result<Statement, CylError> {
        self.consume(Token::Struct, "Expected 'struct'")?;
        let name = match &self.peek().token {
            Token::Identifier(n) => { let n = n.clone(); self.advance(); n },
            _ => return Err(CylError::ParseError { message: "Expected struct name".to_string(), line: self.peek().line, column: self.peek().column })
        };
        let type_parameters = if self.check(&Token::LeftAngle) {
            self.parse_generics()?
        } else {
            Vec::new()
        };
        // Skip any whitespace/comments between generics and '{'
        while self.check(&Token::Semicolon) { self.advance(); }
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
        let type_parameters = if self.check(&Token::LeftAngle) {
            self.parse_generics()?
        } else {
            Vec::new()
        };
        // Skip any whitespace/comments between generics and '{'
        while self.check(&Token::Semicolon) { self.advance(); }
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
