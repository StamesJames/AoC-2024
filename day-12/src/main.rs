use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

const INPUT: &str = "input.txt";

// It is not a nice solution, but it is one :-D
fn main() {
    let (sets, input_vec) = parse_input();
    first(&sets, &input_vec);
    let field_y = input_vec.len();
    let field_x = input_vec[0].len();
    println!("field_x: {}, field_y: {}", field_x, field_y);
    let mut left_done = HashSet::new();
    let mut right_done = HashSet::new();
    let mut top_done = HashSet::new();
    let mut bottom_done = HashSet::new();
    let mut sum = 0;
    for ((rep_x, rep_y), set) in sets.into_iter() {
        let mut fence_sides = 0;
        for (x,y) in set.iter() {
            let (x, y) = (*x,*y);
            let char = input_vec[y][x];
            if !left_done.contains(&(x, y)) && (x < 1 || (input_vec[y][x - 1] != char)) {
                let mut d_y = 1;
                while y >= d_y
                    && input_vec[y - d_y][x] == char
                    && ((x < 1) || input_vec[y - d_y][x - 1] != char )
                {
                    left_done.insert((x, y - d_y));
                    d_y += 1;
                }
                let mut d_y = 1;
                while y + d_y < field_y
                    && input_vec[y + d_y][x] == char
                    && ((x < 1) || input_vec[y + d_y][x - 1] != char )
                {
                    left_done.insert((x, y + d_y));
                    d_y += 1;
                }
                fence_sides += 1
            }
            left_done.insert((x,y));

            if !right_done.contains(&(x, y)) && (x + 1 >= field_x || (input_vec[y][x + 1] != char)) {
                let mut d_y = 1;
                while y >= d_y
                    && input_vec[y - d_y][x] == char
                    && ((x + 1 >= field_x) || input_vec[y - d_y][x + 1] != char )
                {
                    right_done.insert((x, y - d_y));
                    d_y += 1;
                }
                let mut d_y = 1;
                while y + d_y < field_y
                    && input_vec[y + d_y][x] == char
                    && ((x + 1 >= field_x) || input_vec[y + d_y][x + 1] != char )
                {
                    right_done.insert((x, y + d_y));
                    d_y += 1;
                }
                fence_sides += 1
            }
            right_done.insert((x,y));

            if !top_done.contains(&(x, y)) && (y < 1 || (input_vec[y - 1][x] != char)) {
                let mut d_x = 1;
                while x >= d_x
                    && input_vec[y][x - d_x] == char
                    && ((y < 1) || input_vec[y - 1][x - d_x] != char )
                {
                    top_done.insert((x - d_x, y));
                    d_x += 1;
                }
                let mut d_x = 1;
                while x + d_x < field_x
                    && input_vec[y][x + d_x] == char
                    && ((y < 1) || input_vec[y - 1][x + d_x] != char )
                {
                    top_done.insert((x + d_x, y));
                    d_x += 1;
                }
                fence_sides += 1
            }
            top_done.insert((x,y));
            
            if !bottom_done.contains(&(x, y)) && (y + 1 >= field_y || (input_vec[y + 1][x] != char)) {
                let mut d_x = 1;
                while x >= d_x
                    && input_vec[y][x - d_x] == char
                    && ((y + 1 >= field_y) || input_vec[y + 1][x - d_x] != char )
                {
                    bottom_done.insert((x - d_x, y));
                    d_x += 1;
                }
                let mut d_x = 1;
                while x + d_x < field_x
                    && input_vec[y][x + d_x] == char
                    && ((y + 1 >= field_y) || input_vec[y + 1][x + d_x] != char )
                {
                    bottom_done.insert((x + d_x, y));
                    d_x += 1;
                }
                fence_sides += 1
            }
            bottom_done.insert((x,y));
        }
        let char_rep = input_vec[rep_y][rep_x];
        println!("Field: {:?}, at {:?}, has {:?} sides and an area of {:?}", char_rep as char, (rep_x, rep_y), fence_sides, set.len());
        sum += fence_sides * set.len();
    }
    println!("Snd: {sum}")
}

fn first(sets: &HashMap<(usize, usize), HashSet<(usize, usize)>>, input_vec: &Vec<Vec<u8>>) {
    let mut sum = 0;
    for ((rep_x, rep_y), set) in sets.iter() {
        let mut fence = 0;
        for (x, y) in set {
            if *y == 0 || input_vec[y - 1][*x] != input_vec[*y][*x] {
                fence += 1;
            }
            if *x == 0 || input_vec[*y][x - 1] != input_vec[*y][*x] {
                fence += 1;
            }
            if *y == input_vec.len() - 1 || input_vec[y + 1][*x] != input_vec[*y][*x] {
                fence += 1;
            }
            if *x == input_vec[0].len() - 1 || input_vec[*y][x + 1] != input_vec[*y][*x] {
                fence += 1;
            }
        }
        let area = set.len();
        let price = fence * area;
        sum += price;
        let char = input_vec[*rep_y][*rep_x];
        println!(
            "{:?} field at {:?} has area: {:?} and fence: {:?} therefore price: {:?}",
            char as char,
            (rep_y, rep_x),
            area,
            fence,
            price
        );
    }
    println!("Fst: {}", sum)
}

fn parse_input() -> (
    HashMap<(usize, usize), HashSet<(usize, usize)>>,
    Vec<Vec<u8>>,
) {
    let mut sets = HashMap::<(usize, usize), HashSet<(usize, usize)>>::new();
    let mut representatives = HashMap::<(usize, usize), (usize, usize)>::new();
    let input = read_to_string(INPUT).unwrap_or_else(|_| panic!("error reading {INPUT}"));
    let input_vec: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_vec()).collect();

    for (y, line) in input_vec.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if y > 0 && &input_vec[y - 1][x] == char && x > 0 && &input_vec[y][x - 1] == char {
                let left_rep = *representatives.get(&(x - 1, y)).unwrap_or_else(|| {
                    panic!("there should be a representative for ({},{})", x - 1, y)
                });
                let above_rep = *representatives.get(&(x, y - 1)).unwrap_or_else(|| {
                    panic!("there should be a representative for ({},{})", x, y - 1)
                });
                if left_rep != above_rep {
                    let left_set = sets.remove(&left_rep).unwrap_or_else(|| {
                        panic!("there should be a set for left rep {:?}", left_rep)
                    });
                    let above_set = sets.get_mut(&above_rep).unwrap_or_else(|| {
                        panic!("there should be a set for left rep {:?}", above_rep)
                    });
                    for x in left_set {
                        above_set.insert(x);
                        representatives.insert(x, above_rep);
                    }
                    above_set.insert((x, y));
                    representatives.insert((x, y), above_rep);
                } else {
                    representatives.insert((x, y), above_rep);
                    let set = sets.get_mut(&above_rep).unwrap_or_else(|| {
                        panic!(
                            "there should be a set for ({},{}), and ({},{})",
                            x,
                            y - 1,
                            x - 1,
                            y
                        )
                    });
                    set.insert((x, y));
                }
                continue;
            }
            if y > 0 && &input_vec[y - 1][x] == char {
                let above_rep = representatives.get(&(x, y - 1)).unwrap_or_else(|| {
                    panic!("there should be a representative for ({},{})", x, y - 1)
                });
                let set = sets
                    .get_mut(above_rep)
                    .unwrap_or_else(|| panic!("there should be a set for {:?}", above_rep));
                set.insert((x, y));
                representatives.insert((x, y), *above_rep);
                continue;
            }
            if x > 0 && &input_vec[y][x - 1] == char {
                let left_rep = representatives.get(&(x - 1, y)).unwrap_or_else(|| {
                    panic!("there should be a representative for ({},{})", x - 1, y)
                });
                let set = sets
                    .get_mut(left_rep)
                    .unwrap_or_else(|| panic!("there should be a set for left_rep {:?}", left_rep));
                set.insert((x, y));
                representatives.insert((x, y), *left_rep);
                continue;
            }

            let mut set = HashSet::new();
            set.insert((x, y));
            representatives.insert((x, y), (x, y));
            sets.insert((x, y), set);
        }
    }
    (sets, input_vec)
}
