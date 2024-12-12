use std::{collections::HashMap, fs::read_to_string};

const INPUT:&str = "input.txt";

fn main() {
    let stones = parse_input(INPUT);

    let mut sum = 0;
    let mut cash = HashMap::new();
    for stone in stones {
        sum += calc_stone(stone, 75, &mut cash);
    }
    println!("Fst: {sum}")
}

// could be cashed better with intermediat blinck results but was easier to write this way
fn calc_stone(stone:u64, blinks:u64, cash:&mut HashMap<(u64, u64), u64>) -> u64 {
    if let Some(r) = cash.get(&(stone,blinks)) {
        return *r;
    }
    let result;
    let stone_string = stone.to_string();
    if blinks == 0{
        result = 1
    } else if stone == 0 {
        result = calc_stone(1, blinks-1, cash);
    } else if stone_string.len() % 2 == 0 {
        let (left, right) = stone_string.split_at(stone_string.len() / 2);
        let left_stone = left.parse().unwrap_or_else(|_| panic!("error parsing left {left}"));
        let right_stone = right.parse().unwrap_or_else(|_| panic!("error parsing left {right}"));
        let left_result = calc_stone(left_stone, blinks-1, cash);
        let right_result = calc_stone(right_stone, blinks-1, cash);
        result = left_result + right_result;
    } else {
        result = calc_stone(stone * 2024, blinks-1, cash)
    }
    cash.insert((stone, blinks), result);
    return result;
}


fn parse_input(path:&str) -> Vec<u64> {
    let input = read_to_string(path).unwrap_or_else(|_| panic!("error reading file {path}"));
    input.split_whitespace().map(|s| s.parse().unwrap_or_else(|_| panic!("error parsing {s}"))).collect()
}
