mod day_1;

fn is_level_valid(level: &Vec<i32>) -> bool {
    if !(level.is_sorted() || level.is_sorted_by(|x, y| x > y)) {
        //println!("Level not sorted: {:?}", level);
        return false;
    }
    let mut diffs_valid = true;
    for i in 1..level.len() {
        let diff = (level[i-1] - level[i]).abs();
        if diff < 1 || diff > 3 {
            diffs_valid = false;
            //println!("Level not diff: {:?}", level);
            break;
        } 
    }
    return diffs_valid
}

fn main() {
    //println!("Day 1, Task 1 {}.", day_1::first());
    //println!("Day 1, Task 2 {}.", day_1::second());

    let file_str: String = String::from(include_str!("inputs/day2_input"));

    let lines: Vec<String> = file_str.split("\n").map(|x| String::from(x)).collect();
    
    let res:  Vec<Vec<i32>> = lines.into_iter().map(|line| line.split_whitespace().map(|num| num.parse::<i32>().unwrap()).collect()).collect();

    let safe_levels = res.into_iter().filter(|level| is_level_valid(&level)).count();

    println!("{}", safe_levels);

    return ();
}
