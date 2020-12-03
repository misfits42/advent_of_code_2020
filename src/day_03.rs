use std::collections::HashMap;

use super::utils::map::Point2D;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum MapTile {
    Open,
    Tree
}

#[aoc_generator(day3)]
fn generate_input(input: &str) -> (HashMap<Point2D, MapTile>, i64, i64) {
    let mut x = 0;
    let mut y = 0;
    let mut forest_map: HashMap<Point2D, MapTile> = HashMap::new();
    for line in input.lines() {
        x = 0;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        for c in line.chars() {
            let loc = Point2D::new(x, y);
            match c {
                '.' => {
                    forest_map.insert(loc, MapTile::Open);
                },
                '#' => {
                    forest_map.insert(loc, MapTile::Tree);
                },
                _ => {
                    panic!("Day 3 generator - should not get here!");
                }
            }
            x += 1;
        }
        y += 1;
    }
    return (forest_map, x, y);
}

#[aoc(day3, part1)]
fn solve_part_1(input: &(HashMap<Point2D, MapTile>, i64, i64)) -> u64 {
    let width = input.1;
    let height = input.2;
    let forest_map = input.0.clone();
    let mut trees_encountered = 0;
    let mut current_loc = Point2D::new(0, 0);
    loop {
        // Check if bottom of map has been reached
        if current_loc.get_y() == height {
            return trees_encountered;
        }
        // Update x-coord of current location to wrap around
        let wrap_x = current_loc.get_x() % width;
        current_loc.set_x(wrap_x);
        // Check current location for tree
        if *forest_map.get(&current_loc).unwrap() == MapTile::Tree {
            trees_encountered += 1;
        }
        // Move location down slope
        current_loc.move_point(3, 1);
    }
}
