use std::env;
use std::fs;


fn extrapolate_left(values : & mut Vec<Vec<i32>>) -> i32 {
    let mut acc = 0;
    let mut i = values.len() - 1;

    while i > 0 {
        let new_v = values[i][0];
        values[i].insert(0, new_v - acc);

        i -= 1;
        acc = new_v - acc;
    }

    let new_v = values[i][0];    
    values[0].insert(0, new_v - acc);
    values[0][0]
}


fn extrapolate_right(values : & mut Vec<Vec<i32>>) -> i32 {
    let mut acc = 0;
    let mut i = values.len() - 1;

    while i > 0 {
        let new_v = *values[i].last().expect("value");
        values[i].push(new_v + acc);

        i -= 1;
        acc = new_v + acc;
    }

    let new_v = *values[i].last().expect("value");
    values[0].push(new_v + acc);

    *values[0].last().expect("v")
}


fn derivate(values : & mut Vec<Vec<i32>>) {
    let mut derivation_completed = false;
    let mut current_derivative = 0;
    let mut i = 0;

    while derivation_completed == false {
        let mut derivative : Vec<i32> = Vec::new();
        let mut all_zeroes = true;

        while i < values[current_derivative].len() - 1 {
            let new_val = values[current_derivative][i + 1] - values[current_derivative][i];
            
            if new_val != 0 { 
                all_zeroes = false; 
            }

            derivative.push( new_val );
            i = i + 1;
        }

        //println!("{}zeros??", all_zeroes);

        if all_zeroes {derivation_completed = true;}

        values.push(derivative);
        current_derivative += 1;
        i = 0;
    }

}


fn extrapolate_all(parsed_file : & mut Vec<Vec<Vec<i32>>>) {
    let mut sum_left = 0;
    let mut sum_right = 0;
    for line in parsed_file {
        derivate(line);
        sum_left += extrapolate_left(line);
        sum_right += extrapolate_right(line);
    }

    println!("Part 1: {} | Part 2: {}", sum_right, sum_left);
}


fn parse_file(file_path : &str) -> Vec<Vec<Vec<i32>>> {
    let contents : String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut parsed_lines : Vec<Vec<Vec<i32>>> = Vec::new();

    for line in contents.lines() {
        let mut vec : Vec<i32>= Vec::new();
    
        for v in line.split(" ") {
            let n = v.parse::<i32>().unwrap();
            vec.push(n)
        }

        let mut v_temp : Vec<Vec<i32>> = Vec::new();
        v_temp.push(vec);
        parsed_lines.push(v_temp);
    }

    parsed_lines
}


fn main() {
    println!("Hello, welcome to day 9!");
    let args: Vec<String> = env::args().collect();
    let mut parsed_file = parse_file(&args[1]);
    // derivate(& mut parsed_file[2]);
    extrapolate_all(&mut parsed_file);
}
