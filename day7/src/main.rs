use std::env;
use std::fs;
use std::collections::HashMap;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]

struct Pair {
    count : i32,
    card : i32,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]

struct Hand {
    result : i64,
    bet : i64,
}

fn char_to_v(c : char, part2 : bool) -> i32 {
    return match c {
        'T' => 10,
        'J' => if part2 {1} else {11},
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _   => {
            //println!("rip{}", (c as u8 - 0x30));
            (c as u8 - 0x30) as i32
        }
    }    
}

fn insert_card(collection : & mut HashMap<i32, i32> , card : i32, result : i64) -> i64{
    match collection.get(&card) {
        Some(count) => { collection.insert(card, count + 1); }
        None => { collection.insert(card, 1); }
    }

    result * 100 + (card as i64)
}


/*
    5       -> 7
    4       -> 6
    3 + 2   -> 5
    3       -> 4
    2 + 2   -> 3
    2       -> 2
    1       -> 1
    */

fn calc_res(p : &Pair, hand : &Hand, l : usize) -> i64 {
    if p.count == 5 {
        return 7 * 10000000000 + hand.result;
    } else if p.count == 4 {
        return 6* 10000000000 + hand.result;
    } else if p.count == 3 && l == 2 {
        return 5* 10000000000 + hand.result;
    } else if p.count == 3 && l == 3 {
        return 4* 10000000000 + hand.result;
    } else if p.count == 2 && l == 3 {
        return 3* 10000000000 + hand.result;
    } else if p.count == 2 && l == 4 {
        return 2* 10000000000 + hand.result;
    } else {
        return 1* 10000000000 + hand.result;
    }
}

fn calc_res_2(p : &Pair, hand : &Hand, l : usize, joker_count : i32) -> i64 {
    if p.count == 5 || p.count + joker_count == 5 {
        return 7 * 10000000000 + hand.result;
    } else if p.count == 4 || p.count + joker_count == 4 {
        return 6* 10000000000 + hand.result;
    } else if p.count == 3 && l == 2 
        || p.count + joker_count == 3 && l == 2 {
        return 5* 10000000000 + hand.result;
    } else if p.count == 3 && l == 3 
        || p.count + joker_count == 3 && l == 3{
        return 4* 10000000000 + hand.result;
    } else if p.count == 2 && l == 3 
        || p.count + joker_count == 2 && l == 3 {
        return 3* 10000000000 + hand.result;
    } else if p.count == 2 && l == 4 
        || p.count + joker_count == 2 && l == 4 {
        return 2* 10000000000 + hand.result;
    } else {
        return 1* 10000000000 + hand.result;
    }
}



fn create_hand(hand_str : &str, bet : &str, part2 : bool) -> Hand {
    println!("Creating hand {}", hand_str);
    let mut collection :HashMap<i32, i32> = HashMap::new();

    let mut hand = Hand {
        bet : bet.parse::<i64>().unwrap(),
        result : 0 
    };

    let mut joker_count = 0;

    for c in hand_str.chars() {
        let v = char_to_v(c, part2);
        hand.result = insert_card(& mut collection, v, hand.result);

        if v == 1 {
            joker_count += 1;
        }
    }

    let mut sorted_vec: Vec<Pair> = Vec::new();

    for (c, count) in &collection {
        if *c == 1 {continue}

        let new_pair = Pair {card : *c, count : *count};

        match sorted_vec.binary_search(&new_pair) {
            Ok(pos) => {} // element already in vector @ `pos` 
            Err(pos) => {
                sorted_vec.insert(pos, new_pair);
            },
        }
    }

    sorted_vec = sorted_vec.into_iter().rev().collect();

    let l = sorted_vec.len();
    
    // let p = &sorted_vec[0];

    let p = if l == 0 {&Pair { count: 5, card: 1 }} else {&sorted_vec[0]};

    let res = if part2 {calc_res_2(p, &hand, l, joker_count)} else {calc_res(p, &hand, l)};
    hand.result = res;

    hand
}


fn parse_file(file_path : &String, part2 : bool) -> Vec<Hand>{
    let contents : String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut hands: Vec<Hand> = Vec::new();

    for line in contents.lines() {
        let values = line.split(" ").collect::<Vec<_>>();
        hands.push(create_hand(values[0], values[1], part2));
    }

    println!("Sorting...");
    hands.sort();
    println!("Done!");

    hands
}

fn calc_winnings(hands : Vec<Hand>) -> i64 {
    let mut acc = 0;
    let mut i = 0;

    while i < hands.len() {
        acc += (i+1) as i64 * hands[i].bet as i64;
        i += 1;
    }

    acc
}



fn main() {
    println!("Hello, welcome to day 7!");
    let args: Vec<String> = env::args().collect();
    let hands = parse_file(&args[1], args[2].eq("2"));

    println!("Result: {}",calc_winnings(hands));
}
