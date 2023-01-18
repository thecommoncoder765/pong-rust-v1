use ggez;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics;
use ggez::graphics::{Color};
// IMPORTANT: DEPRECATED. Use graphics::Point2; use ggez::nalgebra as nalg;
use ggez::mint;
use ggez::event;

const RACKET_HEIGHT: f32 = 100.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5;
const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT * 0.5;

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("Pong_0", "Elijah Sears")
        .build()
        .expect("Could not create context :(");

    ctx.gfx.set_window_title("Pong -- Elijah Sears"); // This was graphics::set_window_title(&ctx, "Pong -------- Elijah Sears")

    let mut game_state = MainState::new(&mut ctx);
    event::run(ctx, event_loop, game_state) // No semicolon to return GameState
}

struct MainState {
    player_1_pos: mint::Point2<f32>,
    player_2_pos: mint::Point2<f32>,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let (screen_w, screen_h) = ctx.gfx.drawable_size(); // Could be graphics::GraphicsContext::drawable_size() with missing argument
        let (screen_w_half, screen_h_half) = (screen_w*0.5, screen_h*0.5);

        MainState {
            player_1_pos : mint::Point2{x: RACKET_WIDTH_HALF, y: screen_h_half},
            player_2_pos : mint::Point2{x: screen_w-RACKET_WIDTH_HALF, y: screen_h_half},
        }
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

        // Location for player 1
        let p1_draw_param = graphics::DrawParam::default()
            .dest(self.player_1_pos);

        // Location for player 2
        let p2_draw_param = graphics::DrawParam::default()
            .dest(self.player_2_pos);

        // Draws player 1 & 2
        canvas.draw(&racket_mesh, p1_draw_param); // This was graphics::canvas::draw(&mut canvas, &rect_mesh graphics::DrawParam::default());
        // P2
        canvas.draw(&racket_mesh, p2_draw_param);


        canvas.finish(ctx)
    }
}
