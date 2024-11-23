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

    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
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

    pub fn execute(&mut self, op: Instruction) {
        match op.digits() {
            (0, 0, 0, 0) => return,
            (0, 0, 0xE, 0) => self._00E0(),
            (0, 0, 0xE, 0xE) => self._00EE(),
            (_, _, _, _) => unimplemented!("Unimplemented opcode {:?}", op),
        }
    }

    pub fn tick(&mut self) {
        let op = self.fetch();
        self.execute(op);
    }

    // Instructions
    // Clear Screen
    fn _00E0(&mut self) {
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
    }

    // Return from subroutine
    fn _00EE(&mut self) {
        let return_address = self.pop();
        self.pc = return_address;
    }
}
