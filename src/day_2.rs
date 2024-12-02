
pub fn get_input() -> Vec<Vec<i32>> {
    let file_str: String = include_str!("/src/inputs/day2_input");

    let lines = file_str.split("\n").collect();
    
    let res:  Vec<Vec<i32>> = Vec::new();

    line.map(|x| {
        x.split_whitespace().collect()
    });
    
}