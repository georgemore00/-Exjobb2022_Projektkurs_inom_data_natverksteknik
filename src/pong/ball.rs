use ggez::graphics::{Rect};
use super::Player;

pub struct Ball{
    pub rect: Rect,
    pub x_velocity: f32,
    pub y_velocity: f32,
}

impl Ball {
    pub fn new(x : f32, y : f32) -> Ball {
        Ball {
            rect: Rect::new(x, y, 20.0, 20.0),
            x_velocity: 0.0,
            y_velocity: 0.0,
        }
    }

    pub fn update(&mut self, window_height : f32, window_width : f32, left_player : &mut Player, right_player : &mut Player){
        //move ball
        self.rect.x += self.x_velocity;
        self.rect.y += self.y_velocity;

        /*bounce off bottom and top edge*/
        if self.rect.y + self.rect.h >= window_height {
            self.y_velocity = -self.y_velocity
        }
        else if self.rect.y <= 0.0{
            self.y_velocity = -self.y_velocity
        }

        //Bounce off players
        if self.rect.x <= left_player.rect.x + left_player.rect.w{
            //overlaps with player rect
            if self.rect.overlaps(&left_player.rect) {
                self.x_velocity = -self.x_velocity;
            }
            //missed player rect
            else {
                right_player.point_given = true;
            }
        }
        else if self.rect.x + self.rect.w >= window_width - 40.0 {
            if self.rect.overlaps(&right_player.rect) {
                self.x_velocity = -self.x_velocity;
            }
            else {
                left_player.point_given = true;
            }
        }
    }
    
    pub fn reset(&mut self, x : f32, y : f32){
        self.x_velocity = -3.0;
        self.y_velocity = -5.0;
        self.rect.x = x;
        self.rect.y = y;
}

}

