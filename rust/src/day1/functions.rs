use csv::Reader;
use serde::Deserialize;
use std::collections::{HashMap, VecDeque};
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Data {
    list_1: String,
    list_2: String,
}

pub fn challenge() -> Result<(), Box<dyn std::error::Error>> {
    let path = "../inputs/day1.csv";
    let file = File::open(path)?;
    let mut reader = Reader::from_reader(file);

    let mut list1 = VecDeque::new();
    let mut list2 = VecDeque::new();

    for result in reader.deserialize() {
        let record: Data = result?;
        list1.push_back(record.list_1);
        list2.push_back(record.list_2);
    }

    let mut vec1: Vec<String> = list1.into_iter().collect();
    let mut vec2: Vec<String> = list2.into_iter().collect();

    vec1.sort();
    vec2.sort();

    let mut total_difference = 0;
    for i in 0..vec1.len() {
        if let Some(val1) = vec1.get(i).and_then(|s| s.parse::<i32>().ok()) {
            if let Some(val2) = vec2.get(i).and_then(|s| s.parse::<i32>().ok()) {
                total_difference += (val1 - val2).abs();
            }
        }
    }

    println!("Total Difference: {}", total_difference);

    let mut hashmap = HashMap::new();
    for item in vec2 {
        if let Ok(number) = item.parse::<i32>() {
            *hashmap.entry(number).or_insert(0) += 1;
        }
    }

    let mut similarity_score = 0;
    for i in 0..vec1.len() {
        if let Some(val1) = vec1[i].parse::<i32>().ok() {
            if let Some(count) = hashmap.get(&val1).cloned() {
                similarity_score += val1 * count;
            }
        }
    }

    println!("Similarity Score: {}", similarity_score);

    Ok(())
}
