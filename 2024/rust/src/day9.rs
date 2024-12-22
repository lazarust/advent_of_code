use std::fs;

fn convert_string(contents: String) -> String {
    let mut num = 0;
    let mut converted_string = "".to_owned();
    let mut is_free = false;

    for c in contents.chars() {
        if c.is_digit(10) == false {
            break;
        }
        if is_free == false {
            for _ in 0..c.to_digit(10).unwrap() {
                converted_string.push_str(&num.to_string());
            }
            is_free = true;
            num += 1;
            if num == 10 {
                num = 0;
            }
        } else {
            for _ in 0..c.to_digit(10).unwrap() {
                converted_string.push_str(".");
            }
            is_free = false;
        }
    }

    // println!("{}", converted_string);
    converted_string
}

fn rearrange_string(converted_string: String) -> String {
    let mut rearranged_string = converted_string.to_owned();
    for c in converted_string.chars().rev() {
        if rearranged_string.contains(".") == false {
            break;
        }
        rearranged_string = rearranged_string.replacen(".", &c.to_string(), 1);
        rearranged_string.pop();
    }
    // println!("{}", rearranged_string);
    rearranged_string
}

fn calculate_checksum(rearranged_string: String) {
    let mut checksum = 0 as u64;

    for (index, c) in rearranged_string.char_indices() {
        checksum += c.to_digit(10).unwrap() as u64 * index as u64
    }

    println!("{}", checksum);
}

pub fn challenge() {
    let contents =
        fs::read_to_string("../inputs/day9.txt").expect("Should have been able to read the file");

    let converted_string = convert_string(contents);
    let rearranged_string = rearrange_string(converted_string);

    calculate_checksum(rearranged_string);
}
