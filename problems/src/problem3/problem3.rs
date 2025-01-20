use std::fs;
use regex::Regex;



pub fn solve() -> i32 {
    const INPUT_FILE: &str ="/Users/siddhantmittal/Documents/AdventOfCode2024/problems/src/problem3/problem3.input";
    let file_data = fs::read_to_string(INPUT_FILE).expect("unable to read file");
    
    // println!("{}",file_data);
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut output = 0;
    for caps in re.captures_iter(&file_data) {
        let num1 = &caps[1];  // First number
        let num2 = &caps[2];  // Second number
        let (num1, num2): (i32, i32) = (num1.parse().expect("unable to parse"),num2.parse().expect("unable to parse"));
        output += num1*num2;
    }

    output
}