use std::{collections::HashMap, fs::read_to_string, sync::{mpsc, Arc, Mutex}, thread};

const INPUT:&str = "input.txt";

fn main() {
    let stones = parse_input(INPUT);

    let cash = HashMap::new();
    let cash = Arc::new(Mutex::new(cash));
    let (tx, rx) = mpsc::channel();
    for stone in stones {
        let tx_clone = tx.clone();
        let mut cash_clone = cash.clone();
        thread::spawn(move ||
        {
            let val = calc_stone(stone, 75, &mut cash_clone);
            tx_clone.send(val).unwrap();
        });
    }
    drop(tx);
    let mut sum = 0;
    for val in rx {
        println!("reseaved {}", val);
        sum += val;
    }
    println!("Fst: {sum}")
}

// could be cashed better with intermediat blinck results but was easier to write this way
fn calc_stone(stone:u64, blinks:u64, cash:&mut Arc<Mutex<HashMap<(u64, u64), u64>>>) -> u64 {
    {
        let cash = cash.lock().unwrap();
        if let Some(r) = cash.get(&(stone,blinks)) {
            return *r;
        }
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
    {
        let mut cash = cash.lock().unwrap();
        cash.insert((stone, blinks), result);
    }
    return result;
}


fn parse_input(path:&str) -> Vec<u64> {
    let input = read_to_string(path).unwrap_or_else(|_| panic!("error reading file {path}"));
    input.split_whitespace().map(|s| s.parse().unwrap_or_else(|_| panic!("error parsing {s}"))).collect()
}
