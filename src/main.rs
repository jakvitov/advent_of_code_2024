use day_1::first;

mod day_1;
mod day_2;

#[derive(PartialEq)]
enum MulAutomataState {
    Letter(char),
    FirstNum,
    SecondNum,
    Idle
}

struct MulAutomata {
    state: MulAutomataState,
    first_num: String,
    second_num: String
}

fn reset_automata(automata: &mut MulAutomata) {
    automata.state = MulAutomataState::Idle;
    automata.first_num = String::new();
    automata.second_num = String::new();
}

fn mul_nums(first_num: &String, second_num: &String, ) -> i32 {
    dbg!("mul({},{})", first_num, second_num);
    let first = first_num.parse::<i32>().unwrap();
    let second = second_num.parse::<i32>().unwrap();

    return first * second;
}

fn next_char(automata: &mut MulAutomata, c: char, sum: &mut i32) {  

    match c {
        'm' => {reset_automata(automata); automata.state = MulAutomataState::Letter(c)}
        'u' => if automata.state == MulAutomataState::Letter('m') {automata.state = MulAutomataState::Letter(c)}
        'l' => if automata.state == MulAutomataState::Letter('u') {automata.state = MulAutomataState::Letter(c)}
        '(' => if automata.state == MulAutomataState::Letter('l') {automata.state = MulAutomataState::Letter(c)}
        ',' => if automata.state == MulAutomataState::FirstNum {automata.state = MulAutomataState::Letter(c)}
        
        ')' => if automata.state == MulAutomataState::SecondNum {
            *sum += mul_nums(&automata.first_num, &automata.second_num);
            automata.state = MulAutomataState::Idle
        },

        '1'..='9' => {
            if automata.state == MulAutomataState::Letter('(') || automata.state == MulAutomataState::FirstNum {
                automata.first_num.push(c);
                automata.state = MulAutomataState::FirstNum;
            } else if automata.state == MulAutomataState::Letter(',') || automata.state == MulAutomataState::SecondNum {
                automata.second_num.push(c);
                automata.state = MulAutomataState::SecondNum;
            }
        },

        _ => automata.state = MulAutomataState::Idle
    }
}

fn firstt() -> i32{
    let input_str = include_str!("inputs/day3_test_input");
    let automata = &mut MulAutomata{first_num: String::new(), second_num: String::new(), state: MulAutomataState::Idle};
    let sum = &mut 0;

    for i in input_str.chars() {
        next_char(automata, i, sum);
    }

    return *sum;
}

fn main() {
    println!("--------------------------------");
    println!("Day 1, Task 1: {}.", day_1::first());
    println!("Day 1, Task 2: {}.", day_1::second());
    println!("--------------------------------");
    println!("Day 2, Task 1: {}.", day_2::first());
    println!("Day 2, Task 2: {}.", day_2::second());
    println!("--------------------------------");
    println!("Day 3, Task 1: {}.", firstt());


}
