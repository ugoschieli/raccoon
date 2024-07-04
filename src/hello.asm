global _start

section .text

_start:
  mov eax, 1        ; write(
  mov edi, 1        ;   STDOUT_FILENO,
  mov esi, hello      ;   "Hello, world!\n",
  mov edx, 15   ;   sizeof("Hello, world!\n")
  syscall           ; );

  mov eax, 60       ; exit(
  mov edi, 0        ;   EXIT_SUCCESS
  syscall           ; );

section .data
  hello: db "Hello, World !", 10
