use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::Hasher;

#[aoc_generator(day22)]
fn generate_input(input: &str) -> (VecDeque<u64>, VecDeque<u64>) {
    let segments = input.split("\n\n").collect::<Vec<&str>>();
    let decks = {
        let player_1_deck = segments[0]
            .lines()
            .map(|x| x.trim())
            .filter(|x| !x.starts_with("Player"))
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<VecDeque<u64>>();
        let player_2_deck = segments[1]
            .lines()
            .map(|x| x.trim())
            .filter(|x| !x.starts_with("Player"))
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<VecDeque<u64>>();
        (player_1_deck, player_2_deck)
    };
    return decks;
}

#[aoc(day22, part1)]
fn solve_part_1(decks: &(VecDeque<u64>, VecDeque<u64>)) -> u64 {
    let (_winning_player, winning_score, winning_deck) = play_combat(decks).unwrap();
    println!(
        "Winner is player {} with a score of {}!\n>>>> Deck: {:?}",
        _winning_player, winning_score, winning_deck
    );
    return winning_score;
}

#[aoc(day22, part2)]
fn solve_part_2(decks: &(VecDeque<u64>, VecDeque<u64>)) -> u64 {
    let (_winning_player, winning_score, winning_deck) = play_recursive_combat(decks).unwrap();
    println!(
        "Winner is player {} with a score of {}!\n>>>> Deck: {:?}",
        _winning_player, winning_score, winning_deck
    );
    return winning_score;
}

fn play_recursive_combat(
    decks: &(VecDeque<u64>, VecDeque<u64>),
) -> Option<(u64, u64, VecDeque<u64>)> {
    // Initialise set to record states of decks
    let mut deck_states: HashSet<u64> = HashSet::new();
    let mut p1_deck = decks.0.clone();
    let mut p2_deck = decks.1.clone();
    let mut rounds = 0;
    // Keep playing rounds until a player wins the game
    loop {
        rounds += 1;
        println!("==================================================");
        println!("[+] Round {}", rounds);
        println!(">>>> Player 1's deck: {:?}", p1_deck);
        println!(">>>> Player 2's deck: {:?}", p2_deck);
        // Check if same deck state has been seen before
        let deck_state_hash = generate_deck_state_hash(&p1_deck, &p2_deck);
        if deck_states.contains(&deck_state_hash) {
            // Player 1 wins the overall game!
            // // println!("[+] Player 1 wins the game (by repeated state)!");
            let p1_score = calculate_deck_score(&p1_deck);
            return Some((1, p1_score, p1_deck));
        }
        deck_states.insert(deck_state_hash);
        // Each player draws their top card
        let p1_top = p1_deck.pop_front().unwrap();
        let p2_top = p2_deck.pop_front().unwrap();
        println!("Player 1 plays: {}", p1_top);
        println!("Player 2 plays: {}", p2_top);
        // Check if both players have enough cards left to play sub-game of Recursive Combat
        if p1_top <= p1_deck.len() as u64 && p2_top <= p2_deck.len() as u64 {
            // Play a sub-game of Recursive Combat to determine the winner of the round
            let subdeck_copies =
                generate_subdeck_copies(&p1_deck, p1_top as usize, &p2_deck, p2_top as usize);
            let (winning_player, _score, _winning_deck) =
                play_recursive_combat(&subdeck_copies).unwrap();
            if winning_player == 1 {
                println!("Player 1 wins round {} of game ###", rounds);
                p1_deck.push_back(p1_top);
                p1_deck.push_back(p2_top);
            } else if winning_player == 2 {
                println!("Player 2 wins round {} of game ###", rounds);
                p2_deck.push_back(p2_top);
                p2_deck.push_back(p1_top);
            } else {
                // Should only be players 1 or 2!
                return None;
            }
        } else {
            if p1_top > p2_top {
                // Player 1 wins the round
                println!("Player 1 wins round {} of game ###", rounds);
                p1_deck.push_back(p1_top);
                p1_deck.push_back(p2_top);
            } else if p2_top > p1_top {
                // Player 2 wins the round
                println!("Player 2 wins round {} of game ###", rounds);
                p2_deck.push_back(p2_top);
                p2_deck.push_back(p1_top);
            } else {
                // If two cards are of equal value, deck is bad!
                return None;
            }
        }
        // Check if one of the decks if empty - winning condition
        if p1_deck.is_empty() {
            // Player 2 wins the overall game!
            // // println!("[+] Player 2 wins the game (by getting all the cards)!");
            let p2_score = calculate_deck_score(&p2_deck);
            return Some((2, p2_score, p2_deck));
        } else if p2_deck.is_empty() {
            // Player 1 wins the overall game!
            // // println!("[+] Player 1 wins the game (by getting all the cards)!");
            let p1_score = calculate_deck_score(&p1_deck);
            return Some((1, p1_score, p1_deck));
        }
    }
}

fn generate_subdeck_copies(
    p1_deck: &VecDeque<u64>,
    p1_subdeck_len: usize,
    p2_deck: &VecDeque<u64>,
    p2_subdeck_len: usize,
) -> (VecDeque<u64>, VecDeque<u64>) {
    return (
        p1_deck
            .iter()
            .take(p1_subdeck_len)
            .map(|x| *x)
            .collect::<VecDeque<u64>>(),
        p2_deck
            .iter()
            .take(p2_subdeck_len)
            .map(|x| *x)
            .collect::<VecDeque<u64>>(),
    );
}

/// Calculates the score for the given deck. Used to determine the score of the winning player for a
/// game of Combat.
fn calculate_deck_score(deck: &VecDeque<u64>) -> u64 {
    let mut score = 0;
    let mut multiplier = deck.len() as u64;
    for card in deck {
        score += card * multiplier;
        multiplier -= 1;
    }
    return score;
}

fn generate_deck_state_hash(p1_deck: &VecDeque<u64>, p2_deck: &VecDeque<u64>) -> u64 {
    let mut hasher = DefaultHasher::new();
    for p1_card in p1_deck {
        hasher.write_u64(*p1_card);
    }
    for p2_card in p2_deck {
        hasher.write_u64(*p2_card);
    }
    return hasher.finish();
}

/// Plays the Combat card game using the given decks. Assumed that cards are unique across the two
/// given decks.
///
/// Return value indicates the winning player (1 or 2) and the winning score.
fn play_combat(decks: &(VecDeque<u64>, VecDeque<u64>)) -> Option<(u64, u64, VecDeque<u64>)> {
    let mut p1_deck = decks.0.clone();
    let mut p2_deck = decks.1.clone();
    loop {
        // Check if a winning condition has been met
        if p1_deck.is_empty() {
            let p2_score = calculate_deck_score(&p2_deck);
            return Some((2, p2_score, p2_deck));
        } else if p2_deck.is_empty() {
            let p1_score = calculate_deck_score(&p1_deck);
            return Some((1, p1_score, p1_deck));
        }
        // Conduct the round and determine winner of round
        let p1_top = p1_deck.pop_front().unwrap();
        let p2_top = p2_deck.pop_front().unwrap();
        if p1_top > p2_top {
            // Player 1 wins the round
            p1_deck.push_back(p1_top);
            p1_deck.push_back(p2_top);
        } else if p2_top > p1_top {
            // Player 2 wins the round
            p2_deck.push_back(p2_top);
            p2_deck.push_back(p1_top);
        } else {
            // Cards should not have equal value - means we have back decks (ending the game)
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d22_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2020/day22.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(35397, result);
    }

    #[test]
    fn test_d22_p1_001() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day22_test_001.txt").unwrap(),
        );
        let result = solve_part_1(&input);
        assert_eq!(306, result);
    }

    #[test]
    fn test_d22_p2_001() {
        let input = generate_input(
            &std::fs::read_to_string("./input/2020/test/day22_test_001.txt").unwrap(),
        );
        let result = solve_part_2(&input);
        assert_eq!(291, result);
    }
}
