// Following chip-8 spec from http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
use std::cmp;
use rand::{self, Rng};

const STACK_SIZE: u8 = 16;

fn main() {
}


struct Chip8 {
    ram: [u8; 4096],
    disp_buffer: [[u8; 64]; 32],

    keypad: u8,

    stack: [u16; STACK_SIZE as usize],
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
    fn _cycle(&mut self) {
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
                // JP addr 
                let addr: u16 = instruction & 0x0FFF;
                self.pc = addr;
            },
            0x2000 => {
                // CALL addr
                let addr: u16 = instruction & 0x0FFF;

                self.sp += 1;
                if self.sp == STACK_SIZE {
                    // TODO fail here - stack overflow
                } 
                self.stack[self.sp as usize] = self.pc;
                self.pc = addr;
            },
            0x3000 => {
                // SE Vx, byte
                let x: u16 = (instruction & 0x0F00) >> 8;
                let kk: u8 = (instruction & 0x00FF) as u8;
                if self.v_reg[x as usize] == kk {
                    self.pc += 2;
                }
            },
            0x4000 => {
                // SNE Vx, byte 
                let x: u16 = (instruction & 0x0F00) >> 8;
                let kk: u8 = (instruction & 0x00FF) as u8;
                if self.v_reg[x as usize] != kk {
                    self.pc += 2;
                }
            },
            0x5000 => {
                match instruction & 0x000F {
                    0x0000 => {
                        // SE Vx, Vy
                        let x: u16 = (instruction & 0x0F00) >> 8;
                        let y: u16 = (instruction & 0x00F0) >> 4;
                        if self.v_reg[x as usize] == self.v_reg[y as usize] {
                            self.pc += 2;
                        }
                    },
                    _ => {
                        // TODO fail here
                    }
                }

            },
            0x6000 => {
                // LD Vx, byte
                let x: u16 = (instruction & 0x0F00) >> 8;
                let kk: u8 = (instruction & 0x00FF) as u8;
                self.v_reg[x as usize] = kk;
            },
            0x7000 => {
                // ADD Vx, byte
                let x: u16 = (instruction & 0x0F00) >> 8;
                let kk: u8 = (instruction & 0x00FF) as u8;
                self.v_reg[x as usize] += kk;
            },
            0x8000 => {
                let x: u16 = (instruction & 0x0F00) >> 8;
                let y: u16 = (instruction & 0x00F0) >> 4;
                match instruction & 0x000F {
                    0x0000 => {
                        // LD Vx, Vy
                        self.v_reg[x as usize] = self.v_reg[y as usize];
                    },
                    0x0001 => {
                        // OR Vx, Vy
                        self.v_reg[x as usize] |= self.v_reg[y as usize];
                    },
                    0x0002 => {
                        // AND Vx, Vy
                        self.v_reg[x as usize] &= self.v_reg[y as usize];
                    },
                    0x0003 => {
                        // XOR Vx, Vy
                        self.v_reg[x as usize] ^= self.v_reg[y as usize];
                    },
                    0x0004 => {
                        // ADD Vx, Vy
                        let vx: u8 = self.v_reg[x as usize];
                        let vy: u8 = self.v_reg[y as usize];

                        let sum: u8 = self.v_reg[x as usize] + self.v_reg[y as usize];
                        self.v_reg[15] = (sum < cmp::min(vx, vy)) as u8;                         
                        self.v_reg[x as usize] = sum;
                    },
                    0x0005 => {
                        // SUB Vx, Vy
                        let vx: u8 = self.v_reg[x as usize];
                        let vy: u8 = self.v_reg[y as usize];

                        self.v_reg[15] = (vx > vy) as u8; 
                        self.v_reg[x as usize] -= vy;
                    },
                    0x0006 => {
                        // SHR Vx {, Vy}
                        let vx: u8 = self.v_reg[x as usize];
                        self.v_reg[15] = vx & 0x01;
                        self.v_reg[x as usize] >>= 1;
                    },
                    0x0007 => {
                        // SUBN Vx, Vy
                        let vx: u8 = self.v_reg[x as usize];
                        let vy: u8 = self.v_reg[y as usize];
                        self.v_reg[15] = (vy > vx) as u8;                     
                        self.v_reg[x as usize] = vy - vx;
                    },
                    0x000E => {
                        // SHL Vx {, Vy}
                        let vx: u8 = self.v_reg[x as usize];
                        self.v_reg[15] = vx & 0x80;
                        self.v_reg[x as usize] <<= 1;
                    },
                    _ => {
                        // TODO fail here
                    }
                }
            },
            0x9000 => {
                match instruction & 0x000F {
                    0x0000 => {
                        // SNE Vx, Vy
                        let x: u16 = (instruction & 0x0F00) >> 8;
                        let y: u16 = (instruction & 0x00F0) >> 4;
                        if self.v_reg[x as usize] == self.v_reg[y as usize] {
                            self.pc += 2;
                        }
                    },
                    _ => {
                        // TODO fail here
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
                let x: u16 = (instruction & 0x0F00) >> 8;
                let kk: u8 = (instruction & 0x00FF) as u8;

                let mut rng = rand::thread_rng();
                self.v_reg[x as usize] = rng.gen::<u8>() & kk;
            },
            0xD000 => {
                // DRW Vx, Vy, nibble
                let x: u16 = (instruction & 0x0F00) >> 8;
                let y: u16 = (instruction & 0x00F0) >> 4;
                let size_bytes: u16 = instruction & 0x000F;
                
                let mut pixel_erased: bool = false;
                for i in 0u16..size_bytes {
                    // wrap around
                    let y_coord: usize = ((y + i) % 32) as usize;
                    let sprite_byte = self.ram[(self.i + i) as usize]; 

                    for j in (0u16..8).rev() {
                        // wrap around
                        let x_coord: usize = ((x + j) % 64) as usize;

                        let prev_val: u8 = self.disp_buffer[y_coord][x_coord];
                        // get the bit value and shift to LSB 
                        let pixel = (sprite_byte & (1 << j)) >> j;
                        let new_val: u8 = prev_val ^ pixel;

                        if new_val != prev_val {
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
                        let x: u16 = (instruction & 0x0F00) >> 8;
                        if self.keypad == self.v_reg[x as usize] {
                            self.pc += 2;
                        }
                    },
                    0x00A1 => {
                        // SKNP Vx
                        let x: u16 = (instruction & 0x0F00) >> 8;
                        if self.keypad != self.v_reg[x as usize] {
                            self.pc += 2;
                        }
                    },
                    _ => {
                        // TODO fail here
                    }
                }
            },
            0xF000 => {
                let x: u16 = (instruction & 0x0F00) >> 8;
                match instruction & 0x00FF {
                    0x0007 => {
                        // LD Vx, DT
                        self.v_reg[x as usize] = self.dt;
                    },
                    0x000A => {
                        // LD Vx, K
                        // TODO wait until key press, store in Vx
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
                        self.i += self.v_reg[x as usize] as u16;
                    },
                    0x0029 => {
                        // LD F, Vx
                        // TODO
                    },
                    0x0033 => {
                        // TODO
                    },
                    0x0055 => {
                        // LD [I], Vx 
                        let mut store_addr: u16 = self.i;
                        for vx in self.v_reg.iter() {
                            self.ram[store_addr as usize] = *vx; 
                            store_addr += 1; 
                        }
                    },
                    0x0065 => {
                        // LD Vx, [I] 
                        for i in 0..self.v_reg.len() {
                            self.v_reg[i] = self.ram[self.i as usize + i];
                        }
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
    }

}
