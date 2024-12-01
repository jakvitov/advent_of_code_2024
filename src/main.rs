use std::fs;

fn get_input() -> [Vec<i32>; 2] {
       return [vec![], vec![]] 
}

fn main() {
    let input: Result<String, std::io::Error> = fs::read_to_string("day1/day1_input");
    let str:String = input.unwrap();
    println!("{str}");
}
