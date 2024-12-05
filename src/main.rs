mod day_1;
mod day_2;
mod day_3;

#[derive(PartialEq)]
enum XmasAutomataState {
    Letter(char),
    Idle
}

struct XmasAutomata {
    state: XmasAutomataState
}

fn next_char(c: char, automata: &mut XmasAutomata, sum: &mut i32){
    match c {
        'X' => automata.state = XmasAutomataState::Letter(c),
        'M' => if automata.state == XmasAutomataState::Letter('X') {automata.state = XmasAutomataState::Letter(c)} else {automata.state = XmasAutomataState::Idle}
        'A' => if automata.state == XmasAutomataState::Letter('M') {automata.state = XmasAutomataState::Letter(c)} else {automata.state = XmasAutomataState::Idle}
        'S' => if automata.state == XmasAutomataState::Letter('A') {
            automata.state = XmasAutomataState::Idle;
            *sum += 1;
        }
        _ => automata.state = XmasAutomataState::Idle,
    }
}

fn main() {
    println!("--------------------------------");
    println!("Day 1, Task 1: {}.", day_1::first());
    println!("Day 1, Task 2: {}.", day_1::second());
    println!("--------------------------------");
    println!("Day 2, Task 1: {}.", day_2::first());
    println!("Day 2, Task 2: {}.", day_2::second());
    println!("--------------------------------");
    println!("Day 3, Task 1: {}.", day_3::first());
    println!("Day 3, Task 2: {}.", day_3::second());
    println!("--------------------------------");
    //println!("Day 4, Task 1: {}.", day_3::first());

    let mut res = 0;
    let input_str = include_str!("inputs/day4_test_input");

    let mut input_char_arr: Vec<Vec<char>> = Vec::new();
    let rows: Vec<&str> = input_str.split("\n").collect();
    for row in rows {
        input_char_arr.push(row.chars().collect())
    }

    //Result automata
    let mut xmas_automata = XmasAutomata{state: XmasAutomataState::Idle};
    let width = input_char_arr[0].len();
    let height = input_char_arr.len();


    //Rows
    for i in 0..height {
        for j in 0..width {
            next_char(input_char_arr[i][j], &mut xmas_automata, &mut res);
        }
    }
    println!("Rows: {}", res);

    //Reverse rows 
    for i in 0..height {
        for j in (0..width).rev() {
            next_char(input_char_arr[i][j], &mut xmas_automata, &mut res);
        }
    }
    println!("Reverse rows: {}", res);


    //Columns
    for i in 0..width {
        for j in 0..height {
            next_char(input_char_arr[j][i], &mut xmas_automata, &mut res);
        }
    }
    println!("Columns: {}", res);


    //Reverse columns
    for i in 0..width {
        for j in (0..height).rev() {
            next_char(input_char_arr[j][i], &mut xmas_automata, &mut res);
        }
    }
    println!("Reverse columns: {}", res);

    //Height diagonal
    for min in 1..height {
        for i in min..height {
            next_char(input_char_arr[i][i-min], &mut xmas_automata, &mut res);
        }
    }
    println!("Height diagonal: {}", res);

    //Height diagonal rev 
    for min in (1..height).rev() {
        for i in (min..height).rev() {
            next_char(input_char_arr[i][i-min], &mut xmas_automata, &mut res);
        }
    }
    println!("Height diagonal rev: {}", res);


    //Width diagonal
    for min in 1..width {
        for i in min..width {
            next_char(input_char_arr[i-min][i], &mut xmas_automata, &mut res);
        }
    }
    println!("Width diagonal: {}", res);


    //Width diagonal rev
    for min in (1..width).rev() {
        for i in (min..width).rev() {
            next_char(input_char_arr[i-min][i], &mut xmas_automata, &mut res);
        }
    }
    println!("Width diagonal rev: {}", res);


    for i in 0..width {
        next_char(input_char_arr[i][i], &mut xmas_automata, &mut res);
    }
    println!("Full diagonal: {}", res);

    for i in (0..width).rev() {
        next_char(input_char_arr[i][i], &mut xmas_automata, &mut res);
    }
    println!("Full diagonal rev: {}", res);

    println!("{}", res);


}
