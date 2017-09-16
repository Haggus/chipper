extern crate sdl2;
extern crate rand;

mod cpu;
mod font;

use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use cpu::Chip8;

fn main() {
    // Set up render system and register input callbacks
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();

    let window = video.window("Chipper", 1280, 640).position_centered().build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(40, 40, 40));
    canvas.clear();
    canvas.present();

    let mut events = context.event_pump().unwrap();

    // Initialize the Chip8 system and load the game into the memory
    let mut cpu = Chip8::new();
    cpu.load_game("PONG2");

    // Emulation loop
    'game: loop {
        // Store key press state (Press and Release)
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} => break 'game,
                Event::KeyDown { keycode: Some(keycode), ..} => {
                    match keycode {
                        Keycode::Escape => break 'game,
                        _ => {},
                    }
                }
                _ => {}
            }
        }

        // Emulate one cycle
        cpu.emulate_cycle();

        // If the draw flag is set, update the screen
        if cpu.draw_flag {
            canvas.set_draw_color(Color::RGB(184, 186, 60));
            for x in 0..64 {
                for y in 0..32 {
                    if cpu.gfx[(y * 64) + x] == 1 {
                        canvas.fill_rect(Rect::new(x as i32 * 20, y as i32 * 20, 20, 20)).unwrap();
                    }
                }
            }
        }
        canvas.present();
    }
}
