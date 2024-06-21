use std::{any::Any, fs};

use crate::day3::PartNumber;

pub fn day3_2() {
    let file = fs::read_to_string("input/day3").expect("Needs AOC input file to run");
    let total = execute_day3_2(file);

    println!("Result {total}")
}

fn execute_day3_2(file: String) -> u32 {
    let lines = file.split("\n");

    let mut total = 0;
    let line_collection = lines.clone().map(|l| l.trim()).collect::<Vec<&str>>();

    let mut part_numbers: Vec<PartNumber> = vec![];
    built_part_numbers(lines, &mut part_numbers);

    'partloop: for (index, line) in line_collection.clone().into_iter().enumerate() {
        for (char_index, char) in line.chars().enumerate() {
            if char == '*' {
                let adjacent_part_numbers =
                    get_adjacent_part_numbers(index, char_index, &part_numbers);
                if adjacent_part_numbers.len() == 2 {
                    let gear_ration =
                        adjacent_part_numbers[0].number * adjacent_part_numbers[1].number;
                    total += gear_ration;
                }
            }
        }
    }
    total
}

fn get_adjacent_numbers(index: usize, char_index: usize, line_collection: &Vec<&str>) {
    let lines = &line_collection[index - 1..index + 2];

    for line in lines {
        let chars = &line[char_index - 1..index + 2];
        dbg!(chars);
    }
}

fn built_part_numbers(lines: std::str::Split<&str>, part_numbers: &mut Vec<PartNumber>) {
    for (line_index, line) in lines.enumerate() {
        let mut current_line_chars = line.chars().peekable();
        let mut number = String::new();

        let mut index = 0;
        loop {
            let ch = current_line_chars.next();
            if ch == None {
                break;
            }
            let next_ch = current_line_chars.peek();

            match ch {
                Some(ch) if ch.is_digit(10) => {
                    number.push(ch);
                    match next_ch {
                        Some(next_ch) if !next_ch.is_digit(10) => {
                            part_numbers.push(PartNumber {
                                number: number.parse::<u32>().unwrap(),
                                line: line_index,
                                start_index: index,
                                end_index: index + number.len() - 1,
                            });
                            index += number.len() - 1;
                            number.clear();
                        }
                        Some(next_ch) if next_ch.is_digit(10) => {
                            continue;
                        }
                        None => {
                            part_numbers.push(PartNumber {
                                number: number.parse::<u32>().unwrap(),
                                line: line_index,
                                start_index: index,
                                end_index: index + number.len() - 1,
                            });
                            index += number.len() - 1;
                            number.clear();
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
            index += 1;
        }
    }
}

fn get_adjacent_part_numbers(
    line: usize,
    index: usize,
    part_numbers: &Vec<PartNumber>,
) -> Vec<PartNumber> {
    let adjacent_lines = [line - 1, line, line + 1];
    let adjacent_columns = [index - 1, index, index + 1];

    let numbers = part_numbers
        .into_iter()
        .filter(|p| {
            let correct_line = adjacent_lines.contains(&p.line);
            if correct_line {
                for column in adjacent_columns {
                    if column >= p.start_index && p.end_index >= column {
                        return true;
                    }
                }
            }
            false
        })
        .map(|p| p.clone())
        .collect();
    numbers
}

#[test]
fn day3_2_sample() {
    let input = "467..114..\r\n\
...*......\r\n\
..35..633.\r\n\
......#...\r\n\
617*......\r\n\
.....+.58.\r\n\
..592.....\r\n\
......755.\r\n\
...$.*....\r\n\
.664.598..\r\n";

    let action = execute_day3_2(input.to_string());
    assert_eq!(action, 467835)
}
