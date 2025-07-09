use super::helpers::*;
use crate::ast::*;
use crate::error::CylError;
use crate::lexer::Token;

impl Parser {
    // Expression parsing logic
    // TODO: Move the real implementation from the old parser.rs
    // pub fn parse_expression(&mut self) -> Result<Expression, CylError> { ... }

    pub fn parse_expression(&mut self) -> Result<Expression, CylError> {
        self.parse_expression_internal(false)
    }

    pub fn parse_expression_stop_at_left_brace(&mut self) -> Result<Expression, CylError> {
        self.parse_expression_internal(true)
    }

    fn parse_expression_internal(
        &mut self,
        stop_at_left_brace: bool,
    ) -> Result<Expression, CylError> {
        let result = self.parse_assignment_internal(stop_at_left_brace);
        result
    }

    fn parse_assignment_internal(
        &mut self,
        stop_at_left_brace: bool,
    ) -> Result<Expression, CylError> {
        let expr = self.parse_logical_or_internal(stop_at_left_brace)?;
        if self.match_token(&Token::Assign) {
            let value = self.parse_assignment_internal(stop_at_left_brace)?;
            return Ok(Expression::Assignment {
                target: Box::new(expr),
                value: Box::new(value),
            });
        }
        Ok(expr)
    }

    fn parse_logical_or_internal(
        &mut self,
        stop_at_left_brace: bool,
    ) -> Result<Expression, CylError> {
        let mut expr = self.parse_logical_and_internal(stop_at_left_brace)?;
        while self.match_token(&Token::Or) {
            let right = self.parse_logical_and_internal(stop_at_left_brace)?;
            expr = Expression::BinaryOp {
                left: Box::new(expr),
                operator: BinaryOperator::Or,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_logical_and_internal(
        &mut self,
        stop_at_left_brace: bool,
    ) -> Result<Expression, CylError> {
        let mut expr = self.parse_equality_internal(stop_at_left_brace)?;
        while self.match_token(&Token::And) {
            let right = self.parse_equality_internal(stop_at_left_brace)?;
            expr = Expression::BinaryOp {
                left: Box::new(expr),
                operator: BinaryOperator::And,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_equality_internal(
        &mut self,
        stop_at_left_brace: bool,
    ) -> Result<Expression, CylError> {
        let mut expr = self.parse_comparison_internal(stop_at_left_brace)?;
        while let Some(op) = self.match_binary_op(&[Token::Equal, Token::NotEqual]) {
            let right = self.parse_comparison_internal(stop_at_left_brace)?;
            expr = Expression::BinaryOp {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_comparison_internal(
        &mut self,
        stop_at_left_brace: bool,
    ) -> Result<Expression, CylError> {
        let mut expr = self.parse_term_internal(stop_at_left_brace)?;
        while let Some(op) = self.match_binary_op(&[
            Token::Less,
            Token::LessEqual,
            Token::Greater,
            Token::GreaterEqual,
        ]) {
            let right = self.parse_term_internal(stop_at_left_brace)?;
            expr = Expression::BinaryOp {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_term_internal(&mut self, stop_at_left_brace: bool) -> Result<Expression, CylError> {
        let mut expr = self.parse_factor_internal(stop_at_left_brace)?;
        while let Some(op) = self.match_binary_op(&[Token::Plus, Token::Minus]) {
            let right = self.parse_factor_internal(stop_at_left_brace)?;
            expr = Expression::BinaryOp {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_factor_internal(&mut self, stop_at_left_brace: bool) -> Result<Expression, CylError> {
        let mut expr = self.parse_unary_internal(stop_at_left_brace)?;
        while let Some(op) = self.match_binary_op(&[Token::Multiply, Token::Divide, Token::Modulo])
        {
            let right = self.parse_unary_internal(stop_at_left_brace)?;
            expr = Expression::BinaryOp {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_unary_internal(&mut self, stop_at_left_brace: bool) -> Result<Expression, CylError> {
        if self.match_token(&Token::Minus) {
            let right = self.parse_unary_internal(stop_at_left_brace)?;
            Ok(Expression::UnaryOp {
                operator: UnaryOperator::Minus,
                operand: Box::new(right),
            })
        } else if self.match_token(&Token::Not) {
            let right = self.parse_unary_internal(stop_at_left_brace)?;
            Ok(Expression::UnaryOp {
                operator: UnaryOperator::Not,
                operand: Box::new(right),
            })
        } else if self.match_token(&Token::Await) {
            let right = self.parse_unary_internal(stop_at_left_brace)?;
            Ok(Expression::UnaryOp {
                operator: UnaryOperator::Await,
                operand: Box::new(right),
            })
        } else {
            let primary = self.parse_primary_internal(stop_at_left_brace)?;
            self.parse_postfix_internal(primary, stop_at_left_brace)
        }
    }

    fn parse_postfix_internal(
        &mut self,
        mut expr: Expression,
        stop_at_left_brace: bool,
    ) -> Result<Expression, CylError> {
        loop {
            if self.match_token(&Token::Dot) {
                // Member access: expr.identifier
                let member = match &self.peek().token {
                    Token::Identifier(name) => {
                        let name = name.clone();
                        self.advance();
                        name
                    }
                    _ => {
                        return Err(CylError::ParseError {
                            message: "Expected identifier after '.'".to_string(),
                            line: self.peek().line,
                            column: self.peek().column,
                        });
                    }
                };
                expr = Expression::MemberAccess {
                    object: Box::new(expr),
                    property: member,
                };
            } else if self.check(&Token::LeftParen) {
                // Function call: expr(...)
                self.advance();
                let mut args = Vec::new();
                if !self.check(&Token::RightParen) {
                    loop {
                        args.push(self.parse_expression()?);
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                    }
                }
                self.consume(
                    Token::RightParen,
                    "Expected ')' after function call arguments",
                )?;
                expr = Expression::Call {
                    callee: Box::new(expr),
                    arguments: args,
                };
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn parse_primary_internal(&mut self, stop_at_left_brace: bool) -> Result<Expression, CylError> {
        match &self.peek().token {
            Token::Match => {
                return Err(CylError::ParseError {
                    message: "'match' can only be used as a statement, not as an expression"
                        .to_string(),
                    line: self.peek().line,
                    column: self.peek().column,
                });
            }
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
            Token::Null => {
                self.advance();
                Ok(Expression::Null)
            }
            Token::Dynamic => {
                self.advance();
                Ok(Expression::Dynamic)
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();
                if self.check(&Token::LeftBrace) && stop_at_left_brace {
                    // When stop_at_left_brace is true, we're in a pattern context
                    // or similar where we need to stop at '{' - just return the identifier
                    Ok(Expression::Identifier(name))
                } else if self.check(&Token::LeftBrace) && !stop_at_left_brace {
                    self.advance();
                    let mut fields = std::collections::HashMap::new();
                    while !self.check(&Token::RightBrace) && !self.is_at_end() {
                        if let Token::Identifier(field_name) = &self.peek().token {
                            let field_name = field_name.clone();
                            self.advance();
                            self.consume(Token::Colon, "Expected ':' in struct literal")?;
                            let value = self.parse_expression()?;
                            fields.insert(field_name, value);
                            if !self.match_token(&Token::Comma) {
                                break;
                            }
                        } else {
                            return Err(CylError::ParseError {
                                message: "Expected field name in struct literal".to_string(),
                                line: self.peek().line,
                                column: self.peek().column,
                            });
                        }
                    }
                    self.consume(Token::RightBrace, "Expected '}' after struct literal")?;
                    let mut obj = fields;
                    obj.insert(
                        "__struct_name__".to_string(),
                        Expression::StringLiteral(name.clone()),
                    );
                    Ok(Expression::ObjectLiteral(obj))
                } else if self.check(&Token::Assign) {
                    self.advance();
                    let value = self.parse_expression()?;
                    Ok(Expression::Assignment {
                        target: Box::new(Expression::Identifier(name)),
                        value: Box::new(value),
                    })
                } else {
                    Ok(Expression::Identifier(name))
                }
            }
            Token::LeftParen => {
                self.advance();
                // Check for tuple literal: (expr, ...)
                let first = self.parse_expression()?;
                if self.check(&Token::Comma) {
                    // Tuple literal
                    let mut elements = vec![first];
                    while self.match_token(&Token::Comma) {
                        // Allow trailing comma
                        if self.check(&Token::RightParen) {
                            break;
                        }
                        elements.push(self.parse_expression()?);
                    }
                    self.consume(Token::RightParen, "Expected ')' after tuple literal")?;
                    Ok(Expression::TupleLiteral(elements))
                } else {
                    // Parenthesized expression
                    self.consume(Token::RightParen, "Expected ')' after expression")?;
                    Ok(first)
                }
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
}
