addi $t0, $zero, 10
lui $t1, 0x1001

addi $t0, $t0, 2
sw $t0, ($t1)

lw $t2, ($t1)