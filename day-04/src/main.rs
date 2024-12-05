use std::{char, fs::read_to_string};

fn main() {
    let fst = first();
    println!("Fst: {fst}");
    let snd = second();
    println!("Snd: {snd}");
}

fn second() -> i32 {
    let mut count = 0;
    let input= read_to_string("input.txt").unwrap();
    let input = input.as_bytes();
    let mut i = 143;
    while i < input.len() - 142 {
        if input[i] == b'A' {
            let left = (input[i-143] == b'M' && input[i+143] == b'S') || (input[i-143] == b'S' && input[i+143] == b'M');
            let right = (input[i-141] == b'M' && input[i+141] == b'S') || (input[i-141] == b'S' && input[i+141] == b'M');
            if left && right {
                count += 1;
            }
        }
        i += 1;
    }
    return count;
}

fn first() -> i32 {
    let mut count = 0;
    let input= read_to_string("input.txt").unwrap();

    let mut sm = StateMachine {
        state: ReadState::NONE,
    };
    // lines
    for line in input.lines() {
        sm.reset();
        for char in line.chars() {
            if sm.read_char(char) {
                count += 1;
            }
        }

    }
    // rows
    let input:Vec<char> = input.chars().collect();
    for c in 0..140{
        let mut i = c;
        sm.reset();
        while i < input.len() {
            if sm.read_char(input[i]) {
                count += 1;
            }
            // one line is 140 chars long plus 2 chars for the carriage return
            i += 142;
        }
    }
    // horizontal diagonals to the right
    for c in 0..140{
        let mut i = c;
        sm.reset();
        while i < input.len() && input[i] != '\n' && input[i] != '\r' {
            if sm.read_char(input[i]) {
                count += 1;
            }
            // one line is 140 chars long plus 2 chars for the carriage return so 143 gives the right diagonal
            i += 143;
        }
    }
    // horizontal diagonals to the left
    for c in 0..140{
        let mut i = c;
        sm.reset();
        while i < input.len() && input[i] != '\n' && input[i] != '\r' {
            if sm.read_char(input[i]) {
                count += 1;
            }
            // one line is 140 chars long plus 2 chars for the carriage return so 141 gives the left diagonal
            i += 141;
        }
    }
    // vertical diagonals to the right
    // r is the begining of a new row
    let mut r = 142;
    while r < input.len(){
        let mut i = r;
        sm.reset();
        while i < input.len() && input[i] != '\n' && input[i] != '\r' {
            if sm.read_char(input[i]) {
                count += 1;
            }
            // one line is 140 chars long plus 2 chars for the carriage return so 141 gives the left diagonal
            i += 143;
        }
        r += 142;
    }
    // vertical diagonals to the left
    // r is the begining of a new row from the right 
    let mut r = 142 + 139;
    while r < input.len(){
        let mut i = r;
        sm.reset();
        while i < input.len() && input[i] != '\n' && input[i] != '\r' {
            if sm.read_char(input[i]) {
                count += 1;
            }
            // one line is 140 chars long plus 2 chars for the carriage return so 141 gives the left diagonal
            i += 141;
        }
        r += 142;
    }
    return count;
}

enum ReadState {
    NONE,
    X,
    XM,
    XMA,
    XMAS,
    S,
    SA,
    SAM,
    SAMX,
}

struct StateMachine {
    state: ReadState,
}

impl StateMachine {
    fn read_char(&mut self, c: char) -> bool {
        match self.state {
            ReadState::NONE => match c {
                'X' => self.state = ReadState::X,
                'S' => self.state = ReadState::S,
                _ => self.state = ReadState::NONE,
            },
            ReadState::X => match c {
                'M' => self.state = ReadState::XM,
                'X' => self.state = ReadState::X,
                'S' => self.state = ReadState::S,
                _ => self.state = ReadState::NONE,
            },
            ReadState::XM => match c {
                'A' => self.state = ReadState::XMA,
                'X' => self.state = ReadState::X,
                'S' => self.state = ReadState::S,
                _ => self.state = ReadState::NONE,
            },
            ReadState::XMA => match c {
                'S' => {
                    self.state = ReadState::XMAS;
                    return true;
                }
                'X' => self.state = ReadState::X,
                _ => self.state = ReadState::NONE,
            },
            ReadState::XMAS => match c {
                'A' => self.state = ReadState::SA,
                'X' => self.state = ReadState::X,
                'S' => self.state = ReadState::S,
                _ => self.state = ReadState::NONE,
            },
            ReadState::S => match c {
                'A' => self.state = ReadState::SA,
                'X' => self.state = ReadState::X,
                'S' => self.state = ReadState::S,
                _ => self.state = ReadState::NONE,
            },
            ReadState::SA => match c {
                'M' => self.state = ReadState::SAM,
                'X' => self.state = ReadState::X,
                'S' => self.state = ReadState::S,
                _ => self.state = ReadState::NONE,
            },
            ReadState::SAM => match c {
                'X' => {
                    self.state = ReadState::SAMX;
                    return true;
                }
                'S' => self.state = ReadState::S,
                _ => self.state = ReadState::NONE,
            },
            ReadState::SAMX => match c {
                'X' => self.state = ReadState::X,
                'S' => self.state = ReadState::S,
                'M' => self.state = ReadState::XM,
                _ => self.state = ReadState::NONE,
            },
        }
        return false;
    }
    fn reset(&mut self){
        self.state = ReadState::NONE;
    }
}
