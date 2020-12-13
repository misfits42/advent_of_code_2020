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

    // pub fn get_current(&self) -> u64 {
    //     return self.current;
    // }

    pub fn get_bus_id(&self) -> u64 {
        return self.bus_id;
    }

    pub fn calculate_next_arrival_after_timestamp(&mut self, timestamp: u64) -> u64 {
        let rounds = (timestamp - self.offset) / self.bus_id;
        let next_timestamp = self.bus_id * (rounds + 1) + self.offset;
        self.current = next_timestamp;
        return self.current;
    }
}

// impl Iterator for BusTimer {
//     type Item = u64;

//     fn next(&mut self) -> Option<u64> {
//         self.current += self.bus_id;
//         return Some(self.current);
//     }
// }

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
    // Extract the bus IDs and sort
    let mut bus_ids = input.1.iter().map(|x| x.1).collect::<Vec<u64>>();
    bus_ids.sort();
    // Generate bus timers
    println!("Bus slots: {:?}", input.1);
    let mut bus_timers: HashMap<u64, BusTimer> = HashMap::new();
    for (offset, bus_id) in input.1.iter() {
        let bus_timer = BusTimer::new(*bus_id, *offset);
        bus_timers.insert(*bus_id, bus_timer);
    }
    // Determine initial start and end bounds for bus arrival period
    let mut period_start = 0;
    let mut period_end = bus_timers.get(&bus_ids[0]).unwrap().get_offset();
    loop {
        // Update period start and end
        period_start = period_end; //bus_timers.get_mut(&bus_ids[0]).unwrap().calculate_next_arrival_after_timestamp(period_end - 1);
        period_end = period_start + bus_ids[0];
        let mut last_time = period_start;
        let mut success = true;
        // println!("[+] mod: {}", period_start % 7);
        //if period_start > 106800 && period_start < 106900 {
            println!("Start: {} ---- End: {}", period_start, period_end);
        //}
        for i in 1..bus_ids.len() {
            let bus_id = bus_ids[i];
            // Generate next arrival time
            let next_time = bus_timers.get_mut(&bus_id).unwrap().calculate_next_arrival_after_timestamp(last_time);
            println!(">>> bus_id: {} ---- next_time: {}", bus_id, next_time);
            if next_time > period_end {
                success = false;
                break;
            }
            last_time = next_time;
        }
        if success {
            return period_start;
        } else {
            // println!(">>> No good with period starting at timestamp: {}", period_start);
        }
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
}
