/// Represents a single point in two-dimensional Euclidean space.
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct Point2D {
    x: i64,
    y: i64
}

impl Point2D {
    /// Creates a new 2D point.
    pub fn new(x: i64, y: i64) -> Self {
        Self {
            x: x,
            y: y
        }
    }

    /// Gets the value of the x-coordinate.
    pub fn get_x(&self) -> i64 {
        return self.x;
    }

    /// Updates the value of the x-coordinate.
    pub fn set_x(&mut self, x: i64) {
        self.x = x;
    }

    /// Gets the value of the y-coordinate.
    pub fn get_y(&self) -> i64 {
        return self.y;
    }

    /// Updates the value of the y-coordinate.
    pub fn set_y(&mut self, y: i64) {
        self.y = y;
    }

    /// Moves the point by the specified amount in the x- and y-directions.
    pub fn move_point(&mut self, delta_x: i64, delta_y: i64) {
        self.x += delta_x;
        self.y += delta_y;
    }
}
