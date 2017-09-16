mod cpu;
mod font;

use cpu::Chip8;

fn main() {
    // Set up render system and register input callbacks
    println!("Setup graphics");
    println!("Setup input");

    // Initialize the Chip8 system and load the game into the memory
    let mut cpu = Chip8::new();
    cpu.load_game("PONG2");

    // Emulation loop
    loop {
        // Emulate one cycle
        cpu.emulate_cycle();

        // If the draw flag is set, update the screen
        // Store key press state (Press and Release)
    }
}
