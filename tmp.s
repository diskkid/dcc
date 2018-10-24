.intel_syntax noprefix
.global main
main:
  mov rdi, 1
  mov rsi, 2
  add rdi, rsi
  mov r10, 3
  add rdi, r10
  mov r11, 4
  add rdi, r11
  mov r12, 5
  add rdi, r12
  mov r13, 6
  add rdi, r13
  mov r14, 7
  add rdi, r14
  mov r15, 8
  add rdi, r15
  mov rax, rdi
  ret
