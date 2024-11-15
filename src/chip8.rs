// Following chip-8 spec from http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
use std::{cmp, fs, u8::MAX};
use rand::{self, Rng};

// Number Sprites
const SPRITE_ZERO: [u8; 5] = [0xF0, 0x90, 0x90, 0x90, 0xF0];
const SPRITE_ONE: [u8; 5] = [0x20, 0x60, 0x20, 0x20, 0x70];
const SPRITE_TWO: [u8; 5] = [0xF0, 0x10, 0xF0, 0x80, 0xF0]; 
const SPRITE_THREE: [u8; 5] = [0xF0, 0x10, 0xF0, 0x10, 0xF0];
const SPRITE_FOUR: [u8; 5] = [0x90, 0x90, 0xF0, 0x10, 0x10];
const SPRITE_FIVE: [u8; 5] = [0xF0, 0x80, 0xF0, 0x10, 0xF0];
const SPRITE_SIX:  [u8; 5] = [0xF0, 0x80, 0xF0, 0x90, 0xF0];
const SPRITE_SEVEN: [u8; 5] = [0xF0, 0x10, 0x20, 0x40, 0x40];
const SPRITE_EIGHT: [u8; 5] =  [0xF0, 0x90, 0xF0, 0x90, 0xF0];
const SPRITE_NINE: [u8; 5] = [0xF0, 0x90, 0xF0, 0x10, 0xF0]; 
const SPRITE_A: [u8; 5] = [0xF0, 0x90, 0xF0, 0x90, 0x90];
const SPRITE_B: [u8; 5] = [0xE0, 0x90, 0xE0, 0x90, 0xE0];
const SPRITE_C: [u8; 5] = [0xF0, 0x80, 0x80, 0x80, 0xF0];
const SPRITE_D: [u8; 5] = [0xE0, 0x90, 0x90, 0x90, 0xE0];
const SPRITE_E: [u8; 5] = [0xF0, 0x80, 0xF0, 0x80, 0xF0];
const SPRITE_F: [u8; 5] = [0xF0, 0x80, 0xF0, 0x80, 0x80];

pub struct Chip8 {
    ram: [u8; 4096],
    disp_buffer: [[u8; 64]; 32],

    keypad: u16,

    stack: [u16; 16],
    sp: u8, // stack pointer

    pc: u16, // program counter

    // registers
    v_reg: [u8; 16],
    i: u16,

    // decrement @ 60hz
    dt: u8, // delay timer
    st: u8, // sound timer
}

impl Default for Chip8 {
    fn default() -> Self {
        Chip8 {
            ram: [0u8; 4096],
            disp_buffer: [[0u8; 64]; 32],
            keypad: 0,
            stack: [0u16; 16],
            sp: 0,
            pc: 0,
            v_reg: [0u8; 16],
            i: 0,
            dt: 0,
            st: 0,
        }
    }
}

impl Chip8 {
    pub fn init(&mut self, file_path: &String) {
        self.ram[0..5].copy_from_slice(&SPRITE_ZERO);
        self.ram[5..10].copy_from_slice(&SPRITE_ONE);
        self.ram[10..15].copy_from_slice(&SPRITE_TWO); 
        self.ram[15..20].copy_from_slice(&SPRITE_THREE);
        self.ram[20..25].copy_from_slice(&SPRITE_FOUR);
        self.ram[25..30].copy_from_slice(&SPRITE_FIVE);
        self.ram[30..35].copy_from_slice(&SPRITE_SIX); 
        self.ram[35..40].copy_from_slice(&SPRITE_SEVEN);
        self.ram[40..45].copy_from_slice(&SPRITE_EIGHT); 
        self.ram[45..50].copy_from_slice(&SPRITE_NINE); 
        self.ram[50..55].copy_from_slice(&SPRITE_A);
        self.ram[55..60].copy_from_slice(&SPRITE_B);
        self.ram[60..65].copy_from_slice(&SPRITE_C); 
        self.ram[65..70].copy_from_slice(&SPRITE_D);
        self.ram[70..75].copy_from_slice(&SPRITE_E); 
        self.ram[75..80].copy_from_slice(&SPRITE_F); 

        self.pc = 0x0200;

        let bytes: Vec<u8> = fs::read(file_path).expect("Failed to ready file.");
        self.ram[(self.pc as usize)..(self.pc as usize + bytes.len())].copy_from_slice(&bytes);
    }

    pub fn cycle(&mut self) {
        // fetch
        let instruction: u16 = ((self.ram[self.pc as usize] as u16) << 8) | (self.ram[(self.pc + 1) as usize] as u16);
        let x: u16 = (instruction & 0x0F00) >> 8;
        let y: u16 = (instruction & 0x00F0) >> 4;

        // decode 
        match instruction & 0xF000 {
            0x0000 => {
                match instruction & 0x0F00 {
                    0x0000 => {
                        match instruction & 0x00FF {
                            0x00E0 => {
                                // CLS
                                self.disp_buffer = [[0u8; 64]; 32];
                            },
                            0x00EE => {
                                // RET
                                self.pc = self.stack[self.sp as usize];
                                self.sp -= 1;
                            },
                            _ => {
                                // Could be a call to SYS addr (0nnn), ignore
                                self.pc += 2;
                                return
                            }
                        }
                    },
                    _ => {
                        // Could be a call to SYS addr (0nnn), ignore
                        self.pc += 2;
                        return
                    }
                }
            },
            0x1000 => {
                // JP addr 
                let addr: u16 = instruction & 0x0FFF;
                self.pc = addr;
                // To prevent pc increment
                return
            },
            0x2000 => {
                // CALL addr
                let addr: u16 = instruction & 0x0FFF;

                if self.sp >= 16 {
                    panic!("Stack overflow.");
                } 
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                self.pc = addr;
                return // Needed or else the increment outside the match ruins the call
            },
            0x3000 => {
                // SE Vx, byte
                let kk: u8 = (instruction & 0x00FF) as u8;
                if self.v_reg[x as usize] == kk {
                    self.pc += 2;
                }
            },
            0x4000 => {
                // SNE Vx, byte 
                let kk: u8 = (instruction & 0x00FF) as u8;
                if self.v_reg[x as usize] != kk {
                    self.pc += 2;
                }
            },
            0x5000 => {
                match instruction & 0x000F {
                    0x0000 => {
                        // SE Vx, Vy
                        if self.v_reg[x as usize] == self.v_reg[y as usize] {
                            self.pc += 2;
                        }
                    },
                    _ => {
                        panic!("Invalid instruction {:#06x}.", instruction);
                    }
                }

            },
            0x6000 => {
                // LD Vx, byte
                let kk: u8 = (instruction & 0x00FF) as u8;
                self.v_reg[x as usize] = kk;
            },
            0x7000 => {
                // ADD Vx, byte
                let kk: u8 = (instruction & 0x00FF) as u8;
                self.v_reg[x as usize] = self.v_reg[x as usize].wrapping_add(kk);
            },
            0x8000 => {
                match instruction & 0x000F {
                    0x0000 => {
                        // LD Vx, Vy
                        self.v_reg[x as usize] = self.v_reg[y as usize];
                    },
                    0x0001 => {
                        // OR Vx, Vy
                        self.v_reg[x as usize] |= self.v_reg[y as usize];
                        self.v_reg[15] = 0;
                    },
                    0x0002 => {
                        // AND Vx, Vy
                        self.v_reg[x as usize] &= self.v_reg[y as usize];
                        self.v_reg[15] = 0;
                    },
                    0x0003 => {
                        // XOR Vx, Vy
                        self.v_reg[x as usize] ^= self.v_reg[y as usize];
                        self.v_reg[15] = 0;
                    },
                    0x0004 => {
                        // ADD Vx, Vy
                        let vx: u8 = self.v_reg[x as usize];
                        let vy: u8 = self.v_reg[y as usize];

                        let sum: u8 = vx.wrapping_add(vy);
                        self.v_reg[x as usize] = sum;
                        self.v_reg[15] = (sum < cmp::min(vx, vy)) as u8;                         
                    },
                    0x0005 => {
                        // SUB Vx, Vy
                        let vx: u8 = self.v_reg[x as usize];
                        let vy: u8 = self.v_reg[y as usize];

                        self.v_reg[x as usize] = vx.wrapping_sub(vy);
                        self.v_reg[15] = (vx >= vy) as u8; // Chip8 spec I'm following is incorrect, >= is correct. If vx == vy, then no borrow needed
                    },
                    0x0006 => {
                        // SHR Vx {, Vy}
                        self.v_reg[x as usize] = self.v_reg[y as usize];
                        let vx: u8 = self.v_reg[x as usize];
                        self.v_reg[x as usize] >>= 1;
                        self.v_reg[15] = vx & 0x01;
                    },
                    0x0007 => {
                        // SUBN Vx, Vy
                        let vx: u8 = self.v_reg[x as usize];
                        let vy: u8 = self.v_reg[y as usize];
                        self.v_reg[x as usize] = vy.wrapping_sub(vx);
                        self.v_reg[15] = (vy >= vx) as u8;  // Chip8 spec I'm following is incorrect, >= is correct. If vy == vx, then no borrow needed                   
                    },
                    0x000E => {
                        // SHL Vx {, Vy}
                        self.v_reg[x as usize] = self.v_reg[y as usize];
                        let vx: u8 = self.v_reg[x as usize];
                        self.v_reg[x as usize] <<= 1;
                        self.v_reg[15] = (vx & 0x80) >> 7;
                    },
                    _ => {
                        panic!("Invalid instruction {:#06x}.", instruction);
                    }
                }
            },
            0x9000 => {
                match instruction & 0x000F {
                    0x0000 => {
                        // SNE Vx, Vy
                        if self.v_reg[x as usize] != self.v_reg[y as usize] {
                            self.pc += 2;
                        }
                    },
                    _ => {
                        panic!("Invalid instruction {:#06x}.", instruction);
                    }
                }
            },
            0xA000 => {
                // LD I, addr
                let addr: u16 = instruction & 0x0FFF;
                self.i = addr;
            },
            0xB000 => {
                // JP V0, addr
                let addr: u16 = instruction & 0x0FFF;
                self.pc = addr + (self.v_reg[0] as u16);
            },
            0xC000 => {
                // RND Vx, byte
                let kk: u8 = (instruction & 0x00FF) as u8;

                let mut rng = rand::thread_rng();
                self.v_reg[x as usize] = rng.gen::<u8>() & kk;
            },
            0xD000 => {
                // DRW Vx, Vy, nibble
                let vy: u16 = (self.v_reg[y as usize] as u16) % 32;
                let size_bytes: u16 = instruction & 0x000F;
                
                let mut pixel_erased: bool = false;
                for i in 0u16..size_bytes {
                    // wrap around
                    let y_coord: usize = (vy + i) as usize;
                    if y_coord >= 32 {
                        break;
                    }
                    let sprite_byte = self.ram[(self.i + i) as usize]; 

                    let vx: u16 = (self.v_reg[x as usize] as u16) % 64;
                    for j in 0u16..8 {
                        // wrap around
                        let x_coord: usize = (vx + j) as usize;
                        if x_coord >= 64 {
                            break;
                        }
                        let prev_val: u8 = self.disp_buffer[y_coord][x_coord];
                        // get the bit value and shift to LSB 
                        let pixel = (sprite_byte & (1 << (7 - j))) >> (7 - j);
                        let new_val: u8 = prev_val ^ pixel;

                        if new_val == 0 && prev_val == 1{
                           pixel_erased = true; 
                        }
                        
                        self.disp_buffer[y_coord][x_coord] = new_val;
                    }
                }
                self.v_reg[15] = pixel_erased as u8;
            },
            0xE000 => {
                match instruction & 0x00FF {
                    0x009E => {
                        // SKP Vx
                        if self.keypad & (1 << self.v_reg[x as usize]) > 0 {
                            self.pc += 2;
                            self.keypad = 0;
                        }
                    },
                    0x00A1 => {
                        // SKNP Vx
                        if self.keypad & (1 << self.v_reg[x as usize]) == 0 {
                            self.pc += 2;
                            self.keypad = 0;
                        }
                    },
                    _ => {
                        panic!("Invalid instruction {:#06x}.", instruction);
                    }
                }
            },
            0xF000 => {
                match instruction & 0x00FF {
                    0x0007 => {
                        // LD Vx, DT
                        self.v_reg[x as usize] = self.dt;
                    },
                    0x000A => {
                        // LD Vx, K
                        // If no key set, continue w/o incrementing pc -> re-read the same instruction until the key is set.
                        if self.keypad == 0 {
                            return;
                        }
                        
                        let mut key_shift: u8 = 0;
                        // NOTE this will favor lower key presses in the event of multiple keys pressed at the same time
                        while self.keypad & (1u16 << key_shift) == 0 && key_shift < 16 {
                            key_shift += 1;
                        }
                        if key_shift == 16 {
                            // Shouldn't happen since keypad is u16, so panic
                            panic!("Error reading keypad")
                        }

                        self.v_reg[x as usize] = key_shift;
                        self.keypad = 0;
                    },
                    0x0015 => {
                        // LD DT, Vx
                        self.dt = self.v_reg[x as usize];
                    },
                    0x0018 => {
                        // LD ST, Vx
                        self.st = self.v_reg[x as usize];
                    },
                    0x001E => {
                        // ADD I, Vx
                        self.i = self.i.wrapping_add(self.v_reg[x as usize] as u16);
                    },
                    0x0029 => {
                        // LD F, Vx
                        let vx = self.v_reg[x as usize];
                        // This could error if vx is > 16. Currently: up to the program writer to not violate this.

                        // Each sprite is 5 bytes, so the offset is 5
                        self.i = (vx * 5) as u16;
                    },
                    0x0033 => {
                        // LD, B, Vx
                        let vx = self.v_reg[x as usize];

                        self.ram[self.i as usize] = vx / 100;
                        self.ram[(self.i + 1) as usize] = (vx % 100) / 10; 
                        self.ram[(self.i + 2) as usize] = vx % 10; 
                    },
                    0x0055 => {
                        // LD [I], Vx 
                        for idx in 0..(x as usize + 1) {
                            self.ram[self.i as usize] = self.v_reg[idx]; 
                            self.i += 1;
                        }
                    },
                    0x0065 => {
                        // LD Vx, [I] 
                        for idx in 0..(x as usize + 1) {
                            self.v_reg[idx] = self.ram[self.i as usize];
                            self.i += 1;
                        }
                    },
                    _ => {
                        panic!("Invalid instruction {:#06x}.", instruction);
                    }
                }
            },
            _ => {
                panic!("Invalid instruction {:#06x}.", instruction);
            }
        }

        // Go to the next instruction
        self.pc += 2;
    }

    pub fn get_display(&self) -> &[[u8; 64]; 32] {
        return &self.disp_buffer;
    }

    pub fn set_keypad(&mut self, key: u8) {
        self.keypad |= 1u16 << key;
    }

    pub fn update_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            self.st -= 1;
        }
    }
}
