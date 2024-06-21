use std::fs;

pub fn day2() {
    let file = fs::read_to_string("input/day2").expect("Needs AOC input file to run");
    let lines = file.split("\n");
    let mut result = 0;
    'gameloop: for line in lines {
        if line == "" {
            break;
        }
        let mut split = line.split(":");
        let mut intro = split.next().expect("Must contain intro info");
        let games = split.next().expect("Must contain game info").split(";");
        intro = intro.trim_start_matches("Game ");

        for mut game in games {
            game = game.trim();
            let colors = game.split(",");

            for color in colors {
                let mut arr = color.trim().split(" ");
                let amount = arr.next().expect("Game field must contain amout");
                println!("{color} amount {amount}");
                let intamount = amount.parse::<u32>().unwrap();
                let color = arr.next().expect("Game field must contain color");
                let max_allowed_amout: u32 = match color {
                    "red" => 12,
                    "green" => 13,
                    "blue" => 14,
                    _ => panic!("Unexpected color value: '{color}'"),
                };
                if intamount > max_allowed_amout {
                    continue 'gameloop;
                }
            }
            println!("{intro} ||| {game}");
        }
        result += intro.parse::<u32>().unwrap();
    }
    println!("Day2 Total {result}");
}

pub fn day2_2() {
    let file = fs::read_to_string("input/day2").expect("Needs AOC input file to run");
    let lines = file.split("\n");
    let mut result = 0;
    for line in lines {
        if line == "" {
            break;
        }
        let mut split = line.split(":");
        let mut intro = split.next().expect("Must contain intro info");
        let games = split.next().expect("Must contain game info").split(";");
        intro = intro.trim_start_matches("Game ");
        let mut highest_blue: u32 = 0;
        let mut highest_green: u32 = 0;
        let mut highest_red: u32 = 0;

        for mut game in games {
            game = game.trim();
            let colors = game.split(",");

            for color in colors {
                let mut arr = color.trim().split(" ");
                let amount = arr.next().expect("Game field must contain amout");
                println!("{color} amount {amount}");
                let intamount = amount.parse::<u32>().unwrap();
                let color = arr.next().expect("Game field must contain color");
                let calculate_highest_calue = |highest_val: u32| -> u32 {
                    if intamount > highest_val {
                        intamount
                    } else {
                        highest_val
                    }
                };
                match color {
                    "red" => highest_red = calculate_highest_calue(highest_red),
                    "green" => highest_green = calculate_highest_calue(highest_green),
                    "blue" => highest_blue = calculate_highest_calue(highest_blue),
                    _ => panic!("Unexpected color value: '{color}'"),
                };
            }
        }

        let pow = highest_green * highest_blue * highest_red;
        result += pow;
        println!("{line}: pow {pow}");
    }
    println!("Day2 Total {result}");
}
