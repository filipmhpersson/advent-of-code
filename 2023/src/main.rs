use core::panic;
use std::fs;

mod day2;
mod day3;
mod day3_2;
mod day4;
fn main() {
    //day1();
    //day1_1();
    //day2::day2();
    //day2::day2_2();
    //day3::day3();
    //day3_2::day3_2();
    day4::day4();
}

fn day1() {
    let file = fs::read_to_string("input/day1").expect("Needs AOC input file to run");
    let lines = file.split("\n");
    let mut total: u32 = 0;
    for line in lines {
        if line == "" {
            break;
        }

        let mut string = "".to_string();
        for ch in line.chars() {
            if ch.is_digit(10) {
                if string == "" {
                    string = ch.to_string();
                }
                string = string.chars().next().unwrap().to_string() + &ch.to_string();
            }
        }

        let total_line = string.parse::<u32>().unwrap();
        total += total_line;
    }

    println!("AOC Day1 Total; {}", total);
}

fn day1_1() {
    let file = fs::read_to_string("input/day1").expect("Needs AOC input file to run");
    let lines = file.split("\n");
    let mut total: u32 = 0;
    let number_strings = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for line in lines {
        if line == "" {
            break;
        }

        let mut string = "".to_string();
        for (index, ch) in line.chars().enumerate() {
            let peek_index = if index + 5 > line.len() {
                line.len()
            } else {
                index + 5
            };
            let peek_text = &line[index..peek_index];
            println!("Peek: {index} {peek_index} {peek_text}");
            if ch.is_digit(10) {
                if string == "" {
                    string = ch.to_string();
                }
                string = string.chars().next().unwrap().to_string() + &ch.to_string();
            } else {
                let number: Vec<&str> = number_strings
                    .clone()
                    .into_iter()
                    .filter(|s| peek_text.starts_with(&s.to_string()))
                    .collect();
                match number.first() {
                    None => (),
                    Some(number_string) => {
                        let value = match *number_string {
                            "one" => 1,
                            "two" => 2,
                            "three" => 3,
                            "four" => 4,
                            "five" => 5,
                            "six" => 6,
                            "seven" => 7,
                            "eight" => 8,
                            "nine" => 9,
                            value => {
                                panic!("Unexpected string when parsing number_string {value} ")
                            }
                        };
                        if string == "" {
                            string = value.to_string();
                        }

                        string = string.chars().next().unwrap().to_string() + &value.to_string();
                    }
                }
            }
        }

        let total_line = string.parse::<u32>().unwrap();
        println!("{line}, total {total_line}");
        total += total_line;
    }
    println!("AOC Day1 Total; {}", total);
}
