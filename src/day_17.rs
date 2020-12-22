use super::utils::machines::ConwayCube;

#[aoc_generator(day17)]
fn generate_input(input: &str) -> ConwayCube {
    return ConwayCube::new(input);
}

#[aoc(day17, part1)]
fn solve_part_1(conway_cube: &ConwayCube) -> u64 {
    let mut conway_cube = conway_cube.clone();
    for _ in 0..6 {
        conway_cube.conduct_step();
    }
    return conway_cube.count_active_cubes();
}
