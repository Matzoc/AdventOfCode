use std::env;
use std::fs;
use regex::Regex;


fn quadratic_formula(a : i64, b : i64, c : i64) -> [i64;2] {
    let root = ((b*b - 4*a*c) as f64).sqrt();
    let r1 = (-b as f64-root)/2.0*(a as f64);
    let r2 = (-b as f64 +root)/2.0*(a as f64);
    return [
        r1.floor().round() as i64,
        r2.ceil().round() as i64
    ]
}


fn calc_winning_vals(dist : i64, time : i64) -> i64 {
    let xs = quadratic_formula(1, -time, dist);

    let r = if xs[1] >= time {time - 1} else {xs[1]} - if xs[0] > 0 {xs[0] + 1} else {0};
    r
}

fn parse_file(file_path : &String, distances : & mut Vec<i64> , times : & mut Vec<i64>) {
    let contents : String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let re = Regex::new("[ \t]+").unwrap();
    let lines = contents.lines().collect::<Vec<_>>();
    
    
    let times_strs: Vec<&str> = re.split(lines[0].strip_prefix("Time:").expect("str")).collect();
    let distance_strs: Vec<&str> = re.split(lines[1].strip_prefix("Distance:").expect("str")).collect();

    for time in times_strs {
        let t = time.trim();
        if t.eq("") {continue}

        let value = t.parse::<i64>().unwrap();
        times.push(value);
    }

    for dist in distance_strs {
        
        let d = dist.trim();
        if d.eq("") {continue}

        let value = d.parse::<i64>().unwrap();
        distances.push(value);
    }
}




fn main() {
    println!("Hello, welcome to day 6!");
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut distances : Vec<i64> = Vec::new();
    let mut times : Vec<i64> = Vec::new();

    parse_file(file_path, & mut distances, & mut times);
    let mut i = 0;

    let mut acc = 1;
    let mut total_time = 0;
    let mut total_dist = 0;

    while i < distances.len() {
        acc = acc * calc_winning_vals(distances[i], times[i]);
        total_time = total_time * (i64::pow(10, times[i].to_string().len() as u32)) + times[i];
        total_dist = total_dist * (i64::pow(10, distances[i].to_string().len() as u32))  + distances[i];
        i += 1;
    } 



    // println!("time {} dist... {}", total_time, total_dist);


    println!("Total product: {}", acc);
    println!("Extra long race: {}", calc_winning_vals(total_dist, total_time));


}
