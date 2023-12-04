use std::env;
use std::fs;

struct Number {
    x   : i32,
    y   : i32,
    v   : i32,
    len : i32
}

struct Symbol {
    x : i32,
    y : i32,
}

struct Board {
    numbers : Vec<Number>,
    symbols : Vec<Symbol>,
    width   : i32,
    height  : i32,
}

trait GetHitbox {
    fn get_hitbox(&self) -> [i32; 4];
}

impl GetHitbox for Symbol {
    fn get_hitbox(&self) -> [i32; 4]{
        [
            if self.x - 1 > 0 {self.x - 1} else {0},
            if self.y - 1 > 0 {self.y - 1} else {0}, 
            self.x + 1, 
            self.y + 1
        ] 
    }
}

impl GetHitbox for Number {
    fn get_hitbox(&self) -> [i32; 4]{
        [
            self.x,
            self.y, 
            self.x + self.len - 1, 
            self.y
        ] 
    }
}

fn is_symbol(c: char) -> bool {
    return !(c.is_numeric() || c == '.')
}

fn parse_line(board : &mut Board, line_n : i32, line : &str) {
    let line_bytes: &[u8] = line.as_bytes();
    let line_len: usize = line.len();
    let mut j: usize = 0;
    let mut n_len: usize = 0;

    let mut cur_n: i32 = 0;

    while j < line_len {
        // println!("j: {}", j);

        let mut c: char = line_bytes[j] as char;
        
        while j + n_len < line_len && c.is_numeric() {
            println!("j + l: {} {}", j, n_len);
            cur_n = cur_n * 10 + (line_bytes[j + n_len] - 0x30) as i32;
            n_len += 1;
            if j + n_len == line_len {break}
            c = line_bytes[j + n_len] as char;
            println!("j + l: {} {}", j, n_len);
        }

        if cur_n != 0 {
            board.numbers.push(
                Number {
                    x : j as i32, 
                    y : line_n, 
                    v : cur_n as i32, 
                    len : n_len as i32}
                );
            j = j + n_len;
            println!("cur_n is {}", cur_n);
            cur_n = 0;
            n_len = 0;
        }

        if is_symbol(c) {
            board.symbols.push(Symbol {x : j as i32, y : line_n});
        }

        j += 1;
    }
}


fn parse_file(file_path : &String) -> Board {
    let contents : String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let numbers:Vec<Number> = Vec::new();
    let symbols:Vec<Symbol> = Vec::new();

    let mut board : Board = Board {numbers, symbols, width : 0, height : 0};

    let mut i = 0;


    for line in contents.lines() {
        board.width = line.len() as i32;
        
        parse_line(& mut board, i, line);
        i += 1;
    }

    board.height = i;
    board
}


fn intersect(n : &Number, s : &Symbol) -> bool {
    let coords_n = n.get_hitbox();
    let coords_s = s.get_hitbox();

    return 
        coords_n[0] <= coords_s[2] &&
        coords_n[2] >= coords_s[0] &&
        coords_n[1] <= coords_s[3] &&
        coords_n[3] >= coords_s[1]
}

fn calc(board : &Board) -> [i32;2] {
    let mut acc = 0;
    let mut gear_acc = 0;
    let mut gear_count = 0;
    let mut curr_gear = 1;

    for s in &board.symbols {
        for n in &board.numbers {
            if intersect(n, s) {
                acc = acc + n.v;
                gear_count += 1;

                if gear_count <= 2 {
                    curr_gear = curr_gear * n.v
                } else {
                    curr_gear = 0
                }
            }
        }

        if gear_count == 2 {
            gear_acc += curr_gear;
        }

        curr_gear = 1;
        gear_count = 0;
    }
    [acc, gear_acc]
} 


fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let board = parse_file(file_path);
    println!("Parsed board!");


    let res = calc(&board);
    
    println!("Hello, world! {} {}", res[0], res[1]);

}