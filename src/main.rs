use ggez;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics;
use ggez::graphics::{Color};
use ggez::event;

const RACKET_HEIGHT: f32 = 100.0;
const RACKET_WIDTH: f32 = 20.0;

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("Pong_0", "Elijah Sears")
        .build()
        .expect("Could not create context :(");

    ctx.gfx.set_window_title("Pong -------- Elijah Sears"); // This was graphics::set_window_title(&ctx, "Pong -------- Elijah Sears")

    let mut game_state = MainState::new(&mut ctx);
    event::run(ctx, event_loop, game_state) // No semicolon to return GameState
}

struct MainState {
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> Self {
        MainState {}
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        // Draw code here...
        let rect = graphics::Rect::new(10.0, 10.0, 300.0, 150.0);
        let rect_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rect,
            Color::WHITE
        )?;

        canvas.draw(&rect_mesh, graphics::DrawParam::default()); // This was graphics::canvas::draw(&mut canvas, &rect_mesh graphics::DrawParam::default());

        canvas.finish(ctx)
    }
}
