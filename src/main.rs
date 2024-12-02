mod day_1;

fn main() {
    //println!("Day 1, Task 1 {}.", day_1::first());
    //println!("Day 1, Task 2 {}.", day_1::second());

    let file_str: String = String::from(include_str!("inputs/day2_input"));

    let lines: Vec<String> = file_str.split("\n").map(|x| String::from(x)).collect();
    
    let res:  Vec<Vec<i32>> = lines.into_iter().map(|x| x.split_whitespace().map(|num| num.parse::<i32>().unwrap()).collect()).collect();

    
}
