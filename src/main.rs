struct Chip8 {
    // The systems memory map:
    // 0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
    // 0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
    // 0x200-0xFFF - Program ROM and work RAM

    // The Chip 8 has 35 opcodes which are all two bytes long. To store the current opcode, we need a data type that allows us to store two bytes.
    opcode: u16,

    // The Chip 8 has 4K memory in total
    memory: [u8; 4000],

    // CPU registers: The Chip 8 has 15 8-bit general purpose registers named V0,V1 up to VE. The 16th register is used  for the ‘carry flag’.
    v: [u8; 16],

    // There is an Index register I and a program counter (pc) which can have a value from 0x000 to 0xFFF
    i: u16,
    pc: u16,

    // The graphics of the Chip 8 are black and white and the screen has a total of 2048 pixels (64 x 32). This can easily be implemented using an array that hold the pixel state (1 or 0):
    gfx: [u8; 64 * 32],

    // Interupts and hardware registers. The Chip 8 has none, but there are two timer registers that count at 60 Hz. When set above zero they will count down to zero.
    // The system’s buzzer sounds whenever the sound timer reaches zero.
    delay_timer: u8,
    sound_timer: u8,

    // It is important to know that the Chip 8 instruction set has opcodes that allow the program to jump to a certain address or call a subroutine. While the specification don’t mention a stack, you will need to implement one as part of the interpreter yourself. The stack is used to remember the current location before a jump is performed. So anytime you perform a jump or call a subroutine, store the program counter in the stack before proceeding. The system has 16 levels of stack and in order to remember which level of the stack is used, you need to implement a stack pointer (sp).
    stack: [u16; 16],
    sp: u16,

    // Finally, the Chip 8 has a HEX based keypad (0x0-0xF), you can use an array to store the current state of the key.
    key: [u8; 16],
}

impl Chip8 {
    fn new() -> Chip8 {
        // Initialize memory and registers
        Chip8 {
            opcode: 0,
            memory: [0; 4000],
            v: [0; 16],
            i: 0,
            pc: 0,
            gfx: [0; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [0; 16],
        }
    }

    fn loadGame(&mut self) {
        // Load game file to memory
    }

    fn emulateCycle(&self) {
        // Fetch opcode
        // Decode opcode
        // Execute opcode

        // Update timers
    }
}

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
