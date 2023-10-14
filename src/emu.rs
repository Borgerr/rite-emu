use rand::Rng;
use std::fmt::{Debug, Display};

pub enum EmulationError {
    StackOverflow,      // emulated stack exceeds 16 entries
    LoadingError, // invoked when the ROM tried to load is larger than 4 kB, or something else happens
    VacantMemory, // invoked when we run into a sequence of 0000s or similar
    UnknownInstruction, // ran into an instruction that looks kind of valid, but isn't ultimately
}

impl Debug for EmulationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StackOverflow => write!(f, "emulated stack overflowed"),
            Self::LoadingError => write!(f, "ROM failed to load, file likely exceeds 4 kB"),
            Self::VacantMemory => write!(
                f,
                "ROM ran out of memory and encountered an instruction like 0000"
            ),
            Self::UnknownInstruction => write!(f, "ran into an unrecognized instruction"),
        }
    }
}

impl Display for EmulationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StackOverflow => write!(f, "emulated stack overflowed"),
            Self::LoadingError => write!(f, "ROM failed to load, file likely exceeds 4 kB"),
            Self::VacantMemory => write!(
                f,
                "ROM ran out of memory and encountered an instruction like 0000"
            ),
            Self::UnknownInstruction => write!(f, "ran into an unrecognized instruction"),
        }
    }
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
/// * `variables` - 16 one byte variable registers
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
    keys: Vec<bool>, // represent each of the 16 keys,
                     // reflects true if this key is held down and false if otherwise
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
            pc: 0x200, // program loads at index 512
            i: 0,
            delay_timer: 0, // special instructions for incrementing the timers
            sound_timer: 0,
            variables: vec![0; 16], // should always have only 16 elements
            keys: vec![false; 16],  // only 16 keys;
                                    // the text printed on the original COSMAC VIP layout
                                    // corresponds to its index in this vector
        }
    }

    pub fn read_rom(&mut self, rom: Vec<u8>) -> Result<(), EmulationError> {
        let mut rom_iter = rom.iter();
        let mut current_address = 0x200;
        while let Some(data) = rom_iter.next() {
            if current_address >= 4096 {
                return Err(EmulationError::LoadingError);
            }
            self.memory[current_address] = *data;
            current_address += 1;
        }
        Ok(())
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
        }
    }
    /// Decrements `sound_timer`,
    /// should be accessed by `main.rs` during the main loop
    pub fn decrement_sound(&mut self) {
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    /// the main portion of our emulated interpreter
    /// where we call all the individual components of the
    /// fetch, decode, execute loop.
    /// This alters the `Emu`'s state,
    /// but should not return anything since pixel data is
    /// accessible from outside,
    /// and that's really all that should be reflected.
    pub fn fetch_decode_execute_instr(&mut self) -> Result<(), EmulationError> {
        let opcode = self.fetch_instruction();
        self.decode_and_execute(opcode)
    }

    /// returns the 16 bit combination of two successive bytes
    /// with relation to instructions
    fn fetch_instruction(&mut self) -> u16 {
        let upper_half = (self.memory[self.pc as usize] as u16) << 8;
        self.pc += 1;
        let lower_half = self.memory[self.pc as usize] as u16;
        self.pc += 1;
        upper_half + lower_half
    }

    /// extracts information from an opcode.
    ///
    /// # Arguments
    /// * `opcode` - 16-bit opcode from which we get our information
    fn extract_from_opcode(opcode: u16) -> (u16, u16, u16, u16, u16, u16) {
        let instr_type = opcode >> 12; // extracting first nibble
        let x = (opcode >> 8) & 0b1111; // extracting the second nibble
                                        // `& 0b1111` discards the first nibble
        let y = (opcode >> 4) & 0b1111; // extracting the third nibble
        let n = opcode & 0b1111; // simply disregarding all but the last nibble
        let nn = opcode & 0xFF; // extracting the second byte
        let nnn = opcode & 0xFFF; // extracting the second, third, and fourth nibbles
                                  // as one 12-bit number (used for memory addresses)

        (instr_type, x, y, n, nn, nnn)
    }

    /// CHIP-8s have a very simple instruction set,
    /// so we combine these two steps into one,
    /// altering the state depending on the operation
    fn decode_and_execute(&mut self, opcode: u16) -> Result<(), EmulationError> {
        let (instr_type, x, y, n, nn, nnn) = Self::extract_from_opcode(opcode);
        match instr_type {
            0x0 => match nnn {
                0x0e0 => self.clear_screen(),
                0x0ee => self.return_from_subroutine(),
                0x000 => Err(EmulationError::VacantMemory),
                _ => Err(EmulationError::UnknownInstruction),
            },
            0x1 => self.jump(nnn),
            0x6 => self.set_register(x, nn),
            0x7 => self.add_val_to_register(x, nn),
            0xa => self.set_index_register(nnn),
            0xd => self.display(x, y, n),
            0x2 => self.call_subroutine(nnn),
            0x3 => self.skip_if_vx_eq_nn(x, nn),
            0x4 => self.skip_if_vx_neq_nn(x, nn),
            0x5 => self.skip_if_vx_eq_vy(x, y),
            0x9 => self.skip_if_vx_neq_vy(x, y),
            0x8 => match n {
                0x0 => self.set_vx_to_vy(x, y),
                0x1 => self.vx_oreq_vy(x, y),
                0x2 => self.vx_andeq_vy(x, y),
                0x3 => self.vx_xoreq_vy(x, y),
                0x4 => self.vx_pluseq_vy(x, y),
                0x5 => self.vx_minuseq_vy(x, y),
                0x7 => self.vx_equals_vy_minus_vx(x, y),
                0x6 => self.shift_right_1bit(x, y),
                0xe => self.shift_left_1bit(x, y),
                _ => Err(EmulationError::UnknownInstruction),
            },
            0xb => self.jump_with_offset(nnn),
            0xc => self.random_gen(x, nn),
            0xe => match nn {
                0x9e => self.skip_if_key(x),
                0xa1 => self.skip_if_not_key(x),
                _ => Err(EmulationError::UnknownInstruction),
            },

            //_ => Err(EmulationError::UnknownInstruction),
            _ => Ok(()),
        }
    }

    // -------------
    // INSTRUCTIONS
    // -------------

    /// # `00E0`
    /// Turns the entire screen off.
    fn clear_screen(&mut self) -> Result<(), EmulationError> {
        self.pixels = vec![false; 64 * 32];
        Ok(())
    }

    /// # `1NNN`
    /// Sets the program counter to `NNN`.
    fn jump(&mut self, nnn: u16) -> Result<(), EmulationError> {
        self.pc = nnn;
        Ok(())
    }

    /// # `6XNN`
    /// Sets register `VX` to value `NN`.
    fn set_register(&mut self, x: u16, nn: u16) -> Result<(), EmulationError> {
        self.variables[x as usize] = nn as u8;
        Ok(())
    }

    /// # `7XNN`
    /// Adds the value `NN` to register `VX`.
    fn add_val_to_register(&mut self, x: u16, nn: u16) -> Result<(), EmulationError> {
        let mut temp = self.variables[x as usize] as u16;
        temp += nn;
        if temp > 255 {
            temp -= 256;
        }
        self.variables[x as usize] = temp as u8;
        Ok(())
    }

    /// # `ANNN`
    /// Sets the index register to `NNN`.
    fn set_index_register(&mut self, nnn: u16) -> Result<(), EmulationError> {
        self.i = nnn;
        Ok(())
    }

    /// # `2NNN`
    /// PC is set to `NNN`, and the previous PC is pushed on the stack,
    /// so we can return to that later.
    fn call_subroutine(&mut self, nnn: u16) -> Result<(), EmulationError> {
        self.stack_push(self.pc)?;
        self.pc = nnn;
        Ok(())
    }

    /// # `00EE`
    /// Returning from a subroutine by setting the program counter
    /// to whatever is popped from the stack.
    fn return_from_subroutine(&mut self) -> Result<(), EmulationError> {
        self.pc = self.stack_pop();
        Ok(())
    }

    /// # `3XNN`
    /// Skips one instruction if value in `VX` is equal to `NN`.
    fn skip_if_vx_eq_nn(&mut self, x: u16, nn: u16) -> Result<(), EmulationError> {
        if self.variables[x as usize] == nn as u8 {
            self.pc += 2;
        }
        Ok(())
    }

    /// # `4XNN`
    /// Skips one instruction if the value in `VX` is not equal to `NN`.
    fn skip_if_vx_neq_nn(&mut self, x: u16, nn: u16) -> Result<(), EmulationError> {
        if self.variables[x as usize] != nn as u8 {
            self.pc += 2;
        }
        Ok(())
    }

    /// # `5XY0`
    /// Skips one instruction if the value in `VX` is equal to the value in `VY`.
    fn skip_if_vx_eq_vy(&mut self, x: u16, y: u16) -> Result<(), EmulationError> {
        if self.variables[x as usize] == self.variables[y as usize] {
            self.pc += 2;
        }
        Ok(())
    }

    /// # `9XY0`
    /// Skips one instruction if the value in `VX` is not equal to the value in `VY`.
    fn skip_if_vx_neq_vy(&mut self, x: u16, y: u16) -> Result<(), EmulationError> {
        if self.variables[x as usize] != self.variables[y as usize] {
            self.pc += 2;
        }
        Ok(())
    }

    /// # `8XY0`
    /// `VX` is set to the value of `VY`.
    fn set_vx_to_vy(&mut self, x: u16, y: u16) -> Result<(), EmulationError> {
        self.variables[x as usize] = self.variables[y as usize];
        Ok(())
    }

    /// # `8XY1`
    /// `VX` is set to the OR of `VX` and `VY`, leaving `VY` unaffected.
    fn vx_oreq_vy(&mut self, x: u16, y: u16) -> Result<(), EmulationError> {
        self.variables[x as usize] |= self.variables[y as usize];
        Ok(())
    }

    /// # `8XY2`
    /// `VX` is set to the AND of `VX` and `VY`, leaving `VY` unaffected.
    fn vx_andeq_vy(&mut self, x: u16, y: u16) -> Result<(), EmulationError> {
        self.variables[x as usize] &= self.variables[y as usize];
        Ok(())
    }

    /// # `8XY3`
    /// `VX` is set to the XOR of `VX` and `VY`, leaving `VY` unaffected.
    fn vx_xoreq_vy(&mut self, x: u16, y: u16) -> Result<(), EmulationError> {
        self.variables[x as usize] ^= self.variables[y as usize];
        Ok(())
    }

    /// # `8XY4`
    /// `VX` is set to the value of `VX` plus the value of `VY`, leaving `VY` unaffected.
    /// If the result is larger than 255, the flag register `VF` is set to 1.
    fn vx_pluseq_vy(&mut self, x: u16, y: u16) -> Result<(), EmulationError> {
        self.variables[0xf] = 0;
        let mut result: u32 =
            (self.variables[x as usize] as u32) + (self.variables[y as usize] as u32);
        if result >= 256 {
            result -= 256;
            self.variables[0xf] = 1;
        }
        self.variables[x as usize] = result as u8;
        Ok(())
    }

    /// # `8XY5`
    /// `VX` is set to the value of `VX` minus the value of `VY`, leaving `VY` unaffected.
    /// If the result has underflow, `VF` is set to 0. Otherwise, `VF` is set to 1.
    fn vx_minuseq_vy(&mut self, x: u16, y: u16) -> Result<(), EmulationError> {
        self.variables[0xf] = 1;
        let mut x_val = self.variables[x as usize] as i16;
        let y_val = self.variables[y as usize] as i16;
        x_val -= y_val;
        if x_val < 0 {
            x_val += 256;
            self.variables[0xf] = 0;
        }
        self.variables[x as usize] = x_val as u8;
        Ok(())
    }

    /// # `8XY7`
    /// `VX` is set to the value of `VY` minus the value of `VX`, leaving `VY` unaffected.
    /// If the result has underflow, `VX` is set to 0. Otherwise, `VF` is set to 1.
    fn vx_equals_vy_minus_vx(&mut self, x: u16, y: u16) -> Result<(), EmulationError> {
        self.variables[0xf] = 1;
        let x_val = self.variables[x as usize] as i16;
        let mut y_val = self.variables[y as usize] as i16;
        y_val -= x_val;
        if y_val < 0 {
            y_val += 256;
            self.variables[0xf] = 0;
        }
        self.variables[x as usize] = y_val as u8;
        Ok(())
    }

    /// # `8XY6`
    /// THIS INSTRUCTION IS AMBIGUOUS!
    /// Some implementations may have a different functioning,
    /// specifically setting `VX` to `VY` before shifting that value to the left
    /// by one bit.
    ///
    /// Shifts the value in `VX` to the left by one bit,
    /// and then sets `VF` to the bit that was shifted out.
    fn shift_left_1bit(&mut self, x: u16, _y: u16) -> Result<(), EmulationError> {
        // y is of course unused at the moment
        // but we can change this implementation to follow the other behavior
        // by only altering this function (or adding some larger-scale configuration)
        let to_shift = self.variables[x as usize];
        if to_shift & 0xf0 != 0 {
            // leftmost bit is 1
            self.variables[0xf] = 1;
        } else {
            self.variables[0xf] = 0;
        }
        self.variables[x as usize] = to_shift << 1;

        Ok(())
    }

    /// # `8XYE`
    /// THIS INSTRUCTION IS AMBIGUOUS!
    /// Some implementations may have a different functioning,
    /// specifically setting `VX` to `VY` before shifting that value to the right
    /// by one bit.
    ///
    /// Shifts the value in `VX` to the right by one bit,
    /// and then sets `VF` to the bit that was shifted out.
    fn shift_right_1bit(&mut self, x: u16, _y: u16) -> Result<(), EmulationError> {
        // same situation as Emu.shift_left_1bit
        let to_shift = self.variables[x as usize];
        if to_shift & 0x1 != 0 {
            // rightmost bit is 1
            self.variables[0xf] = 1;
        } else {
            self.variables[0xf] = 0;
        }
        self.variables[x as usize] = to_shift >> 1;

        Ok(())
    }

    /// # `BNNN`
    /// THIS INSTRUCTION IS AMBIGUOUS!
    /// Some implementations may have a different functioning,
    /// basically working as an alternate `BXNN`.
    ///
    /// Program counter jumps to the value of
    /// `NNN` plus the value stored in `V0`.
    fn jump_with_offset(&mut self, nnn: u16) -> Result<(), EmulationError> {
        self.pc = nnn + (self.variables[0x0] as u16);
        Ok(())
    }

    /// # `CXNN`
    /// Generates a random number, binary ANDs with value `NN`,
    /// and puts that result in `VX`.
    fn random_gen(&mut self, x: u16, nn: u16) -> Result<(), EmulationError> {
        let mut rng = rand::thread_rng();
        let generated: u8 = rng.gen();
        self.variables[x as usize] = generated & (nn as u8);
        Ok(())
    }

    // TODO: EX9E, EXA1 instructions and beyond

    fn skip_if_key(&mut self, x: u16) -> Result<(), EmulationError> {
        let key_pos = self.variables[x as usize] as usize;
        if self.keys[key_pos] {
            self.pc += 2;
        }

        Ok(())
    }

    fn skip_if_not_key(&mut self, x: u16) -> Result<(), EmulationError> {
        let key_pos = self.variables[x as usize] as usize;
        if !self.keys[key_pos] {
            self.pc += 2;
        }

        Ok(())
    }

    /// # `DXYN`
    /// Draws an `N` pixels tall sprite from memory location
    /// that the index register is currently pointing at,
    /// at horizontal X coordinate in `VX` and vertical Y coordinate in `VY`.
    /// All pixels that are "on" will flip the pixels on the screen.
    ///
    /// If any pixels on the screen were turned "off" by doing this,
    /// `VF` register is set to 1. Otherwise, it's set to 0.
    fn display(&mut self, x: u16, y: u16, n: u16) -> Result<(), EmulationError> {
        // starting position wraps, so we can do the same as
        // binary anding (or modulo) the display
        // the actual drawing of the sprite does not wrap however
        let mut x = (self.variables[x as usize] & 63) as usize;
        let mut y = (self.variables[y as usize] & 31) as usize;
        self.variables[0xf] = 0;

        //(x + y * 64) as usize

        for byte_index in 0..n {
            let mut sprite_byte = self.memory[(self.i + byte_index) as usize];
            if y == 31 {
                break;
            }

            // for each bit in this sprite row...
            for i in 0..8 {
                if sprite_byte & 0x80 != 0 {
                    // leftmost bit is "turned on", 2^i
                    if self.pixels[(x + y * 64) as usize] {
                        self.pixels[(x + y * 64) as usize] = false;
                        self.variables[0xf] = 1;
                    } else {
                        self.pixels[(x + y * 64) as usize] = true;
                    }
                }
                x += 1;
                if x == 63 || i == 7 {
                    x -= i + 1;
                    y += 1;
                    break;
                }
                sprite_byte <<= 1;
            }
        }

        Ok(())
    }

    // -----------
    // KEYPRESSES
    // -----------
    /// tells the emulator that a key was pressed
    pub fn keypress(&mut self, key_index: usize) {
        self.keys[key_index] = true;
    }
    /// tells the emulator that a key was released
    pub fn keyrelease(&mut self, key_index: usize) {
        self.keys[key_index] = false;
    }
}

// these tests are kind of sparse since we have a few ROMs that test for us

#[test]
fn test_instruction_fetch() {
    // tests on the first two bytes of font data
    // that the return is correct
    let mut emulator = Emu::new();
    emulator.pc = 0x050;
    assert_eq!(emulator.fetch_instruction(), 0xF090);
}

#[test]
fn test_opcode_extraction() {
    // just getting a lot of arbitrary hex
    // and seeing if it all works
    let first_nibble = 0x1;
    let second_nibble = 0xa;
    let third_nibble = 0xc;
    let fourth_nibble = 0x9;
    let nibbles_34 = 0xc9;
    let nibbles_234 = 0xac9;

    let (instr_type, x, y, n, nn, nnn) = Emu::extract_from_opcode(0x1ac9);

    assert_eq!(first_nibble, instr_type);
    assert_eq!(second_nibble, x);
    assert_eq!(third_nibble, y);
    assert_eq!(fourth_nibble, n);
    assert_eq!(nibbles_34, nn);
    assert_eq!(nibbles_234, nnn);
}
