#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct Point3D {
    x: i64,
    y: i64,
    z: i64
}

impl Point3D {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self {
            x: x,
            y: y,
            z: z
        }
    }

    /// Gets all of the points surrounding the current point in three dimensions.
    pub fn get_surrounding_points(&self) -> Vec<Point3D> {
        let mut output: Vec<Point3D> = vec![];
        for d_x in -1..=1 {
            for d_y in -1..=1 {
                for d_z in -1..=1 {
                    if d_x == 0 && d_y == 0 && d_z == 0 {
                        continue;
                    }
                    output.push(self.move_point(d_x, d_y, d_z));
                }
            }
        }
        return output;
    }

    /// Returns the result of moving the current point by the specified amounts in each dimension.
    pub fn move_point(&self, delta_x: i64, delta_y: i64, delta_z: i64) -> Point3D {
        return Point3D::new(self.x + delta_x, self.y + delta_y, self.z + delta_z);
    }
}