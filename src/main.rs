use ggez;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics;
use ggez::graphics::{Color};
// IMPORTANT: DEPRECATED. Use graphics::Point2; use ggez::nalgebra as nalg;
use ggez::mint;
use ggez::event;
use ggez::input::keyboard::{KeyCode};
use rand::{thread_rng, Rng};

const RACKET_HEIGHT: f32 = 100.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5;
const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT * 0.5;

const BALL_SIZE: f32 = 30.0;
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;

const PLAYER_SPEED: f32 = 300.0;

// This function makes sure the racket does not go above/below screen
fn clamp_to_screen(value: &mut f32, low: f32, high: f32) {
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    }
}

fn move_racket(pos: &mut mint::Point2<f32>, keycode: KeyCode, y_dir: f32, ctx: &mut Context) {
    let dt = ctx.time.delta().as_secs_f32(); // Deprecated. Was ggez::timer::delta(ctx).as_secs_f32()
    let screen_h = ctx.gfx.drawable_size().1;
    if ctx.keyboard.is_key_pressed(keycode){
        pos.y -= y_dir * PLAYER_SPEED * dt; // Change to += for inverted controls!
    }
    clamp_to_screen(&mut pos.y, RACKET_HEIGHT_HALF, screen_h-RACKET_HEIGHT_HALF);
}

fn randomize_vec(vec: &mut mint::Vector2<f32>, x: f32, y: f32) {
    let mut rng = thread_rng();
    vec.x = match rng.gen_bool(0.5) {
        true => x,
        false => -x,
    };
    vec.y = match rng.gen_bool(0.5) {
        true => y,
        false => -y,
    };
}

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
    ball_pos: mint::Point2<f32>,
    ball_vel: mint::Vector2<f32>,
    player_1_score: i32,
    player_2_score: i32,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let (screen_w, screen_h) = ctx.gfx.drawable_size(); // Could be graphics::GraphicsContext::drawable_size() with missing argument
        let (screen_w_half, screen_h_half) = (screen_w*0.5, screen_h*0.5);

        let mut ball_vel = mint::Vector2{x: 0.0, y: 0.0};
        randomize_vec(&mut ball_vel, 50.0, 50.0);
        MainState {
            player_1_pos : mint::Point2{x: RACKET_WIDTH_HALF, y: screen_h_half},
            player_2_pos : mint::Point2{x: screen_w-RACKET_WIDTH_HALF, y: screen_h_half},
            ball_pos : mint::Point2{x: screen_w_half, y: screen_h_half},
            ball_vel : ball_vel,
            player_1_score : 0,
            player_2_score : 0,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ctx.time.delta().as_secs_f32();
        move_racket(&mut self.player_1_pos, KeyCode::W, 1.0, ctx); // For these function calls, I inverted the operators by using -=, not +=
        move_racket(&mut self.player_1_pos, KeyCode::S, -1.0, ctx);
        move_racket(&mut self.player_2_pos, KeyCode::Up, 1.0, ctx); // Could change these two to O and L instead of arrow keys
        move_racket(&mut self.player_2_pos, KeyCode::Down, -1.0, ctx);
        self.ball_pos.x += self.ball_vel.x * dt;
        self.ball_pos.y += self.ball_vel.y * dt;

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

        let ball_rect = graphics::Rect::new(-BALL_SIZE_HALF, -BALL_SIZE_HALF, BALL_SIZE, BALL_SIZE);
        let ball_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            ball_rect,
            Color::WHITE
        )?;

        // Location for player 1
        let p1_draw_param = graphics::DrawParam::default()
            .dest(self.player_1_pos);

        // Location for player 2
        let p2_draw_param = graphics::DrawParam::default()
            .dest(self.player_2_pos);

        // Ball Location
        let ball_draw_param = graphics::DrawParam::default()
            .dest(self.ball_pos);

        // Draws to canvas
        canvas.draw(&racket_mesh, p1_draw_param); // This was graphics::canvas::draw(&mut canvas, &rect_mesh graphics::DrawParam::default());
        canvas.draw(&racket_mesh, p2_draw_param);
        canvas.draw(&ball_mesh, ball_draw_param);

        canvas.finish(ctx)
    }
}
