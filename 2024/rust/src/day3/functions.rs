use regex::Regex;
use std::fs;

pub fn challenge() -> Result<(), Box<dyn std::error::Error>> {
    let contents =
        fs::read_to_string("../inputs/day3.txt").expect("Should have been able to read the file");

    let re = Regex::new(r"(mul\(\d{1,},\d{1,}\)|do\(\)|don't\(\))").unwrap();
    let mut results = vec![];
    for (_, [data]) in re.captures_iter(&contents).map(|c| c.extract()) {
        println!("{}", data);
        results.push(data);
    }

    let mut enabled = true;
    let mut running_total = 0;
    for result in results {
        println!("{}", result);
        if result == "don't()" {
            enabled = false;
        } else if result == "do()" {
            enabled = true;
        } else if enabled == true {
            let re2 = Regex::new(r"(\d{1,})").unwrap();
            let mut results2 = vec![];
            for (_, [data2]) in re2.captures_iter(result).map(|d| d.extract()) {
                println!("{}", data2);
                results2.push(data2.parse::<u64>()?);
            }
            running_total += results2[0] * results2[1];
        }
    }

    println!("Total: {}", running_total);

    Ok(())
}
