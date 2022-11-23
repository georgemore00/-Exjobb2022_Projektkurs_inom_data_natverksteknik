use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::conf::WindowMode;
use ggez::input::keyboard::{KeyCode};

mod pong;
use pong::Ball;
use pong::Player;


const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const MIDDLE_OF_SCREEN: f32 = SCREEN_HEIGHT / 2.0;
const PADDING: f32 = 20.0;
const MY_PLAYER_START_X: f32 = PADDING;


struct GameState{
    ball : Ball,
    my_player: Player
}

impl GameState {
    pub fn new(_ctx: &mut Context, ball: Ball, my_player : Player) -> GameState {
        GameState {
            ball: ball,
            my_player : my_player
        }
    }
}


impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
         // Update code here...
         self.ball.update(SCREEN_HEIGHT, &self.my_player);

        //move my player with W and S keys
        let k_ctx = &_ctx.keyboard;
        if k_ctx.is_key_pressed(KeyCode::S) {
            if self.my_player.rect.y + self.my_player.rect.h< SCREEN_HEIGHT{
                self.my_player.rect.y += 4.5;
            }
        }else if k_ctx.is_key_pressed(KeyCode::W) {
            if self.my_player.rect.y + self.my_player.rect.h> 0.0 {
                self.my_player.rect.y -= 4.5;
            }
        }
        Ok(())
     }
 
     fn draw(&mut self, ctx: &mut Context) -> GameResult {
        //create black canvas
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        //create ball object
        let ball_mesh = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        self.ball.rect,
        Color::WHITE)
        .expect("error creating ball mesh");


        //create my player object
        let my_player_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.my_player.rect,
            Color::WHITE)
            .expect("error creating ball mesh");


        //draw canvas, ball, players
        let draw_param = graphics::DrawParam::default();
        canvas.draw(&ball_mesh, draw_param);
        canvas.draw(&my_player_mesh, draw_param);
        canvas.finish(ctx)
     }
 }

fn main() {
    // Create context and event loop.
    let (mut ctx, event_loop) = 
        ContextBuilder::new("Pong in Rust", "Cool Game Author")
        .window_mode(WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT).resizable(true))
        .build()
        .expect("Error, could not create ggez context!");
    ctx.gfx.set_window_title("Pong");

    //create ball
    let ball = Ball::new(SCREEN_WIDTH/2.0, MIDDLE_OF_SCREEN);

    //create my player
    let my_player = Player::new(MY_PLAYER_START_X, MIDDLE_OF_SCREEN);

    //init game state
    let state = GameState::new(&mut ctx, ball, my_player);

    // Starts GameState by running the event loop
    event::run(ctx, event_loop, state);
}
