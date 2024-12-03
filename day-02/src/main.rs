use std::fs::read_to_string;

fn main() {
    let fst = first();
    println!("First Solution: {:?}", fst);
    let snd = second();
    println!("Second Solution: {:?}", snd);
}

fn second()->(i32, i32){
    let input = read_to_string("input.txt").unwrap();
    let mut safe_count = 0;
    let mut unsafe_count = 0;
    for line in input.lines() {
        let nums : Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        if check_safety_vec(&nums, None, true){
            safe_count += 1;
        } else {
            unsafe_count += 1;
        }
    }

    return (safe_count, unsafe_count);
}

fn first()->(i32, i32){
    let input = read_to_string("input.txt").unwrap();
    let mut safe_count = 0;
    let mut unsafe_count = 0;
    for line in input.lines() {
        let nums : Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        if check_safety_vec(&nums, None, false){
            safe_count += 1;
        } else {
            unsafe_count += 1;
        }
    }

    return (safe_count, unsafe_count);
}

fn check_safety(first_val:i32, second_val:i32, increasing:bool)->bool {
    first_val.abs_diff(second_val) <= 3 &&
    (
        first_val < second_val && increasing ||
        first_val > second_val && !increasing
    )
}

fn check_increasing(nums:&[i32],skip:Option<usize>)->bool{
    let mut first_val= 0;
    let mut second_val= 1;
    if let Some(skip) = skip{
        if skip == 0{
            first_val = 1;
            second_val = 2;
        } else if skip == 1{
            second_val = 2;
        }
    }
    return nums[first_val] < nums[second_val];
}

fn check_safety_vec(nums:&[i32], skip:Option<usize>, can_skip:bool)->bool{
    let mut last_val:Option<i32> = None;
    let increasing = check_increasing(nums, skip);
    for (i, num) in nums.iter().enumerate(){
        if let Some(skip) = skip {
            if skip == i{
                continue;
            }
        }
        if let Some(last_val) = last_val{
            if !check_safety(last_val, *num, increasing){
                if can_skip && skip.is_none(){
                    if check_safety_vec(nums, Some(i), false) {
                        return true;
                    } else if check_safety_vec(nums, Some(i-1), false) {
                        return true;
                    } else if check_safety_vec(nums, Some(0), false){
                        return true;
                    }
                }
                return false;
            }
        }
        last_val = Some(*num);
    }
    return true;
}