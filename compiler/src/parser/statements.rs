use super::helpers::*;
use crate::ast::*;
use crate::error::CylError;
use crate::lexer::Token;

// Statement and variable declaration parsing logic will be moved here.

impl Parser {
    pub fn parse_statement(&mut self) -> Result<Statement, CylError> {
        // Allow lone semicolons as no-op statements (skip them)
        if self.check(&Token::Semicolon) {
            self.advance();
            // Optionally, return a Statement::NoOp or just skip
            return self.parse_statement();
        }
        match &self.peek().token {
            Token::Import => self.parse_import(),
            Token::Fn => {
                self.advance(); // Advance past 'fn' so parse_function sees the name
                let stmt = self.parse_function(false)?;
                // Do NOT advance past RightBrace here; parse_block already consumes the function body
                // Skip optional semicolon after function declaration (forgiveness)
                if self.check(&Token::Semicolon) {
                    self.advance();
                }
                Ok(stmt)
            }
            Token::Async => {
                self.advance();
                self.consume(Token::Fn, "Expected 'fn' after 'async'")?;
                let stmt = self.parse_function(true)?;
                // Skip optional semicolon after async function declaration (forgiveness)
                if self.check(&Token::Semicolon) {
                    self.advance();
                }
                Ok(stmt)
            }
            Token::Struct => {
                self.consume(Token::Struct, "Expected 'struct'")?;
                let stmt = self.parse_struct()?;
                // Skip optional semicolon after struct declaration (forgiveness)
                if self.check(&Token::Semicolon) {
                    self.advance();
                }
                Ok(stmt)
            }
            Token::Enum => {
                self.consume(Token::Enum, "Expected 'enum'")?;
                let stmt = self.parse_enum()?;
                // Skip optional semicolon after enum declaration (forgiveness)
                if self.check(&Token::Semicolon) {
                    self.advance();
                }
                Ok(stmt)
            }
            Token::Let | Token::Const => {
                let stmt = self.parse_declare()?;

                Ok(stmt)
            }
            Token::Return => self.parse_return(),
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::For => self.parse_for(),
            Token::Match => {
                self.advance();
                self.parse_match()
            }
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
                let is_decl = self
                    .tokens
                    .get(self.current + 1)
                    .is_some_and(|t| matches!(t.token, Token::Less | Token::Colon | Token::Assign));
                if is_decl {
                    let stmt = self.parse_declare()?;
                    if self.check(&Token::Semicolon) {
                        self.advance();
                    } else if !self.check(&Token::RightBrace) {
                        return Err(CylError::ParseError {
                            message: "Expected ';' after declaration".to_string(),
                            line: self.peek().line,
                            column: self.peek().column,
                        });
                    }
                    Ok(stmt)
                } else {
                    // Accept member access (dot) as valid expression statement
                    let expr = self.parse_expression()?;
                    self.consume(Token::Semicolon, "Expected ';' after expression")?;
                    Ok(Statement::Expression(expr))
                }
            }
            Token::LeftBrace => {
                // Parse a block as a statement (do not require semicolon after)
                let block = self.parse_block()?;
                Ok(Statement::Block(block))
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
                    Token::Identifier(n) => {
                        let n = n.clone();
                        self.advance();
                        n
                    }
                    _ => {
                        return Err(CylError::ParseError {
                            message: "Expected identifier after 'let'".to_string(),
                            line: self.peek().line,
                            column: self.peek().column,
                        })
                    }
                };
                (true, name)
            }
            Token::Const => {
                self.advance();
                let name = match &self.peek().token {
                    Token::Identifier(n) => {
                        let n = n.clone();
                        self.advance();
                        n
                    }
                    _ => {
                        return Err(CylError::ParseError {
                            message: "Expected identifier after 'const'".to_string(),
                            line: self.peek().line,
                            column: self.peek().column,
                        })
                    }
                };
                (false, name)
            }
            Token::Identifier(n) => {
                let name = n.clone();
                self.advance();
                (true, name)
            }
            _ => {
                return Err(CylError::ParseError {
                    message: "Expected 'let', 'const', or identifier for declaration".to_string(),
                    line: self.peek().line,
                    column: self.peek().column,
                })
            }
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
                    }
                    Token::Greater => {
                        self.advance();
                        break;
                    }
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
        self.consume(Token::If, "Expected 'if'")?;
        let condition = self.parse_expression()?;
        let then_block = self.parse_block()?;
        let else_block = if self.match_token(&Token::Else) {
            if self.check(&Token::If) {
                // else if
                Some(Box::new(self.parse_if()?))
            } else {
                // else { ... } -- parse as a block, wrap as Statement::BlockStatement
                let block = self.parse_block()?;
                // For now, wrap the block as a Statement::Expression with a Block expression if needed
                // But since Statement does not have Block, just return the first statement or a dummy if empty
                let stmt = block
                    .statements
                    .into_iter()
                    .next()
                    .unwrap_or(Statement::Break);
                Some(Box::new(stmt))
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

    pub fn parse_while(&mut self) -> Result<Statement, CylError> {
        Err(CylError::ParseError {
            message: "parse_while not yet implemented".to_string(),
            line: 0,
            column: 0,
        })
    }
    pub fn parse_for(&mut self) -> Result<Statement, CylError> {
        Err(CylError::ParseError {
            message: "parse_for not yet implemented".to_string(),
            line: 0,
            column: 0,
        })
    }
    pub fn parse_match(&mut self) -> Result<Statement, CylError> {
        let expr = self.parse_expression_stop_at_left_brace()?;
        // DEBUG: After parsing match subject, print current token

        self.consume(Token::LeftBrace, "Expected '{' after match expression")?;
        // DEBUG: Print parser state after consuming '{'

        // HARD ERROR: If next token is not Identifier or Underscore, panic
        match &self.peek().token {
            Token::Identifier(_) | Token::Underscore | Token::RightBrace => {}
            other => {
                return Err(CylError::ParseError {
                    message: format!("BUG: Unexpected token after '{{' in match: {:?}", other),
                    line: self.peek().line,
                    column: self.peek().column,
                });
            }
        }
        let mut arms = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            // DEBUG: At start of match arms loop

            // DEBUG: Print the current token and position before parsing the pattern

            let pattern = self.parse_pattern()?;
            self.consume(Token::FatArrow, "Expected '=>' after pattern")?;
            let body = if self.check(&Token::LeftBrace) {
                self.parse_block()?
            } else {
                let value = self.parse_expression()?;
                BlockStatement {
                    statements: vec![Statement::Expression(value)],
                }
            };
            arms.push(MatchArm {
                pattern,
                guard: None,
                body,
            });
            if self.check(&Token::Comma) {
                self.advance();
            }
        }
        self.consume(Token::RightBrace, "Expected '}' after match arms")?;
        Ok(Statement::Match(MatchStatement {
            expression: expr,
            arms,
        }))
    }

    /// Recursively parse a pattern for match arms, supporting qualified, tuple, and nested patterns.
    fn parse_pattern(&mut self) -> Result<Pattern, CylError> {
        // DEBUG: Entering parse_pattern

        // Accept wildcard '_'
        if self.check(&Token::Underscore) {
            self.advance();
            return Ok(Pattern::Wildcard);
        }
        // Parse qualified names: e.g., Ok, JsonValue.Object, etc.
        let mut path = Vec::new();
        while let Token::Identifier(name) = &self.peek().token {
            path.push(name.clone());
            self.advance();
            if self.check(&Token::Dot) {
                self.advance();
            } else {
                break;
            }
        }
        if path.is_empty() {
            return Err(CylError::ParseError {
                message: "Expected pattern identifier".to_string(),
                line: self.peek().line,
                column: self.peek().column,
            });
        }
        // If next is '(', parse tuple/enum/constructor pattern (possibly nested)
        if self.check(&Token::LeftParen) {
            self.advance();
            let mut subpatterns = Vec::new();
            if !self.check(&Token::RightParen) {
                loop {
                    let subpat = self.parse_pattern()?;
                    subpatterns.push(subpat);
                    if !self.match_token(&Token::Comma) {
                        break;
                    }
                }
            }
            self.consume(Token::RightParen, "Expected ')' in pattern")?;
            Ok(Pattern::TupleOrEnum(path.join("."), subpatterns))
        } else if self.check(&Token::LeftBrace) {
            // Struct pattern: Name { field1, field2, ... }
            let name = path.join(".");
            self.advance();
            let mut fields = Vec::new();
            let mut first = true;
            while !self.check(&Token::RightBrace) && !self.is_at_end() {
                if !first {
                    // Only consume comma between fields, not after last field
                    self.consume(Token::Comma, "Expected ',' between struct pattern fields")?;
                }
                first = false;
                // Parse field: field_name [: pattern]
                let field_name = match &self.peek().token {
                    Token::Identifier(f) => {
                        let f = f.clone();
                        self.advance();
                        f
                    }
                    _ => {
                        return Err(CylError::ParseError {
                            message: "Expected field name in struct pattern".to_string(),
                            line: self.peek().line,
                            column: self.peek().column,
                        });
                    }
                };
                let pat = if self.check(&Token::Colon) {
                    self.advance();
                    self.parse_pattern()?
                } else {
                    Pattern::Identifier(field_name.clone())
                };
                fields.push((field_name, pat));
            }
            self.consume(Token::RightBrace, "Expected '}' after struct pattern")?;
            // DEBUG: After parsing struct pattern, print next token

            Ok(Pattern::Struct { name, fields })
        } else if path.len() == 1 {
            Ok(Pattern::Identifier(path.remove(0)))
        } else {
            // Qualified enum/variant without tuple: e.g., JsonValue.Null
            Ok(Pattern::TupleOrEnum(path.join("."), vec![]))
        }
    }
    pub fn parse_try(&mut self) -> Result<Statement, CylError> {
        Err(CylError::ParseError {
            message: "parse_try not yet implemented".to_string(),
            line: 0,
            column: 0,
        })
    }
    pub fn parse_import(&mut self) -> Result<Statement, CylError> {
        self.consume(Token::Import, "Expected 'import'")?;
        let module = match &self.peek().token {
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();
                name
            }
            Token::StringType => {
                self.advance();
                "string".to_string()
            }
            Token::IntType => {
                self.advance();
                "int".to_string()
            }
            Token::FloatType => {
                self.advance();
                "float".to_string()
            }
            Token::BoolType => {
                self.advance();
                "bool".to_string()
            }
            Token::CharType => {
                self.advance();
                "char".to_string()
            }
            _ => {
                return Err(CylError::ParseError {
                    message: "Expected module name after 'import'".to_string(),
                    line: self.peek().line,
                    column: self.peek().column,
                });
            }
        };
        self.consume(Token::Semicolon, "Expected ';' after import statement")?;

        Ok(Statement::Import(ImportStatement {
            module,
            items: None,
        }))
    }
}
