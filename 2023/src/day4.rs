use std::{cell::RefCell, cmp, collections::HashMap, fs};

struct Card {
    own_numbers: Vec<u8>,
    winning_numbers: Vec<u8>,
}

pub fn day4() {
    let file = fs::read_to_string("input/da4").expect("Needs AOC input file to run");
    let total = calculate_winnings(file.clone());
    println!("day 4 {total}");
    let totalday2 = calculate_winnings_2(file);
    println!("day 4 2 {totalday2}");
}

fn calculate_winnings(input: String) -> u32 {
    let mut total = 0;
    let card_lines = input.split('\n');

    for card_line in card_lines {
        if card_line == "" {
            break;
        }
        let index_start_slice = card_line.find(":").unwrap_or(0);
        let card_line = &card_line[index_start_slice + 2..];

        let card_line: Vec<&str> = card_line.split("|").collect();
        let my_numbers: Vec<u32> = card_line[1]
            .split(" ")
            .map(|n| n.trim())
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        let winning_numbers: Vec<u32> = card_line[0]
            .split(" ")
            .map(|n| n.trim())
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        let mut card_total = 0;
        let mut first_point = true;

        for my in &my_numbers {
            for winning in &winning_numbers {
                if my == winning {
                    if first_point {
                        card_total += 1;
                        first_point = false;
                    } else {
                        card_total = card_total * 2;
                    }
                    continue;
                }
            }
        }
        total += card_total;
    }

    total
}

fn calculate_winnings_2(input: String) -> usize {
    let mut total = 0;
    let card_lines: Vec<&str> = input.split('\n').collect();

    fun_name(&card_lines, 0, card_lines.len(), &mut total, 0);

    total
}

fn fun_name(card_lines: &[&str], start: usize, stop: usize, total_cards: &mut usize, depth: usize) {
    let mut start = start;
    let mut amount_of_times =  vec![1; card_lines.len()];
    for (index, card_line_str) in card_lines.into_iter().enumerate() {
        if *card_line_str == "" {
            break;
        }
        let index_start_slice = card_line_str.find(":").unwrap_or(0);
        let card_line = &card_line_str[index_start_slice + 2..];

        let card_line: Vec<&str> = card_line.split("|").collect();
        let my_numbers: Vec<u32> = card_line[1]
            .split(" ")
            .map(|n| n.trim())
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        let winning_numbers: Vec<u32> = card_line[0]
            .split(" ")
            .map(|n| n.trim())
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        let mut card_matches = 0;

        let mut winnings = 0;
        for my in &my_numbers {
            for winning in &winning_numbers {
                if my == winning {
                    winnings += 1
                }
            }
        }

        let mut index_to_update = index + 1;
        let my_times = amount_of_times[index];
        println!("{} times: {}", card_line_str, my_times);
        for i in 0..winnings {
            println!("Trying to add to {}", index_to_update + i);
            amount_of_times[index_to_update + i] += my_times;

        }
            *total_cards += my_times;

    }
}


#[test]
fn day4test() {
    let data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
    let re = calculate_winnings(data.to_string());

    assert_eq!(re, 13);
}
#[test]
fn day4_2_test() {
    let data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
    let re = calculate_winnings_2(data.to_string());
    assert_eq!(re, 32);
}
