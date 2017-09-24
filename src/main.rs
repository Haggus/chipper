extern crate sdl2;
extern crate rand;
extern crate clap;
extern crate rodio;

mod cpu;
mod font;

use std::time::Instant;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use clap::{App, Arg};

use cpu::Chip8;

fn main() {
    let matches = App::new("Chipper")
        .version("1.0")
        .author("Mateusz Mrowiec <matt.mrowiec@gmail.com>")
        .about("Chip8 emulator/interpreter")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .get_matches();

    let input_file = matches.value_of("INPUT").unwrap();
    println!("Using input file: {}", input_file);

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
    cpu.load_game(input_file);

    let mut last_frame = Instant::now();

    // Emulation loop
    'game: loop {
        // Store key press state (Press and Release)
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'game,
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Escape => break 'game,

                        // Keycodes correspond to keypad
                        //
                        // +---+---+---+---+
                        // | 1 | 2 | 3 | C |
                        // +---+---+---+---+
                        // | 4 | 5 | 6 | D |
                        // +---+---+---+---+
                        // | 7 | 8 | 9 | E |
                        // +---+---+---+---+
                        // | A | 0 | B | F |
                        // +---+---+---+---+
                        Keycode::Num1 => cpu.key[0x1] = 1,
                        Keycode::Num2 => cpu.key[0x2] = 1,
                        Keycode::Num3 => cpu.key[0x3] = 1,
                        Keycode::Num4 => cpu.key[0xC] = 1,
                        Keycode::Q => cpu.key[0x4] = 1,
                        Keycode::W => cpu.key[0x5] = 1,
                        Keycode::E => cpu.key[0x6] = 1,
                        Keycode::R => cpu.key[0xD] = 1,
                        Keycode::A => cpu.key[0x7] = 1,
                        Keycode::S => cpu.key[0x8] = 1,
                        Keycode::D => cpu.key[0x9] = 1,
                        Keycode::F => cpu.key[0xE] = 1,
                        Keycode::Z => cpu.key[0xA] = 1,
                        Keycode::X => cpu.key[0x0] = 1,
                        Keycode::C => cpu.key[0xB] = 1,
                        Keycode::V => cpu.key[0xF] = 1,
                        _ => {}
                    }
                }
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Num1 => cpu.key[0x1] = 0,
                        Keycode::Num2 => cpu.key[0x2] = 0,
                        Keycode::Num3 => cpu.key[0x3] = 0,
                        Keycode::Num4 => cpu.key[0xC] = 0,
                        Keycode::Q => cpu.key[0x4] = 0,
                        Keycode::W => cpu.key[0x5] = 0,
                        Keycode::E => cpu.key[0x6] = 0,
                        Keycode::R => cpu.key[0xD] = 0,
                        Keycode::A => cpu.key[0x7] = 0,
                        Keycode::S => cpu.key[0x8] = 0,
                        Keycode::D => cpu.key[0x9] = 0,
                        Keycode::F => cpu.key[0xE] = 0,
                        Keycode::Z => cpu.key[0xA] = 0,
                        Keycode::X => cpu.key[0x0] = 0,
                        Keycode::C => cpu.key[0xB] = 0,
                        Keycode::V => cpu.key[0xF] = 0,
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        if last_frame.elapsed().subsec_nanos() > 100_000_000 / 60 {
            cpu.emulate_cycle();

            last_frame = Instant::now();
        }

        // If the draw flag is set, update the screen
        if cpu.draw_flag {
            canvas.set_draw_color(Color::RGB(40, 40, 40));
            canvas.clear();

            canvas.set_draw_color(Color::RGB(184, 186, 60));
            for x in 0..64 {
                for y in 0..32 {
                    if cpu.gfx[(y * 64) + x] == 1 {
                        canvas.fill_rect(Rect::new(x as i32 * 20, y as i32 * 20, 20, 20)).unwrap();
                    }
                }
            }
            cpu.draw_flag = false;
        }
        canvas.present();
    }
}
