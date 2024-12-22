use std::{collections::HashSet, fs::read_to_string};

use regex::Regex;

const INPUT: &str = "input.txt";
const FIELD_X: i32 = 101;
const FIELD_Y: i32 = 103;

fn main() {
    let mut robots = parse_input(INPUT);
    let robot_count = robots.len();
    // first(&robots);
    let mut robots_pos: HashSet<(i32, i32)> = robots
        .iter()
        .map(|Robot { position, .. }| *position)
        .collect();
    for s in 0..20000 {
        let mut count_upper = 0;
        let mut count_lower = 0;
        for x in 24..55 {
            // Ok so I first printed the field every second and noticed some patterns, where robots aligned in horizontal or vertical lines.
            // To be honest after googleing Advent of code 2024 day 14 part 2 I saw a picture of the tree and saw that it has a frame
            // becuase then I thought the patters are the frame reaccuring, I figured it is most easy to find it, and that is exactly what I do here
            // I havn't googled for a solution but I was verry confused because there were no informations about what a picture of a christmas tree is and what is meant by "most of the robots"
            // A first approach of me was also to count how many unique places areoccupied by robots, because I thought maybe the "most of the robots" information was a clue that most of them end up in the same field and therefor I could notice the picture if the unique field occupied was dramatically lower then the number of robots.
            if robots_pos.contains(&(x, 36)) {
                count_upper += 1;
            }
            if robots_pos.contains(&(x, 68)) {
                count_lower += 1;
            }
        }
        if count_lower > 15 && count_upper > 15 {
            println!("S: {s}");
            print_robots(&robots_pos);
        }
        robots.iter_mut().for_each(|Robot { position, velocity }| {
            position.0 = (position.0 + velocity.0).rem_euclid(FIELD_X);
            position.1 = (position.1 + velocity.1).rem_euclid(FIELD_Y);
        });
        robots_pos = robots
            .iter()
            .map(|Robot { position, .. }| *position)
            .collect();
    }
}

fn print_robots(robots: &HashSet<(i32, i32)>) {
    println!("");
    for y in 0..FIELD_Y {
        for x in 0..FIELD_X {
            if robots.contains(&(x, y)) {
                print!("O")
            } else {
                print!(".")
            }
        }
        println!("")
    }
}

fn first(robots: &Vec<Robot>) {
    let end_positions: Vec<(i32, i32)> = robots
        .iter()
        .map(|Robot { position, velocity }| {
            (
                (position.0 + velocity.0 * 100).rem_euclid(FIELD_X),
                (position.1 + velocity.1 * 100).rem_euclid(FIELD_Y),
            )
        })
        .collect();
    println!("end_positions: {:?}", end_positions);
    let quadrant_counts = end_positions.iter().fold(
        (0, 0, 0, 0),
        |(left_top, right_top, right_bot, left_bot), position| {
            if position.0 < FIELD_X / 2 && position.1 < FIELD_Y / 2 {
                (left_top + 1, right_top, right_bot, left_bot)
            } else if position.0 > FIELD_X / 2 && position.1 < FIELD_Y / 2 {
                (left_top, right_top + 1, right_bot, left_bot)
            } else if position.0 > FIELD_X / 2 && position.1 > FIELD_Y / 2 {
                (left_top, right_top, right_bot + 1, left_bot)
            } else if position.0 < FIELD_X / 2 && position.1 > FIELD_Y / 2 {
                (left_top, right_top, right_bot, left_bot + 1)
            } else {
                (left_top, right_top, right_bot, left_bot)
            }
        },
    );
    println!("quadrant_counts: {:?}", quadrant_counts);
    let result = quadrant_counts.0 * quadrant_counts.1 * quadrant_counts.2 * quadrant_counts.3;
    println!("Fst: {}", result);
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

fn parse_input(path: &str) -> Vec<Robot> {
    let input = read_to_string(path).unwrap_or_else(|_| panic!("error reading {path}"));
    let re = Regex::new(r"p=(-?\d*),(-?\d*) v=(-?\d*),(-?\d*)").unwrap();
    re.captures_iter(&input)
        .map(|e| e.extract())
        .map(|(_, [p_x, p_y, v_x, v_y])| Robot {
            position: (p_x.parse().unwrap(), p_y.parse().unwrap()),
            velocity: (v_x.parse().unwrap(), v_y.parse().unwrap()),
        })
        .collect()
}
