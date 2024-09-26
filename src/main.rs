// Following chip-8 spec from http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
fn main() {
}

struct Chip8 {
    ram: [u8; 4096],
    disp_buffer: [u8; 2048],

    keypad: u16,

    stack: u16,
    sp: u8, // stack pointer

    pc: u16, // program counter

    // registers
    v0: u16,
    v1: u16,
    v2: u16,
    v3: u16,
    v4: u16,
    v5: u16,
    v6: u16,
    v7: u16,
    v8: u16,
    v9: u16,
    va: u16,
    vb: u16,
    vc: u16,
    vd: u16,
    ve: u16,
    vf: u16,
    i: u16,

    dt: u8, // delay timer
    st: u8, // sound timer
}

impl Default for Chip8 {
    fn default() -> Self {
        Chip8 {
            ram: [0u8; 4096],
            disp_buffer: [0u8; 2048],
            keypad: 0,
            stack: 0,
            sp: 0,
            pc: 0x0200,
            v0: 0,
            v1: 0,
            v2: 0,
            v3: 0,
            v4: 0,
            v5: 0,
            v6: 0,
            v7: 0,
            v8: 0,
            v9: 0,
            va: 0,
            vb: 0,
            vc: 0,
            vd: 0,
            ve: 0,
            vf: 0,
            i: 0,
            dt: 0,
            st: 0,
        }
    }
}

