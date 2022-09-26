section .text
global fooasm

fooasm:
    mov rax, rdi
    imul rax, 2
    ret
