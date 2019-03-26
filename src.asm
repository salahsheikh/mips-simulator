j main
foobar:
addi $t0, $zero, 1
jr $ra

main:
addi $t1, $zero, 2
jal foobar
addi $t2, $zero, 3
lui $s0, 0x1001
sw $t2, ($s0)
lw $t3, ($s0)