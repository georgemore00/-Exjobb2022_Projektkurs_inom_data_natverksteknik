use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::conf::WindowMode;
use ggez::input::keyboard::{KeyCode};

mod pong;
use pong::Ball;
use pong::Player;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const MIDDLE_OF_SCREEN: f32 = WINDOW_HEIGHT / 2.0;
const PADDING: f32 = 20.0;
const MY_PLAYER_START_X: f32 = PADDING;
const ENEMY_PLAYER_START_X: f32 = WINDOW_WIDTH - 40.0;

struct GameState{
    ball : Ball,
    my_player: Player,
    enemy_player: Player
}

impl GameState {
    pub fn new(_ctx: &mut Context, ball: Ball, my_player : Player, enemy_player : Player) -> GameState {
        GameState {
            ball : ball,
            my_player : my_player,
            enemy_player : enemy_player
        }
    }
}

// Called 60 times per second
impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...

        //READ from server / Write to server code...!!!!!!!!!!!!

        //move my player with W and S keys
        let k_ctx = &_ctx.keyboard;
        if k_ctx.is_key_pressed(KeyCode::S) {
            if self.my_player.rect.y + self.my_player.rect.h < WINDOW_HEIGHT{
                self.my_player.rect.y += 4.5;
            }
        }
        else if k_ctx.is_key_pressed(KeyCode::W) {
            if self.my_player.rect.y > 0.0 {
                self.my_player.rect.y -= 4.5;
            }
        }

        //move enemy player with up and down keys !!!!!!!!!!! TA BORT SENARE
        let u_ctx = &_ctx.keyboard;
        if u_ctx.is_key_pressed(KeyCode::Down) {
            if self.enemy_player.rect.y + self.my_player.rect.h < WINDOW_HEIGHT{
                self.enemy_player.rect.y += 4.5;
            }
        }
        else if u_ctx.is_key_pressed(KeyCode::Up) {
            if self.enemy_player.rect.y > 0.0 {
                self.enemy_player.rect.y -= 4.5;
            }
        }

        self.ball.update(WINDOW_HEIGHT, WINDOW_WIDTH, &mut self.my_player, &mut self.enemy_player);

        if self.my_player.point_given {
            self.my_player.score += 1;
            self.my_player.point_given = false;
            self.ball.check_reset(WINDOW_WIDTH/2.0, WINDOW_HEIGHT/2.0);
            self.my_player.reset_position(MY_PLAYER_START_X, MIDDLE_OF_SCREEN);
            println!("my_player won round")
        }
        if self.enemy_player.point_given {
            self.enemy_player.score += 1;
            self.enemy_player.point_given = false;
            self.ball.check_reset(WINDOW_WIDTH/2.0, WINDOW_HEIGHT/2.0);
            self.enemy_player.reset_position(ENEMY_PLAYER_START_X, MIDDLE_OF_SCREEN);
            println!("enemy_player won round")
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
        .expect("Error creating ball mesh");

        //create my player object
        let my_player_mesh = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        self.my_player.rect,
        Color::WHITE)
        .expect("Error creating player mesh");

        //create enemy player object
        let enemy_player_mesh = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        self.enemy_player.rect,
        Color::WHITE)
        .expect("Error creating player mesh");

        //draw canvas, ball, players
        let draw_param = graphics::DrawParam::default();
        canvas.draw(&ball_mesh, draw_param);
        canvas.draw(&my_player_mesh, draw_param);
        canvas.draw(&enemy_player_mesh, draw_param);
        canvas.finish(ctx)
    }
}

fn main() {
    // Create context and event loop.
    let (mut ctx, event_loop) = 
        ContextBuilder::new("Pong in Rust", "Cool Game Author")
        .window_mode(WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT).resizable(true))
        .build()
        .expect("Error, could not create ggez context!");
    ctx.gfx.set_window_title("Pong");

    //create ball
    let ball = Ball::new(WINDOW_WIDTH/2.0, MIDDLE_OF_SCREEN);

    //create my player
    let my_player = Player::new(MY_PLAYER_START_X, MIDDLE_OF_SCREEN);

    //create enemy player
    let enemy_player = Player::new(ENEMY_PLAYER_START_X, MIDDLE_OF_SCREEN);

    //init game state
    let state = GameState::new(&mut ctx, ball, my_player, enemy_player);

    // Starts GameState by running the event loop
    event::run(ctx, event_loop, state);
}