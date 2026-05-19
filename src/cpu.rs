const DEBUG: bool = true;

#[derive(Debug, Clone, Copy)]
enum Opcode {
    Add,
    Sub,
    Ld,
    St,
    Jmp,
    Je,
    Mov,
    Hlt,
}

impl Opcode {
    fn to_byte(self) -> u8 {
        match self {
            Opcode::Add => 0,
            Opcode::Sub => 1,
            Opcode::Ld => 2,
            Opcode::St => 3,
            Opcode::Jmp => 4,
            Opcode::Je => 5,
            Opcode::Mov => 6,
            Opcode::Hlt => 7,
        }
    }

    fn from_byte(byte: u8) -> Self {
        match byte {
            0 => Opcode::Add,
            1 => Opcode::Sub,
            2 => Opcode::Ld,
            3 => Opcode::St,
            4 => Opcode::Jmp,
            5 => Opcode::Je,
            6 => Opcode::Mov,
            7 => Opcode::Hlt,
            _ => panic!("unknown opcode: {}", byte),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    opcode: Opcode,
    op1: u8,
    op2: u8,
}

pub struct CPU {
    regs: [u8; 4],
    pc: usize,
    mem: [u8; 256],
    zero_flag: bool,
    halted: bool,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            regs: [0; 4],
            pc: 0,
            mem: [0; 256],
            zero_flag: false,
            halted: false,
        }
    }

    pub fn run_add(&mut self, a: u8, b: u8) -> u8 {
        self.set_reg(0, a);
        self.set_reg(1, b);

        self.load_program(&[
            Instruction {
                opcode: Opcode::Add,
                op1: 0,
                op2: 1,
            },
            Instruction {
                opcode: Opcode::Hlt,
                op1: 0,
                op2: 0,
            },
        ]);

        self.run();
        self.get_reg(0)
    }

    pub fn run_sub(&mut self, a: u8, b: u8) -> u8 {
        self.set_reg(0, a);
        self.set_reg(1, b);

        self.load_program(&[
            Instruction {
                opcode: Opcode::Sub,
                op1: 0,
                op2: 1,
            },
            Instruction {
                opcode: Opcode::Hlt,
                op1: 0,
                op2: 0,
            },
        ]);

        self.run();
        self.get_reg(0)
    }

    pub fn run_mul(&mut self, a: u8, b: u8) -> u8 {
        self.set_reg(0, 0);
        self.set_reg(1, b);
        self.set_reg(2, a);
        self.set_reg(3, 1);

        self.load_program(&[
            Instruction {
                opcode: Opcode::Add,
                op1: 0,
                op2: 2,
            },
            Instruction {
                opcode: Opcode::Sub,
                op1: 1,
                op2: 3,
            },
            Instruction {
                opcode: Opcode::Je,
                op1: 0,
                op2: 12,
            },
            Instruction {
                opcode: Opcode::Jmp,
                op1: 0,
                op2: 0,
            },
            Instruction {
                opcode: Opcode::Hlt,
                op1: 0,
                op2: 0,
            },
        ]);

        self.run();
        self.get_reg(0)
    }

    pub fn run_div(&mut self, a: u8, b: u8) -> u8 {
        self.set_reg(0, 0);
        self.set_reg(1, a);
        self.set_reg(2, b);
        self.set_reg(3, 1);

        self.load_program(&[
            Instruction {
                opcode: Opcode::Sub,
                op1: 1,
                op2: 2,
            },
            Instruction {
                opcode: Opcode::Add,
                op1: 0,
                op2: 3,
            },
            Instruction {
                opcode: Opcode::Je,
                op1: 0,
                op2: 12,
            },
            Instruction {
                opcode: Opcode::Jmp,
                op1: 0,
                op2: 0,
            },
            Instruction {
                opcode: Opcode::Hlt,
                op1: 0,
                op2: 0,
            },
        ]);
        self.run();
        self.get_reg(0)
    }

    fn set_reg(&mut self, idx: usize, val: u8) {
        if idx >= self.regs.len() {
            panic!("invalid register: {}", idx);
        }
        self.regs[idx] = val;
    }

    fn get_reg(&self, idx: usize) -> u8 {
        if idx >= self.regs.len() {
            panic!("invalid register: {}", idx);
        }
        self.regs[idx]
    }

    fn load_program(&mut self, program: &[Instruction]) {
        let mut i = 0;

        for inst in program {
            self.mem[i] = inst.opcode.to_byte();
            self.mem[i + 1] = inst.op1;
            self.mem[i + 2] = inst.op2;
            i += 3;
        }

        self.pc = 0;
        self.halted = false;
        self.zero_flag = false;
    }

    fn fetch(&self) -> Instruction {
        if self.pc + 2 >= self.mem.len() {
            panic!("PC out of bounds: {}", self.pc);
        }

        Instruction {
            opcode: Opcode::from_byte(self.mem[self.pc]),
            op1: self.mem[self.pc + 1],
            op2: self.mem[self.pc + 2],
        }
    }

    fn reg(&self, idx: u8) -> usize {
        let i = idx as usize;
        if i >= self.regs.len() {
            panic!("invalid register: {}", i);
        }
        i
    }

    fn addr(&self, addr: u8) -> usize {
        let a = addr as usize;
        if a >= self.mem.len() {
            panic!("invalid memory address: {}", a);
        }
        a
    }

    fn execute(&mut self, inst: Instruction) {
        match inst.opcode {
            Opcode::Add => {
                let r1 = self.reg(inst.op1);
                let r2 = self.reg(inst.op2);
                let result = self.regs[r1].wrapping_add(self.regs[r2]);
                self.regs[r1] = result;
                self.zero_flag = result == 0;
                self.pc += 3;
            }
            Opcode::Sub => {
                let r1 = self.reg(inst.op1);
                let r2 = self.reg(inst.op2);
                let result = self.regs[r1].wrapping_sub(self.regs[r2]);
                self.regs[r1] = result;
                self.zero_flag = result == 0;
                self.pc += 3;
            }
            Opcode::Ld => {
                let r1 = self.reg(inst.op1);
                let addr = self.addr(inst.op2);
                self.regs[r1] = self.mem[addr];
                self.pc += 3;
            }
            Opcode::St => {
                let r1 = self.reg(inst.op1);
                let addr = self.addr(inst.op2);
                self.mem[addr] = self.regs[r1];
                self.pc += 3;
            }
            Opcode::Jmp => {
                self.pc = self.addr(inst.op2);
            }
            Opcode::Je => {
                if self.zero_flag {
                    self.pc = self.addr(inst.op2);
                } else {
                    self.pc += 3;
                }
            }
            Opcode::Mov => {
                let r1 = self.reg(inst.op1);
                let r2 = self.reg(inst.op2);
                self.regs[r1] = self.regs[r2];
                self.pc += 3;
            }
            Opcode::Hlt => {
                self.halted = true;
            }
        }
    }

    pub fn step(&mut self) {
        let inst = self.fetch();

        if DEBUG {
            println!("PC={} {:?} \n-----------------------------------------------\nR0={} R1={} R2={} R3={} | ZF={}",
                self.pc,
                inst,
                self.regs[0],
                self.regs[1],
                self.regs[2],
                self.regs[3],
                self.zero_flag
            );
        }

        self.execute(inst);
    }

    pub fn run(&mut self) {
        while !self.halted {
            self.step();
        }
    }
}
