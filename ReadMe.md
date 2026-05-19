# falog_tiny_cpu

## 概要

Rustで実装した最小構成のCPUエミュレータです。

レジスタ、メモリ、プログラムカウンタを持つシンプルなCPUを構築し、
ADD / SUB / JMP / JE などの基本命令を実行します。

このプロジェクトは、CPUの基本的な動作だけでなく、
命令セットの設計がアルゴリズムの実装にどのような影響を与えるかを
理解することを目的としています。

---

## 特徴

- 最小限の命令セット
- fetch → execute の実行サイクル
- 加算・減算の実装
- 乗算は繰り返し加算で実現
- 除算は繰り返し減算で実現（制約あり）
- デバッグ出力による状態遷移の可視化

---

## 割り算について

このCPUはゼロ判定（JE: Jump if Equal to zero）のみをサポートしており、
`>=` や `<` といった大小比較を直接表現することができません。

そのため、減算を用いた除算アルゴリズムは、
結果が整数で割り切れる場合にのみ正しく動作します。

割り切れない場合（例: 7 / 2）は終了条件を判定できず、
無限ループに陥ります。

これは、命令セットの表現力がアルゴリズムの正しさや
実装可能性にどのように影響するかを示す例となっています。

---

## デバッグモード

デバッグ出力はデフォルトで無効になっています。

有効にするには `src/cpu.rs` の以下を変更してください：

```rust
const DEBUG: bool = false;
```

↓

```rust
const DEBUG: bool = true;
```

有効時には以下の情報が出力されます：

- プログラムカウンタ（PC）
- 実行中の命令
- レジスタ状態（R0〜R3）

CPUの内部動作をステップごとに確認できます。

---

## 実行例

```rust
let mut cpu = CPU::new();

println!("2 + 3 = {}", cpu.run_add(2, 3));      // 5
println!("5 - 2 = {}", cpu.run_sub(5, 2));      // 3
println!("3 * 4 = {}", cpu.run_mul(3, 4));      // 12
println!("10 / 2 = {}", cpu.run_div(10, 2));    // 5
println!("7 / 2 = {}", cpu.run_div(7, 2));      // 無限ループ
```

---

## 実行ログ例（デバッグ有効）

### 乗算: 3 * 4

```
R0=0 R1=4 R2=3 R3=1
PC=0 Instruction { opcode: Add, op1: 0, op2: 2 }

R0=3 R1=4 R2=3 R3=1
PC=3 Instruction { opcode: Sub, op1: 1, op2: 3 }

R0=3 R1=3 R2=3 R3=1
PC=6 Instruction { opcode: Je, op1: 1, op2: 12 }

...

R0=12 R1=0 R2=3 R3=1
PC=12 Instruction { opcode: Hlt, op1: 0, op2: 0 }

結果: 3 * 4 = 12
```

### 除算: 10 / 2

```
R0=0 R1=10 R2=2 R3=1
PC=0 Instruction { opcode: Sub, op1: 1, op2: 2 }

R0=0 R1=8 R2=2 R3=1
PC=3 Instruction { opcode: Add, op1: 0, op2: 3 }

...

R0=5 R1=0 R2=2 R3=1
PC=12 Instruction { opcode: Hlt, op1: 0, op2: 0 }

結果: 10 / 2 = 5
```

---

## 通常出力（デバッグ無効）

```
2 + 3 = 5
5 - 2 = 3
3 * 4 = 12
10 / 2 = 5
```

---

## 補足

本プロジェクトはあえて命令セットを最小限に抑えています。

命令を増やして機能を拡張するのではなく、
制約のある環境でどこまで表現できるかを通じて、
CPU設計とアルゴリズムの関係を理解することに重点を置いています。

---

## Overview

A minimal CPU emulator implemented in Rust.

This project simulates a simple CPU with registers, memory, and a program counter.
It executes a small set of instructions such as ADD, SUB, JMP, and JE.

The goal of this project is to explore how a CPU works at a fundamental level,
including how instruction design affects algorithm implementation.

---

## Features

- Minimal instruction set
- Fetch → Execute cycle
- Addition and subtraction
- Multiplication implemented via repeated addition
- Division implemented via repeated subtraction (with limitations)
- Debug output for visualizing internal state transitions

---

## About Division

This CPU only supports zero-based branching using the `JE` (Jump if Equal to zero) instruction.
It does not support relational comparisons such as `>=` or `<`.

Because of this limitation, division using repeated subtraction only works
when the result is an exact integer.

For non-divisible cases (e.g., `7 / 2`), the program cannot detect when to stop,
resulting in an infinite loop.

This demonstrates how the expressiveness of an instruction set
directly impacts the correctness and completeness of algorithms.

---

## Debug Mode

Debug output is disabled by default.

To enable it, edit the following line in `src/cpu.rs`:

```rust
const DEBUG: bool = false;
```

Change it to:

```rust
const DEBUG: bool = true;
```

When enabled, the CPU will print:

- Program Counter (PC)
- Current instruction
- Register states (R0–R3)

This helps visualize how the CPU executes each step.

---

## Example

```rust
let mut cpu = CPU::new();

println!("2 + 3 = {}", cpu.run_add(2, 3));      // 5
println!("5 - 2 = {}", cpu.run_sub(5, 2));      // 3
println!("3 * 4 = {}", cpu.run_mul(3, 4));      // 12
println!("10 / 2 = {}", cpu.run_div(10, 2));    // 5
println!("7 / 2 = {}", cpu.run_div(7, 2));      // infinite loop
```

---

## Example Output (Debug Enabled)

### Multiplication: 3 * 4

```
R0=0 R1=4 R2=3 R3=1
PC=0 Instruction { opcode: Add, op1: 0, op2: 2 }

R0=3 R1=4 R2=3 R3=1
PC=3 Instruction { opcode: Sub, op1: 1, op2: 3 }

R0=3 R1=3 R2=3 R3=1
PC=6 Instruction { opcode: Je, op1: 1, op2: 12 }

...

R0=12 R1=0 R2=3 R3=1
PC=12 Instruction { opcode: Hlt, op1: 0, op2: 0 }

Result: 3 * 4 = 12
```

### Division: 10 / 2

```
R0=0 R1=10 R2=2 R3=1
PC=0 Instruction { opcode: Sub, op1: 1, op2: 2 }

R0=0 R1=8 R2=2 R3=1
PC=3 Instruction { opcode: Add, op1: 0, op2: 3 }

...

R0=5 R1=0 R2=2 R3=1
PC=12 Instruction { opcode: Hlt, op1: 0, op2: 0 }

Result: 10 / 2 = 5
```

---

## Normal Output (Debug Disabled)

```
2 + 3 = 5
5 - 2 = 3
3 * 4 = 12
10 / 2 = 5
```

---

## Notes

This project is intentionally minimal.

Rather than adding more instructions,
it focuses on showing how limited instruction sets
affect what can and cannot be computed.
