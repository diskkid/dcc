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

enum IR {
    IMM(usize, i64),
    MOV(usize, usize),
    RETURN(usize),
    ADD(usize, usize),
    SUB(usize, usize),
    KILL(usize),
    NOP,
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

fn gen_x86(instructions: Vec<IR>) {
    let regs = ["rdi", "rsi", "r10", "r11", "r12", "r13", "r14", "r15"];
    for ir in instructions {
        match ir {
            IR::IMM(lhs, rhs) => {
                println!("  mov {}, {}", regs[lhs], rhs);
            },
            IR::MOV(lhs, rhs) => {
                println!("  mov {}, {}", regs[lhs], regs[rhs]);
            },
            IR::RETURN(lhs) => {
                println!("  mov rax, {}", regs[lhs]);
                println!("  ret");
            },
            IR::ADD(lhs, rhs) => {
                println!("  add {}, {}", regs[lhs], regs[rhs]);
            },
            IR::SUB(lhs, rhs) => {
                println!("  sub {}, {}", regs[lhs], regs[rhs]);
            },
            IR::KILL(_) => {},
            IR::NOP => {},
        }
    }
}

fn gen_ir(tree: Tree) -> Vec<IR> {
    let mut instructions = vec![];
    let mut cur = 0;
    let r = gen_ir_sub(tree, &mut instructions, &mut cur);
    instructions.push(IR::RETURN(r));
    instructions
}

fn gen_ir_sub(tree: Tree, mut instructions: &mut Vec<IR>, mut cur: &mut usize) -> usize {
    match tree {
        Tree::Number(v) => {
            let reg = *cur;
            *cur += 1;
            instructions.push(IR::IMM(reg, v));
            reg
        },
        Tree::Node(op, lhs, rhs) => {
            let dst = gen_ir_sub(*lhs, &mut instructions, &mut cur);
            let src = gen_ir_sub(*rhs, &mut instructions, &mut cur);
            match op {
                Op::Plus => {
                    instructions.push(IR::ADD(dst, src));
                },
                Op::Minus => {
                    instructions.push(IR::SUB(dst, src));
                },
            }
            instructions.push(IR::KILL(src));
            dst
        },
    }
}

fn alloc(reg_map: &mut [Option<usize>], used: &mut [bool], ir_reg: usize) -> usize {
    let regs = ["rdi", "rsi", "r10", "r11", "r12", "r13", "r14", "r15"];

    match reg_map[ir_reg] {
        Some(r) => r,
        None => {
            for i in 0..regs.len() {
                if used[i] {
                    continue
                } else {
                    used[i] = true;
                    reg_map[ir_reg] = Some(i);
                    return i
                }
            }
            panic!("Could not assign register");
        },
    }
}

fn kill(used: &mut [bool], r: usize) {
    used[r] = false;
}

fn alloc_regs(instructions: &mut Vec<IR>, mut reg_map: &mut [Option<usize>]) {
    let mut used = [false;1000];
    for ir in instructions {
        match ir {
            IR::IMM(lhs, rhs) => {
                *ir = IR::IMM(alloc(&mut reg_map, &mut used, *lhs), *rhs);
            },
            IR::MOV(lhs, rhs) => {
                *ir = IR::MOV(alloc(&mut reg_map, &mut used, *lhs), alloc(&mut reg_map, &mut used, *rhs));
            },
            IR::ADD(lhs, rhs) => {
                *ir = IR::ADD(alloc(&mut reg_map, &mut used, *lhs), alloc(&mut reg_map, &mut used, *rhs));
            },
            IR::SUB(lhs, rhs) => {
                *ir = IR::SUB(alloc(&mut reg_map, &mut used, *lhs), alloc(&mut reg_map, &mut used, *rhs));
            },
            IR::RETURN(lhs) => {
                match reg_map[*lhs] {
                    Some(r) => {
                        kill(&mut used, r);
                    },
                    None => panic!("Used register is not assigned.")
                }
            },
            IR::KILL(lhs) => {
                match reg_map[*lhs] {
                    Some(r) => {
                        kill(&mut used, r);
                    },
                    None => panic!("Used register is not assigned.")
                }
                *ir = IR::NOP;
            },
            IR::NOP => {},
        }
    }
}

fn main() {
    let mut args = std::env::args();
    if let Some(program) = args.nth(1) {
        let tokens = tokenize(program);
        let mut tokens = tokens.iter().peekable();
        let tree = expr(&mut tokens);
        let mut instructions = gen_ir(tree);
        let mut reg_map = [None;1000];
        alloc_regs(&mut instructions, &mut reg_map);

        // Print the prologue
        println!(".intel_syntax noprefix");
        println!(".global main");
        println!("main:");
        gen_x86(instructions);
    }
}
