// Following chip-8 spec from http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
fn main() {
}

struct Chip8 {
    ram: [u8; 4096],
    disp_buffer: [u8; 2048],

    keypad: u16,

    stack: [u16; 16],
    sp: u8, // stack pointer

    pc: u16, // program counter

    // registers
    v_reg: [u16; 16],
    i: u16,

    // decrement @ 60hz
    dt: u8, // delay timer
    st: u8, // sound timer
}

impl Default for Chip8 {
    fn default() -> Self {
        Chip8 {
            ram: [0u8; 4096],
            disp_buffer: [0u8; 2048],
            keypad: 0,
            stack: [0u16; 16],
            sp: 0,
            pc: 0,
            v_reg: [0u16; 16],
            i: 0,
            dt: 0,
            st: 0,
        }
    }
}

impl Chip8 {
    fn _cycle(&self) {
        // fetch
        let instruction: u16 = ((self.ram[self.pc as usize] as u16) << 4) | (self.ram[(self.pc + 1) as usize] as u16);

        // decode 
        match instruction & 0xF000 {
            0x0000 => {
                match instruction & 0x0F00 {
                    0x0000 => {
                        match instruction & 0x00FF {
                            0x00E0 => {
                                // TODO CLS
                            },
                            0x00EE => {
                                // TODO RET
                            },
                            _ => {
                                // TODO fail here
                            }
                        }
                    },
                    _ => {
                        // TODO fail here
                    }
                }
            },
            0x1000 => {
                // TODO JP 
                let jp: u16 = instruction & 0x0FFF;
            },
            0x2000 => {
                // TODO CALL
            },
            0x3000 => {
                // TODO SE Vx
            },
            0x4000 => {
                // TODO
            },
            0x5000 => {
                // TODO
            },
            0x6000 => {
                // TODO
            },
            0x7000 => {
                // TODO
            },
            0x8000 => {
                // TODO
            },
            0x9000 => {
                // TODO
            },
            0xA000 => {
                // TODO
            },
            0xB000 => {
                // TODO
            },
            0xC000 => {
                // TODO
            },
            0xD000 => {
                // TODO
            },
            0xE000 => {
                // TODO
            },
            0xF000 => {
                // TODO
            },
            _ => {
                // TODO fail here
            }
        }
    }

}
