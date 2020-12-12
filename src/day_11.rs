use std::collections::HashMap;

use super::utils::map::Point2D;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum TileState {
    Floor,
    SeatEmpty,
    SeatOccupied
}

#[aoc_generator(day11)]
fn generate_input(input: &str) -> HashMap<Point2D, TileState> {
    let mut seat_locs: HashMap<Point2D, TileState> = HashMap::new();
    let mut y = 0;
    for line in input.lines() {
        let mut x = 0;
        // Remove leading and trailing whitespace, then ignore empty lines
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // Parse tiles in line
        for tile in line.chars() {
            let location = Point2D::new(x, y);
            match tile {
                '.' => {
                    seat_locs.insert(location, TileState::Floor);
                },
                'L' => {
                    seat_locs.insert(location, TileState::SeatEmpty);
                },
                _ => panic!("Day 11 - bad character in input file ---- {}", tile),
            }
            x += 1;
        }
        y += 1;
    }
    return seat_locs;
}

#[aoc(day11, part1)]
fn solve_part_1(seating_map: &HashMap<Point2D, TileState>) -> u64 {
    let mut seating_map = seating_map.clone();
    loop {
        let mut to_empty: Vec<Point2D> = vec![];
        let mut to_occupied: Vec<Point2D> = vec![];
        for (location, state) in seating_map.iter() {
            match state {
                TileState::SeatEmpty => {
                    // Check each surrounding seat to see if none are occupied
                    let mut toggle = true;
                    for check_loc in location.get_surrounding_points() {
                        if seating_map.contains_key(&check_loc) &&
                                *seating_map.get(&check_loc).unwrap() == TileState::SeatOccupied
                        {
                            toggle = false;
                            break;
                        }
                    }
                    if toggle {
                        to_occupied.push(*location);
                    }
                },
                TileState::SeatOccupied => {
                    let mut toggle = false;
                    let mut count = 0;
                    for check_loc in location.get_surrounding_points() {
                        if seating_map.contains_key(&check_loc) &&
                                *seating_map.get(&check_loc).unwrap() == TileState::SeatOccupied
                        {
                            count += 1;
                        }
                        if count >= 4 {
                            toggle = true;
                            break;
                        }
                    }
                    if toggle {
                        to_empty.push(*location);
                    }
                },
                TileState::Floor => (),
            }
        }
        // Check if state stayed stable
        if to_empty.is_empty() && to_occupied.is_empty() {
            let mut count = 0;
            for (_loc, state) in seating_map.iter() {
                if *state == TileState::SeatOccupied {
                    count += 1;
                }
            }
            return count;
        }
        // Toggle seats as needed
        for loc in to_empty {
            seating_map.insert(loc, TileState::SeatEmpty);
        }
        for loc in to_occupied {
            seating_map.insert(loc, TileState::SeatOccupied);
        }
    }
}

#[aoc(day11, part2)]
fn solve_part_2(seating_map: &HashMap<Point2D, TileState>) -> u64 {
    let mut seating_map = seating_map.clone();
    let max_x = seating_map.keys().map(|loc| loc.get_x()).max().unwrap();
    let max_y = seating_map.keys().map(|loc| loc.get_y()).max().unwrap();
    loop {
        let mut to_empty: Vec<Point2D> = vec![];
        let mut to_occupied: Vec<Point2D> = vec![];
        for (location, state) in seating_map.iter() {
            match state {
                TileState::SeatEmpty => {
                    // Check each surrounding seat to see if none are occupied
                    let mut toggle = true;
                    for check_loc in get_first_visible_points(&seating_map, location, max_x, max_y) {
                        if seating_map.contains_key(&check_loc) &&
                                *seating_map.get(&check_loc).unwrap() == TileState::SeatOccupied
                        {
                            toggle = false;
                            break;
                        }
                    }
                    if toggle {
                        to_occupied.push(*location);
                    }
                },
                TileState::SeatOccupied => {
                    let mut toggle = false;
                    let mut count = 0;
                    for check_loc in get_first_visible_points(&seating_map, location, max_x, max_y) {
                        if seating_map.contains_key(&check_loc) &&
                                *seating_map.get(&check_loc).unwrap() == TileState::SeatOccupied
                        {
                            count += 1;
                        }
                        if count >= 5 {
                            toggle = true;
                            break;
                        }
                    }
                    if toggle {
                        to_empty.push(*location);
                    }
                },
                TileState::Floor => (),
            }
        }
        // Check if state stayed stable
        if to_empty.is_empty() && to_occupied.is_empty() {
            let mut count = 0;
            for (_loc, state) in seating_map.iter() {
                if *state == TileState::SeatOccupied {
                    count += 1;
                }
            }
            return count;
        }
        // Toggle seats as needed
        for loc in to_empty {
            seating_map.insert(loc, TileState::SeatEmpty);
        }
        for loc in to_occupied {
            seating_map.insert(loc, TileState::SeatOccupied);
        }
    }
}

fn get_first_visible_points(seating_map: &HashMap<Point2D, TileState>, location: &Point2D,
        max_x: i64, max_y: i64) -> Vec<Point2D>
{
    let mut output: Vec<Point2D> = vec![];
    // Up
    let mut check_point = *location;
    loop {
        if check_point.get_y() < 0 {
            break;
        }
        check_point.move_point(0, -1);
        if seating_map.contains_key(&check_point) &&
                *seating_map.get(&check_point).unwrap() != TileState::Floor
        {
            output.push(check_point);
            break;
        }
    }
    // Diag - up right
    let mut check_point = *location;
    loop {
        if check_point.get_x() > max_x || check_point.get_y() < 0 {
            break;
        }
        check_point.move_point(1, -1);
        if seating_map.contains_key(&check_point) &&
                *seating_map.get(&check_point).unwrap() != TileState::Floor
        {
            output.push(check_point);
            break;
        }
    }
    // Right
    let mut check_point = *location;
    loop {
        if check_point.get_x() > max_x {
            break;
        }
        check_point.move_point(1, 0);
        if seating_map.contains_key(&check_point) &&
                *seating_map.get(&check_point).unwrap() != TileState::Floor
        {
            output.push(check_point);
            break;
        }
    }
    // Diag - down right
    let mut check_point = *location;
    loop {
        if check_point.get_x() > max_x || check_point.get_y() > max_y {
            break;
        }
        check_point.move_point(1, 1);
        if seating_map.contains_key(&check_point) &&
                *seating_map.get(&check_point).unwrap() != TileState::Floor
        {
            output.push(check_point);
            break;
        }
    }
    // Down
    let mut check_point = *location;
    loop {
        if check_point.get_y() > max_y {
            break;
        }
        check_point.move_point(0, 1);
        if seating_map.contains_key(&check_point) &&
                *seating_map.get(&check_point).unwrap() != TileState::Floor
        {
            output.push(check_point);
            break;
        }
    }
    // Diag - down left
    let mut check_point = *location;
    loop {
        if check_point.get_x() < 0 || check_point.get_y() > max_y  {
            break;
        }
        check_point.move_point(-1, 1);
        if seating_map.contains_key(&check_point) &&
                *seating_map.get(&check_point).unwrap() != TileState::Floor
        {
            output.push(check_point);
            break;
        }
    }
    // Left
    let mut check_point = *location;
    loop {
        if check_point.get_x() < 0 {
            break;
        }
        check_point.move_point(-1, 0);
        if seating_map.contains_key(&check_point) &&
                *seating_map.get(&check_point).unwrap() != TileState::Floor
        {
            output.push(check_point);
            break;
        }
    }
    // Diag - up left
    let mut check_point = *location;
    loop {
        if check_point.get_y() < 0 || check_point.get_x() < 0 {
            break;
        }
        check_point.move_point(-1, -1);
        if seating_map.contains_key(&check_point) &&
                *seating_map.get(&check_point).unwrap() != TileState::Floor
        {
            output.push(check_point);
            break;
        }
    }
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d11_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day11.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(2476, result);
    }

    #[test]
    fn test_d11_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day11.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(2257, result);
    }
}
