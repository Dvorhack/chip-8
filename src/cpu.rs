use sdl2::libc::rand;
use rand::Rng;
use std::fs;
use std::process::exit;


const CPU_RAM: usize = 4096;
const CPU_REGS: usize = 16;
const CPU_STACK: usize = 48;
pub const CHIP8_WIDTH: usize = 64;
pub const CHIP8_HEIGHT: usize = 32;
const CPU_OP_LEN: u16 = 2;

const FONT_SET:[u8; 80] = [ 
  0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
  0x20, 0x60, 0x20, 0x20, 0x70, // 1
  0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
  0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
  0x90, 0x90, 0xF0, 0x10, 0x10, // 4
  0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
  0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
  0xF0, 0x10, 0x20, 0x40, 0x40, // 7
  0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
  0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
  0xF0, 0x90, 0xF0, 0x90, 0x90, // A
  0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
  0xF0, 0x80, 0x80, 0x80, 0xF0, // C
  0xE0, 0x90, 0x90, 0x90, 0xE0, // D
  0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
  0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

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
    input_req: bool,
    input_req_reg: usize,
    pub draw_flag: bool
}

impl CPU {
    pub fn new() -> Self {
        let mut ram = [0u8; CPU_RAM];
        for i in 0..FONT_SET.len() {
            ram[i] = FONT_SET[i];
        }

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
            input_req: false,
            input_req_reg: 0,
            draw_flag: false
        }
    }

    /// Execute one CPU clock cycle
    /// Unless an input is requested
    pub fn tick(&mut self, keys: [bool; 16]){
        self.input = keys;

        // If input request (op FX0A), do not exec opcode
        // Check if input
        if self.input_req{
            for i in 0..self.input.len(){
                if self.input[i]{
                    self.input_req = false;
                    self.regs[self.input_req_reg] = i as u8;
                    break;
                }
            }
        }else{
            if self.delay_timer > 0 {
                self.delay_timer -= 1
            }
            if self.sound_timer > 0 {
                self.sound_timer -= 1
            }

            // Get opcode
            let opcode =   ((self.ram[self.pc as usize] as u16) << 8) + 
                            (self.ram[(self.pc as usize)+1] as u16 );

            self.pc += CPU_OP_LEN;

            self.execute_opcode(opcode);
        }

    }

    fn execute_opcode(&mut self, op: u16){
        println!("{op:x} {:x}",self.pc);

        let op1 = ((op & 0xF000) >> 12) as usize;
        let op2 = ((op & 0x0F00) >>  8) as usize;
        let op3 = ((op & 0x00F0) >>  4) as usize;
        let op4 = ((op & 0x000F) >>  0) as usize;

        match (op1, op2, op3, op4) {
            (0x0, 0x0, 0xE, 0x0) => { /* Clear screen */ }
            (0x0, 0x0, 0xE, 0xE) => { /* Return from subroutine */
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            }
            (0x1, _, _, _) => { /* Jump NNN */
                self.pc = op & 0xfff;
            }
            (0x2, _, _, _) => { /* Call subroutine NNN */
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = op & 0xfff;
            }
            (0x3, _, _, _) => { /* Skips the next instruction if VX equals NN */
                if self.regs[op2 as usize] == (op & 0xff) as u8 {
                    self.pc += CPU_OP_LEN;
                }
            }
            (0x4, _, _, _) => { /* Skips the next instruction if VX does not equal NN */
                if self.regs[op2 as usize] != (op & 0xff) as u8 {
                    self.pc += CPU_OP_LEN;
                }
            }
            (0x5, _, _, 0) => { /* Skips the next instruction if VX equals VY  */
                if self.regs[op2 as usize] == self.regs[op3 as usize] {
                    self.pc += CPU_OP_LEN;
                }
            }
            (0x6, _, _, _) => { /* Sets VX to NN */
                self.regs[op2] = (op & 0xff) as u8;
            }
            (0x7, _, _, _) => { /* Adds NN to VX */
                self.regs[op2] = (self.regs[op2]as u16 + (op & 0xff) as u16) as u8;
            }
            (0x8, _, _, 0) => { /* Sets VX to the value of VY. */
                self.regs[op2] = self.regs[op3];
            }
            (0x8, _, _, 1) => { /* Sets VX to VX or VY */
                self.regs[op2] |= self.regs[op3];
            }
            (0x8, _, _, 2) => { /* Sets VX to VX and VY */
                self.regs[op2] &= self.regs[op3];
            }
            (0x8, _, _, 3) => { /* Sets VX to VX xor VY */
                self.regs[op2] ^= self.regs[op3];
            }
            (0x8, _, _, 4) => { /* Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there is not. */
                if (self.regs[op2] as u16 + self.regs[op3] as u16) >= 0x100{
                    self.regs[0xF] = 1;
                }
                self.regs[op2] = (self.regs[op2]as u16 + self.regs[op3] as u16) as u8;
            }
            (0x8, _, _, 5) => { /* VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there is not.  */
                if self.regs[op2] < self.regs[op3] {
                    self.regs[0xF] = 0;
                }
                self.regs[op2] = self.regs[op2].wrapping_sub(self.regs[op3]);
            }
            (0x8, _, _, 6) => { /* Stores the least significant bit of VX in VF and then shifts VX to the right by 1. */
                self.regs[0xF] = self.regs[op2] & 0x1;
                self.regs[op2] >>= 1;
            }
            (0x8, _, _, 7) => { /* Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there is not. */
                if self.regs[op2] > self.regs[op3] {
                    self.regs[0xF] = 0;
                }
                self.regs[op2] = self.regs[op3] - self.regs[op2];
            }
            (0x8, _, _, 0xE) => { /* Stores the most significant bit of VX in VF and then shifts VX to the left by 1 */
                self.regs[0xF] = self.regs[op2] >> 7;
                self.regs[op2] <<= 1;
            }
            (0x9, _, _, 0x0) => { /* Skips the next instruction if VX does not equal VY. */
                if self.regs[op2 as usize] != self.regs[op3 as usize] {
                    self.pc += CPU_OP_LEN;
                }
            }
            (0xA, _, _, _) => { /* Sets I to the address NNN. */
                self.i = op & 0xfff;
            }
            (0xB, _, _, _) => { /* Jumps to the address NNN plus V0.  */
                self.pc = (op & 0xfff) + self.regs[0] as u16;
            }
            (0xC, _, _, _) => { /* Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.  */
                self.regs[op2] = rand::thread_rng().gen::<u8>() & (op &0xff) as u8;
            }
            (0xD, _, _, _) => { /* Draw at position VX, VY the sprite of width 8 and height N à location I */
                self.regs[0xF] = 0;
                for byte in 0..op4 {
                    let y = (self.regs[op3] as usize + byte) % CHIP8_HEIGHT;
                    for bit in 0..8 {
                        let x = (self.regs[op2] as usize + bit) % CHIP8_WIDTH;
                        let color = (self.ram[self.i as usize + byte] >> (7 - bit)) & 1;
                        self.regs[0x0f] |= color & self.vmem[y][x];
                        self.vmem[y][x] ^= color;
        
                    }
                }
                self.draw_flag = true;    
            }
            (0xE, _, 0x9, 0xE) => { /* Skips the next instruction if the key stored in VX is pressed  */
                if self.input[self.regs[op2] as usize]{
                    self.pc += CPU_OP_LEN;
                }
            }
            (0xE, _, 0xA, 0x1) => { /* Skips the next instruction if the key stored in VX is not pressed  */
                if !self.input[self.regs[op2] as usize]{
                    self.pc += CPU_OP_LEN;
                }
            }
            (0xF, _, 0x0, 0x7) => { /* Sets VX to the value of the delay timer.   */
                self.regs[op2] = self.delay_timer;
            }
            (0xF, _, 0x0, 0xA) => { /* A key press is awaited, and then stored in VX  */
                self.input_req = true;
                self.input_req_reg = op2;
            }
            (0xF, _, 1, 5) => { /* Sets the delay timer to VX. */
                self.delay_timer = self.regs[op2];
            }
            (0xF, _, 1, 8) => { /* Sets the sound timer to VX. */
                self.sound_timer = self.regs[op2];
            }
            (0xF, _, 1, 0xE) => { /* Adds VX to I. VF is not affected */
                self.i += self.regs[op2] as u16;
            }
            (0xF, _, 2, 9) => { /* Sets I to the location of the sprite for the character in VX */
                self.i = (self.regs[op2] as u16) * 5;
            }
            (0xF, _, 3, 3) => { /* stores binary repr op VX in mem */
                self.ram[self.i as usize] = self.regs[op2] / 100;
                self.ram[self.i as usize + 1] = (self.regs[op2] % 100) / 10;
                self.ram[self.i as usize + 2] = self.regs[op2] % 10;
            }
            (0xF, _, 5, 5) => { /* Stores from V0 to VX (including VX) in memory, starting at address I */
                for i in 0..op2+1{
                    self.ram[self.i as usize + i] = self.regs[i];
                }
            }
            (0xF, _, 6, 5) => { /* loads from V0 to VX (including VX) from memory, starting at address I */
                for i in 0..op2+1{
                    self.regs[i] = self.ram[self.i as usize + i];
                }
            }
            _ => {
                println!("Unknown opcode {op:x}");
                exit(0);
            }
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