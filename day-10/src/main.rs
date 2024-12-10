use std::{collections::HashSet, fs::read_to_string};

const INPUT: &str = "input.txt";

fn main() {
    let field = parse_input(INPUT);
    let field_y = field.len();
    let field_x = field[0].len();
    first(field_x, field_y, &field);
    fun_name(field_x, field_y, &field);
}

fn fun_name(field_x: usize, field_y: usize, field: &Vec<Vec<u32>>) {
    let mut ratings: Vec<Vec<Option<usize>>> = vec![vec![None; field_x]; field_y];
    let mut rating_sum = 0;
    for y in 0..field_y {
        for x in 0..field_x {
            if field[y][x] == 0 && ratings[y][x].is_none() {
                let rating = calc_rating(y, x, field_y, field_x, &field, &mut ratings);
                rating_sum += rating;
            }
        }
    }
    println!("Fsd: {rating_sum}");
}

fn calc_rating(y:usize, x:usize, field_y:usize, field_x:usize, field:&Vec<Vec<u32>>, ratings:&mut Vec<Vec<Option<usize>>>) -> usize {
    let field_num = field[y][x];
    let mut rating_sum = 0;
    if field_num == 9 {
        ratings[y][x] = Some(1);
        return 1;
    }
    let mut cords_to_check = Vec::with_capacity(4);
    if y > 0 && field[y - 1][x] == field_num + 1{
        cords_to_check.push((y - 1, x));
    }
    if x > 0 && field[y][x - 1] == field_num + 1{
        cords_to_check.push((y, x - 1));
    }
    if y + 1 < field_y && field[y + 1][x] == field_num + 1{
        cords_to_check.push((y + 1, x));
    }
    if x + 1 < field_x && field[y][x + 1] == field_num + 1{
        cords_to_check.push((y, x + 1));
    }
    for (n_y, n_x) in cords_to_check {
        match &ratings[n_y][n_x] {
            Some(r) => rating_sum += r,
            None => {
                let new_rating = calc_rating(n_y, n_x, field_y, field_x, field, ratings);
                rating_sum += new_rating;
            }
        }
    }
    ratings[y][x] = Some(rating_sum);
    return rating_sum;
}

fn first(field_x: usize, field_y: usize, field: &Vec<Vec<u32>>) {
    let mut scores: Vec<Vec<Option<HashSet<(usize, usize)>>>> = vec![vec![None; field_x]; field_y];
    let mut score_sum = 0;
    for y in 0..field_y {
        for x in 0..field_x {
            if field[y][x] == 0 && scores[y][x].is_none() {
                calc_score(y, x, field_y, field_x, &field, &mut scores);
                if let Some(s) = &scores[y][x] {
                    score_sum += s.len()
                }else{
                    panic!("score should be calculated after calc_score");
                }
            }
        }
    }
    println!("Fsd: {score_sum}");
}

fn calc_score(
    y: usize,
    x: usize,
    field_y: usize,
    field_x: usize,
    field: &Vec<Vec<u32>>,
    scores: &mut Vec<Vec<Option<HashSet<(usize, usize)>>>>,
) {
    let field_num = field[y][x];
    let mut result_set = HashSet::new();
    if field_num == 9 {
        result_set.insert((y,x));
        scores[y][x] = Some(result_set);
        return;
    }
    let mut cords_to_check = Vec::with_capacity(4);
    if y > 0 && field[y - 1][x] == field_num + 1{
        cords_to_check.push((y - 1, x));
    }
    if x > 0 && field[y][x - 1] == field_num + 1{
        cords_to_check.push((y, x - 1));
    }
    if y + 1 < field_y && field[y + 1][x] == field_num + 1{
        cords_to_check.push((y + 1, x));
    }
    if x + 1 < field_x && field[y][x + 1] == field_num + 1{
        cords_to_check.push((y, x + 1));
    }
    for (n_y, n_x) in cords_to_check {
        match &scores[n_y][n_x] {
            Some(s) => s.iter().for_each(|t| {
                result_set.insert(*t);
            }),
            None => {
                calc_score(n_y, n_x, field_y, field_x, field, scores);
                if let Some(s) = &scores[n_y][n_x] {
                    s.iter().for_each(|t| {
                        result_set.insert(*t);
                    });
                } else {
                    panic!("after one it should be right")
                }
            }
        }
    }
    scores[y][x] = Some(result_set);
}

fn parse_input(path: &str) -> Vec<Vec<u32>> {
    let input = read_to_string(path).expect("error reading files");
    let mut result = vec![];
    for line in input.lines() {
        let mut line_vec = vec![];
        for char in line.chars() {
            let num = char.to_digit(10).expect("not a digit");
            line_vec.push(num);
        }
        result.push(line_vec);
    }
    result
}
