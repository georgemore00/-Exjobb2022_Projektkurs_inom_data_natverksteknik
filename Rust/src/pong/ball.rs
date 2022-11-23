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
            x_velocity: -3.0,
            y_velocity: -5.0,
        }
    }

    pub fn update(&mut self, window_height : f32, left_player : &Player){
        self.rect.x += self.x_velocity;
        self.rect.y += self.y_velocity;

        /*bounce off bottom and top edge*/
        if self.rect.y >= window_height - self.rect.h{
            self.y_velocity = -self.y_velocity
        }
        else if  self.rect.y <= self.rect.h{
            self.y_velocity = -self.y_velocity
        }
        
        /* bounce off left and right edge 
        ta bort sen,bara så att man har något att stirra på medan man utvecklar :p
        */
        if self.rect.x >= 800.0 - self.rect.w{
            self.x_velocity = -self.x_velocity
        }
        else if self.rect.x <= self.rect.w {
            self.x_velocity = -self.x_velocity
        }
    }   
}

