use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Canvas, Color, DrawParam, Mesh};
use ggez::input::keyboard;
use ggez::{Context, ContextBuilder, GameResult};

use std::fs::read;
use std::io::stdin;

mod emu;
use emu::{Emu, EmulationError};

// this file essentially comes from the ggez template
// look there if you want more explanation for what all these things do
// otherwise you can check stuff out with intellisense

fn main() {
    // CHIP-8s use a 32 x 64 pixel screen!
    let width = 64;
    let height = 32;

    // Make a Context...
    let cb = ContextBuilder::new("Rite", "ash")
        .window_setup(WindowSetup::default().title("rite-emu"))
        .window_mode(WindowMode::default().dimensions((width * 15) as f32, (height * 15) as f32));

    let (mut ctx, event_loop) = cb.build().expect("guh, could not create ggez context.");

    // get filepath for ROM
    println!("relative path to ROM: ");
    let mut filepath = String::new();
    stdin()
        .read_line(&mut filepath)
        .expect("failed to read line");
    filepath = filepath.trim().to_string();
    // get ROM data
    let rom = read(filepath).expect("Error reading the given ROM filepath");

    let state = MainState::new(&mut ctx, rom).expect("Error reading the given ROM filepath");

    // Run!
    event::run(ctx, event_loop, state);
}

struct MainState {
    emulator: emu::Emu,
    squares: Vec<Mesh>, // the actually drawn representation of pixels
}

impl MainState {
    pub fn new(ctx: &mut Context, rom: Vec<u8>) -> Result<MainState, EmulationError> {
        /*
        let mut squares: Vec<Mesh> = vec![];
        for i in 0..32 {
            for j in 0..64 {
                let x = (j * 15) as f32;
                let y = (i * 15) as f32;
                let rectangle = Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect {
                        x,
                        y,
                        w: 15.0,
                        h: 15.0,
                    },
                    Color::BLACK,
                );
                squares.push(rectangle.unwrap());
            }
        }
        */

        let mut emulator = Emu::new();
        let _ = emulator.read_rom(rom);

        Ok(MainState {
            emulator,
            squares: vec![],
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Something here about doing so many instructions per frame
        // utilize a TimeContext for this
        const DESIRED_FPS: u32 = 60;

        while ctx.time.check_update_time(DESIRED_FPS) {
            // check if we're on target for 60 fps
            // and if so, do the thing.
            for _i in 0..11 {
                // 10-12 instructions per frame at 60 FPS
                self.emulator.fetch_decode_execute_instr();
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::WHITE);
        for y in 0..32 {
            for x in 0..64 {
                let pixel_index = (x + (y * 64)) as usize;
                let x = (x * 15) as f32;
                let y = (y * 15) as f32;
                if self.emulator.pixels[pixel_index] {
                    // pixel is turned on
                    canvas.draw(
                        &graphics::Quad,
                        DrawParam::default()
                            .color(Color::WHITE)
                            .scale([15., 15.])
                            .dest([x, y]),
                    );
                } else {
                    // pixel is turned off
                    canvas.draw(
                        &graphics::Quad,
                        DrawParam::default()
                            .color(Color::BLACK)
                            .scale([15., 15.])
                            .dest([x, y]),
                    );
                }
            }
        }

        canvas.finish(ctx)?;

        ggez::timer::yield_now();

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        match input.scancode {
            // all scancodes taken from
            // https://www.win.tue.nl/~aeb/linux/kbd/scancodes-1.html
            // since the URL has "linux" as a directory, I'm concerned if this works the same on windows
            // we will check this out later but it all works on my machine
            // MacOS can suffer (I don't have an accessible mac)
            0x01 => ctx.request_quit(), // escape key

            // first four correspond to 1 2 3 C on COSMAC VIP keypad layout
            0x02 => self.emulator.keypad_1_press(), // QWERTY position of 1 key
            0x03 => self.emulator.keypad_2_press(), // QWERTY position of 2 key
            0x04 => self.emulator.keypad_3_press(), // QWERTY position of 3 key
            0x05 => self.emulator.keypad_c_press(), // QWERTY position of 4 key

            // second four correspond to 4 5 6 D on COSMAC VIP keypad layout
            0x10 => self.emulator.keypad_4_press(), // QWERTY position of Q key
            0x11 => self.emulator.keypad_5_press(), // QWERTY position of W key
            0x12 => self.emulator.keypad_6_press(), // QWERTY position of E key
            0x13 => self.emulator.keypad_d_press(), // QWERTY position of R key

            // third four correspond to 7 8 9 E on COSMAC VIP keypad layout
            0x1e => self.emulator.keypad_7_press(), // QWERTY position of A key
            0x1f => self.emulator.keypad_8_press(), // QWERTY position of S key
            0x20 => self.emulator.keypad_9_press(), // QWERTY position of D key
            0x21 => self.emulator.keypad_e_press(), // QWERTY position of F key

            // fourth four correspond to A 0 B F on COSMAC VIP keypad layout
            0x2c => self.emulator.keypad_a_press(), // QWERTY position of Z key
            0x2d => self.emulator.keypad_0_press(), // QWERTY position of X key
            0x2e => self.emulator.keypad_b_press(), // QWERTY position of C key
            0x2f => self.emulator.keypad_f_press(), // QWERTY position of V key
            _ => (),
        }

        Ok(())
    }
}
