use std::fs::File;
use std::io::Read;
use rand::random;

use font::FONTSET;

pub struct Chip8 {
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
    pub gfx: [u8; 64 * 32],
    pub draw_flag: bool,

    // Interupts and hardware registers. The Chip 8 has none, but there are two timer registers that count at 60 Hz. When set above zero they will count down to zero.
    // The system’s buzzer sounds whenever the sound timer reaches zero.
    delay_timer: u8,
    sound_timer: u8,

    // It is important to know that the Chip 8 instruction set has opcodes that allow the program to jump to a certain address or call a subroutine. While the specification don’t mention a stack, you will need to implement one as part of the interpreter yourself. The stack is used to remember the current location before a jump is performed. So anytime you perform a jump or call a subroutine, store the program counter in the stack before proceeding. The system has 16 levels of stack and in order to remember which level of the stack is used, you need to implement a stack pointer (sp).
    stack: [u16; 16],
    sp: u16,

    // Finally, the Chip 8 has a HEX based keypad (0x0-0xF), you can use an array to store the current state of the key.
    pub key: [u8; 16],
}

impl Chip8 {
    pub fn new() -> Chip8 {
        // Initialize memory and registers
        let mut mem: [u8; 4000] = [0; 4000];

        // Load fonts to memory
        for f in 0..FONTSET.len() {
            mem[f] = FONTSET[f];
        }

        Chip8 {
            opcode: 0,
            memory: mem,
            v: [0; 16],
            i: 0,
            pc: 0x200, // program counter starts at 0x200
            gfx: [0; 64 * 32],
            draw_flag: false,
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [0; 16],
        }
    }

    pub fn load_game(&mut self, game: &str) {
        // Load game file to memory
        let mut file = File::open(game).unwrap();
        // Start loading at 0x200 (512)
        let size = file.read(&mut self.memory[0x200..]).unwrap();

        println!("Game {} loaded ({} bytes)", game, size);
    }

    pub fn emulate_cycle(&mut self) {
        // Fetch opcode
        let first = self.memory[self.pc as usize] as u16;
        let second = self.memory[(self.pc + 1) as usize] as u16;
        self.opcode = first << 8 | second;
        println!("Opcode fetched: {:x}", self.opcode);

        let vx = (self.opcode & 0x0F00) >> 8;
        let vy = (self.opcode & 0x00F0) >> 4;
        let nn = self.opcode & 0x00FF;
        let address = self.opcode & 0x0FFF;

        // Decode & execute opcode
        match self.opcode & 0xF000 {
            0x0000 => {
                match self.opcode & 0x000F {
                    0x000E => {
                        self.sp -= 1;
                        self.pc = self.stack[self.sp as usize];

                        self.pc += 2;
                        println!("Return from subroutine");
                    },
                    _ => panic!("Unknown opcode: {:x}", self.opcode),
                }
            },
            0x1000 => {
                self.pc = address;
                println!("Jump to {:x}", self.pc);
            },
            0x2000 => {
                // Store current program counter on the stack
                self.stack[self.sp as usize] = self.pc;

                // Increase the stack pointer to prevent overwriting the current stack
                self.sp += 1;

                self.pc = address;
                println!("Call subroutine at {:x}", self.pc);
                // Because it is a subroutine, we should not increase program counter
            },
            0x3000 => {
                if self.v[vx as usize] == nn as u8 {
                    println!("Register V[{:x}] is equal to {:x}. Skipping the next instruction", vx, nn);
                    self.pc += 4;
                } else {
                    println!("Register V[{:x}] is NOT equal to {:x}", vx, nn);
                    self.pc += 2;
                }
            },
            0x4000 => {
                if self.v[vx as usize] != nn as u8 {
                    println!("Register V[{:x}] is NOT equal to {:x}. Skipping the next instruction", vx, nn);
                    self.pc += 4;
                } else {
                    println!("Register V[{:x}] is equal to {:x}", vx, nn);
                    self.pc += 2;
                }
            },
            0x6000 => {
                self.v[vx as usize] = nn as u8;
                self.pc += 2;
                println!("Set V[{:x}] to {:x}", vx, nn);
            },
            0x7000 => {
                let (value, _) = self.v[vx as usize].overflowing_add(nn as u8);
                self.v[vx as usize] = value;

                self.pc += 2;
                println!("Add {:x} to V[{:x}]", nn, vx);
            },
            0x8000 => {
                match self.opcode & 0x000F {
                    0x0000 => {
                        self.v[vx as usize] = self.v[vy as usize];

                        self.pc += 2;
                        println!("Set V[{:x}] to the value of V[{:x}]", vx, vy);
                    },
                    0x0002 => {
                        self.v[vx as usize] = self.v[vx as usize] & self.v[vy as usize];

                        self.pc += 2;
                        println!("Sets V[{:x}] to V[{:x}] and V[{:x}] (Bitwise OR)", vx, vx, vy);
                    },
                    0x0004 => {
                        let (value, overflow) = self.v[vx as usize].overflowing_add(self.v[vy as usize]);
                        if overflow {
                            self.v[0xF] = 1;
                        } else {
                            self.v[0xF] = 0;
                        }
                        self.v[vx as usize] = value;

                        self.pc += 2;
                        println!("Add V[{:x}] to V[{:x}]", vy, vx);
                    },
                    0x0005 => {
                        let (value, overflow) = self.v[vx as usize].overflowing_sub(self.v[vy as usize]);
                        if overflow {
                            self.v[0xF] = 0;
                        } else {
                            self.v[0xF] = 1;
                        }
                        self.v[vx as usize] = value;

                        self.pc += 2;
                        println!("Subtract V[{:x}] from V[{:x}]", vy, vx);
                    },
                    _ => panic!("Unknown opcode: {:x}", self.opcode),
                }
            },
            0xA000 => {
                self.i = address;
                self.pc += 2;
                println!("Set I to {:x}", address);
            },
            0xC000 => {
                self.v[vx as usize] = (random::<u8>() as u16 & nn) as u8;
                self.pc += 2;
                println!("Random value saved in V[{:x}]", vx);
            },
            0xD000 => {
                let x = self.v[vx as usize] as u16;
                let y = self.v[vy as usize] as u16;
                let height = self.opcode & 0x000F;
                println!("Draw to screen. Lines: {}, starting at x={}, y={}", height, x, y);

                self.v[0xF] = 0;
                for line in 0..height {
                    let data = self.memory[(self.i + line) as usize];

                    for b in 0..8 {
                        if (data & (0x80 >> b)) != 0 {
                            let pixel = x + b as u16 + ((y + line) * 64) as u16;

                            if self.gfx[pixel as usize] == 1 {
                                self.v[0xF] = 1;
                            }

                            self.gfx[pixel as usize] ^= 1;
                        }
                    }
                }

                self.draw_flag = true;
                self.pc += 2;
            },
            0xE000 => {
                match self.opcode & 0x00FF {
                    0x00A1 => {
                        if self.key[self.v[vx as usize] as usize] != 1 {
                            println!("Key {:x} is NOT pressed. Skipping the next instruction", vx);
                            self.pc += 4;
                        } else {
                            println!("Key {:x} is pressed", vx);
                            self.pc += 2;
                        }
                    },
                    _ => panic!("Unknown opcode: {:x}", self.opcode),
                }
            },
            0xF000 => {
                match self.opcode & 0x00FF {
                    0x0007 => {
                        self.v[vx as usize] = self.delay_timer;

                        self.pc += 2;
                        println!("Set V[{:x}] to value of delay timer ({})", vx, self.delay_timer);
                    },
                    0x0015 => {
                        self.delay_timer = self.v[vx as usize];

                        self.pc += 2;
                        println!("Set delay timer to \"{}\"", vx);
                    },
                    0x0018 => {
                        self.sound_timer = self.v[vx as usize];

                        self.pc += 2;
                        println!("Set sound timer to \"{}\"", vx);
                    },
                    0x0029 => {
                        self.i = self.memory[(self.v[vx as usize] * 5) as usize] as u16;

                        self.pc += 2;
                        println!("Set I to address of digit \"{}\"", vx);
                    },
                    0x0033 => {
                        self.memory[self.i as usize] = self.v[vx as usize] / 100;
                        self.memory[self.i as usize + 1] = (self.v[vx as usize] / 10) % 10;
                        self.memory[self.i as usize + 2] = (self.v[vx as usize] % 100) % 10;

                        self.pc += 2;
                        println!("Binary-coded decimal saved into memory");
                    },
                    0x0065 => {
                        for i in 0..vx + 1 {
                            self.v[i as usize] = self.memory[self.i as usize];
                        }
                        self.pc += 2;
                        println!("V[0] - V[{:x}] values have been replaced with {:x}", vx, self.memory[self.i as usize]);
                    },
                    _ => panic!("Unknown opcode: {:x}", self.opcode),
                }
            },
            _ => panic!("opcode has not been implemented yet"),
        };

        // Update timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("Beep!");
            }
            self.sound_timer -= 1;
        }
    }
}
