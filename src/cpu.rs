use sdl2::libc::rand;
use rand::Rng;
use std::fs;


const CPU_RAM: usize = 4096;
const CPU_REGS: usize = 16;
const CPU_STACK: usize = 48;
pub const CHIP8_WIDTH: usize = 64;
pub const CHIP8_HEIGHT: usize = 32;

pub struct CPU {
    ram: [u8; CPU_RAM],
    stack: [u16; CPU_STACK],
    pub vmem: [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT],

    regs: [u8; CPU_REGS],
    i: u16,
    pc: u16,
    sp: u16,


    delay_timer: u8,
    sound_timer: u8,
    input: [bool; 16],
    pub draw_flag: bool
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
            draw_flag: false
        }
    }

    pub fn tick(&mut self, keys: [bool; 16]){
        self.input = keys;
        self.pc += 1;

        if self.pc == 500+0x200 {
            
            for x in 0..CHIP8_HEIGHT {
                for y in 0..CHIP8_WIDTH {
                    self.vmem[x][y] = rand::thread_rng().gen::<u8>() % 2 ;
                }
            }
            self.draw_flag = true;
            self.pc = 0x200;
        }

    }

    pub fn load_game(&mut self, path :&str){
        println!("Loading game {path}");
        let data = fs::read(path).unwrap();
        for (i, &byte) in data.iter().enumerate() {
            let addr = 0x200 + i;
            if addr < 4096 {
                self.ram[0x200 + i] = byte;
            } else {
                break;
            }
        }
        //println!("{:?}", &self.ram[0x200..0x200+10]);
    }
}