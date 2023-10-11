use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Canvas, Color, DrawParam, Mesh};
use ggez::{Context, ContextBuilder, GameResult};

mod emu;

fn main() {
    // CHIP-8s use a 32 x 64 pixel screen!
    let width = 64;
    let height = 32;

    // Make a Context.
    let cb = ContextBuilder::new("Rite", "ash")
        .window_setup(WindowSetup::default().title("Rite"))
        .window_mode(WindowMode::default().dimensions((width * 15) as f32, (height * 15) as f32));

    let (mut ctx, event_loop) = cb.build().expect("guh, could not create ggez context.");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let state = MainState::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, state);
}

struct MainState {
    emulator: emu::Emu,
    squares: Vec<Mesh>, // the actually drawn representation of pixels
}

impl MainState {
    pub fn new(ctx: &mut Context) -> MainState {
        // Load/create resources such as images here.
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
        MainState {
            emulator: emu::Emu::new(),
            squares,
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::WHITE);
        for i in 0..(64 * 32) {
            if self.emulator.pixels[i] {
                // pixel is turned on
                canvas.draw(&self.squares[i], DrawParam::default().color(Color::WHITE));
            } else {
                // pixel is turned off
                canvas.draw(&self.squares[i], DrawParam::default().color(Color::BLACK));
            }
        }
        canvas.finish(ctx)
    }
}
