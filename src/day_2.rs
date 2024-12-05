
fn get_input() -> Vec<Vec<i32>> {
    let file_str: String = String::from(include_str!("inputs/day2_input"));

    let lines: Vec<String> = file_str.split("\n").map(|x| String::from(x)).collect();
    
    return lines.into_iter().map(|line| line.split_whitespace().map(|num| num.parse::<i32>().unwrap()).collect()).collect()
}

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

fn is_valid_toleration(level: &Vec<i32>) -> bool {
    if is_level_valid(level) {
        return true;
    }
    for i in 0..level.len() {
        //Horrible copying of one vector over again :)
        let mut adj_level: Vec<i32> = level.to_vec();
        adj_level.remove(i);
        if is_level_valid(&adj_level) {
            return true;
        }
    }
    return false;
}

pub fn first() -> usize {
    let levels  = get_input();
    return levels.into_iter().filter(|level| is_level_valid(&level)).count();
}

pub fn second() -> usize {
    let levels = get_input();
    return levels.into_iter().filter(|level| is_valid_toleration(level)).count();
}

