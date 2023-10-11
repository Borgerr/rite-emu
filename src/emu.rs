pub struct Emu {
    pub pixels: Vec<bool>, // true if on, false if off.
    the_stack: Vec<u16>,   // stack for 16-bit addresses
    memory: Vec<u8>,       // memory; should really only be up to 4 kB large
    pc: u16,               // program counter, points to current instruction in memory
    i: u16,                // index register, points at locations in memory
    delay_timer: u8,       // decremented at a rate of 60 Hz until it reaches zero
    sound_timer: u8,       // functions like the delay timer, but gives a beeping noise
    // as long as it isn't 0
    variables: Vec<u8>, // 16 variable registers
}

impl Emu {
    pub fn new() -> Self {
        let mut memory: Vec<u8> = vec![0; 4096];
        // font stuff. this is a LOT of hex,
        // but this is basically just the standard font to use with CHIP-8.
        // Each line corresponds to a sprite for its commented character
        let fonts = vec![
            // load font data
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];
        let mut fonts_iter = fonts.iter();
        for i in 0x050..0x09F {
            if let Some(hex) = fonts_iter.next() {
                memory[i] = *hex;
            }
        }

        Emu {
            pixels: vec![false; 64 * 32], // display is 32 by 64
            the_stack: vec![],
            memory,
            // maybe change these later VVV
            pc: 0,
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            variables: vec![0; 16], // should always have only 16 elements
        }
    }
}
