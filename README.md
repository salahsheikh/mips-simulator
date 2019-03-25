# mips-simulator
A MIPS simulator written in Rust

Currently it can perform basic arithmetic and logical instructions in addition to jumping to labels.

# Usage


Sample final output for input (includes both final data segment memory and register values): 
```assembly 
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
```
```
+-------+------------+----+------------+----+------------+----+------------+
| $zero | 0x00000000 | t0 | 0x00000001 | s0 | 0x10010000 | t8 | 0x00000000 |
+-------+------------+----+------------+----+------------+----+------------+
| at    | 0x00000000 | t1 | 0x00000002 | s1 | 0x00000000 | t9 | 0x00000000 |
+-------+------------+----+------------+----+------------+----+------------+
| v0    | 0x00000000 | t2 | 0x00000003 | s2 | 0x00000000 | k0 | 0x00000000 |
+-------+------------+----+------------+----+------------+----+------------+
| v1    | 0x00000000 | t3 | 0x00000003 | s3 | 0x00000000 | k1 | 0x00000000 |
+-------+------------+----+------------+----+------------+----+------------+
| a0    | 0x00000000 | t4 | 0x00000000 | s4 | 0x00000000 | gp | 0x00000000 |
+-------+------------+----+------------+----+------------+----+------------+
| a1    | 0x00000000 | t5 | 0x00000000 | s5 | 0x00000000 | sp | 0x7fffeffc |
+-------+------------+----+------------+----+------------+----+------------+
| a2    | 0x00000000 | t6 | 0x00000000 | s6 | 0x00000000 | fp | 0x00000000 |
+-------+------------+----+------------+----+------------+----+------------+
| a3    | 0x00000000 | t7 | 0x00000000 | s7 | 0x00000000 | ra | 0x00400010 |
+-------+------------+----+------------+----+------------+----+------------+
| PC    | 0x00400024                                     |    |            |
+-------+------------+----+------------+----+------------+----+------------+
0x10010000: 0x00000003
0x10010004: 0x00000000
0x10010008: 0x00000000
```

# Features

| Feature  | Status |
| ------------- | :-------------: |
| All R Instructions  | Incomplete  |
| All I Instructions  | Incomplete  |
| All J Instructions | Incomplete |
| Instruction parsing | Complete |
| Machine code parsing | Incomplete |
| Tests | Incomplete |
| Breakpoints | Incomplete |

# Dependences 
clap: 2.32
prettytable-rs: 0.8