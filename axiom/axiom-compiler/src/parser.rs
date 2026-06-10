use std::iter::Peekable;
use std::vec::IntoIter;

use crate::ast::{BinOp, Expr, Parameter, Statement, UnaryOp};
use crate::lexer::Token;

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Precedence {
    Lowest = 1,
    Equals = 2,
    LessGreater = 3,
    Sum = 4,
    Product = 5,
    Prefix = 6,
    Call = 7,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    fn peek_token(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    fn next_token(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    fn peek_precedence(&mut self) -> Precedence {
        match self.peek_token() {
            Some(Token::EqualEqual) | Some(Token::NotEqual) => Precedence::Equals,
            Some(Token::Less) | Some(Token::Greater) | Some(Token::LessEqual) | Some(Token::GreaterEqual) => {
                Precedence::LessGreater
            }
            Some(Token::Plus) | Some(Token::Minus) => Precedence::Sum,
            Some(Token::Star) | Some(Token::Slash) | Some(Token::Percent) => Precedence::Product,
            Some(Token::LeftParen) => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }

    pub fn parse_program(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        while let Some(tok) = self.peek_token() {
            if *tok == Token::Eof {
                break;
            }
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
        }
        statements
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.peek_token() {
            Some(Token::Let) | Some(Token::Mut) => self.parse_let_statement(),
            Some(Token::Return) => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let tok = self.next_token()?;
        let mutable = tok == Token::Mut;

        let name = match self.next_token() {
            Some(Token::Identifier(id)) => id,
            _ => return None,
        };

        // Expect `=`
        match self.next_token() {
            Some(Token::Equal) => {}
            _ => return None,
        }

        let value = self.parse_expression(Precedence::Lowest)?;

        // Optional semicolon handling could be added here if needed, 
        // AXIOM spec says semicolons are unnecessary, so we just return.
        Some(Statement::Let {
            name,
            value,
            mutable,
        })
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        self.next_token(); // consume 'return'
        
        let value = if let Some(Token::RightBrace) | Some(Token::Eof) = self.peek_token() {
            None
        } else {
            self.parse_expression(Precedence::Lowest)
        };

        Some(Statement::Return(value))
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expr = self.parse_expression(Precedence::Lowest)?;
        Some(Statement::Expression(expr))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expr> {
        let tok = self.next_token()?;
        
        let mut left = match tok {
            Token::Identifier(id) => Expr::Identifier(id),
            Token::Number(val) => Expr::Number(val),
            Token::String(val) => Expr::String(val),
            Token::True => Expr::Boolean(true),
            Token::False => Expr::Boolean(false),
            Token::Null => Expr::Null,
            Token::Minus => {
                let right = self.parse_expression(Precedence::Prefix)?;
                Expr::UnaryOp {
                    op: UnaryOp::Negate,
                    operand: Box::new(right),
                }
            }
            Token::Bang | Token::Not => {
                let right = self.parse_expression(Precedence::Prefix)?;
                Expr::UnaryOp {
                    op: UnaryOp::Not,
                    operand: Box::new(right),
                }
            }
            Token::LeftParen => {
                let exp = self.parse_expression(Precedence::Lowest)?;
                if let Some(Token::RightParen) = self.next_token() {
                    exp
                } else {
                    return None; // Missing closing paren
                }
            }
            Token::LeftBrace => self.parse_block()?,
            _ => return None,
        };

        while let Some(tok) = self.peek_token() {
            if *tok == Token::Eof {
                break;
            }
            
            if precedence < self.peek_precedence() {
                let op_tok = self.next_token()?;
                left = self.parse_infix_expression(left, op_tok)?;
            } else {
                break;
            }
        }

        Some(left)
    }

    fn parse_infix_expression(&mut self, left: Expr, op_tok: Token) -> Option<Expr> {
        let op = match op_tok {
            Token::Plus => BinOp::Add,
            Token::Minus => BinOp::Subtract,
            Token::Star => BinOp::Multiply,
            Token::Slash => BinOp::Divide,
            Token::Percent => BinOp::Modulo,
            Token::EqualEqual => BinOp::Equal,
            Token::NotEqual => BinOp::NotEqual,
            Token::Less => BinOp::LessThan,
            Token::Greater => BinOp::GreaterThan,
            Token::LessEqual => BinOp::LessThanOrEqual,
            Token::GreaterEqual => BinOp::GreaterThanOrEqual,
            Token::AmpersandAmpersand | Token::And => BinOp::And,
            Token::PipePipe | Token::Or => BinOp::Or,
            _ => return None,
        };

        let precedence = match op_tok {
            Token::EqualEqual | Token::NotEqual => Precedence::Equals,
            Token::Less | Token::Greater | Token::LessEqual | Token::GreaterEqual => Precedence::LessGreater,
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Star | Token::Slash | Token::Percent => Precedence::Product,
            _ => Precedence::Lowest,
        };

        let right = self.parse_expression(precedence)?;

        Some(Expr::BinaryOp {
            left: Box::new(left),
            op,
            right: Box::new(right),
        })
    }

    fn parse_block(&mut self) -> Option<Expr> {
        let mut statements = Vec::new();

        while let Some(tok) = self.peek_token() {
            if *tok == Token::RightBrace || *tok == Token::Eof {
                break;
            }
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            } else {
                self.next_token(); // skip malformed
            }
        }

        if let Some(Token::RightBrace) = self.next_token() {
            Some(Expr::Block(statements))
        } else {
            None
        }
    }
}
