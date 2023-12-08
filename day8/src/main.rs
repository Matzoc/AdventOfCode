use std::env;
use std::fs;
use std::collections::HashMap;


struct Map {
    directions : String,
    adjacency_matrix : HashMap<String, [String;2]>,
    start : Vec<String>
}


fn parse_file(file_path : &String) -> Map {
    let contents : String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut iterator = contents.lines();
    let directions = iterator.next().expect("str expected").to_string();
    
    let adjacency_matrix: HashMap<String, [String;2]> = HashMap::new();
    let mut map = Map {directions, adjacency_matrix, start : Vec::new()};
        
    // eat the empty line
    iterator.next();

    for line in iterator {

        let values = line.split(" = ").collect::<Vec<_>>();

        let adjacencies = values[1].split(", ").collect::<Vec<_>>();
        let left = adjacencies[0].strip_prefix("(").expect("str");
        let right = adjacencies[1].strip_suffix(")").expect("str");
        let k = values[0].to_string();

        if k.ends_with("A") {
            map.start.push(k.clone());
        }

        map.adjacency_matrix.insert(
            k,
            [left.to_string(), right.to_string()]
        );
    }
    map
}

fn dir_to_ix(c : char) -> u8 {
    match c {
        'L' => 0,
        _ => 1
    }
}


fn ends_with_z(values: &Vec<&String>) -> bool {
    for v in values {
        if !v.ends_with("Z") {return false}
    }

    return true
}

fn min_found(values : &Vec<i64>) -> bool {
    for v in values {
        if *v == 0 {
            return false
        }
    }

    return true
}

fn lcm(values : &Vec<i64>, start : usize, end : usize) -> i64 {
    if end - start == 1 {
        values[start] * values[end] / gcd(values[start], values[end])
    } else {
        let rest = lcm(values, start + 1, end);
        values[start] * rest / gcd(values[start], rest)
    }
}

fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}


fn calc_path_len2(map : Map) -> i64 {
    let mut step : i64 = 0;
    let mut current: Vec<&String> = Vec::new();
    let mut min :Vec<i64> = Vec::new();

    for v in &map.start {
        current.push(v);
        println!("Adding: {}", v);
        min.push(0);
    }

    let dir_len = map.directions.len();
    let directions = map.directions.as_bytes();

    while !min_found(&min) {
        let i = (step % dir_len as i64) as usize;
        let dir = dir_to_ix(directions[i] as char) as usize; 

        let mut j = 0;

        while j < current.len() {
            let aux = &map.adjacency_matrix.get(current[j]).expect("v")[dir];
            current[j] = aux;
            println!("aux: {}", aux);

            if aux.ends_with("Z") {
                min[j] = (step + 1) as i64;
            }

            j += 1;
        }

        step += 1;
        println!("Steps: {}", step);
    }
    for v in &min {
        println!("{}", v);

    }

    lcm(&min, 0, min.len() - 1) as i64
}

fn calc_path_len(start : &str, end : &str, map : Map) -> i64 {
    let mut step : i64 = 0;
    let mut current: &String = &start.to_string();
    let dir_len = map.directions.len();
    let directions = map.directions.as_bytes();

    while !current.eq(end) {
        let i = (step % dir_len as i64) as usize;
        let dir = dir_to_ix(directions[i] as char) as usize; 

        current = &map.adjacency_matrix.get(current).expect("v")[dir];

        step += 1;
    }

    step
}

fn main() {
    println!("Hello, welcome to day 7!");
    let args: Vec<String> = env::args().collect();
    let map = parse_file(&args[1]);
    println!("Steps: {}", calc_path_len2(map));
    // println!("Steps: {}", calc_path_len("AAA", "ZZZ", map));

}
