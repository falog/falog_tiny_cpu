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

このCPUでは比較命令が存在しないため、
一般的な割り算（`a >= b` の判定を含む）は実装できません。

減算の繰り返しによって近似的に表現することは可能ですが、
割り切れる場合にのみ正しく動作します。

この制約を通して、命令セットの設計がアルゴリズムに与える影響を学ぶことができます。

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

This CPU does not support comparison instructions,
so general division (which requires checking `a >= b`) cannot be fully implemented.

Division can be approximated using repeated subtraction,
but it only works correctly when the result is an exact integer.

This limitation demonstrates how instruction set design directly affects
what kind of algorithms can be implemented.

---

## 🚀 Example

```rust
let mut cpu = CPU::new();

println!("2 + 3 = {}", cpu.run_add(2, 3));
println!("5 - 2 = {}", cpu.run_sub(5, 2));
println!("3 * 4 = {}", cpu.run_mul(3, 4));
println!("10 / 2 = {}", cpu.run_div(10, 2));