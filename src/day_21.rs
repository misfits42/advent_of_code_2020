use std::collections::HashMap;
use std::collections::HashSet;

use regex::Regex;

#[derive(Clone)]
struct Food {
    allergens: HashSet<String>,
    ingredients: HashSet<String>,
}

#[aoc_generator(day21)]
fn generate_input(input: &str) -> Vec<Food> {
    // Create empty vector to store food list, being listed allergens and ingredients
    let mut food_list: Vec<Food> = vec![];
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
            food_list.push(Food {
                allergens: allergens,
                ingredients: ingredients,
            });
        }
    }
    return food_list;
}

#[aoc(day21, part1)]
fn solve_part_1(food_list: &Vec<Food>) -> u64 {
    let inert_ingredients = determine_inert_ingredients(food_list);
    // Count number of times the allergen-free ingredients occur across all foods
    let mut count = 0;
    for ingredient in inert_ingredients {
        for Food {
            allergens: _,
            ingredients,
        } in food_list.iter()
        {
            if ingredients.contains(&ingredient) {
                count += 1;
            }
        }
    }
    return count;
}

#[aoc(day21, part2)]
fn solve_part_2(food_list: &Vec<Food>) -> String {
    // Get the list of inert ingredients and ingredients that may contain an allergen
    let inert_ingredients = determine_inert_ingredients(food_list);
    let mut allergen_ingredients = determine_potential_allergen_ingredients(food_list);
    // Remove each inert ingredient from the lists of candidate ingredients for each allergen
    for ingredient in inert_ingredients {
        for candidate_ingredients in allergen_ingredients.values_mut() {
            candidate_ingredients.remove(&ingredient);
        }
    }
    // Reduce the allergen candidate ingredients until each allergen has a single unique ingredient
    // associated with it
    loop {
        let mut all_reduced = true;
        // Determine what ingredients are to be removed from other lists to reduce
        let mut unique_ingredients: HashSet<String> = HashSet::new();
        for (_allergen, candidate_ingredients) in allergen_ingredients.iter() {
            if candidate_ingredients.len() == 1 {
                unique_ingredients.insert(candidate_ingredients.iter().next().unwrap().clone());
            } else {
                all_reduced = false;
            }
        }
        // Check if all allergens have a single unique ingredient associated with them
        if all_reduced {
            break;
        }
        // Conduct next round of reductions
        for ingredient in unique_ingredients {
            for candidate_ingredients in allergen_ingredients.values_mut() {
                if candidate_ingredients.len() == 1 {
                    continue;
                }
                candidate_ingredients.remove(&ingredient);
            }
        }
    }
    // Generate the canonical list of dangerous ingredients - sorted alphabetically by allergen
    let mut dangerous_ingredients: Vec<String> = vec![];
    let mut allergens_sorted: Vec<String> = allergen_ingredients
        .keys()
        .map(|x| x.clone())
        .collect::<Vec<String>>();
    allergens_sorted.sort();
    for allergen in allergens_sorted {
        dangerous_ingredients.push(
            allergen_ingredients
                .get(&allergen)
                .unwrap()
                .iter()
                .next()
                .unwrap()
                .clone(),
        );
    }
    return dangerous_ingredients.join(",");
}

/// Determines all ingredients that are contained within foods with each observed listed allergen.
fn determine_potential_allergen_ingredients(
    food_list: &Vec<Food>,
) -> HashMap<String, HashSet<String>> {
    // For each allergen, find the union of all ingredient sets in which the allergen occurs
    let mut allergen_ingredients: HashMap<String, HashSet<String>> = HashMap::new();
    for Food {
        allergens,
        ingredients,
    } in food_list.iter()
    {
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
    return allergen_ingredients;
}

/// Determines the set of all ingredients observed for all foods in the given list.
fn determine_all_ingredients(food_list: &Vec<Food>) -> HashSet<String> {
    // Determine set of all ingredients observed
    let mut all_ingredients: HashSet<String> = HashSet::new();
    for Food {
        allergens: _,
        ingredients,
    } in food_list.iter()
    {
        all_ingredients = all_ingredients
            .union(&ingredients)
            .map(|x| x.clone())
            .collect::<HashSet<String>>();
    }
    return all_ingredients;
}

/// Determines what ingredients observed in the given food list definitely do not contain one of the
/// observed allergens.
fn determine_inert_ingredients(food_list: &Vec<Food>) -> HashSet<String> {
    // Determine set of all ingredients observed
    let all_ingredients = determine_all_ingredients(food_list);
    // For each allergen, find the union of all ingredient sets in which the allergen occurs
    let allergen_ingredients = determine_potential_allergen_ingredients(food_list);
    // Combine set of all ingredients that may contain an allergen
    let mut potential_allergen_ingredients: HashSet<String> = HashSet::new();
    for (_allergen, ingredients) in allergen_ingredients {
        potential_allergen_ingredients = potential_allergen_ingredients
            .union(&ingredients)
            .map(|x| x.clone())
            .collect::<HashSet<String>>();
    }
    // Determine allergen-free ingredients by removing potentially allergen-containing ingredients
    let inert_ingredients = all_ingredients
        .difference(&potential_allergen_ingredients)
        .map(|x| x.clone())
        .collect::<HashSet<String>>();
    return inert_ingredients;
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
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day21_test_001.txt").unwrap(),
        );
        let result = solve_part_1(&input);
        assert_eq!(5, result);
    }

    #[test]
    fn test_d21_p1_002() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day21_test_001.txt").unwrap(),
        );
        let result = solve_part_2(&input);
        assert_eq!(String::from("mxmxvkd,sqjhc,fvjkl"), result);
    }
}
