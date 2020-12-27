use std::collections::HashMap;
use std::collections::HashSet;

use regex::Regex;

#[aoc_generator(day21)]
fn generate_input(input: &str) -> Vec<(HashSet<String>, HashSet<String>)> {
    // Create empty vector to store food list, being listed allergens and ingredients
    let mut food_list: Vec<(HashSet<String>, HashSet<String>)> = vec![];
    let food_regex = Regex::new(r"^(.*) \(contains (.*)\)$").unwrap();
    for line in input.lines() {
        let line = line.trim();
        if food_regex.is_match(line) {
            let captures = food_regex.captures(line).unwrap();
            let ingredients = captures[1]
                .split(" ")
                .map(|x| x.to_string())
                .collect::<HashSet<String>>();
            let allergens = captures[2]
                .split(", ")
                .map(|x| x.to_string())
                .collect::<HashSet<String>>();
            food_list.push((allergens, ingredients));
        }
    }
    return food_list;
}

#[aoc(day21, part1)]
fn solve_part_1(food_list: &Vec<(HashSet<String>, HashSet<String>)>) -> u64 {
    // Determine set of all ingredients observed
    let mut all_ingredients: HashSet<String> = HashSet::new();
    // For each allergen, find the union of all ingredient sets in which the allergen occurs
    let mut allergen_ingredients: HashMap<String, HashSet<String>> = HashMap::new();
    for (allergens, ingredients) in food_list.iter() {
        // Build up set of all ingredients observed
        all_ingredients = all_ingredients
            .union(&ingredients)
            .map(|x| x.clone())
            .collect::<HashSet<String>>();
        // Build up intersection of ingredients for each allergen
        for allergen in allergens {
            if !allergen_ingredients.contains_key(allergen) {
                allergen_ingredients.insert(allergen.clone(), ingredients.clone());
            } else {
                let intersection = allergen_ingredients
                    .get(allergen)
                    .unwrap()
                    .intersection(&ingredients)
                    .map(|x| x.clone())
                    .collect::<HashSet<String>>();
                allergen_ingredients.insert(allergen.clone(), intersection);
            }
        }
    }
    // Combine set of all ingredients that may contain an allergen
    let mut potential_allergen_ingredients: HashSet<String> = HashSet::new();
    for (_allergen, ingredients) in allergen_ingredients {
        potential_allergen_ingredients = potential_allergen_ingredients
            .union(&ingredients)
            .map(|x| x.clone())
            .collect::<HashSet<String>>();
    }
    // Determine allergen-free ingredients by removing potentially allergen-containing ingredients
    let allergen_free_ingredients = all_ingredients
        .difference(&potential_allergen_ingredients)
        .map(|x| x.clone())
        .collect::<HashSet<String>>();
    // Count number of times the allergen-free ingredients occur across all foods
    let mut count = 0;
    for ingredient in allergen_free_ingredients {
        for (_allergens, ingredients) in food_list.iter() {
            if ingredients.contains(&ingredient) {
                count += 1;
            }
        }
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d21_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day21.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(2659, result);
    }

    #[test]
    fn test_d21_p1_001() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/test/day21_test_001.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(5, result);
    }
}
