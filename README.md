# mips-simulator
A MIPS simulator written in Rust

Currently it can perform basic arithmetic and logical instructions in addition to jumping to labels.

# Usage

Sample final output for input:
```mips
addi $t0, $zero, 0x0f00
srl $t1, $t0, 4
sll $t2, $t0, 4
```

```
+----+----------+
| 0  | 0        |
+----+----------+
| 1  | 0        |
+----+----------+
| 2  | 0        |
+----+----------+
| 3  | 0        |
+----+----------+
| 4  | 0        |
+----+----------+
| 5  | 0        |
+----+----------+
| 6  | 0        |
+----+----------+
| 7  | 0        |
+----+----------+
| 8  | f00      |
+----+----------+
| 9  | f0       |
+----+----------+
| 10 | f000     |
+----+----------+
| 11 | 0        |
+----+----------+
| 12 | 0        |
+----+----------+
| 13 | 0        |
+----+----------+
| 14 | 0        |
+----+----------+
| 15 | 0        |
+----+----------+
| 16 | 0        |
+----+----------+
| 17 | 0        |
+----+----------+
| 18 | 0        |
+----+----------+
| 19 | 0        |
+----+----------+
| 20 | 0        |
+----+----------+
| 21 | 0        |
+----+----------+
| 22 | 0        |
+----+----------+
| 23 | 0        |
+----+----------+
| 24 | 0        |
+----+----------+
| 25 | 0        |
+----+----------+
| 26 | 0        |
+----+----------+
| 27 | 0        |
+----+----------+
| 28 | 0        |
+----+----------+
| 29 | 7fffeffc |
+----+----------+
| 30 | 0        |
+----+----------+
| 31 | 0        |
+----+----------+
| PC | 40000c   |
+----+----------+
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