use crate::ast::{BinaryOperator, BlockStatement, Type, UnaryOperator};
use crate::error::CylError;
use crate::lexer::Token;

pub struct Parser {
    pub tokens: Vec<crate::lexer::TokenWithLocation>,
    pub current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<crate::lexer::TokenWithLocation>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn is_at_end(&self) -> bool {
        self.peek().token == Token::Eof
    }

    pub fn peek(&self) -> &crate::lexer::TokenWithLocation {
        &self.tokens[self.current]
    }

    pub fn previous(&self) -> &crate::lexer::TokenWithLocation {
        &self.tokens[self.current - 1]
    }

    pub fn advance(&mut self) -> &crate::lexer::TokenWithLocation {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    pub fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(&self.peek().token) == std::mem::discriminant(token)
        }
    }

    pub fn match_token(&mut self, token: &Token) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn consume(
        &mut self,
        token: Token,
        message: &str,
    ) -> Result<&crate::lexer::TokenWithLocation, CylError> {
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

    pub fn match_binary_op(&mut self, tokens: &[Token]) -> Option<BinaryOperator> {
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

    #[allow(dead_code)]
    pub fn match_unary_op(&mut self, tokens: &[Token]) -> Option<UnaryOperator> {
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

    pub fn parse_generics(&mut self) -> Result<Vec<String>, CylError> {
        let mut type_parameters = Vec::new();
        if self.check(&Token::Less) {
            self.advance();
            if self.check(&Token::Greater) {
                self.advance();
                return Ok(type_parameters);
            }
            while !self.check(&Token::Greater) && !self.is_at_end() {
                if let Token::Identifier(param) = &self.peek().token {
                    type_parameters.push(param.clone());
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
                } else if self.check(&Token::Greater) {
                    break;
                } else {
                    // allow no comma
                    break;
                }
            }
            self.consume(Token::Greater, "Expected '>' after generic parameters")?;
            // Do NOT advance here; leave at the next token after '>'
        }
        Ok(type_parameters)
    }

    pub fn parse_type(&mut self) -> Result<Type, CylError> {
        if self.check(&Token::LeftParen) {
            self.advance();
            let mut types = Vec::new();
            while !self.check(&Token::RightParen) && !self.is_at_end() {
                types.push(self.parse_type()?);
                if !self.check(&Token::RightParen) {
                    self.consume(Token::Comma, "Expected ',' between tuple types")?;
                }
            }
            self.consume(Token::RightParen, "Expected ')' after tuple types")?;
            let tuple_type = Type::Tuple(types);
            if self.match_token(&Token::Question) {
                return Ok(Type::Optional(Box::new(tuple_type)));
            } else {
                return Ok(tuple_type);
            }
        }
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
            Token::Dynamic => {
                self.advance();
                Type::Dynamic
            }
            Token::Null => {
                self.advance();
                Type::Null
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();
                // Support generic types: Identifier < type, ... >
                if self.check(&Token::Less) || self.check(&Token::LeftAngle) {
                    // Accept either Less or LeftAngle for '<'
                    self.advance();
                    let mut generic_types = Vec::new();
                    while !self.check(&Token::Greater)
                        && !self.check(&Token::RightAngle)
                        && !self.is_at_end()
                    {
                        let t = self.parse_type()?;
                        // Convert type to string for AST compatibility
                        generic_types.push(format!("{t:?}"));
                        if self.check(&Token::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    if self.check(&Token::Greater) || self.check(&Token::RightAngle) {
                        self.advance();
                    } else {
                        return Err(CylError::ParseError {
                            message: "Expected '>' after generic type parameters".to_string(),
                            line: self.peek().line,
                            column: self.peek().column,
                        });
                    }
                    Type::Generic(name, generic_types)
                } else {
                    Type::Custom(name)
                }
            }
            Token::LeftBracket => {
                self.advance();
                let element_type = self.parse_type()?;
                self.consume(Token::RightBracket, "Expected ']' after array element type")?;
                Type::Array(Box::new(element_type))
            }
            _ => {
                return Err(CylError::ParseError {
                    message: "Expected type".to_string(),
                    line: self.peek().line,
                    column: self.peek().column,
                })
            }
        };
        if self.match_token(&Token::Question) {
            Ok(Type::Optional(Box::new(base_type)))
        } else {
            Ok(base_type)
        }
    }

    pub fn parse_block(&mut self) -> Result<BlockStatement, CylError> {
        self.consume(Token::LeftBrace, "Expected '{'")?;
        let mut statements = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            // Skip any stray semicolons between statements
            while self.check(&Token::Semicolon) {
                self.advance();
            }
            if self.check(&Token::RightBrace) || self.is_at_end() {
                break;
            }
            statements.push(self.parse_statement()?);
        }
        self.consume(Token::RightBrace, "Expected '}'")?;
        Ok(BlockStatement { statements })
    }
}
