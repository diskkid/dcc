use super::ir::IR;

fn alloc(reg_map: &mut [Option<usize>], used: &mut [bool], ir_reg: usize) -> usize {
    let regs = ["rdi", "rsi", "r10", "r11", "r12", "r13", "r14", "r15"];

    match reg_map[ir_reg] {
        Some(r) => r,
        None => {
            for i in 0..regs.len() {
                if used[i] {
                    continue;
                } else {
                    used[i] = true;
                    reg_map[ir_reg] = Some(i);
                    return i;
                }
            }
            panic!("Could not assign register");
        }
    }
}

fn kill(used: &mut [bool], r: usize) {
    used[r] = false;
}

pub fn alloc_regs(instructions: &mut Vec<IR>, mut reg_map: &mut [Option<usize>]) {
    let mut used = [false; 8];
    for ir in instructions {
        match ir {
            IR::IMM(lhs, rhs) => {
                *ir = IR::IMM(alloc(&mut reg_map, &mut used, *lhs), *rhs);
            }
            IR::MOV(lhs, rhs) => {
                *ir = IR::MOV(
                    alloc(&mut reg_map, &mut used, *lhs),
                    alloc(&mut reg_map, &mut used, *rhs),
                );
            }
            IR::ADD(lhs, rhs) => {
                *ir = IR::ADD(
                    alloc(&mut reg_map, &mut used, *lhs),
                    alloc(&mut reg_map, &mut used, *rhs),
                );
            }
            IR::SUB(lhs, rhs) => {
                *ir = IR::SUB(
                    alloc(&mut reg_map, &mut used, *lhs),
                    alloc(&mut reg_map, &mut used, *rhs),
                );
            }
            IR::MUL(lhs, rhs) => {
                *ir = IR::MUL(
                    alloc(&mut reg_map, &mut used, *lhs),
                    alloc(&mut reg_map, &mut used, *rhs),
                );
            }
            IR::DIV(lhs, rhs) => {
                *ir = IR::DIV(
                    alloc(&mut reg_map, &mut used, *lhs),
                    alloc(&mut reg_map, &mut used, *rhs),
                );
            }
            IR::RETURN(lhs) => match reg_map[*lhs] {
                Some(r) => {
                    kill(&mut used, r);
                }
                None => panic!("Used register is not assigned."),
            },
            IR::KILL(lhs) => {
                match reg_map[*lhs] {
                    Some(r) => {
                        kill(&mut used, r);
                    }
                    None => panic!("Used register is not assigned."),
                }
                *ir = IR::NOP;
            }
            IR::NOP => {}
        }
    }
}
