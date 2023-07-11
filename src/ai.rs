#[derive(Debug, Clone)]
pub struct AI {
    move_time: f32, //ms
}

impl AI {
    pub fn new(move_time: f32) -> Self {
        AI { move_time }
    }
    pub fn best_move(&mut self) {}
}
