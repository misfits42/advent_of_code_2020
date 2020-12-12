use super::utils::map::CardinalDirection;
use super::utils::map::Point2D;

use regex::Regex;

#[derive(Copy, Clone, Debug)]
enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl Action {
    fn from_string(input: &str) -> Option<Action> {
        match input {
            "N" => return Some(Action::North),
            "S" => return Some(Action::South),
            "E" => return Some(Action::East),
            "W" => return Some(Action::West),
            "L" => return Some(Action::Left),
            "R" => return Some(Action::Right),
            "F" => return Some(Action::Forward),
            _ => return None,
        }
    }
}

#[aoc_generator(day12)]
fn generate_input(input: &str) -> Vec<(Action, i64)> {
    let mut instructions: Vec<(Action, i64)> = vec![];
    let line_regex = Regex::new(r"^(N|S|E|W|L|R|F)(\d+)$").unwrap();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line_regex.is_match(line) {
            let captures = line_regex.captures(line).unwrap();
            let action = Action::from_string(&captures[1]).unwrap();
            let value = captures[2].parse::<i64>().unwrap();
            instructions.push((action, value));
        }
    }
    return instructions;
}

#[aoc(day12, part1)]
fn solve_part_1(instructions: &Vec<(Action, i64)>) -> u64 {
    // Initialise ship direction and position
    let mut ship_dir = CardinalDirection::East;
    let mut ship_loc = Point2D::new(0, 0);
    for (action, value) in instructions {
        let value = *value;
        match action {
            Action::North => ship_loc.move_point(0, -value),
            Action::South => ship_loc.move_point(0, value),
            Action::East => ship_loc.move_point(value, 0),
            Action::West => ship_loc.move_point(-value, 0),
            Action::Left => ship_dir = ship_dir.rotate_left(value as u64),
            Action::Right => ship_dir = ship_dir.rotate_right(value as u64),
            Action::Forward => match ship_dir {
                CardinalDirection::North => ship_loc.move_point(0, -value),
                CardinalDirection::South => ship_loc.move_point(0, value),
                CardinalDirection::East => ship_loc.move_point(value, 0),
                CardinalDirection::West => ship_loc.move_point(-value, 0),
            },
        }
    }
    return ship_loc.calculate_manhattan_distance(&Point2D::new(0, 0));
}

#[aoc(day12, part2)]
fn solve_part_2(instructions: &Vec<(Action, i64)>) -> u64 {
    // Initialise ship direction, location and waypoint
    let mut ship_loc = Point2D::new(0, 0);
    let mut waypoint = Point2D::new(10, -1); // record waypoint as delta from ship location
    for (action, value) in instructions.iter() {
        let value = *value;
        match action {
            Action::North => waypoint.move_point(0, -value),
            Action::South => waypoint.move_point(0, value),
            Action::East => waypoint.move_point(value, 0),
            Action::West => waypoint.move_point(-value, 0),
            Action::Left => {
                let rotations = value / 90;
                for _ in 0..rotations {
                    let new_x = waypoint.get_y();
                    let new_y = -waypoint.get_x();
                    waypoint = Point2D::new(new_x, new_y);
                }
            },
            Action::Right => {
                let rotations = value / 90;
                for _ in 0..rotations {
                    let new_x = -waypoint.get_y();
                    let new_y = waypoint.get_x();
                    waypoint = Point2D::new(new_x, new_y);
                }
            },
            Action::Forward => {
                for _ in 0..value {
                    ship_loc.move_point(waypoint.get_x(), waypoint.get_y());
                }
            }
        }
    }
    return ship_loc.calculate_manhattan_distance(&Point2D::new(0, 0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d12_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day12.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(2280, result);
    }

    #[test]
    fn test_d12_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day12.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(38693, result);
    }
}
