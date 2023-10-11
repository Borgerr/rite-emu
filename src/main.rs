use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
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
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> MainState {
        // Load/create resources such as images here.
        MainState {
            emulator: emu::Emu::new(),
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        // Draw code here...
        canvas.finish(ctx)
    }
}
