/// Represents one of the four cardinal directions.
#[derive(Copy, Clone)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West
}

impl CardinalDirection {
    /// Determines the resulting direction by conducting single 90 rotation to left (CCW).
    pub fn rotate_left_90_degrees(&self) -> CardinalDirection {
        match self {
            CardinalDirection::North => return CardinalDirection::West,
            CardinalDirection::East => return CardinalDirection::North,
            CardinalDirection::South => return CardinalDirection::East,
            CardinalDirection::West => return CardinalDirection::South,
        }
    }

    /// Determines the resulting direction by conducting single 90 rotation to right (CW).
    pub fn rotate_right_90_degrees(&self) -> CardinalDirection {
        match self {
            CardinalDirection::North => return CardinalDirection::East,
            CardinalDirection::East => return CardinalDirection::South,
            CardinalDirection::South => return CardinalDirection::West,
            CardinalDirection::West => return CardinalDirection::North,
        }
    }

    /// Determines the resulting direction by conducting N rotations to the right (CW), where N is
    /// number of 90 degrees in specified value.
    pub fn rotate_right(&self, degrees: u64) -> CardinalDirection {
        let rotations = degrees / 90;
        let mut direction = *self;
        for _ in 0..rotations {
            direction = direction.rotate_right_90_degrees();
        }
        return direction;
    }

    /// Determines the resulting direction by conducting N rotations to the left (CCW), where N is
    /// number of 90 degrees in specified value.
    pub fn rotate_left(&self, degrees: u64) -> CardinalDirection {
        let rotations = degrees / 90;
        let mut direction = *self;
        for _ in 0..rotations {
            direction = direction.rotate_left_90_degrees();
        }
        return direction;
    }
}
