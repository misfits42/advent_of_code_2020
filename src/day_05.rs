#[aoc_generator(day5)]
fn generate_input(input: &str) -> Vec<String> {
    let mut seat_specs: Vec<String> = vec![];
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.len() != 10 {
            panic!("Day 5 - input line length incorrect!");
        }
        seat_specs.push(line.to_string());
    }
    return seat_specs;
}

#[aoc(day5, part1)]
fn solve_part_1(seat_specs: &Vec<String>) -> u64 {
    let mut highest_seat_id = 0;
    for seat_spec in seat_specs {
        let seat_id = calculate_seat_id(seat_spec);
        if seat_id > highest_seat_id {
            highest_seat_id = seat_id;
        }
    }
    return highest_seat_id;
}

#[aoc(day5, part2)]
fn solve_part_2(seat_specs: &Vec<String>) -> u64 {
    unimplemented!();
}

fn calculate_seat_id(seat_spec: &String) -> u64 {
    let chars = seat_spec.chars().collect::<Vec<char>>();
    // Start by finding row number
    let mut r_lower = 0;
    let mut r_upper = 127;
    for i in 0..7 {
        if chars[i] == 'F' {
            r_upper = r_lower + (r_upper - r_lower) / 2;
        } else if chars[i] == 'B' {
            r_lower = r_upper - (r_upper - r_lower) / 2;
        }
    }
    // Now look for column number
    let mut c_lower = 0;
    let mut c_upper = 7;
    for i in 7..10 {
        if chars[i] == 'L' {
            c_upper = c_lower + (c_upper - c_lower) / 2;
        } else if chars[i] == 'R' {
            c_lower = c_upper - (c_upper - c_lower) / 2;
        }
    }
    println!("seat_spec: {} ---- row: {} ---- column: {} ---- seat_id: {}", seat_spec, r_lower, c_lower, r_lower * 8 + c_lower);
    // Now we have found the row and column number
    return r_lower * 8 + c_lower;
}
