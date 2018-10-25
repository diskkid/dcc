use super::ir::{IR};

pub fn gen_x86(instructions: Vec<IR>) {
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

