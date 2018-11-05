use super::token::{Token, TokenType};
use std::iter::Peekable;

#[derive(Debug)]
pub enum Tree {
    Number(i64),
    ReturnStmt(Box<Tree>),
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
    fn new(op: Op, lhs: Tree, rhs: Tree) -> Tree {
        Tree::Node(op, Box::new(lhs), Box::new(rhs))
    }

    fn new_num_node(value: i64) -> Tree {
        Tree::Number(value)
    }
}

fn number<'a, I>(tokens: &mut Peekable<I>) -> Tree
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

fn mul<'a, I>(mut tokens: &mut Peekable<I>) -> Tree
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

fn expr<'a, I>(mut tokens: &mut Peekable<I>) -> Tree
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

fn stmt<'a, I>(mut tokens: &mut Peekable<I>) -> Tree
where I: Iterator<Item = &'a Token> {
    expect(&mut tokens, TokenType::Return);
    tokens.next();
    let tree = expr(&mut tokens);
    expect(&mut tokens, TokenType::Semicolon);
    tokens.next();
    Tree::ReturnStmt(Box::new(tree))
}

fn expect<'a, I>(mut tokens: &mut Peekable<I>, token_type: TokenType)
where I: Iterator<Item = &'a Token> {
    if let Some(token) = tokens.peek() {
        if token.t != token_type {
            panic!("Expect {:?} but got {:?}", token_type, token.t);
        }
    } else {
        panic!("No token found");
    }
}

pub fn parse(tokens: Vec<Token>) -> Tree {
    let mut tokens = tokens.iter().peekable();
    stmt(&mut tokens)
}
