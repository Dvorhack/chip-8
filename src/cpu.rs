

const CPU_RAM: usize = 4096;
const CPU_REGS: usize = 16;
const CPU_STACK: usize = 48;
const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;

pub struct CPU {
    ram: [u8; CPU_RAM],
    stack: [u16; CPU_STACK],
    vmem: [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT],

    regs: [u8; CPU_REGS],
    i: u16,
    pc: u16,
    sp: u16,


    delay_timer: u8,
    sound_timer: u8,
    input: [bool; 16]
}

impl CPU {
    pub fn new() -> Self {
        let mut ram = [0u8; CPU_RAM];
        // for i in 0..FONT_SET.len() {
        //     ram[i] = FONT_SET[i];
        // }

        CPU {
            vmem: [[0; CHIP8_WIDTH]; CHIP8_HEIGHT],
            ram: ram,
            stack: [0; CPU_STACK],

            regs: [0; 16],
            i: 0,
            pc: 0x200,
            sp: 0,

            delay_timer: 0,
            sound_timer: 0,
            input: [false; 16],
        }
    }

    pub fn tick(&mut self, keys: [bool; 16]) -> bool{
        self.input = keys;
        self.pc += 1;

        true

    }
}