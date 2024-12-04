use std::{fs::read_to_string, result};

use regex::Regex;

fn main() {
    let fst = first();
    println!("First Solution: {fst}");
    let snd = second();
    println!("Second Solution: {snd}");
}
fn second() -> i32 {
    let input = read_to_string("input.txt").unwrap();
    let mut sum = 0;
    let re = Regex::new(r"mul\(([0-9]?[0-9]?[0-9]),([0-9]?[0-9]?[0-9])\)").unwrap();
    
    let (mut first, mut second) = split_once_or_all(&input, "don't()");
    while !first.is_empty() {
        let result: i32 = re
            .captures_iter(first)
            .map(|e| e.extract())
            .map(|(_, [fst, snd])| (fst.parse().unwrap(), snd.parse().unwrap()))
            .map(|(fst, snd): (i32, i32)| fst * snd)
            .sum();
        sum += result;
        let (_,do_block) = split_once_or_all(second, "do()");
        (first, second) = split_once_or_all(do_block, "don't()");
    }


    return sum;
}

fn split_once_or_all<'a>(s:&'a str, d:&str)->(&'a str, &'a str){
    if let Some(t) = s.split_once(d) {
        t
    }else {
        (s, "")
    }
}

fn first() -> i32 {
    let input = read_to_string("input.txt").unwrap();
    let re = Regex::new(r"mul\(([0-9]?[0-9]?[0-9]),([0-9]?[0-9]?[0-9])\)").unwrap();
    let result: i32 = re
        .captures_iter(&input)
        .map(|e| e.extract())
        .map(|(_, [fst, snd])| (fst.parse().unwrap(), snd.parse().unwrap()))
        .map(|(fst, snd): (i32, i32)| fst * snd)
        .sum();

    return result;
}
