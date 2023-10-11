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
        Emu {
            pixels: vec![false; 64 * 32], // display is 32 by 64
            the_stack: vec![],
            memory: vec![0; 4096], // 4 kB == 4096 u8s
            // maybe change these later VVV
            pc: 0,
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            variables: vec![0; 16], // should always have only 16 elements
        }
    }
}
