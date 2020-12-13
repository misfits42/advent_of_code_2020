struct BusTimer {
    bus_id: u64,
    current: u64,
}

impl BusTimer {
    pub fn new(bus_id: u64) -> Self {
        Self {
            bus_id: bus_id,
            current: 0,
        }
    }

    pub fn get_current(&self) -> u64 {
        return self.current;
    }

    pub fn get_bus_id(&self) -> u64 {
        return self.bus_id;
    }
}

impl Iterator for BusTimer {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.current += self.bus_id;
        return Some(self.current);
    }
}

#[aoc_generator(day13)]
fn generate_input(input: &str) -> (u64, Vec<u64>) {
    let lines = input.lines().collect::<Vec<&str>>();
    // Get earliest time for catching bus
    let earliest_timestamp = lines[0].parse::<u64>().unwrap();
    let mut split = lines[1].split(",").collect::<Vec<&str>>();
    split.retain(|x| *x != "x");
    let bus_ids = split.iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    return (earliest_timestamp, bus_ids);
}

#[aoc(day13, part1)]
fn solve_part_1(input: &(u64, Vec<u64>)) -> u64 {
    let earliest_timestamp = input.0;
    // Generate bus timers
    let mut bus_timers: Vec<BusTimer> = vec![];
    for bus_id in input.1.iter() {
        let bus_timer = BusTimer::new(*bus_id);
        bus_timers.push(bus_timer);
    }
    // Process each bus timer until current arrival time is greater than or equal to early TS
    let mut timestamp_result = u64::MAX;
    let mut bus_id_result = 0;
    for bus_timer in bus_timers.iter_mut() {
        while bus_timer.next().unwrap() < earliest_timestamp {}
        if bus_timer.get_current() < timestamp_result {
            timestamp_result = bus_timer.get_current();
            bus_id_result = bus_timer.get_bus_id();
        }
    }
    return (timestamp_result - earliest_timestamp) * bus_id_result;
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
}
