use std::env;
use std::fs;


fn parse_set<'a>(set : &str, acc : &'a mut [i32; 3]) -> &'a [i32; 3] {
    for color in set.split(",") {
        // println!("{}", color);

        let v :Vec<&str> = color.split(" ").collect::<Vec<_>>();
        // println!("{} {}", v[1], v[2]);
        let value = v[1].parse::<i32>().unwrap();

        match v[2] {
            "red" => { if acc[0] < value {acc[0] = value;} }
            "green" => { if acc[1] < value {acc[1] = value;} }
            "blue" => { if acc[2] < value {acc[2] = value;} }
            &_ => todo!()
        }
    }

    acc
}

fn parse_game(line : &str) -> [i32; 3] {
    let mut array: [i32; 3] = [0; 3];

    for game in line.split(";") {
        parse_set(game, & mut array);
    }
    println!("Game: {} {} {}", array[0], array[1], array[2]);

    array
}

fn parse_file(file_path : &String) -> Vec<[i32; 3]> {
    let contents : String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut games:Vec<[i32; 3]> = Vec::new();

    for line in contents.lines() {
        let parsed_line = line.split(":").collect::<Vec<_>>();
        games.push(parse_game(parsed_line[1]));
    }

    games
}

fn calc_possible_games(games : &Vec<[i32; 3]>, possible : [i32; 3]) -> i32 {
    let mut total = 0;
    let mut i = 0;

    for g in games {
        if g[0] <= possible[0] && g[1] <= possible[1] && g[2] <= possible[2] {
            total += i + 1
        }

        i += 1;
    }

    total
}


fn calc_power(games : &Vec<[i32; 3]>) -> i32 {
    let mut total = 0;

    for g in games {
        total = total + g[0] * g[1] * g[2];
    }

    total
}

fn main() {
    println!("Hello, world!");
    // let res = parse_set(" 1 blue, 1 red", &mut array);
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let games: Vec<[i32; 3]> = parse_file(file_path);

    let res = calc_possible_games(&games, [12,13,14]);
    let power = calc_power(&games);


    println!("Hello, world! {} {}", res, power);

}
