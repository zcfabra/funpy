use crate::{
    ast::{AssignmentStmt, BinaryExpr, Identifier, IntegerNode, Node},
    token::{self, TokenType},
};
use token::Token;

#[derive(Debug)]
pub enum ParseError {
    InvalidTypeData,
    NotImplementedToken(String),
    ReachedEnd,
}

pub struct Parser {
    tokens: Vec<Token>,
    n_tokens: usize,
    l: usize,
    r: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Precedence {
    Lowest,
    AddSub,
    MulDiv,
    EqNotEq,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        return Parser {
            n_tokens: tokens.len(),
            tokens: tokens,
            l: 0,
            r: 0,
        };
    }
    fn step(&mut self) {
        self.r += 1;
        self.l = self.r;
    }
    fn incr_leading(&mut self) -> Result<(), ParseError> {
        if self.r == self.n_tokens {
            return Err(ParseError::ReachedEnd);
        }

        self.r += 1;
        return Ok(());
    }

    fn incr_trailing(&mut self) {
        self.l += 1;
    }

    pub fn parse(
        &mut self,
        precedence: Precedence,
    ) -> Result<Box<dyn Node>, ParseError> {
        let mut node: Option<Box<dyn Node>> = None;
        while self.r < self.n_tokens {
            let tok = self.tokens[self.r].clone();
            match tok.ttype {
                TokenType::Add
                | TokenType::Sub
                | TokenType::Mul
                | TokenType::Div => {
                    let new_precedence = Self::get_precedence(&tok.ttype);
                    if new_precedence < precedence {
                        if node.is_none() {
                            node = Some(self.get_operand_node()?);
                        }
                        return Ok(node.unwrap());
                    }
                    if node.is_none() {
                        node = Some(self.get_operand_node()?);
                    }
                    self.step();
                    node = Some(Self::get_binary_node(
                        tok.clone(),
                        node.unwrap(),
                        self.parse(new_precedence)?,
                    )?);
                }
                TokenType::LParen => {
                    self.incr_leading()?;
                    node = Some(self.parse(Precedence::Lowest)?);
                }
                TokenType::RParen => {
                    if node.is_none() {
                        node = Some(self.get_operand_node()?);
                    }
                    self.incr_leading()?;
                    return Ok(node.unwrap());
                }
                TokenType::Assignment => {
                    let identifier: Identifier =
                        Identifier::new(self.tokens[self.l].clone());
                    self.step();
                    let expr = self.parse(Precedence::Lowest)?;
                    return Ok(Box::new(AssignmentStmt::new(
                        identifier, expr,
                    )));
                }
                _ => {
                    self.incr_leading()?;
                }
            }
        }

        if node.is_none() {
            node = Some(self.get_operand_node()?);
        }
        return Ok(node.unwrap());
    }

    fn get_precedence(token_type: &TokenType) -> Precedence {
        match token_type {
            TokenType::Add | TokenType::Sub => Precedence::AddSub,
            TokenType::Mul | TokenType::Div => Precedence::MulDiv,
            _ => Precedence::Lowest,
        }
    }

    pub fn get_operand_node(&mut self) -> Result<Box<dyn Node>, ParseError> {
        while self.l < self.n_tokens
            && self.tokens[self.l].ttype == TokenType::LParen
        {
            self.incr_trailing();
        }
        let tok = self.tokens[self.l].clone();
        match tok.ttype {
            TokenType::Int => {
                return Ok(Box::new(IntegerNode::new(tok)?));
            }
            TokenType::Identifier => {
                return Ok(Box::new(Identifier::new(tok)))
            }
            _ => {
                return Err(ParseError::InvalidTypeData);
            }
        }
    }

    pub fn get_binary_node(
        op_token: Token,
        l: Box<dyn Node>,
        r: Box<dyn Node>,
    ) -> Result<Box<dyn Node>, ParseError> {
        return Ok(Box::new(BinaryExpr::new(op_token, l, r)));
    }
}
