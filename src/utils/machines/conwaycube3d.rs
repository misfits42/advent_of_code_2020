use std::collections::HashMap;

use super::super::map::Point3D;

/// Represents the state of a single location (cube) within the 3D Conway Cube as being in either
/// the active or inactive state.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum CubeState {
    Active,
    Inactive,
}

/// Represents the three-dimensional Conway Cube introduced in AOC 2020 Day 17 Part 1.
#[derive(Clone)]
pub struct ConwayCube3D {
    grid_state: HashMap<Point3D, CubeState>,
    steps: u64,
}

impl ConwayCube3D {
    /// Creates a new 3D Conway Cube from the raw input. The 2D plane specified by the raw input
    /// is taken to occupy the X-Y plane (z = 0), with the top-left-most location in the raw input
    /// located at {x: 0, y: 0}
    pub fn new(raw_input: &str) -> Self {
        let mut grid_state: HashMap<Point3D, CubeState> = HashMap::new();
        let mut x;
        let mut y = 0;
        for line in raw_input.lines() {
            let line = line.trim();
            x = 0;
            for c in line.chars() {
                match c {
                    '.' => {
                        grid_state.insert(Point3D::new(x, y, 0), CubeState::Inactive);
                    }
                    '#' => {
                        grid_state.insert(Point3D::new(x, y, 0), CubeState::Active);
                    }
                    _ => panic!("ConwayCube3D - bad character in raw input!"),
                }
                x += 1;
            }
            y += 1;
        }
        ConwayCube3D {
            grid_state: grid_state,
            steps: 0,
        }
    }

    /// Counts the number of cubes in the active state.
    pub fn count_active_cubes(&self) -> u64 {
        let mut count = 0;
        for (_loc, state) in self.grid_state.iter() {
            if *state == CubeState::Active {
                count += 1;
            }
        }
        return count;
    }

    /// Conducts a single step for the Conway cube, following the rules set out in AOC 2020 Day 17.
    pub fn conduct_step(&mut self) {
        self.steps += 1;
        // insert inactive cubes around all points if not alreay in the conway cube
        let mut new_inactive_points: Vec<Point3D> = vec![];
        for (loc, _state) in self.grid_state.iter() {
            for neighbour in loc.get_surrounding_points() {
                if !self.grid_state.contains_key(&neighbour) {
                    new_inactive_points.push(neighbour);
                }
            }
        }
        for loc in new_inactive_points {
            self.grid_state.insert(loc, CubeState::Inactive);
        }
        // Conduct the next step
        let mut to_active: Vec<Point3D> = vec![];
        let mut to_inactive: Vec<Point3D> = vec![];
        for (loc, state) in self.grid_state.iter() {
            match state {
                CubeState::Active => {
                    let count = self.count_neighbour_state(*loc, CubeState::Active);
                    if count != 2 && count != 3 {
                        to_inactive.push(*loc);
                    }
                },
                CubeState::Inactive => {
                    let count = self.count_neighbour_state(*loc, CubeState::Active);
                    if count == 3 {
                        to_active.push(*loc);
                    }
                }
            }
        }
        for loc in to_active {
            self.grid_state.insert(loc, CubeState::Active);
        }
        for loc in to_inactive {
            self.grid_state.insert(loc, CubeState::Inactive);
        }
    }

    /// Counts the number of neighbours to the given location that are in the specified state.
    fn count_neighbour_state(&self, loc: Point3D, state: CubeState) -> u64 {
        let mut count = 0;
        let neighbours = loc.get_surrounding_points();
        for neighbour in neighbours.iter() {
            match state {
                CubeState::Active => {
                    if self.grid_state.contains_key(neighbour)
                        && *self.grid_state.get(&neighbour).unwrap() == CubeState::Active
                    {
                        count += 1;
                    }
                }
                CubeState::Inactive => {
                    if self.grid_state.contains_key(neighbour)
                        && *self.grid_state.get(&neighbour).unwrap() == CubeState::Inactive
                        || !self.grid_state.contains_key(&neighbour)
                    {
                        count += 1;
                    }
                }
            }
        }
        return count;
    }
}
