use std::env;
use std::fs;

fn patch_str(t : &mut String) -> String {
    t.replace("one", "o1e").replace("two", "t2o")
        .replace("three", "t3e").replace("four", "f4r")
        .replace("five", "f5e").replace("six", "s6x")
        .replace("seven", "s7n").replace("eight", "e8t")
        .replace("nine", "n9e").replace("zero", "z0o")
}

fn get_calibration_value(txt : &[u8]) -> u8 {
    let mut acc = 0;
    let len = txt.len();
    let mut i :usize = 0;
    let mut left = true;
    let mut right = true;


    while (left || right) && i < len {
        let mut c: char = txt[i] as char;
        if left && c.is_numeric() {
            acc = acc + (txt[i] - 0x30) * 10;
            left = false;
        }
        c = txt[len - i - 1] as char;
        if right && c.is_numeric() {
            acc = acc + (txt[len - i - 1] - 0x30);
            right = false;
        }
        i += 1;
    }

    println!("{}", acc);
    acc
}


fn parse_file(file_path: &String) -> i32 {
    let contents : String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut total : i32 = 0;
    let parts: std::str::Lines<'_> = contents.lines();

    for part in parts {
        let p = patch_str(&mut part.to_string());
        let v : u8 = get_calibration_value(p.as_bytes());
        total = total + v as i32;
    }

    total
}


fn main() {
    // println!("Welcome to the Advent Code Event! Here is day 1...");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let total = parse_file(file_path);

    // println!("The calibration value is: {}", total);
    println!("{}", total);
}
