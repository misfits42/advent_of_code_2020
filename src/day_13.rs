use super::utils::math::ntheory::*;

struct BusTimer {
    bus_id: i64,
    offset: i64,
}

impl BusTimer {
    pub fn new(bus_id: i64, offset: i64) -> Self {
        Self {
            bus_id: bus_id,
            offset: offset,
        }
    }

    pub fn get_bus_id(&self) -> i64 {
        return self.bus_id;
    }

    pub fn calculate_next_arrival_after_timestamp(&mut self, timestamp: i64) -> i64 {
        let diff = {
            if timestamp < self.offset {
                0
            } else {
                timestamp - self.offset
            }
        };
        let rounds = diff / self.bus_id;
        let next_timestamp = self.bus_id * (rounds + 1);
        return next_timestamp;
    }
}

#[aoc_generator(day13)]
fn generate_input(input: &str) -> (i64, Vec<(i64, i64)>) {
    let lines = input.lines().collect::<Vec<&str>>();
    // Get earliest time for catching bus
    let earliest_timestamp = lines[0].parse::<i64>().unwrap();
    let mut i = 0;
    let mut bus_slots: Vec<(i64, i64)> = vec![];
    for id in lines[1].split(",") {
        if id != "x" {
            bus_slots.push((i, id.parse::<i64>().unwrap()));
        }
        i += 1;
    }
    return (earliest_timestamp, bus_slots);
}

#[aoc(day13, part1)]
fn solve_part_1(input: &(i64, Vec<(i64, i64)>)) -> i64 {
    let earliest_timestamp = input.0;
    // Generate bus timers
    let mut bus_timers: Vec<BusTimer> = vec![];
    for (_time, bus_id) in input.1.iter() {
        let bus_timer = BusTimer::new(*bus_id, 0);
        bus_timers.push(bus_timer);
    }
    // Process each bus timer until current arrival time is greater than or equal to early TS
    let mut timestamp_result = i64::MAX;
    let mut bus_id_result = 0;
    for bus_timer in bus_timers.iter_mut() {
        let next_time = bus_timer.calculate_next_arrival_after_timestamp(earliest_timestamp);
        if next_time < timestamp_result {
            timestamp_result = next_time;
            bus_id_result = bus_timer.get_bus_id();
        }
    }
    return (timestamp_result - earliest_timestamp) * bus_id_result;
}

#[aoc(day13, part2)]
fn solve_part_2(input: &(i64, Vec<(i64, i64)>)) -> i64 {
    // Extract the bus IDs
    let bus_ids = input.1.iter().map(|x| x.1).collect::<Vec<i64>>();
    // Determine offsets as applicable to the earliest timestamp being searched for
    let offsets = input.1.iter().map(|x| (x.1 - x.0) % x.1).collect::<Vec<i64>>();
    let result = solve_crt(&offsets, &bus_ids);
    if result.is_some() {
        return result.unwrap();
    }
    panic!("Day 13 Part 2 - did not find valid solution!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d13_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day13.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(1835, result);
    }

    #[test]
    fn test_d13_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day13.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(247086664214628, result);
    }

    #[test]
    fn test_d13_p1_001() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day13_test_001.txt").unwrap(),
        );
        let result = solve_part_1(&input);
        assert_eq!(295, result);
    }

    #[test]
    fn test_d13_p2_001() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day13_test_001.txt").unwrap(),
        );
        let result = solve_part_2(&input);
        assert_eq!(1068781, result);
    }

    #[test]
    fn test_d13_p2_002() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day13_test_002.txt").unwrap(),
        );
        let result = solve_part_2(&input);
        assert_eq!(3417, result);
    }

    #[test]
    fn test_d13_p2_003() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day13_test_003.txt").unwrap(),
        );
        let result = solve_part_2(&input);
        assert_eq!(754018, result);
    }

    #[test]
    fn test_d13_p2_004() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day13_test_004.txt").unwrap(),
        );
        let result = solve_part_2(&input);
        assert_eq!(779210, result);
    }

    #[test]
    fn test_d13_p2_005() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day13_test_005.txt").unwrap(),
        );
        let result = solve_part_2(&input);
        assert_eq!(1261476, result);
    }

    #[test]
    fn test_d13_p2_006() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day13_test_006.txt").unwrap(),
        );
        let result = solve_part_2(&input);
        assert_eq!(1202161486, result);
    }
}
