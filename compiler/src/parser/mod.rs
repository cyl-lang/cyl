mod expressions;
mod functions;
pub mod helpers;
mod patterns;
mod statements;

// The main Parser struct and parse() entrypoint can remain here, delegating to the submodules.

use crate::ast::*;
use crate::error::CylError;
use crate::parser::helpers::Parser;

impl Parser {
    pub fn parse(&mut self) -> Result<Program, CylError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            // Skip stray semicolons or right braces at the top level (before and after parsing a statement)
            while self.peek().token == crate::lexer::Token::Semicolon
                || self.peek().token == crate::lexer::Token::RightBrace
            {
                self.advance();
            }
            // If the next token is Eof, break cleanly
            if self.peek().token == crate::lexer::Token::Eof {
                break;
            }
            // Only parse statements for valid top-level starters
            match &self.peek().token {
                crate::lexer::Token::Import
                | crate::lexer::Token::Fn
                | crate::lexer::Token::Async
                | crate::lexer::Token::Struct
                | crate::lexer::Token::Enum
                | crate::lexer::Token::Let
                | crate::lexer::Token::Const
                | crate::lexer::Token::Identifier(_)
                | crate::lexer::Token::Return
                | crate::lexer::Token::If
                | crate::lexer::Token::While
                | crate::lexer::Token::For
                | crate::lexer::Token::Match
                | crate::lexer::Token::Try
                | crate::lexer::Token::Break
                | crate::lexer::Token::Continue => {
                    statements.push(self.parse_statement()?);
                    // After parsing a statement, skip any stray semicolons or right braces
                    while self.peek().token == crate::lexer::Token::Semicolon
                        || self.peek().token == crate::lexer::Token::RightBrace
                    {
                        self.advance();
                    }
                }
                _ => {
                    // Unexpected token at top level, break
                    break;
                }
            }
        }
        Ok(Program { statements })
    }
}
