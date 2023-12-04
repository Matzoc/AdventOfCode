use std::env;
use std::fs;
use std::collections::HashMap;

fn parse_line(line : &str) -> i32 {
    let mut acc = 0;
    
    let parsed_line = line.split(":").collect::<Vec<_>>()[1]
        .split("|").collect::<Vec<_>>();

    let mut winning_numbers: HashMap<i32, i32> = HashMap::new();

    // println!("Parsed line 1|{}| Parsed line 2|{}|", parsed_line[0], parsed_line[1]);

    for n in parsed_line[0].trim().split(" ") {
        // println!("N:{}", n);
        if n.eq("") {continue}
        let value = n.parse::<i32>().unwrap();

        winning_numbers.insert(value, value);
    }

    for n in parsed_line[1].trim().split(" ") {
        // println!("N:{}", n);
        if n.eq("") {continue}

        let value = n.parse::<i32>().unwrap();

        if winning_numbers.contains_key(&value) {
            acc += 1;
        } 
    }

    println!("Total hits: {}", acc);

    acc
}



fn parse_file(file_path : &String) -> i32 {
    let contents : String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let lines_len = contents.lines().count();
    let lines = contents.lines();
    let mut curr_line = 0;

    let mut cards:Vec<[i32;2]> = vec![[1, 0]; lines_len];

    // calculate the existing cards
    for line in lines {
        let res = parse_line(line);

        cards[curr_line][1] = res;
        let mut j:i32 = 1;
        while j <= res && curr_line + (j as usize) < lines_len {
            // multiply the increase by the number of copies of the current card
            cards[curr_line + j as usize][0] = cards[curr_line + j as usize][0] + 1 * cards[curr_line][0]; 
            j += 1;
        }
        curr_line += 1;
    }

    let mut acc = 0;

    for card in cards {
        println!("cards: {}", card[0]);
        acc += card[0];
    }

    // for the first chall we can use this code
    // for line in lines {
    //     let res = parse_line(line);
    //     if res <= 1 {
    //         acc += res;
    //     } else {
    //         acc += i32::pow(2, (res - 1) as u32);
    //     }
    // }

    acc
}

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let res = parse_file(file_path);

    println!("The final result is {}.", res);
}
