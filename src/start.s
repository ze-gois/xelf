   .text
   .align  4
   .globl  _start
   .hidden _start
   .section .text._start
   .type   _start,@function
   _start:
       # Store the original stack pointer
       mov     %rsp, %rdi
       mov     %rbp, %rsi
       call    entry

       # We shouldn't return, but clean up anyway
       mov     %rbp, %rsp
       pop     %rbp
       hlt
