use std::collections::HashSet;
use std::fs;

fn check_rules(rules_to_apply: Vec<String>, page: &str) -> bool {
    for applying_rule in rules_to_apply {
        let split_rule = applying_rule.split("|").collect::<Vec<&str>>();
        let first_num = split_rule[0];
        let later_num = split_rule[1];
        for num in page.split(",") {
            if num == first_num {
                break;
            } else if num == later_num {
                return false;
            }
        }
    }
    true
}

fn apply_rules(rules_to_apply: Vec<String>, page: &str) -> String {
    let mut page_nums: Vec<&str> = page.split(",").collect();
    let mut made_changes = false;

    for applying_rule in rules_to_apply.clone() {
        let split_rule = applying_rule.split("|").collect::<Vec<&str>>();
        let first_num = split_rule[0];
        let later_num = split_rule[1];

        if let Some(first_pos) = page_nums.iter().position(|&x| x == first_num) {
            if let Some(later_pos) = page_nums.iter().position(|&x| x == later_num) {
                if later_pos < first_pos {
                    page_nums.swap(first_pos, later_pos);
                    made_changes = true;
                }
            }
        }
    }

    if made_changes && !check_rules(rules_to_apply.clone(), &page_nums.join(",")) {
        apply_rules(rules_to_apply, &page_nums.join(","))
    } else {
        page_nums.join(",")
    }
}

pub fn challenge() -> Result<(), Box<dyn std::error::Error>> {
    let contents =
        fs::read_to_string("../inputs/day5.txt").expect("Should have been able to read the file");

    let split_contents = contents.split("\n\n").collect::<Vec<&str>>();

    let mut rules_hash = HashSet::new();
    let rules = split_contents[0];
    let pages = split_contents[1];

    for rule in rules.split("\n") {
        rules_hash.insert(rule);
    }

    let mut good_pages: Vec<String> = Vec::new();
    let mut bad_pages: Vec<String> = Vec::new();

    for page in pages.split("\n") {
        let mut rules_to_apply: Vec<String> = Vec::new();
        let page_nums = page.split(",");
        for i in 0..page_nums.clone().count() {
            for j in i..page_nums.clone().count() {
                let search_term = page_nums.clone().nth(i).unwrap().to_owned()
                    + "|"
                    + page_nums.clone().nth(j).unwrap();
                if rules_hash.contains(&*search_term.clone()) {
                    rules_to_apply.push(search_term);
                }
                let search_term = page_nums.clone().nth(j).unwrap().to_owned()
                    + "|"
                    + page_nums.clone().nth(i).unwrap();
                if rules_hash.contains(&*search_term.clone()) {
                    rules_to_apply.push(search_term);
                }
            }
        }

        if check_rules(rules_to_apply.clone(), page) {
            good_pages.push(page.to_string())
        } else {
            let adjusted_page = apply_rules(rules_to_apply, page);
            bad_pages.push(adjusted_page)
        }
    }

    let mut running_sum = 0;

    for page in bad_pages {
        let page_nums = page.split(",").collect::<Vec<&str>>();
        let page_num_count = page_nums.len();
        let mid_num = page_nums[page_num_count / 2];
        if mid_num != "" {
            running_sum += mid_num.parse::<u32>().unwrap();
        }
    }

    println!("{running_sum}");

    Ok(())
}
