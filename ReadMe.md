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

### 割り算について

このCPUはゼロ判定（JE）のみをサポートしており、
大小比較（>=）を直接表現することができません。

そのため、減算を用いた割り算アルゴリズムを
正しく終了させることができず、
現在の実装では結果は常に正しくなりません（多くの場合 0 になります）。

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

Because of this limitation, a correct division algorithm based on
repeated subtraction cannot be properly terminated.

As a result, the current implementation does not produce correct results
and often returns 0.

This limitation highlights how the expressiveness of an instruction set
affects the correctness of algorithms.

---

## 🚀 Example

```rust
let mut cpu = CPU::new();

println!("2 + 3 = {}", cpu.run_add(2, 3));
println!("5 - 2 = {}", cpu.run_sub(5, 2));
println!("3 * 4 = {}", cpu.run_mul(3, 4));
println!("10 / 2 = {}", cpu.run_div(10, 2));
println!("7 / 2 = {}", cpu.run_div(7, 2));