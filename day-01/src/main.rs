use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let first = first();
    println!("First Solution: {first}");
    let second = second();
    println!("Second Solution: {second}");
}

// would bee cooler with a Binary Heap but the sorted Iterator is nightly only. 
fn first() -> u32 {
    let input = read_to_string("input").unwrap();
    let mut left_list = vec![];
    let mut right_list = vec![];
    for line in input.lines() {
        let mut split = line.split_whitespace();
        let left_entry =  split.next().map(|s| s.parse::<i32>());
        let right_entry = split.next().map(|s| s.parse::<i32>());
        match (left_entry, right_entry) {
            (Some(Ok(left_entry)), Some(Ok(right_entry))) => {
                left_list.push(left_entry);
                right_list.push(right_entry);
            },
            _ => ()
        };
    }
    left_list.sort();
    right_list.sort();
    let count = left_list.iter().zip(right_list.iter()).fold(0, |acc,(left_e, right_e)| acc + right_e.abs_diff(*left_e));
    
    return count
}


fn second() -> i32 {
    let input = read_to_string("input").unwrap();
    let mut left_list = vec![];
    let mut right_count = HashMap::new();
    for line in input.lines(){
        let mut split = line.split_whitespace();
        let left_entry =  split.next().map(|s| s.parse::<i32>());
        let right_entry = split.next().map(|s| s.parse::<i32>());
        match (left_entry, right_entry) {
            (Some(Ok(left_entry)), Some(Ok(right_entry))) => {
                left_list.push(left_entry);
                if let Some(count) = right_count.get_mut(&right_entry){
                    *count += 1;
                } else {
                    right_count.insert(right_entry, 1);
                }
            },
            _ => ()
        };    }
    let score = left_list.iter().fold(0, |acc,v| acc + v * right_count.get(v).unwrap_or(&0));
    return score;

}
