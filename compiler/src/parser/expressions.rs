use crate::ast::*;
use crate::error::CylError;
use crate::lexer::Token;
use super::helpers::*;

impl Parser {
    // Expression parsing logic
    // TODO: Move the real implementation from the old parser.rs
    // pub fn parse_expression(&mut self) -> Result<Expression, CylError> { ... }

    pub fn parse_expression(&mut self) -> Result<Expression, CylError> {
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
                // If next token is Assign, treat as assignment expression
                if self.check(&Token::Assign) {
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
                        if self.check(&Token::RightParen) { break; }
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
