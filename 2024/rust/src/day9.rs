use std::fs;

fn convert_string(contents: String) -> Vec<i32> {
    let mut num = 0;
    let mut converted_vector: Vec<i32> = Vec::new();
    let mut is_free = false;

    for c in contents.chars() {
        if c.is_digit(10) == false {
            break;
        }
        if is_free == false {
            for _ in 0..c.to_digit(10).unwrap() {
                converted_vector.push(num as i32);
            }
            is_free = true;
            num += 1;
        } else {
            for _ in 0..c.to_digit(10).unwrap() {
                converted_vector.push(-1);
            }
            is_free = false;
        }
    }

    converted_vector
}

fn rearrange_string(converted_vector: Vec<i32>) -> Vec<i32> {
    let mut rearranged_vector = converted_vector.clone();
    let mut pos = 0;

    for i in (0..rearranged_vector.len()).rev() {
        if rearranged_vector[i] != -1 {
            while pos < rearranged_vector.len() && rearranged_vector[pos] != -1 {
                pos += 1;
            }
            if pos < rearranged_vector.len() {
                rearranged_vector.swap(i, pos);
                rearranged_vector.pop();
                pos += 1;
            } else {
                break;
            }
        }
    }

    rearranged_vector.truncate(pos);
    rearranged_vector
}

fn calculate_checksum(rearranged_vector: Vec<i32>) {
    let mut checksum = 0 as u64;
    let mut count_i = 0;

    for (index, &c) in rearranged_vector.iter().enumerate() {
        if c != -1 {
            checksum += c as u64 * (index as u64 - count_i);
        } else {
            count_i += 1
        }
    }

    println!("{}", checksum);
}

pub fn challenge() {
    let contents =
        fs::read_to_string("../inputs/day9.txt").expect("Should have been able to read the file");

    let converted_vector = convert_string(contents);
    let rearranged_vector = rearrange_string(converted_vector);
    calculate_checksum(rearranged_vector);
}
