use std::{fs, i32};
use std::path::Path;
use std::collections::HashMap;
pub fn a(file_path : &str) -> (i32, Vec<i32>, Vec<i32>) {
    //let current_dir = env::current_dir().expect("Failed to get current directory");
    //println!("Current directory: {:?}", current_dir);
    //Read in the filepath

    let path: &Path = Path::new(file_path);
    //read the file from the upper directory
    let data: String = fs::read_to_string(path).expect("Should have been able to read the {file_path} file");

    let mut l1: Vec<i32> = Vec::new();
    let mut l2: Vec<i32> = Vec::new();

    // for line in string add the first number to l1 and the second number to l2
    for line in data.lines() {
        let values: Vec<&str>= line.split_whitespace().collect();
        assert!(values.len() == 2);
        let num1: i32 = values[0].parse().unwrap();
        let num2: i32 = values[1].parse().unwrap();
        l1.push(num1);
        l2.push(num2);
    }

    assert!(l1.len() == l2.len());
    l1.sort();
    l2.sort();

    let mut distance: i32 = 0;
    for (num1, num2) in l1.iter().zip(l2.iter()) {
        distance = distance + i32::abs(num1-num2);
    }
    //println!("{:?}", distance);
    return (distance, l1, l2);
}

pub fn b(l1: &Vec<i32>, l2: &Vec<i32>) -> i32 {
    //let mut h1: HashMap<i32,i32> = HashMap::new();
    let mut h2: HashMap<i32,i32> = HashMap::new();
    //for &i in &l1 {
    //    *h1.entry(i).or_insert(0) += 1;
    //}
    for &i in l2 {
        *h2.entry(i).or_insert(0) += 1;
    }
    let mut scores: Vec<i32> = Vec::new();
    for &i in l1 {
        if h2.contains_key(&i) {
            scores.push(&i * h2.get(&i).unwrap());
        }
    }
    //println!("{:?}", &l2);

    return scores.iter().sum();
}