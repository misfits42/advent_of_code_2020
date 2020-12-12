use std::collections::HashMap;

use super::utils::map::Point2D;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum TileState {
    // Floor,
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
                    ()
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
fn solve_part_1(seat_locs: &HashMap<Point2D, TileState>) -> u64 {
    let mut seat_locs = seat_locs.clone();
    let mut rounds_conducted = 0;
    loop {
        let mut to_empty: Vec<Point2D> = vec![];
        let mut to_occupied: Vec<Point2D> = vec![];
        for (location, state) in seat_locs.iter() {
            match state {
                TileState::SeatEmpty => {
                    // Check each surrounding seat to see if none are occupied
                    let mut toggle = true;
                    for check_loc in location.get_surrounding_points() {
                        if seat_locs.contains_key(&check_loc) &&
                                *seat_locs.get(&check_loc).unwrap() == TileState::SeatOccupied
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
                        if seat_locs.contains_key(&check_loc) &&
                                *seat_locs.get(&check_loc).unwrap() == TileState::SeatOccupied
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
                // TileState::Floor => (),
            }
        }
        rounds_conducted += 1;
        // Check if state stayed stable
        if to_empty.is_empty() && to_occupied.is_empty() {
            let mut count = 0;
            for (_loc, state) in seat_locs.iter() {
                if *state == TileState::SeatOccupied {
                    count += 1;
                }
            }
            return count;
        }
        // Toggle seats as needed
        for loc in to_empty {
            seat_locs.insert(loc, TileState::SeatEmpty);
        }
        for loc in to_occupied {
            seat_locs.insert(loc, TileState::SeatOccupied);
        }

    }
}