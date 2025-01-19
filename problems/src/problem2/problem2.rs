use std::fs;


pub fn solve() -> i32{
    let mut safe = 0;
    const INPUT_FILE: &str = "./problem2.input";
    let file_data = fs::read_to_string(INPUT_FILE).expect("There was an error reading the file");

    for line in file_data.lines() {
        let numbers: Vec<i32> = line
        .split_whitespace()  // Split by whitespace
        .filter_map(|s| s.parse::<i32>().ok())  // Try to parse and filter invalid entries
        .collect();
        
        let direction = if numbers[1]-numbers[0] > 0 { 1 } else { -1 };
        safe += 1;
        for i in 1..numbers.len() {
            let diff = numbers[i]-numbers[i-1];
            let curr = if diff > 0 { 1 } else { -1 };
            if direction != curr || diff.abs()< 1 || diff.abs() > 3 {
                safe -= 1;
                break;
            }
        }
        
    }

    safe
    
}