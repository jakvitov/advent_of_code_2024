use std::fs;

fn main() {
    let read_file: Result<String, std::io::Error> = fs::read_to_string("src/day1/day1_input");
    let input:String = read_file.unwrap();

    let rows: Vec<&str> = input.split("\n").collect();
    
    let mut first_list: Vec<i32> = vec![];
    let mut second_list: Vec<i32> = vec![];

    for row in rows {
        let items: Vec<&str> = row.split_whitespace().collect();
        first_list.push(items[0].parse::<i32>().unwrap());
        second_list.push(items[1].parse::<i32>().unwrap());
    };

    first_list.sort();
    second_list.sort();
   
    let mut sum = 0;

    for i in 0..first_list.len() {
        sum = sum + (first_list[i] - second_list[i]).abs()
    }

    println!("{}", sum);

}
