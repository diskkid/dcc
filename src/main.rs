extern crate dcc;
use dcc::codegen;
use dcc::ir;
use dcc::parse;
use dcc::regalloc;
use dcc::token;

fn main() {
    let mut args = std::env::args();
    if let Some(program) = args.nth(1) {
        let tokens = token::tokenize(program);
        let tree = parse::parse(tokens);
        let mut instructions = ir::gen_ir(tree);
        let mut reg_map = [None; 1000];
        regalloc::alloc_regs(&mut instructions, &mut reg_map);
        println!(".intel_syntax noprefix");
        println!(".global main");
        println!("main:");
        codegen::gen_x86(instructions);
    }
}
