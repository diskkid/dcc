fn main() {
    let mut args = std::env::args();
    if let Some(num) = args.nth(1) {
        println!(".intel_syntax noprefix
.global main
main:
  mov rax, {}
  ret
", num);
    }
}
