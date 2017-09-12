mod cpu;

use cpu::Chip8;

fn main() {
    // Set up render system and register input callbacks
    println!("Setup graphics");
    println!("Setup input");

    // Initialize the Chip8 system and load the game into the memory
    let mut cpu = Chip8::new();
    cpu.loadGame();

    // Emulation loop
    loop {
        // Emulate one cycle
        cpu.emulateCycle();

        // If the draw flag is set, update the screen
        println!("If Chip8 draw flag, then draw graphics");

        // Store key press state (Press and Release)
        println!("Chip8 set keys");

        break;
    }
}
