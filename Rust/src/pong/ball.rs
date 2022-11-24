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
            y_velocity: -5.0
        }
    }

    pub fn update(&mut self, window_height : f32, window_width : f32, my_player : &mut Player, enemy_player : &mut Player){
        self.rect.x += self.x_velocity;
        self.rect.y += self.y_velocity;

        /*bounce off bottom and top edge*/
        if self.rect.y + self.rect.h >= window_height {
            self.y_velocity = -self.y_velocity
        }
        else if self.rect.y <= 0.0{
            self.y_velocity = -self.y_velocity
        }
        
        /* bounce off left and right edge 
        ta bort sen,bara så att man har något att stirra på medan man utvecklar :p
        */
        if self.rect.x + self.rect.w >= window_width {
            self.x_velocity = -self.x_velocity
        }
        else if self.rect.x <= 0.0 {
            self.x_velocity = -self.x_velocity
        }

        //align with player rect
        if self.rect.x <= my_player.rect.x + my_player.rect.w{
            //overlaps with player rect
            if self.rect.overlaps(&my_player.rect) {
                self.x_velocity = -self.x_velocity;
                //self.rect.x = my_player.rect.x + (my_player.rect.w-1.0); Ingen aning om vi ska ha dehär
            }
            //missed player rect
            else {
                enemy_player.point_given = true;
            }
        }
        else if self.rect.x + self.rect.w >= window_width - 40.0 {
            if self.rect.overlaps(&enemy_player.rect) {
                self.x_velocity = -self.x_velocity;
                //self.rect.x = enemy_player.rect.x + 1.0; Ingen aning om vi ska ha dehär
            }
            else {
                my_player.point_given = true;
            }
        }
        
    }

    pub fn check_reset(&mut self, x : f32, y : f32){
            self.x_velocity = -3.0;
            self.y_velocity = -5.0;
            self.rect.x = x;
            self.rect.y = y;
    }

}