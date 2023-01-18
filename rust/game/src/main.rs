

use std::thread;

use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::conf::WindowMode;
use ggez::input::keyboard::{KeyCode};

mod pong;
use pong::Ball;
use pong::Player;

mod client;
use client::Client;
use client::Message;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const MIDDLE_OF_SCREEN: f32 = WINDOW_HEIGHT / 2.0;
const PADDING: f32 = 20.0;
const LEFT_SIDE_START_X: f32 = PADDING;
const RIGHT_SIDE_START_X: f32 = WINDOW_WIDTH - 40.0;

struct GameState{
    ball : Ball,
    my_player: Player,
    enemy_player: Player,
    client: Client,
    has_started : bool,
}

impl GameState {
    pub fn new(_ctx: &mut Context, ball: Ball, my_player : Player, enemy_player : Player, client: Client) -> GameState {
        GameState {
            ball: ball,
            my_player : my_player,
            enemy_player : enemy_player,
            client : client,
            has_started: false
        }
    }

    //will be called once both clients are connected
    pub fn start_game(&mut self){
        self.ball.x_velocity = 3.0;
        self.ball.y_velocity = -5.0;
        self.has_started = true;
    }

    pub fn check_if_scored(&mut self) {
         //check if someone scored, if so give points and reset positions
         if self.my_player.point_given {
            self.my_player.score += 1;
            self.my_player.point_given = false;
            self.ball.reset(WINDOW_WIDTH/2.0, WINDOW_HEIGHT/2.0);
            self.my_player.reset_position(self.client.client_nr, LEFT_SIDE_START_X, RIGHT_SIDE_START_X, MIDDLE_OF_SCREEN);
            println!("my_player won round")
        }
        if self.enemy_player.point_given {
            self.enemy_player.score += 1;
            self.enemy_player.point_given = false;
            self.ball.reset(WINDOW_WIDTH/2.0, WINDOW_HEIGHT/2.0);
            self.my_player.reset_position(self.client.client_nr, LEFT_SIDE_START_X, RIGHT_SIDE_START_X, MIDDLE_OF_SCREEN);
            println!("enemy_player won round")
        }
    }

    // sends my players X and Y position to the server
    pub fn send_data(&mut self){
        let message = Message::new(self.my_player.rect.x, self.my_player.rect.y);
        self.client.write(&message.data).unwrap();
    }

    // receives enemy player positions from the server and moves the enemy player
    pub fn receive_data(&mut self){
        let received = self.client.read().unwrap();
        let new_enemy_y_position = Message::get_y_positions(received);
        println!("message recieved from enemy player: {}", new_enemy_y_position);
        self.enemy_player.rect.y = new_enemy_y_position;
        if self.has_started == false{
            self.start_game();
        }
    }
}

// Called 60 times per second
impl EventHandler for GameState {
    // Logic lives here...
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let send_thread = thread::spawn(|| {
            self.send_data();
        });
    
        let receive_thread = thread::spawn(|| {
            self.receive_data();
        });
    
        let check_thread = thread::spawn(|| {
            self.check_if_scored();
        });
    
        //move my player with W and S keys
        let k_ctx = &_ctx.keyboard;
        self.my_player.move_with_keys(k_ctx, WINDOW_HEIGHT);
    
        //update ball
        if(self.client.client_nr == 1) {
            self.ball.update(WINDOW_HEIGHT, WINDOW_WIDTH, &mut self.my_player, &mut self.enemy_player);
        } else {
            self.ball.update(WINDOW_HEIGHT, WINDOW_WIDTH, &mut self.enemy_player, &mut self.my_player);
        }
    
        send_thread.join().unwrap();
        receive_thread.join().unwrap();
        check_thread.join().unwrap();
    
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
    /*Create context and event loop.*/
    let (mut ctx, event_loop) = 
        ContextBuilder::new("Pong in Rust", "Cool Game Author")
        .window_mode(WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT).resizable(true))
        .build()
        .expect("Error, could not create ggez context!");
    ctx.gfx.set_window_title("Pong");

    /*Creates clients and connect to server*/
    let mut client = Client::new();
    let client_nr = client.read_client_id().unwrap();
    client.client_nr = client_nr;
   

    //Create players and set positions according to client id
    let mut my_player = Player::new(0.0,0.0);
    let mut enemy_player = Player::new(0.0,0.0);
    if client.client_nr == 1 {
        my_player = Player::new(LEFT_SIDE_START_X, MIDDLE_OF_SCREEN);
        enemy_player = Player::new(RIGHT_SIDE_START_X, MIDDLE_OF_SCREEN);
    }else {
        my_player = Player::new(RIGHT_SIDE_START_X, MIDDLE_OF_SCREEN);
        enemy_player = Player::new(LEFT_SIDE_START_X, MIDDLE_OF_SCREEN);
    }

    //Create ball and gamestate
    let ball = Ball::new(WINDOW_WIDTH/2.0, MIDDLE_OF_SCREEN);
    let state = GameState::new(&mut ctx, ball, my_player, enemy_player, client);

    /*Starts Game by running the event loop*/
    event::run(ctx, event_loop, state);
}
