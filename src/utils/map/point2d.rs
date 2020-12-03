#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct Point2D {
    x: i64,
    y: i64
}

impl Point2D {
    pub fn new(x: i64, y: i64) -> Self {
        Self {
            x: x,
            y: y
        }
    }

    pub fn get_x(&self) -> i64 {
        return self.x;
    }

    pub fn set_x(&mut self, x: i64) {
        self.x = x;
    }

    pub fn get_y(&self) -> i64 {
        return self.y;
    }

    pub fn move_point(&mut self, delta_x: i64, delta_y: i64) {
        self.x += delta_x;
        self.y += delta_y;
    }
}
