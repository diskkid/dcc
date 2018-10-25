use super::parse::{Tree, Op};

pub enum IR {
    IMM(usize, i64),
    MOV(usize, usize),
    RETURN(usize),
    ADD(usize, usize),
    SUB(usize, usize),
    KILL(usize),
    NOP,
}

pub fn gen_ir(tree: Tree) -> Vec<IR> {
    let mut instructions = vec![];
    let mut cur = 0;
    let r = gen_ir_sub(tree, &mut instructions, &mut cur);
    instructions.push(IR::RETURN(r));
    instructions
}

pub fn gen_ir_sub(tree: Tree,
                  mut instructions: &mut Vec<IR>,
                  mut cur: &mut usize) -> usize {
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
