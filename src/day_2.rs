
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

//Is the array sorted with toleration of one field, returned in the second argument
fn is_sorted_toleration(level:  &Vec<i32>) -> (bool, Option<usize>) {
    let asc = level[1] > level[0];
    for i in 1..level.len() {
        if (asc && level[i] < level[i-1]) || (!asc && level[i] > level[i-1]) {
            return (false, Some(i))
        }
    }
    //We are sorted
    return (true, None)
}

fn is_level_valid_toleration(level_input: &Vec<i32>) -> bool {
    let level = &mut level_input.to_vec();
    let (sorted, invalid) = is_sorted_toleration(level);
    if !sorted {
        //Remove the first invalid
        level.remove(invalid.unwrap());
        if !is_level_valid(level) {
            println!("Invalid after removal: {:?}.", level);
        }
        return is_level_valid(level)
    }

    //Calculating the difference
    for i in 1..level.len() {
        let diff = (level[i-1] - level[i]).abs();
        if diff < 1 || diff > 3 {
            level.remove(i);
            if !is_level_valid(level) {
                println!("Invalid after removal: {:?}.", level);
            }
            return is_level_valid(level);
        } 
    }

    return true;
} 

pub fn first() -> usize {
    let levels  = get_input();
    return levels.into_iter().filter(|level| is_level_valid(&level)).count();
}

pub fn second() -> usize {
    let levels = get_input();
    return levels.into_iter().filter(|level| is_level_valid_toleration(level)).count();
}

