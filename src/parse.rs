use super::token::{Token, TokenType};
use std::iter::Peekable;

#[derive(Debug)]
pub enum Tree {
    Number(i64),
    Node(Op, Box<Tree>, Box<Tree>),
}

#[derive(Debug)]
pub enum Op {
    Plus,
    Minus,
    Mul,
    Div,
}

impl Tree {
    pub fn new(op: Op, lhs: Tree, rhs: Tree) -> Tree {
        Tree::Node(op, Box::new(lhs), Box::new(rhs))
    }

    pub fn new_num_node(value: i64) -> Tree {
        Tree::Number(value)
    }
}

pub fn number<'a, I>(tokens: &mut Peekable<I>) -> Tree
where
    I: Iterator<Item = &'a Token>,
{
    if let Some(token) = tokens.next() {
        match token.t {
            TokenType::Number(v) => Tree::new_num_node(v),
            _ => panic!("Unexpected token {:?}", token),
        }
    } else {
        panic!("No token found in number()")
    }
}

pub fn mul<'a, I>(mut tokens: &mut Peekable<I>) -> Tree
where
    I: Iterator<Item = &'a Token>,
{
    let mut lhs = number(&mut tokens);
    while let Some(token) = tokens.peek() {
        match token.t {
            TokenType::Mul => {
                tokens.next();
                let rhs = number(&mut tokens);
                lhs = Tree::new(Op::Mul, lhs, rhs)
            }
            TokenType::Div => {
                tokens.next();
                let rhs = number(&mut tokens);
                lhs = Tree::new(Op::Div, lhs, rhs)
            }
            _ => return lhs,
        }
    }
    panic!("No token found in mul()")
}

pub fn expr<'a, I>(mut tokens: &mut Peekable<I>) -> Tree
where
    I: Iterator<Item = &'a Token>,
{
    let mut lhs = mul(&mut tokens);
    while let Some(token) = tokens.peek() {
        match token.t {
            TokenType::Plus => {
                tokens.next();
                let rhs = mul(&mut tokens);
                lhs = Tree::new(Op::Plus, lhs, rhs)
            }
            TokenType::Minus => {
                tokens.next();
                let rhs = mul(&mut tokens);
                lhs = Tree::new(Op::Minus, lhs, rhs)
            }
            _ => return lhs,
        }
    }
    panic!("No token found in expr()")
}

pub fn parse(tokens: Vec<Token>) -> Tree {
    let mut tokens = tokens.iter().peekable();
    expr(&mut tokens)
}
