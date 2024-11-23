use super::instruction::Instruction;

const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 4096;
const STACK_SIZE: usize = 16;

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const START_ADDR: u16 = 0x200;
const NUM_KEYS: usize = 16;

pub struct Emulator {
    // Current location in memory
    pc: u16,
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_reg: [u8; NUM_REGS],
    i_reg: u16,
    sp: u16,
    stack: [u16; STACK_SIZE],
    keys: [bool; NUM_KEYS],
    // Timers
    dt: u8,
    st: u8,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_HEIGHT * SCREEN_WIDTH],
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            dt: 0,
            st: 0,
        }
    }

    pub fn fetch(&mut self) -> Instruction {
        let higher_byte = self.ram[self.pc as usize] as u16;
        let lower_byte = self.ram[(self.pc + 1) as usize] as u16;
        let op = (higher_byte << 8) | lower_byte;
        self.pc += 1;

        Instruction(op)
    }

    pub fn tick_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            if self.st == 1 {
                //play sound
            }
            self.st -= 1;
        }
    }

    pub fn execute(&mut self, op: &Instruction) {
        match op.digits() {
            (0, 0, 0, 0)     => return,
            (0, 0, 0xE, 0)   => self._00e0(),
            (0, 0, 0xE, 0xE) => self._00ee(),
            (1, _, _, _)     => self._1nnn(op),
            (2, _, _, _)     => self._2nnn(op),
            (3, _, _, _)     => self._3xkk(op),
            (4, _, _, _)     => self._4xkk(op),
            (5, _, _, 0)     => self._5xy0(op),
            (_, _, _, _)     => unimplemented!("Unimplemented opcode {:?}", op),
        }
    }

    pub fn tick(&mut self) {
        let op = self.fetch();
        self.execute(&op);
    }

    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    // Instructions
    // Clear Screen
    fn _00e0(&mut self) {
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
    }

    // Return from subroutine
    fn _00ee(&mut self) {
        let return_address = self.pop();
        self.pc = return_address;
    }

    // Jump
    fn _1nnn(&mut self, op: &Instruction) {
        self.pc = op.nnn();
    }

    // Call subroutine.
    fn _2nnn(&mut self, op: &Instruction) {
        self.push(self.pc);
        self.pc = op.nnn();
    }

    // Skip next if Vx = kk
    fn _3xkk(&mut self, op: &Instruction) {
        let x = op.x();
        let kk = op.kk();

        let v_x = self.v_reg[x as usize] as u16;
        if v_x == kk {
            self.pc += 2;
        }
    }

    // Skip next if Vx != kk
    fn _4xkk(&mut self, op: &Instruction) {
        let x = op.x();
        let kk = op.kk();

        let v_x = self.v_reg[x as usize] as u16;
        if v_x != kk {
            self.pc += 2;
        }
    }

    // Skip if Vx == Vy
    fn _5xy0(&mut self, op: &Instruction) {
        let x = op.x();
        let y = op.y();

        let v_x = self.v_reg[x as usize] as u16;
        let v_y = self.v_reg[y as usize] as u16;
        if v_x == v_y {
            self.pc += 2;
        }
    }
}
