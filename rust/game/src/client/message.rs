
pub struct Message{
    pub data: String
}

impl Message {
    pub fn new(x : f32, y : f32) -> Message{
        Message {
            data: x.to_string() + "," + &y.to_string(),
        }
    }

    pub fn get_y_positions(data : &str) -> f32{
        data.split(",").collect::<Vec<&str>>()[1].to_string().parse::<f32>().unwrap()
    }
}