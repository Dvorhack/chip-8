extern crate sdl2;
mod cpu;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::thread;

use cpu::CPU;

fn setupGraphics(sdl_context: sdl2::Sdl) {
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
}

fn getKeypad(sdl_context: sdl2::Sdl){

    let mut event_pump = sdl_context.event_pump().unwrap();

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                println!("End");
            },
            _ => {}
        }
    }
}

pub fn main() {
    let sleep_duration = Duration::from_millis(2);

    let sdl_context = sdl2::init().unwrap();

    let cpu = CPU::new();
    
    
    while let Ok(keypad) = getKeypad(sdl_context) {

        let output = cpu.tick(keypad);

        if output.vram_changed {
            // display_driver.draw(output.vram);
        }

        if output.beep {
            // audio_driver.start_beep();
        } else {
            // audio_driver.stop_beep();
        }

        thread::sleep(sleep_duration);
    }
}