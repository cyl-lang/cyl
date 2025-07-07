use crate::lexer::Token;
use crate::error::CylError;
use crate::ast::*;
use super::helpers::*;

// Statement and variable declaration parsing logic will be moved here.

impl Parser {
    pub fn parse_statement(&mut self) -> Result<Statement, CylError> {
        match &self.peek().token {
            Token::Import => self.parse_import(),
            Token::Fn | Token::Async => {
                let stmt = self.parse_function()?;
                Ok(stmt)
            },
            Token::Struct => {
                let stmt = self.parse_struct()?;
                Ok(stmt)
            },
            Token::Enum => {
                let stmt = self.parse_enum()?;
                Ok(stmt)
            },
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
                // Robust lookahead for declaration forms:
                // identifier = ...
                // identifier <...> = ...
                // identifier : ... = ...
                // Only treat as declaration if next token is <, :, or =
                let is_decl = self.tokens.get(self.current + 1).map_or(false, |t| {
                    matches!(t.token, Token::Less | Token::Colon | Token::Assign)
                });
                if is_decl {
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

    pub fn parse_declare(&mut self) -> Result<Statement, CylError> {
        // Accept: let x = 42; const PI <float> = 3.14; x = 42; y <float> = 3.14; z: int = 1;
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
        // For variable declarations, treat <...> as a type annotation, not a generic
        let var_type = if self.check(&Token::Less) {
            self.advance();
            let mut type_names = Vec::new();
            let mut saw_identifier = false;
            // Accept any sequence of tokens until '>' for type inference, only collect identifiers
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
                    // Skip any other token (comma, whitespace, comments, etc.)
                    _ => {
                        self.advance();
                    }
                }
            }
            if saw_identifier {
                if type_names.len() == 1 {
                    Some(Type::Custom(type_names.remove(0)))
                } else {
                    Some(Type::Generic("_anon".to_string(), type_names))
                }
            } else {
                None
            }
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
        self.consume(Token::Return, "Expected 'return'")?;
        if self.check(&Token::Semicolon) {
            self.advance();
            Ok(Statement::Return(ReturnStatement { value: None }))
        } else {
            let expr = self.parse_expression()?;
            self.consume(Token::Semicolon, "Expected ';' after return value")?;
            Ok(Statement::Return(ReturnStatement { value: Some(expr) }))
        }
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
    pub fn parse_import(&mut self) -> Result<Statement, CylError> {
        Err(CylError::ParseError { message: "parse_import not yet implemented".to_string(), line: 0, column: 0 })
    }
}
