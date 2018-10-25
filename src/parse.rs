use std::iter::Peekable;
use super::token::{Token, TokenType};

#[derive(Debug)]
pub enum Tree {
    Number(i64),
    Node(Op, Box<Tree>, Box<Tree>),
}

#[derive(Debug)]
pub enum Op {
    Plus,
    Minus,
}

impl Tree {
    pub fn new(op: Op, lhs: Tree, rhs: Tree) -> Tree {
        Tree::Node(
            op,
            Box::new(lhs),
            Box::new(rhs)
        )
    }

    pub fn new_num_node(value: i64) -> Tree {
        Tree::Number(value)
    }
}

pub fn number<'a, I>(tokens: &mut Peekable<I>) -> Tree
where I: Iterator<Item = &'a Token> {
    if let Some(token) = tokens.next() {
        match token.t {
            TokenType::Number(v) => Tree::new_num_node(v),
            _ => panic!("Unexpected token {:?}", token),
        }
    } else {
        panic!("No token");
    }
}

pub fn expr<'a, I>(mut tokens: &mut Peekable<I>) -> Tree
where I: Iterator<Item = &'a Token> {
    let mut lhs = number(&mut tokens);
    while let Some(token) = tokens.next() {
        let op = match token.t {
            TokenType::Plus => Op::Plus,
            TokenType::Minus => Op::Minus,
            _ => panic!("Unexpected token {:?}", token),
        };
        let rhs = number(&mut tokens);
        lhs = Tree::new(op, lhs, rhs);
    }
    lhs
}

pub fn parse(tokens: Vec<Token>) -> Tree {
    let mut tokens = tokens.iter().peekable();
    expr(&mut tokens)
}
