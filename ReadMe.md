# falog_tiny_cpu

## 概要

Rustで実装した最小構成のCPUエミュレータです。

レジスタ、メモリ、プログラムカウンタを持つシンプルなCPUを構築し、
基本的な命令（ADD / SUB / JMP / JE など）を用いて計算を行います。

### 特徴

- シンプルな命令セット
- fetch → execute の実行サイクル
- 加算・減算・乗算（繰り返し）を実装
- デバッグ出力による状態遷移の可視化

### 割り算について

このCPUはゼロ判定（JE）のみをサポートしており、
大小比較（>=）を直接表現することができません。

そのため、減算を用いた割り算アルゴリズムは
「割り切れる場合」に限って正しく動作します。

割り切れない場合（例: 7 / 2）は、
終了条件を満たせず無限ループになります。

この制約は、命令セットの表現力がアルゴリズムの正しさに
どのような影響を与えるかを示しています。

---

## Summary

A minimal CPU emulator implemented in Rust.

This project builds a simple CPU with registers, memory, and a program counter,
and executes basic instructions such as ADD, SUB, JMP, and JE.

### Features

- Minimal instruction set
- Fetch → Execute cycle
- Addition, subtraction, and multiplication (via repeated addition)
- Debug output for visualizing internal state transitions

### About Division

This CPU only supports zero-based branching (JE) and does not provide
a way to express relational comparisons such as `>=`.

Because of this limitation, division using repeated subtraction
works correctly only when the result is an exact integer.

For non-divisible cases (e.g., 7 / 2), the program cannot terminate
and results in an infinite loop.

This limitation highlights how the expressiveness of an instruction set
affects the correctness of algorithms.

---

## 🚀 Example

```rust
let mut cpu = CPU::new();

println!("2 + 3 = {}", cpu.run_add(2, 3));      // 5
println!("5 - 2 = {}", cpu.run_sub(5, 2));      // 3
println!("3 * 4 = {}", cpu.run_mul(3, 4));      // 12
println!("10 / 2 = {}", cpu.run_div(10, 2));    // 5
println!("7 / 2 = {}", cpu.run_div(7, 2));      // infinite loop