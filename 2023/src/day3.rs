use std::fs;

pub fn day3() {
    let file = fs::read_to_string("input/day3").expect("Needs AOC input file to run");
    let total = execute_day3(file);

    println!("Result {total}")
}

fn execute_day3(file: String) -> u32 {
    let lines = file.split("\n");

    let mut total = 0;
    let line_collection = lines.clone().map(|l| l.trim()).collect::<Vec<&str>>();

    let mut part_numbers: Vec<PartNumber> = vec![];
    built_part_numbers(lines, &mut part_numbers);

    'partloop: for part in part_numbers.into_iter() {
        let PartNumber {
            line,
            number,
            start_index,
            end_index,
        } = part;

        let start = if line == 0 { 0 } else { line - 1 };
        let end = if line == line_collection.len() {
            line_collection.len()
        } else {
            line + 2
        };

        let relevant_lines = &line_collection[start..end];
        let mut rel_line = 0;
        println!(
            "Matching number {} with index {} on line {}",
            part.number, part.start_index, part.line
        );
        for rel in relevant_lines {
            if check_for_match_in_line(&part, rel, rel_line + start) {
                total += part.number;

                println!("Found match");
                continue 'partloop;
            }
            rel_line += 1;
        }
        println!("no match");
        if relevant_lines.len() == 3 {}
    }
    total
}

fn check_for_match_in_line(part: &PartNumber, line: &str, line_index: usize) -> bool {
    if line == "" {
        return false;
    }
    let start_path = if part.start_index == 0 {
        0
    } else {
        part.start_index - 1
    };
    let end_path = if part.end_index == line.len() {
        line.len()
    } else {
        part.end_index + 1
    };

    if line_index != part.line {
        let end_range = if end_path == line.len() {
            end_path
        } else {
            end_path + 1
        };
        let str = &line[start_path..end_range];
        dbg!(str);
        for s in str.chars() {
            if s != '.' {
                return true;
            }
        }
    } else {
        let first = &line.chars().nth(start_path);
        if let Some(first) = first {
            if *first != '.' && part.start_index != 0 {
                return true;
            }
        }

        let last = &line.chars().nth(end_path);
        if let Some(last) = last {
            if *last != '.' && part.end_index != line.len() {
                return true;
            }
        }
    }
    false
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

#[derive(Debug, Clone)]
pub struct PartNumber {
    pub number: u32,
    pub line: usize,
    pub start_index: usize,
    pub end_index: usize,
}
