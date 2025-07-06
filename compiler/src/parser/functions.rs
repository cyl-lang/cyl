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
        // Parse generics: <T, U> (optional, allow no comma)
        let type_parameters = if self.check(&Token::LeftAngle) {
            let mut params = Vec::new();
            self.advance();
            while !self.check(&Token::RightAngle) && !self.is_at_end() {
                if let Token::Identifier(param) = &self.peek().token {
                    params.push(param.clone());
                    self.advance();
                } else {
                    return Err(CylError::ParseError {
                        message: "Expected generic parameter name".to_string(),
                        line: self.peek().line,
                        column: self.peek().column,
                    });
                }
                if self.check(&Token::Comma) {
                    self.advance();
                } else if self.check(&Token::RightAngle) {
                    break;
                } else {
                    // allow no comma
                    break;
                }
            }
            self.consume(Token::RightAngle, "Expected '>' after generic parameters")?;
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
                self.consume(Token::Colon, "Expected ':' after parameter name")?;
                let param_type = self.parse_type()?;
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
}
