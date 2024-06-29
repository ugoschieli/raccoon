	.text
	.globl	main
main:
	pushq %rbp
	leaq my_string, %rdi
	call printf
	leaq my_string2, %rdi
	call printf
	xorq %rax, %rax
	popq %rbp
	ret
	.data
my_string:
		.asciz "Hello"
my_string2:
		.asciz " World\n"
