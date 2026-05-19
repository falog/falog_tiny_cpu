mod cpu;
use cpu::CPU;

fn main() {
    let mut cpu = CPU::new();

    println!("2 + 3 = {}", cpu.run_add(2, 3));
    println!("5 - 2 = {}", cpu.run_sub(5, 2));
    println!("3 * 4 = {}", cpu.run_mul(3, 4));
    println!("10 / 2 = {}", cpu.run_div(10, 2));
    println!("7 / 2 = {}", cpu.run_div(7, 2));
}