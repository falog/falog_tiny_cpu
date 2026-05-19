const DEBUG: bool = false;

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

// 命令は3byte固定: [opcode, op1, op2]
// PCはバイト単位で進む（通常は +3）
// ジャンプ系は命令インデックスを受け取り、内部で *3 する
impl Opcode {
    fn to_byte(self) -> u8 {
        match self {
            // 0: r1 = r1 + r2
            Opcode::Add => 0,
            // 1: r1 = r1 - r2
            Opcode::Sub => 1,
            // 2: r1 = mem[addr]
            Opcode::Ld => 2,
            // 3: mem[addr] = r1
            Opcode::St => 3,
            // 4: PC = addr * 3
            Opcode::Jmp => 4,
            // 5: if r == 0 { PC = addr * 3 } else { PC += 3 }
            Opcode::Je => 5,
            // 6: r1 = r2
            Opcode::Mov => 6,
            // 7: halt
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
    halted: bool,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            regs: [0; 4],
            pc: 0,
            mem: [0; 256],
            halted: false,
        }
    }

    pub fn run_add(&mut self, a: u8, b: u8) -> u8 {
        const R0: usize = 0;
        const R1: usize = 1;

        self.set_reg(R0, a);
        self.set_reg(R1, b);

        self.load_program(&[
            Instruction {
                opcode: Opcode::Add,
                op1: R0 as u8,
                op2: R1 as u8,
            },
            Instruction {
                opcode: Opcode::Hlt,
                op1: 0,
                op2: 0,
            },
        ]);

        self.run();
        self.get_reg(R0)
    }

    pub fn run_sub(&mut self, a: u8, b: u8) -> u8 {
        const R0: usize = 0;
        const R1: usize = 1;

        self.set_reg(R0, a);
        self.set_reg(R1, b);

        self.load_program(&[
            Instruction {
                opcode: Opcode::Sub,
                op1: R0 as u8,
                op2: R1 as u8,
            },
            Instruction {
                opcode: Opcode::Hlt,
                op1: 0,
                op2: 0,
            },
        ]);

        self.run();
        self.get_reg(R0)
    }

    pub fn run_mul(&mut self, a: u8, b: u8) -> u8 {
        const RESULT: usize = 0;
        const COUNTER: usize = 1;
        const VALUE: usize = 2;
        const ONE: usize = 3;
        const LOOP: usize = 0;
        const HLT_ADDR: usize = 4;

        self.set_reg(RESULT, 0);
        self.set_reg(COUNTER, b);
        self.set_reg(VALUE, a);
        self.set_reg(ONE, 1);

        self.load_program(&[
            // LOOP:
            Instruction {
                opcode: Opcode::Add,
                op1: RESULT as u8,
                op2: VALUE as u8,
            },
            Instruction {
                opcode: Opcode::Sub,
                op1: COUNTER as u8,
                op2: ONE as u8,
            },
            Instruction {
                opcode: Opcode::Je,
                op1: COUNTER as u8,
                op2: HLT_ADDR as u8,
            },
            Instruction {
                opcode: Opcode::Jmp,
                op1: 0,
                op2: LOOP as u8,
            },
            // HLT:
            Instruction {
                opcode: Opcode::Hlt,
                op1: 0,
                op2: 0,
            },
        ]);

        self.run();
        self.get_reg(RESULT)
    }

    pub fn run_div(&mut self, a: u8, b: u8) -> u8 {
        if b == 0 {
            panic!("division by zero");
        }

        const RESULT: usize = 0;
        const COUNTER: usize = 1;
        const VALUE: usize = 2;
        const ONE: usize = 3;
        const LOOP: usize = 0;
        const HLT_ADDR: usize = 4;

        self.set_reg(RESULT, 0);
        self.set_reg(COUNTER, a);
        self.set_reg(VALUE, b);
        self.set_reg(ONE, 1);

        self.load_program(&[
            // LOOP:
            Instruction {
                opcode: Opcode::Sub,
                op1: COUNTER as u8,
                op2: VALUE as u8,
            },
            Instruction {
                opcode: Opcode::Add,
                op1: RESULT as u8,
                op2: ONE as u8,
            },
            Instruction {
                opcode: Opcode::Je,
                op1: COUNTER as u8,
                op2: HLT_ADDR as u8,
            },
            Instruction {
                opcode: Opcode::Jmp,
                op1: 0,
                op2: LOOP as u8,
            },
            // HLT:
            Instruction {
                opcode: Opcode::Hlt,
                op1: 0,
                op2: 0,
            },
        ]);

        self.run();
        self.get_reg(RESULT)
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
                self.pc += 3;
            }
            Opcode::Sub => {
                let r1 = self.reg(inst.op1);
                let r2 = self.reg(inst.op2);
                let result = self.regs[r1].wrapping_sub(self.regs[r2]);
                self.regs[r1] = result;
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
                self.pc = self.addr(inst.op2) * 3;
            }
            Opcode::Je => {
                let r = self.reg(inst.op1);

                if self.regs[r] == 0 {
                    self.pc = self.addr(inst.op2) * 3;
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
            println!("PC={} {:?} \n-----------------------------------------------\nR0={} R1={} R2={} R3={} \n",
                self.pc,
                inst,
                self.regs[0],
                self.regs[1],
                self.regs[2],
                self.regs[3],

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
