use std::collections::VecDeque;

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
    let (_winning_player, winning_score) = play_combat_card_game(decks).unwrap();
    return winning_score;
}

/// Plays the Combat card game using the given decks. Assumed that cards are unique across the two
/// given decks.
/// 
/// Return value indicates the winning player (1 or 2) and the winning score.
fn play_combat_card_game(decks: &(VecDeque<u64>, VecDeque<u64>)) -> Option<(u64, u64)> {
    let mut player_1_deck = decks.0.clone();
    let mut player_2_deck = decks.1.clone();
    loop {
        // Check if a winning condition has been met
        if player_1_deck.is_empty() {
            let mut winning_score = 0;
            let mut multiplier = player_2_deck.len() as u64;
            for card in player_2_deck {
                winning_score += card * multiplier;
                multiplier -= 1;
            }
            return Some((2, winning_score));
        } else if player_2_deck.is_empty() {
            let mut winning_score = 0;
            let mut multiplier = player_1_deck.len() as u64;
            for card in player_1_deck {
                winning_score += card * multiplier;
                multiplier -= 1;
            }
            return Some((1, winning_score));
        }
        // Conduct the round and determine winner of round
        let player_1_top = player_1_deck.pop_front().unwrap();
        let player_2_top = player_2_deck.pop_front().unwrap();
        if player_1_top > player_2_top { // Player 1 wins the round
            player_1_deck.push_back(player_1_top);
            player_1_deck.push_back(player_2_top);
        } else if player_2_top > player_1_top { // Player 2 wins the round
            player_2_deck.push_back(player_2_top);
            player_2_deck.push_back(player_1_top);
        } else { // Cards should not have equal value - means we have back decks (ending the game)
            return None;
        }
    }
}
