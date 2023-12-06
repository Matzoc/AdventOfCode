use std::env;
use std::fs;

struct Interval {
    start   : i64,
    range   : i64,
    output  : i64,
}

trait InInterval {
    fn in_interval(&self, v : i64) -> bool;
    fn get_result(&self, v : i64) -> i64;
}

impl InInterval for Interval {
    fn in_interval(&self, v : i64) -> bool {
        return v >= self.start && v < self.start + self.range;
    }

    fn get_result(&self, v : i64) -> i64 {
        if self.in_interval(v) {
            v - self.start + self.output
        } else { -1 }
    }
}

fn parse_seeds(seeds_txt : &str, seeds : &mut Vec<i64>) {
    for seed in seeds_txt.strip_prefix("seeds: ").expect("Str expected").split(" ") {
        let value = seed.parse::<i64>().unwrap();
        seeds.push(value);
    }
}

fn parse_map(line : &str) -> Interval{
    let v :Vec<&str> = line.split(" ").collect::<Vec<_>>();

    let interval : Interval = Interval {
        start : v[1].parse::<i64>().unwrap(), 
        range : v[2].parse::<i64>().unwrap(), 
        output : v[0].parse::<i64>().unwrap() 
    };

    return interval
} 

fn parse_file(file_path : &String, parsed_seeds : &mut Vec<i64>, parsed_maps : & mut [Vec<Interval>; 7]) {
    let contents : String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut res = contents.split("\n\n");

    parse_seeds(res.next()
        .expect("String expected"), parsed_seeds);

    let mut i = 0;

    for section in res {
        // println!("Processing map... {}", section);
        let mut lines = section.lines();
        lines.next();
        for interval_str in lines {
            parsed_maps[i].push(parse_map(interval_str));
        }

        i += 1;
    }
}

fn calc_seed_normal(seeds : &Vec<i64>, maps : & mut [Vec<Interval>; 7]) {
    let mut smallest_res = std::i64::MAX;

    for seed in seeds {
        let mut v: i64 = *seed;
        for mp in maps.iter() {
            let mut temp = -1;

            for intv in mp {
                temp = intv.get_result(v);

                if temp != -1 {
                    break;
                }
            }
            
            if temp != -1 {
                v = temp;
            }
        }

        if v < smallest_res {
            smallest_res = v
        }
    }

    println!("Result... {}", smallest_res);
}

fn calc_seed_ranges(seeds : &Vec<i64>, maps : & mut [Vec<Interval>; 7]) {
    let mut smallest_res = std::i64::MAX;
    let mut smallest_i = 0;
    let mut smallest_j = 0;

    let mut i = 0;
    while i < seeds.len()/2 {
        let mut j = 0;

        while j < seeds[2 * i + 1] {
            let mut v = seeds[2 * i] + j;
            for mp in maps.iter() {
                let mut temp = -1;

                for intv in mp {
                    temp = intv.get_result(v);

                    if temp != -1 {
                        break;
                    }
                }
                
                if temp != -1 {
                    v = temp;
                }
            }

            if v < smallest_res {
                smallest_res = v;
                smallest_i = i;
                smallest_j = j;
            }

            j += 50;
        }        
        i += 1;
    }

    println!("Possible result range... {} {} {}", smallest_res, smallest_i, smallest_j);


    let mut j = smallest_j - 50;

    while j < smallest_j + 50 {
        let mut v = seeds[2 * smallest_i] + j;
        for mp in maps.iter() {
            let mut temp = -1;

            for intv in mp {
                temp = intv.get_result(v);

                if temp != -1 {
                    break;
                }
            }
            
            if temp != -1 {
                v = temp;
            }
        }

        if v < smallest_res {
            smallest_res = v;
        }

        j += 1;
    }        

    println!("Result... {}", smallest_res);
}

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut maps: [Vec<Interval>; 7] = Default::default();
    let mut seeds : Vec<i64> = Vec::new();

    parse_file(file_path, & mut seeds, & mut maps);
    calc_seed_normal(&seeds, &mut maps);
    calc_seed_ranges(&seeds, &mut maps);
}
