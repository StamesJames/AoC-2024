use std::fs::read_to_string;

use regex::Regex;

const INPUT: &str = "input.txt";

#[derive(Debug)]
struct Machine {
    a_button: (i64, i64),
    b_button: (i64, i64),
    price: (i64, i64),
}

impl Machine {
    fn solve(&self) -> Option<(i64, i64)> {
        let det = self.a_button.0 * self.b_button.1 - self.b_button.0 * self.a_button.1;
        let mut a = 0;
        let mut b = 0;
        if det != 0 {
            a = self.b_button.1 * self.price.0 - self.b_button.0 * self.price.1;
            if a % det != 0 {
                return None;
            }
            a = a / det;
            b = -self.a_button.1 * self.price.0 + self.a_button.0 * self.price.1;
            if b % det != 0{
                return None;
            }
            b =  b/ det;
        }
        Some((a, b))
    }
    fn solve_snd(&self) -> Option<(i64, i64)> {
        let det = self.a_button.0 * self.b_button.1 - self.b_button.0 * self.a_button.1;
        let mut a = 0;
        let mut b = 0;
        if det != 0 {
            a = self.b_button.1 * (self.price.0 + 10000000000000) - self.b_button.0 * (self.price.1 + 10000000000000);
            if a % det != 0 {
                return None;
            }
            a = a / det;
            b = -self.a_button.1 * (self.price.0 + 10000000000000) + self.a_button.0 * (self.price.1 + 10000000000000);
            if b % det != 0{
                return None;
            }
            b =  b/ det;
        }
        Some((a, b))
    }
}

fn main() {
    let machines = parse_input(INPUT);
    // println!("{:?}", machines);
    first(&machines);
    second(&machines);
}

fn second(machines: &Vec<Machine>) {
    let mut sum = 0;
    for machine in machines {
        if let Some(sol) = machine.solve_snd(){

            if sol.0 >= 0 && sol.1 >= 0 {
                sum += sol.0 * 3 + sol.1;
            } else {
                if machine.price.0 % machine.a_button.0 == 0
                && machine.price.1 % machine.a_button.1 == 0
                && machine.price.0 % machine.b_button.0 == 0
                && machine.price.1 % machine.b_button.1 == 0
                {
                    println!("inf sols");
                }
            }
        }
    }
    println!("Fst: {sum}");
}
fn first(machines: &Vec<Machine>) {
    let mut sum = 0;
    for machine in machines {
        if let Some(sol) = machine.solve(){

            if sol.0 >= 0 && sol.0 <= 100 && sol.1 >= 0 && sol.1 <= 100 {
                sum += sol.0 * 3 + sol.1;
            } else {
                if machine.price.0 % machine.a_button.0 == 0
                && machine.price.1 % machine.a_button.1 == 0
                && machine.price.0 % machine.b_button.0 == 0
                && machine.price.1 % machine.b_button.1 == 0
                {
                    println!("inf sols");
                }
            }
        }
    }
    println!("Fst: {sum}");
}

fn parse_input(path: &str) -> Vec<Machine> {
    let input = read_to_string(path).unwrap_or_else(|_| panic!("error reading {path}"));
    let re = Regex::new(
        r"(?m)Button A: X\+(\d+), Y\+(\d+)\r?\nButton B: X\+(\d+), Y\+(\d+)\r?\nPrize: X=(\d+), Y=(\d+)"
    )
    .unwrap();
    let mut machines = vec![];
    for (_, [x_a, y_a, x_b, y_b, x_p, y_p]) in re.captures_iter(&input).map(|c| c.extract()) {
        machines.push(Machine {
            a_button: (x_a.parse().unwrap(), y_a.parse().unwrap()),
            b_button: (x_b.parse().unwrap(), y_b.parse().unwrap()),
            price: (x_p.parse().unwrap(), y_p.parse().unwrap()),
        });
    }
    return machines;
}
