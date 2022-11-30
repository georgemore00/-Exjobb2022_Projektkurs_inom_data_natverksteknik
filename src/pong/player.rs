use ggez::{graphics::{Rect}};
use ggez::input::keyboard::KeyboardContext;
use ggez::input::keyboard::{KeyCode};




pub struct Player{
    pub rect: Rect,
    pub point_given: bool,
    pub score: i32,
    pub player_speed : f32
}

impl Player {
    pub fn new(x : f32, y : f32) -> Player {
        Player {
            rect: Rect::new(x, y, 20.0, 60.0),
            point_given: false,
            score: 0,
            player_speed : 5.0
        }
    }

    pub fn reset_position(&mut self, client_nr :i32, left_x : f32, right_x : f32, y : f32) {
        if client_nr == 1 {
            self.rect.x = left_x;
            self.rect.y = y;
        } else {
            self.rect.x = right_x;
            self.rect.y = y;
        }
    }

    pub fn move_with_keys(&mut self, k_ctx : &KeyboardContext, window_height: f32 ){
        if k_ctx.is_key_pressed(KeyCode::S) {
            if self.rect.y + self.rect.h < window_height{
                self.rect.y += self.player_speed;
            }
        }else if k_ctx.is_key_pressed(KeyCode::W) {
            if self.rect.y + self.rect.h> 0.0 {
                self.rect.y -= self.player_speed;
            }
        }
    }
}

/*
let k_ctx = &_ctx.keyboard;
        if k_ctx.is_key_pressed(KeyCode::S) {
            if self.my_player.rect.y + self.my_player.rect.h < WINDOW_HEIGHT{
                self.my_player.rect.y += PLAYER_SPEED;
            }
        }else if k_ctx.is_key_pressed(KeyCode::W) {
            if self.my_player.rect.y + self.my_player.rect.h> 0.0 {
                self.my_player.rect.y -= PLAYER_SPEED;
            }
        }
*/