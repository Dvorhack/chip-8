extern crate sdl2;
mod cpu;

use sdl2::pixels::Color;
use sdl2::pixels;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::render::Canvas;
use sdl2::rect::Rect;
use sdl2::video::Window;
use std::thread;
use std::process::exit;

use cpu::CPU;
use cpu::{CHIP8_HEIGHT, CHIP8_WIDTH};

const SCALE_FACTOR: u32 = 10;

fn setup_graphics(sdl_context: &sdl2::Sdl) -> Canvas<Window> {
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", CHIP8_WIDTH as u32 *SCALE_FACTOR, CHIP8_HEIGHT as u32*SCALE_FACTOR)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, CHIP8_HEIGHT.try_into().unwrap(), CHIP8_WIDTH.try_into().unwrap()));
    canvas.clear();
    canvas.present();

    canvas
}

fn get_keypad(sdl_context: &sdl2::Sdl) -> Result<[bool; 16], ()>{

    let mut event_pump = sdl_context.event_pump().unwrap();

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                println!("End");
                exit(0);
            },
            _ => {}
        }
    }
    Ok([true; 16])
}

fn color(value: u8) -> pixels::Color {
    if value == 0 {
        pixels::Color::RGB(0, 0, 0)
    } else {
        pixels::Color::RGB(0, 250, 0)
    }
}

fn draw_screen(canvas: &mut Canvas<Window>, pixels: [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT]) {
    for (y, row) in pixels.iter().enumerate() {
        for (x, &col) in row.iter().enumerate() {
            let x = (x as u32) * SCALE_FACTOR;
            let y = (y as u32) * SCALE_FACTOR;

            canvas.set_draw_color(color(col));
            let _ = canvas
                .fill_rect(Rect::new(x as i32, y as i32, SCALE_FACTOR, SCALE_FACTOR));
        }
    }
    canvas.present();
}

pub fn main() {
    let sleep_duration = Duration::from_millis(2);

    let sdl_context = sdl2::init().unwrap();
    let mut canvas = setup_graphics(&sdl_context);

    let mut cpu = CPU::new();
    cpu.load_game("games/PONG");
    
    
    while let Ok(keys) = get_keypad(&sdl_context) {

        cpu.tick(keys);

        if cpu.draw_flag {
            // display_driver.draw(output.vram);
            println!("Drawing");
            draw_screen(&mut canvas, cpu.vmem);
            cpu.draw_flag = false;
        }

        // if output.beep {
        //     // audio_driver.start_beep();
        // } else {
        //     // audio_driver.stop_beep();
        // }

        thread::sleep(sleep_duration);
    }
}