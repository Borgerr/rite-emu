enum EmulationError {
    StackOverflow,
}

/// Represents the actual emulation of a CHIP-8 system.
///
/// # Fields
/// * `pixels` - public field representing on/off data for the screen's pixels
/// * `the_stack` - stack for 16-bit addresses
/// * `memory` - 4 kB of memory, represented as a `Vec<u8>`
/// * `pc` - the program counter, decodes to current instruction in memory
/// * `i` - index register, points at various locations in memory
/// * `delay_timer` - weird delay thing that CHIP-8 programs use
/// * `sound_timer` - like `delay_timer` but for sound
/// * `variables` - 16 one byte variable registers.
pub struct Emu {
    pub pixels: Vec<bool>, // true if on, false if off.
    the_stack: Vec<u16>,   // stack for 16-bit addresses
    memory: Vec<u8>,       // memory; should really only be up to 4 kB large
    pc: u16,               // program counter, points to current instruction in memory
    i: u16,                // index register, points at locations in memory
    delay_timer: u8,       // decremented at a rate of 60 Hz until it reaches zero
    sound_timer: u8,       // functions like the delay timer, but gives a beeping noise
    // as long as it isn't 0
    variables: Vec<u8>, // 16 variable registers- could be represented instead with 0 - F.
                        // F (the last register) is used as a flag register,
                        // i.e. instructions may set it to 1 or 0 from some rule.
}

impl Emu {
    /// returns an instance of Emu.
    /// Everything is initialized to the basic emulation environment,
    /// but without the actual program.
    pub fn new() -> Self {
        let mut memory: Vec<u8> = vec![0; 4096];
        // font stuff. this is a LOT of hex,
        // but this is basically just the standard font to use with CHIP-8.
        // Each line corresponds to a sprite for its commented character
        // everything up to fonts was originally reserved for the CHIP-8 interpreter.
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
            delay_timer: 255,
            sound_timer: 255,
            variables: vec![0; 16], // should always have only 16 elements
        }
    }

    /// Pushing to the stack with the mandate of a 16 entry limit
    ///
    /// # Arguments:
    /// * self
    /// * `entry` - 16-bit entry to be placed on the stack. Should be something like an address.
    fn stack_push(&mut self, entry: u16) -> Result<(), EmulationError> {
        self.the_stack.push(entry);
        if self.the_stack.len() > 16 {
            Err(EmulationError::StackOverflow)
        } else {
            Ok(())
        }
    }
    /// Pops from the stack.
    /// I just think this is nicer within the context of OOP,
    /// given the presence of `stack_push`.
    fn stack_pop(&mut self) -> u16 {
        if let Some(val) = self.the_stack.pop() {
            val
        } else {
            0 // maybe revisit this alternative return value
        }
    }

    /// Decrements `delay_timer`,
    /// should be accessed by `main.rs` during the main loop
    pub fn decrement_delay(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        } else {
            self.delay_timer = 255;
        }
    }
    /// Decrements `sound_timer`,
    /// should be accessed by `main.rs` during the main loop
    pub fn decrement_sound(&mut self) {
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        } else {
            self.delay_timer = 255;
        }
    }

    // -----------
    // KEYPRESSES
    // -----------
    pub fn keypad_1_press(&mut self) {
        println!("COSMAC VIP layout 1 key pressed");
    }
    pub fn keypad_2_press(&mut self) {
        println!("COSMAC VIP layout 2 key pressed");
    }
    pub fn keypad_3_press(&mut self) {
        println!("COSMAC VIP layout 3 key pressed");
    }
    pub fn keypad_c_press(&mut self) {
        println!("COSMAC VIP layout C key pressed");
    }
    pub fn keypad_4_press(&mut self) {
        println!("COSMAC VIP layout 4 key pressed");
    }
    pub fn keypad_5_press(&mut self) {
        println!("COSMAC VIP layout 5 key pressed");
    }
    pub fn keypad_6_press(&mut self) {
        println!("COSMAC VIP layout 6 key pressed");
    }
    pub fn keypad_d_press(&mut self) {
        println!("COSMAC VIP layout D key pressed");
    }
    pub fn keypad_7_press(&mut self) {
        println!("COSMAC VIP layout 7 key pressed");
    }
    pub fn keypad_8_press(&mut self) {
        println!("COSMAC VIP layout 8 key pressed");
    }
    pub fn keypad_9_press(&mut self) {
        println!("COSMAC VIP layout 9 key pressed");
    }
    pub fn keypad_e_press(&mut self) {
        println!("COSMAC VIP layout E key pressed");
    }
    pub fn keypad_a_press(&mut self) {
        println!("COSMAC VIP layout A key pressed");
    }
    pub fn keypad_0_press(&mut self) {
        println!("COSMAC VIP layout 0 key pressed");
    }
    pub fn keypad_b_press(&mut self) {
        println!("COSMAC VIP layout B key pressed");
    }
    pub fn keypad_f_press(&mut self) {
        println!("COSMAC VIP layout F key pressed");
    }
}
