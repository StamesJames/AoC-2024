use std::{
    cmp::Ordering, collections::{HashMap, HashSet}, fs::read_to_string
};

fn main() {
    let input = read_to_string("input.txt").expect("File not read");
    let (rules, lists) = input.split_once("\r\n\r\n").expect("split not found");
    let rules: Vec<(usize, usize)> = rules
        .lines()
        .map(|r| r.split_once("|").unwrap())
        .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
        .collect();
    let lists: Vec<Vec<usize>> = lists
        .lines()
        .map(|l| l.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();
    let (_, righties) = make_leftie_rightie(&rules);
    let (printable_list, not_printable_list) = first(lists, &righties);
    println!(
        "Fst: {}",
        printable_list.iter().fold(0, |acc, l| acc + l[l.len() / 2])
    );
    let ordered_list = second(not_printable_list, &righties);
    println!(
        "Snd: {}",
        ordered_list.iter().fold(0, |acc, l| acc + l[l.len() / 2])
    );
}

fn second(mut not_printable_list: Vec<Vec<usize>>, righties: &HashMap<usize, HashSet<usize>>) -> Vec<Vec<usize>> {
    for list in not_printable_list.iter_mut(){
        list.sort_by(|a,b| {
            match righties.get(b) {
                Some(b_righties) => {
                    if b_righties.contains(a) {
                        Ordering::Less
                    }else {
                        Ordering::Equal
                    }
                },
                None => std::cmp::Ordering::Equal,
            }
        });
    }
    return not_printable_list;
}

fn first(
    lists: Vec<Vec<usize>>,
    righties: &HashMap<usize, HashSet<usize>>,
) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut printable_lists = vec![];
    let mut not_printable_lists = vec![];
    for list in lists {
        if list.is_sorted_by(|a,b| {
            match righties.get(b) {
                Some(b_righties) => {
                    if b_righties.contains(a) {
                        false
                    }else {
                        true
                    }
                },
                None => true,
            }
        }) {
            printable_lists.push(list);
        }else{
            not_printable_lists.push(list);
        }
    }
    return (printable_lists, not_printable_lists);
}

fn make_leftie_rightie(
    rules: &Vec<(usize, usize)>,
) -> (
    HashMap<usize, HashSet<usize>>,
    HashMap<usize, HashSet<usize>>,
) {
    let mut lefties: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut righties: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (left, right) in rules.iter() {
        righties.insert(*left, HashSet::new());
        righties.insert(*right, HashSet::new());
        lefties.insert(*left, HashSet::new());
        lefties.insert(*right, HashSet::new());
    }
    for (left, right) in rules {
        let righties_of_left = righties.get_mut(&left).unwrap();
        righties_of_left.insert(*right);
        let lefties_of_right = lefties.get_mut(&right).unwrap();
        lefties_of_right.insert(*left);
    }

    return (lefties, righties);
}
