use std::fs;

fn check_safe(nums: std::str::Split<'_, &str>) -> i32 {
    let nums_vec: Vec<i32> = nums.flat_map(|num| num.parse().ok()).collect();

    for i in 0..nums_vec.len() {
        let mut new_nums = nums_vec.clone();
        new_nums.remove(i);
        if check_single_sequence(&new_nums) {
            return 1;
        }
    }

    0
}

fn check_single_sequence(nums: &[i32]) -> bool {
    if nums.is_empty() || nums.len() == 1 {
        return true;
    }

    let mut is_increasing = false;
    let mut is_decreasing = false;

    for i in 1..nums.len() {
        if (nums[i] - nums[i - 1]).abs() != 1
            && (nums[i] - nums[i - 1]).abs() != 2
            && (nums[i] - nums[i - 1]).abs() != 3
        {
            return false;
        }

        if nums[i] == nums[i - 1] {
            return false;
        }

        if nums[i] > nums[i - 1] {
            if is_decreasing {
                return false;
            }
            is_increasing = true;
        } else if nums[i] < nums[i - 1] {
            if is_increasing {
                return false;
            }
            is_decreasing = true;
        }
    }

    is_increasing || is_decreasing
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
