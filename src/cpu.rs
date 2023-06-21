

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