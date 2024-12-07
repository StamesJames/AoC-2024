use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let (input, mut obstacles, player_position, player_direction, field_y, field_x) = parse_input();
    second(
        player_position,
        player_direction,
        field_x,
        field_y,
        &mut obstacles,
        &input,
    );
    first(
        player_position,
        player_direction,
        field_x,
        field_y,
        &obstacles,
        &input,
    );
}



fn second(
    mut player_position: (i32, i32),
    mut player_direction: (i32, i32),
    field_x: i32,
    field_y: i32,
    obstacles: &mut HashSet<(i32, i32)>,
    input: &str,
) {
    let mut first_pos = HashMap::new();
    let mut visited_positions = HashSet::new();
    let mut visited_positions_with_direction = HashSet::new();
    let player_start_position = player_position;
    let mut additional_obstacles = HashSet::new();
    let mut next_pos;
    loop {
        visited_positions_with_direction.insert((player_position, player_direction));
        visited_positions.insert(player_position);
        first_pos.entry(player_position).or_insert(player_direction);

        next_pos = tup_add(player_position, player_direction);
        if next_pos.0 < 0 || next_pos.1 < 0 || next_pos.0 >= field_x || next_pos.1 >= field_y {
            break;
        }
        if obstacles.contains(&next_pos) {
            player_direction = rot(player_direction);
        } else {
            if !obstacles.contains(&next_pos)
                && !additional_obstacles.contains(&next_pos)
                && next_pos != player_start_position
                && !visited_positions.contains(&next_pos)
            {
                // let mut alt_obstacles = obstacles.clone();
                let alt_dir = rot(player_direction);
                // alt_obstacles.insert(next_pos);
                obstacles.insert(next_pos);
                let looped = check_for_loop(
                    player_position,
                    alt_dir,
                    &obstacles,
                    visited_positions_with_direction.clone(),
                    field_x,
                    field_y,
                );
                obstacles.remove(&next_pos);
                if looped {
                    additional_obstacles.insert(next_pos);
                }
            }
            player_position = next_pos;
        }
    }
    println!();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let (x, y) = (x.try_into().unwrap(), y.try_into().unwrap());
            match c {
                '#' => print!("#"),
                '.' => {
                    if additional_obstacles.contains(&(x, y)) {
                        print!("O");
                    } else if let Some(c) = first_pos.get(&(x, y)) {
                        match c {
                            (0, 1) => print!("v"),
                            (0, -1) => print!("^"),
                            (1, 0) => print!(">"),
                            (-1, 0) => print!("<"),
                            _ => panic!("direction should not be like this {:?}", c),
                        }
                    } else {
                        print!(".")
                    }
                }
                '^' => print!("*"),
                _ => panic!("{c} should not be here"),
            }
        }
        println!()
    }
    println!("Snd: {}", additional_obstacles.len());
}

fn check_for_loop(
    mut player_position: (i32, i32),
    mut player_direction: (i32, i32),
    obstacles: &HashSet<(i32, i32)>,
    mut visited_pos_dir: HashSet<((i32, i32), (i32, i32))>,
    field_x: i32,
    field_y: i32,
) -> bool {
    let mut next_pos;
    let mut loop_count = 0;
    let mut rot_count = 0;
    loop {
        next_pos = tup_add(player_position, player_direction);
        if visited_pos_dir.contains(&(player_position, player_direction)) {
            if loop_count == 0 && rot_count == 0 {
                // println!("loop: {loop_count}, {rot_count} pos: {:?}", player_position);
            }
            break true;
        }
        if next_pos.0 < 0 || next_pos.1 < 0 || next_pos.0 >= field_x || next_pos.1 >= field_y {
            // println!("out:  {loop_count}, {rot_count}");
            break false;
        }
        if obstacles.contains(&next_pos) {
            rot_count += 1;
            player_direction = rot(player_direction);
        } else {
            visited_pos_dir.insert((player_position, player_direction));
            loop_count += 1;
            player_position = next_pos;
        }
    }
}

fn first(
    mut player_position: (i32, i32),
    mut player_direction: (i32, i32),
    field_x: i32,
    field_y: i32,
    obstacles: &HashSet<(i32, i32)>,
    input: &str,
) {
    let mut visited_positions = HashSet::new();
    visited_positions.insert(player_position);
    let mut first_directions = HashMap::new();
    first_directions.insert(player_position, player_direction);
    let mut next_pos;
    let mut step_count = 0;
    loop {
        next_pos = tup_add(player_position, player_direction);
        if next_pos.0 < 0 || next_pos.1 < 0 || next_pos.0 >= field_x || next_pos.1 >= field_y {
            break;
        }
        if obstacles.contains(&next_pos) {
            player_direction = rot(player_direction);
        } else {
            player_position = next_pos;
            visited_positions.insert(player_position);
            first_directions
                .entry(player_position)
                .or_insert(player_direction);
            step_count += 1;
        }
    }
    println!();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let (x, y) = (x.try_into().unwrap(), y.try_into().unwrap());
            match c {
                '#' => print!("#"),
                '.' => {
                    if let Some(c) = first_directions.get(&(x, y)) {
                        match c {
                            (0, 1) => print!("v"),
                            (0, -1) => print!("^"),
                            (1, 0) => print!(">"),
                            (-1, 0) => print!("<"),
                            _ => panic!("direction should not be like this"),
                        }
                    } else {
                        print!(".")
                    }
                }
                '^' => print!("*"),
                _ => panic!("should not be here"),
            }
        }
        println!()
    }
    println!("next_pos: {:?}", next_pos);
    println!("Fst: {}", visited_positions.len());
    println!("step_count: {}", step_count);
}
fn tup_add((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> (i32, i32) {
    (x1 + x2, y1 + y2)
}
fn rot((x, y): (i32, i32)) -> (i32, i32) {
    (-y, x)
}

fn parse_input() -> (String, HashSet<(i32, i32)>, (i32, i32), (i32, i32), i32, i32) {
    let input = read_to_string("input.txt").unwrap();
    let mut obstacles = HashSet::new();
    let mut player_position: (i32, i32) = (0, 0);
    let mut player_direction: (i32, i32) = (0, 0);
    let field_y: i32 = input.lines().count().try_into().unwrap();
    let field_x: i32 = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .count()
        .try_into()
        .unwrap();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let (x, y) = (x.try_into().unwrap(), y.try_into().unwrap());
            match c {
                '#' => {
                    obstacles.insert((x, y));
                }
                '^' => {
                    player_position = (x, y);
                    player_direction = (0, -1);
                }
                'v' => {
                    player_position = (x, y);
                    player_direction = (0, 1);
                }
                '<' => {
                    player_position = (x, y);
                    player_direction = (-1, 0);
                }
                '>' => {
                    player_position = (x, y);
                    player_direction = (1, 0);
                }
                _ => (),
            }
        }
    }
    (input, obstacles, player_position, player_direction, field_y, field_x)
}