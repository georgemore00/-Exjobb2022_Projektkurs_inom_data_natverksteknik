use ggez::{graphics::{Rect}};


pub struct Player{
    pub rect: Rect,
    pub point_given: bool,
    pub score: i32
}

impl Player {
    pub fn new(x : f32, y : f32) -> Player {
        Player {
            rect: Rect::new(x, y, 20.0, 60.0),
            score: 0,
            point_given: false
        }
    }

    pub fn reset_position(&mut self, x : f32, y : f32){
        self.rect.x = x;
        self.rect.y = y;
    }

}