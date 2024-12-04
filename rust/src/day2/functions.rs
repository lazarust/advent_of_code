use std::fs;

fn check_safe(nums: std::str::Split<'_, &str>) -> u32 {
    let mut is_increasing = 0;
    let mut is_decreasing = 0;
    let mut previous_num = 0;
    for num in nums {
        let num2 = num.parse::<u32>().unwrap();
        if previous_num == 0 {
            previous_num = num2;
        } else {
            if num2 == previous_num {
                return 0;
            }
            if num2 > previous_num && is_decreasing == 1 {
                return 0;
            }
            if num2 < previous_num && is_increasing == 1 {
                return 0;
            }
            if num2 > previous_num
                && is_decreasing == 0
                && num2 - previous_num >= 1
                && num2 - previous_num <= 3
            {
                is_increasing = 1;
                previous_num = num2;
            } else if num2 < previous_num
                && is_increasing == 0
                && previous_num - num2 >= 1
                && previous_num - num2 <= 3
            {
                is_decreasing = 1;
                previous_num = num2;
            } else {
                return 0;
            }
        }
    }
    if is_increasing != 0 || is_decreasing != 0 {
        return 1;
    }
    return 0;
}

pub fn challenge() {
    let contents =
        fs::read_to_string("../inputs/day2.txt").expect("Should have been able to read the file");

    let rows = contents.split("\n");
    let mut count_safe = 0;
    for row in rows {
        let nums = row.split(" ");
        if nums.clone().count() > 1 {
            let safety = check_safe(nums);
            count_safe += safety;
        }
    }

    println!("SAFE: {}", count_safe);
}
