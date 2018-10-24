use std::iter::Peekable;

#[derive(Debug)]
enum TokenType {
    Number(i64),
    Plus,
    Minus,
    EOF,
}

#[derive(Debug)]
struct Token {
    t: TokenType,
    input: String,
}

#[derive(Debug)]
enum Op {
    Plus,
    Minus,
}

#[derive(Debug)]
enum Tree {
    Number(i64),
    Node(Op, Box<Tree>, Box<Tree>),
}

impl Tree {
    fn new(op: Op, lhs: Tree, rhs: Tree) -> Tree {
        Tree::Node(
            op,
            Box::new(lhs),
            Box::new(rhs)
        )
    }

    fn new_num_node(value: i64) -> Tree {
        Tree::Number(value)
    }
}

fn tokenize(program: String) -> Vec<Token> {
    let mut chars = program.chars().peekable();
    let mut tokens = vec![];
    while let Some(c) = chars.next() {
        match c {
            ' ' => continue,
            '+' => {
                tokens.push(Token{
                    t: TokenType::Plus,
                    input: c.to_string()
                });
            },
            '-' => {
                tokens.push(Token{
                    t: TokenType::Minus,
                    input: c.to_string()
                });
            },
            '0'...'9' => {
                let mut num = c.to_string();
                loop {
                    if let Some(c) = chars.peek() {
                        match c {
                            '0'...'9' => {
                                num.push(*c);
                            },
                            _ => break,
                        }
                        chars.next();
                    } else {
                        break;
                    }
                }
                let val = num.parse::<i64>().unwrap();
                tokens.push(Token{
                    t: TokenType::Number(val),
                    input: num,
                });
            },
            _ => panic!("Unexpected char {}", c)
        }
    }
    tokens
}

fn number<'a, I: Iterator<Item = &'a Token>>(tokens: &mut Peekable<I>) -> Tree {
    if let Some(token) = tokens.next() {
        match token.t {
            TokenType::Number(v) => Tree::new_num_node(v),
            _ => panic!("Unexpected token {:?}", token),
        }
    } else {
        panic!("No token");
    }
}

fn expr<'a, I: Iterator<Item = &'a Token>>(mut tokens: &mut Peekable<I>) -> Tree {
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

fn gen(tree: Tree) -> String {
    let regs = ["rdi", "rsi", "r10", "r11", "r12", "r13", "r14", "r15"];
    let mut cur = 0;
    gen_main(tree, &regs, &mut cur)
}

fn gen_main(tree: Tree, regs: &[&str], mut cur: &mut usize) -> String {
    match tree {
        Tree::Number(v) => {
            let reg = regs[*cur];
            *cur += 1;
            println!("  mov {}, {}", reg, v);
            reg.to_string()
        }
        Tree::Node(op, lhs, rhs) => {
            let dst = gen_main(*lhs, &regs, &mut cur);
            let src = gen_main(*rhs, &regs, &mut cur);
            match op {
                Op::Plus => {
                    println!("  add {}, {}", dst, src);
                    dst.to_string()
                },
                Op::Minus => {
                    println!("  sub {}, {}", dst, src);
                    dst.to_string()
                },
            }
        },
    }
}

fn main() {
    let mut args = std::env::args();
    if let Some(program) = args.nth(1) {
        let tokens = tokenize(program);
        let mut tokens = tokens.iter().peekable();
        let tree = expr(&mut tokens);

        // Print the prologue
        println!(".intel_syntax noprefix");
        println!(".global main");
        println!("main:");
        println!("  mov rax, {}", gen(tree));
        println!("  ret");
    }
}
