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
        // Back at start of new line
        x = 0;
        // Remove leading and trailing whitespace, then ignore empty lines
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // Record element at current location on map
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
    return ride_through_forest(&forest_map, height, width, 3, 1);
}

#[aoc(day3, part2)]
fn solve_part_2(input: &(HashMap<Point2D, MapTile>, i64, i64)) -> u64 {
    let forest_map = input.0.clone();
    let width = input.1;
    let height = input.2;
    // Initialise the slopes we need to check
    let slopes: Vec<(i64, i64)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    // Determine number of trees encountered on each slope
    let mut result_record: Vec<u64> = vec![];
    for (delta_x, delta_y) in slopes {
        let trees_encountered = ride_through_forest(&forest_map, height, width, delta_x, delta_y);
        result_record.push(trees_encountered);
    }
    // Determine the product of all results from forest rides for given slopes
    let mut output = 1;
    for result in result_record {
        output *= result;
    }
    return output;
}

/// Conducts a toboggan ride through the given forest map (specified width and height) with slope
/// defined by given x and y deltas per step.
fn ride_through_forest(forest_map: &HashMap<Point2D, MapTile>, height: i64, width: i64, delta_x: i64,
        delta_y: i64) -> u64
{
    // Start the toboggan at top left map tile - (0, 0)
    let mut trees_encountered = 0;
    let mut current_loc = Point2D::new(0, 0);
    loop {
        // Check if bottom of map has been reached or exceeded
        if current_loc.get_y() >= height {
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
        current_loc.move_point(delta_x, delta_y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d03_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day3.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(230, result);
    }

    #[test]
    fn test_d03_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day3.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(9533698720, result);
    }
}
