use std::{clone, fs::read_to_string, vec};

const INPUT: &str = "input.txt";

fn main() {
    first();
    let (files, mut blocks) = parse_input_second(INPUT);
    let mut i = blocks.len() - 1;
    if blocks[i].files.is_empty() {
        i -= 1;
    }
    while i > 0 {
        let next_block = &mut blocks[i];
        if next_block.files.len() != 1 {
            panic!("something wrong with files");
        }
        next_block.free_space = next_block.size;
        let next_file = next_block.files.pop().unwrap();
        for block in &mut blocks[0..i+1]{
            if block.insert(&next_file) {
                break;
            }
        }
        i -= 2;
    }
    let mut i = 0;
    let mut checksum = 0;
    for Block { free_space, files, .. } in blocks {
        for file in files {
            let n = i + file.size - 1;
            let size_gaus = (n * n + n) / 2;
            let i_gaus;
            if i == 0 {
                i_gaus = 0;
            } else {
                let i = i - 1;
                i_gaus = (i * i + i) / 2;
            }
            let sum = size_gaus - i_gaus;
            checksum += sum * file.i;
            i += file.size;
        }
        i += free_space;
    }
    println!("checksum: {checksum}");
}

#[derive(Clone)]
struct File {
    size: usize,
    i: usize,
}
struct Block {
    size: usize,
    free_space: usize,
    files: Vec<File>,
}
impl Block {
    fn insert(&mut self, file: &File) -> bool {
        if self.free_space >= file.size {
            self.free_space -= file.size;
            self.files.push(file.clone());
            true
        } else {
            false
        }
    }
}

fn parse_input_second(path: &str) -> (Vec<File>, Vec<Block>)  {
    let input = read_to_string(path).expect("error reading file");
    let mut is_file = true;
    let mut blocks = vec![];
    let mut files = vec![];
    let mut i = 0;
    for line in input.lines() {
        for char in line.chars() {
            let size: usize = char
                .to_digit(10)
                .expect("non digit found")
                .try_into()
                .unwrap();
            let new_block;
            if is_file {
                let file = File { size, i };
                new_block = Block {
                    free_space: 0,
                    files: vec![file.clone()],
                    size,
                };
                files.push(file);
                i += 1;
            }else {
                new_block = Block{
                    free_space:size,
                    files: vec![],
                    size
                };
            }
            blocks.push(new_block);
            is_file = !is_file;
        }
    }
    return (files, blocks);
}

fn first() {
    let (mut files, empties) = parse_input(INPUT);
    println!("files: {:?}", files);
    println!("empties: {:?}", empties);
    let mut result_files = vec![];
    let mut i = 0;
    let mut j = files.len() - 1;
    'outer: loop {
        result_files.push((i, files[i]));
        if i >= j {
            break;
        }
        let mut empty_space = empties[i];
        while empty_space > 0 {
            if files[j] > empty_space {
                result_files.push((j, empty_space));
                files[j] -= empty_space;
                break;
            } else {
                result_files.push((j, files[j]));
                empty_space -= files[j];
                files[j] = 0;
                j -= 1;
                if i >= j {
                    break 'outer;
                }
            }
        }
        i += 1;
    }
    println!("{:?}", result_files);
    let mut i = 0;
    let mut checksum = 0;
    for (x, count) in result_files {
        let n = i + count - 1;
        let count_gaus = (n * n + n) / 2;
        let i_gaus;
        if i == 0 {
            i_gaus = 0;
        } else {
            let i = i - 1;
            i_gaus = (i * i + i) / 2;
        }
        let sum = count_gaus - i_gaus;
        checksum += sum * x;
        i += count;
    }
    println!("checksum: {checksum}");
}

fn parse_input(path: &str) -> (Vec<usize>, Vec<usize>) {
    let input = read_to_string(path).expect("error reading file");
    let mut is_file = true;
    let mut files_vec = vec![];
    let mut empty_vec = vec![];
    for line in input.lines() {
        for char in line.chars() {
            let char_parsed: usize = char
                .to_digit(10)
                .expect("non digit found")
                .try_into()
                .unwrap();
            if is_file {
                files_vec.push(char_parsed);
            } else {
                empty_vec.push(char_parsed);
            }
            is_file = !is_file;
        }
    }
    return (files_vec, empty_vec);
}
