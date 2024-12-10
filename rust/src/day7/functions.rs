use std::fs;

fn evaluate_expression(ints: &[i128], operators: &[String]) -> Option<i128> {
    let mut stack = Vec::<i128>::new();

    for (index, &num) in ints.iter().enumerate() {
        if index > 0 {
            let op = operators[index - 1].clone();
            match stack.pop() {
                Some(a) => {
                    if op == '+'.to_string() {
                        stack.push(a + num);
                    } else if op == '*'.to_string() {
                        stack.push(a * num);
                    } else if op == "||".to_string() {
                        let new_string = a.to_string() + &num.to_string();
                        stack.push(
                            new_string
                                .parse()
                                .map_err(|e| format!("Failed to parse integer: {}", e))
                                .ok()?,
                        );
                    }
                }
                None => return None,
            }
        } else {
            stack.push(num);
        }
    }

    Some(*stack.last().unwrap())
}

fn generate_operator_combinations(
    ints_len: usize,
    current_combination: &mut Vec<String>,
    combinations: &mut Vec<Vec<String>>,
) {
    if current_combination.len() == ints_len - 1 {
        combinations.push(current_combination.clone());
        return;
    }

    for op in ['+'.to_string(), '*'.to_string(), "||".to_string()] {
        current_combination.push(op);
        generate_operator_combinations(ints_len, current_combination, combinations);
        current_combination.pop();
    }
}

pub fn challenge() -> Result<(), Box<dyn std::error::Error>> {
    let contents =
        fs::read_to_string("../inputs/day7.txt").expect("Should have been able to read the file");

    let mut valid_equations = 0;
    let mut sum = 0;

    for line in contents.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            eprintln!("Invalid line format: {}", line);
            continue;
        }

        let desired_total: i128 = parts[0]
            .parse()
            .map_err(|e| format!("Failed to parse desired total: {}", e))?;
        let ints_as_strings: Vec<&str> = parts[1].split(' ').collect();
        let mut ints: Vec<i128> = Vec::new();
        for int in ints_as_strings {
            if !int.is_empty() {
                let parsed_int: i128 = int
                    .parse()
                    .map_err(|e| format!("Failed to parse integer: {}", e))?;
                ints.push(parsed_int);
            }
        }

        let ints_len = ints.len();

        let mut combinations = Vec::new();
        generate_operator_combinations(ints_len, &mut Vec::new(), &mut combinations);

        for ops in combinations {
            let expression_sum = evaluate_expression(&ints, &ops);
            if expression_sum == Some(desired_total) {
                valid_equations += 1;
                sum += expression_sum.unwrap();
                break; // Stop at the first valid combination
            }
        }
    }

    println!("Total valid equations: {}", valid_equations);
    println!("Sum: {}", sum);
    Ok(())
}
