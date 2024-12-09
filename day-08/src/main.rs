use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

const INPUT: &str = "input.txt";

use points::Point;
mod points;
fn main() {
    let (field_x, field_y, antennas) = parse_input(INPUT);
    first(&antennas, field_x, field_y);
    second(&antennas, field_x, field_y);
}
fn second(antennas: &HashMap<char, Vec<Point>>, field_x: usize, field_y: usize) {
    let mut antinodes = HashSet::new();
    for (_, antennas) in antennas {
        for (i, antenna_1) in antennas.iter().enumerate() {
            for antenna_2 in &antennas[i+1..] {
                if antenna_1 == antenna_2 {
                    // should not happen but better safe then sorry :-D
                    continue;
                }
                let from_1_to_2 = (antenna_2 - antenna_1).shortest();
                println!("shortest {}", from_1_to_2);
                let mut next_point = *antenna_2;
                while check_field_bounds(field_x, field_y, next_point) {
                    antinodes.insert(next_point);
                    next_point += from_1_to_2 ;
                }
                let from_2_to_1 = -from_1_to_2;
                let mut next_point = *antenna_2 + from_2_to_1;
                while check_field_bounds(field_x, field_y, next_point) {
                    antinodes.insert(next_point);
                    next_point += from_2_to_1;
                }
            }
        }
    }
    let input = read_to_string(INPUT).unwrap();
    for (y, line) in input.lines().enumerate() {
        println!();
        for (x, char) in line.chars().enumerate() {
            if char != '.' {
                print!("{char}")
            }
            else if antinodes.contains(&Point::new(x.try_into().unwrap(), y.try_into().unwrap())) {
                print!("#");
            } else {
                print!("{char}");
            }
        }
    }
    println!();
    println!("Snd: {}", antinodes.len());
}

fn first(antennas: &HashMap<char, Vec<Point>>, field_x: usize, field_y: usize) {
    let mut antinodes = HashSet::new();
    for (_, antennas) in antennas {
        for (i, antenna_1) in antennas.iter().enumerate() {
            for antenna_2 in &antennas[i+1..] {
                let from_1_to_2 = antenna_2 - antenna_1;
                let antinode_1 = antenna_1 - &from_1_to_2;
                let antinode_2 = antenna_2 + &from_1_to_2;
                if check_field_bounds(field_x, field_y, antinode_2) {
                    // println!("antinode {}", antinode_1);
                    antinodes.insert(antinode_2);
                }
                if check_field_bounds(field_x, field_y, antinode_1) {
                    // println!("antinode {}", antenna_2);
                    antinodes.insert(antinode_1);
                }
            }
        }
    }
    let input = read_to_string(INPUT).unwrap();
    for (y, line) in input.lines().enumerate() {
        println!();
        for (x, char) in line.chars().enumerate() {
            if antinodes.contains(&Point::new(x.try_into().unwrap(), y.try_into().unwrap())) {
                print!("#");
            } else {
                print!("{char}");
            }
        }
    }
    println!();
    println!("Fst: {}", antinodes.len());
}

fn check_field_bounds(field_x: usize, field_y: usize, point: Point) -> bool {
    point.x >= 0
        && point.x < field_x.try_into().unwrap()
        && point.y >= 0
        && point.y < field_y.try_into().unwrap()
}

fn parse_input(input: &str) -> (usize, usize, HashMap<char, Vec<Point>>) {
    let input = read_to_string(input).expect("Error reading file");
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    let field_y = input.lines().count();
    let field_x = input
        .lines()
        .next()
        .expect("input should have a line")
        .chars()
        .count();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            }
            let entry = antennas.entry(char).or_insert(vec![]);
            entry.push(Point {
                x: x.try_into().unwrap(),
                y: y.try_into().unwrap(),
            });
        }
    }
    (field_x, field_y, antennas)
}
