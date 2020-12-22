#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct Point4D {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl Point4D {
    pub fn new(x: i64, y: i64, z: i64, w: i64) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    pub fn get_surrounding_points(&self) -> Vec<Point4D> {
        let mut output: Vec<Point4D> = vec![];
        for d_x in -1..=1 {
            for d_y in -1..=1 {
                for d_z in -1..=1 {
                    for d_w in -1..=1 {
                        if d_x == 0 && d_y == 0 && d_z == 0 && d_w == 0 {
                            continue;
                        }
                        output.push(self.move_point(d_x, d_y, d_z, d_w));
                    }
                }
            }
        }
        return output;
    }

    pub fn move_point(&self, delta_x: i64, delta_y: i64, delta_z: i64, delta_w: i64) -> Point4D {
        return Point4D::new(self.x + delta_x, self.y + delta_y, self.z + delta_z, self.w + delta_w);
    }
}
