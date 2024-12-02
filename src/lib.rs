use std::{fs, i32};
use std::path::Path;

pub fn day_1_A(file_path : &str) -> (i32, Vec<i32>, Vec<i32>) {
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
    println!("{:?}", distance);
    return (distance, l1, l2);
}

pub fn day_1_B(l1: Vec<i32>, l2: Vec<i32>) -> i32 {
    return i32::MIN  
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn test_day1_A() {
        let (distance, _l1, _l2) = day_1_A("./data/day1.txt");
        assert!(distance == 2375403);
    }
    
    #[test]
    fn test_day1_B() {
        let (distance, l1, l2) = day_1_A("./data/day1.txt");

        assert!(distance == 2375403);
    }
}
