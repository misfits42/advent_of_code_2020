use std::collections::HashMap;

struct BusTimer {
    bus_id: u64,
    offset: u64,
    current: u64,
}

impl BusTimer {
    pub fn new(bus_id: u64, offset: u64) -> Self {
        Self {
            bus_id: bus_id,
            offset: offset,
            current: 0,
        }
    }

    pub fn get_offset(&self) -> u64 {
        return self.offset;
    }

    pub fn get_bus_id(&self) -> u64 {
        return self.bus_id;
    }

    pub fn calculate_next_arrival_after_timestamp(&mut self, timestamp: u64) -> u64 {
        let diff = {
            if timestamp < self.offset {
                0
            } else {
                timestamp - self.offset
            }
        };
        let rounds = diff / self.bus_id;
        let next_timestamp = self.bus_id * (rounds + 1); // + self.offset;
        self.current = next_timestamp;
        return self.current;
    }
}

#[aoc_generator(day13)]
fn generate_input(input: &str) -> (u64, Vec<(u64, u64)>) {
    let lines = input.lines().collect::<Vec<&str>>();
    // Get earliest time for catching bus
    let earliest_timestamp = lines[0].parse::<u64>().unwrap();
    let mut i = 0;
    let mut bus_slots: Vec<(u64, u64)> = vec![];
    for id in lines[1].split(",") {
        if id != "x" {
            bus_slots.push((i, id.parse::<u64>().unwrap()));
        }
        i += 1;
    }
    return (earliest_timestamp, bus_slots);
}

#[aoc(day13, part1)]
fn solve_part_1(input: &(u64, Vec<(u64, u64)>)) -> u64 {
    let earliest_timestamp = input.0;
    // Generate bus timers
    let mut bus_timers: Vec<BusTimer> = vec![];
    for (_time, bus_id) in input.1.iter() {
        let bus_timer = BusTimer::new(*bus_id, 0);
        bus_timers.push(bus_timer);
    }
    // Process each bus timer until current arrival time is greater than or equal to early TS
    let mut timestamp_result = u64::MAX;
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
fn solve_part_2(input: &(u64, Vec<(u64, u64)>)) -> u64 {
    // Extract the bus IDs
    let bus_ids = input.1.iter().map(|x| x.1).collect::<Vec<u64>>();
    // Generate bus timers
    let mut bus_timers: HashMap<u64, BusTimer> = HashMap::new();
    for (offset, bus_id) in input.1.iter() {
        let bus_timer = BusTimer::new(*bus_id, *offset);
        bus_timers.insert(*bus_id, bus_timer);
    }
    // Determine initial start and end bounds for bus arrival period
    let mut period_start = 100000000000000;
    let mut period_end = period_start + bus_ids[0];
    let mut periods_checked = 0;
    loop {
        periods_checked += 1;
        if periods_checked % 10000000 == 0 {
             println!("Period start: {}", period_start);
        }
        let mut last_time = period_start;
        let mut success = true;
        for i in 1..bus_ids.len() {
            let bus_id = bus_ids[i];
            // Generate next arrival time
            last_time = bus_timers.get_mut(&bus_id).unwrap().calculate_next_arrival_after_timestamp(last_time);
            // Calculate offset from period start and check if it matches the original bus offset
            let offset = {
                if period_start == 0 {
                    last_time
                } else {
                    last_time % period_start
                }
            };
            // Check if we have run over the end of the current period
            if last_time > period_end {
                success = false;
                break;
            }
            if offset != bus_timers.get(&bus_id).unwrap().get_offset() {
                success = false;
                break;
            }
        }
        // We have found the success condition
        if success {
            return period_start;
        }
        // Current iteration not successful - go to next
        period_start = {
            if last_time > period_end {
                (last_time / bus_ids[0] + 1) * bus_ids[0]
            } else {
                period_end
            }
        };
        period_end = period_start + bus_ids[0];
    }
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
