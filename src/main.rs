use ggez;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics;
use ggez::graphics::{Color};
use ggez::event;

const RACKET_HEIGHT: f32 = 100.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5;
const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT * 0.5;


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
        let racket_rect = graphics::Rect::new(-RACKET_WIDTH_HALF, -RACKET_HEIGHT_HALF, RACKET_WIDTH, RACKET_HEIGHT);
        let racket_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            racket_rect,
            Color::WHITE
        )?;

        canvas.draw(&racket_mesh, graphics::DrawParam::default()); // This was graphics::canvas::draw(&mut canvas, &rect_mesh graphics::DrawParam::default());

        canvas.finish(ctx)
    }
}
