use std::fs::read_to_string;

fn main() {
    let calibrations = parse_input();
    first(calibrations);
    let calibrations_string = parse_input_string_list();
    second(calibrations_string);
}

fn second(calibrations: Vec<(String, Vec<String>)>) {
    let mut sum = 0;
    for (left_string, right) in calibrations.iter() {
        let left_parsed = left_string.parse().unwrap();
        if try_operators(&*left_string, left_parsed, &right[0..right.len()]) {
            sum += left_parsed;
        }
    }

    println!("Fst: {sum}");
}
fn first(calibrations: Vec<(usize, Vec<usize>)>) {
    let mut sum = 0;
    for (left, right) in calibrations.iter() {
        if try_operators_withou_concat(*left, &right) {
            sum += left;
        }
    }

    println!("Fst: {sum}");
}

fn try_operators(left_string: &str, left_parsed:usize, right_strings: &[String]) -> bool {
    let right_last_string: &String = right_strings.last().expect("right should never be empty");
    let right_last_parsed:usize = right_last_string.parse().expect("right last should parse to usize");
    
    if right_strings.len() == 1 {
        if right_last_parsed == left_parsed
        {
            return true;
        } else {
            return false;
        }
    }
    let mul_possible;
    if left_parsed % right_last_parsed == 0 {
        let new_left = left_parsed / right_last_parsed;
        mul_possible = try_operators(&new_left.to_string(), new_left, &right_strings[0..right_strings.len()-1]);
        
    } else {
        mul_possible = false;
    }
    let add_possible;
    if left_parsed >= right_last_parsed {
        let new_left = left_parsed - right_last_parsed;
        add_possible = try_operators(&new_left.to_string(), new_left, &right_strings[0..right_strings.len()-1]);
    } else {
        add_possible = false;
    }
    let concat_possible;
    if left_string.ends_with(right_last_string) && left_string.len() > right_last_string.len(){
        let new_left_str= &left_string[0..&left_string.len()-&right_last_string.len()];
        let new_left = new_left_str.parse().expect("new_left should parse to usize");
        concat_possible = try_operators(new_left_str, new_left, &right_strings[0..right_strings.len()-1]);
    } else {
        concat_possible = false;
    }

    return mul_possible || add_possible || concat_possible;
}

fn try_operators_withou_concat(left: usize, right: &[usize]) -> bool {
    let right_last = *right.last().unwrap();
    if right.len() == 1 {
        if right_last == left {
            return true;
        } else {
            return false;
        }
    }
    let mul_possible;
    if left % right_last == 0 {
        mul_possible = try_operators_withou_concat(left / right_last, &right[0..right.len() - 1]);
    } else {
        mul_possible = false;
    }
    let add_possible;
    if left >= right_last {
        add_possible = try_operators_withou_concat(left - right_last, &right[0..right.len() - 1]);
    } else {
        add_possible = false;
    }
    return mul_possible || add_possible;
}

fn parse_input() -> Vec<(usize, Vec<usize>)> {
    let input = read_to_string("input.txt").unwrap();
    let mut calibrations = vec![];
    for line in input.lines() {
        let (left, right) = line.split_once(":").unwrap();
        let left: usize = left.parse().unwrap();
        let right: Vec<usize> = right
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        calibrations.push((left, right));
    }
    return calibrations;
}

fn parse_input_string_list() -> Vec<(String, Vec<String>)> {
    let input = read_to_string("input.txt").unwrap();
    let mut calibrations = vec![];
    for line in input.lines() {
        let (left, right) = line.split_once(":").unwrap();
        let left = left.to_string();
        let right: Vec<String> = right
            .split_ascii_whitespace()
            .map(|s| s.trim().to_string())
            .collect();
        calibrations.push((left, right));
    }
    return calibrations;
}

