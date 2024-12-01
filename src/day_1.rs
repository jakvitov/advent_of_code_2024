use std::fs;
use std::collections::HashMap;

pub fn get_location_lists() -> (Vec<i32>, Vec<i32>) {
    let read_file: Result<String, std::io::Error> = fs::read_to_string("src/inputs/day1_input");
    let input:String = read_file.unwrap();

    let rows: Vec<&str> = input.split("\n").collect();
    
    let mut first_list: Vec<i32> = vec![];
    let mut second_list: Vec<i32> = vec![];

    for row in rows {
        let items: Vec<&str> = row.split_whitespace().collect();
        first_list.push(items[0].parse::<i32>().unwrap());
        second_list.push(items[1].parse::<i32>().unwrap());
    };

    return (first_list, second_list)
}

pub fn first() -> i32 {
    let location_lists = get_location_lists();
    let mut first_list  = location_lists.0;
    let mut second_list  = location_lists.1;

    first_list.sort();
    second_list.sort();

    let mut sum = 0;

    for i in 0..first_list.len() {
        sum = sum + (first_list[i] - second_list[i]).abs()
    }

    return sum;
}

pub fn second() -> i32{
    let localtion_lists = get_location_lists();
    let first_list = localtion_lists.0;
    let second_list = localtion_lists.1;

    //Create a hash map from the second list containing {number -> its presence}
    let mut second_map: HashMap<i32, i32> = HashMap::new();

    for i in second_list {
        //Update if key is present
        if second_map.contains_key(&i) {
            second_map.insert(i, second_map.get(&i).unwrap() + 1);
        } else {
            second_map.insert(i, 1);
        }
    }

    let mut res = 0;
    for i in first_list {
        let mult = second_map.get(&i);
        if (mult.is_some()) {
            res = res * mult.unwrap()
        }
    }

    return res;

}