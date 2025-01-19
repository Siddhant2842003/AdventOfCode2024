use std::fs;

pub fn solve() -> i32{
    let (mut list1, mut list2): (Vec<i32>, Vec<i32>) = (Vec::new(), Vec::new());
    const INPUT_FILE: &str = "./problem1.input";
    let contents = fs::read_to_string(INPUT_FILE).expect("There was an error reading the file");
    for line in contents.lines() {
        // println!("{}", i);
        let line_vec: Vec<&str> = line.split_whitespace().collect();
        let num1: i32= String::from(line_vec[0]).trim().parse().expect("unable to parse");
        list1.push(num1);
        let num2: i32= String::from(line_vec[1]).trim().parse().expect("unable to parse");
        list2.push(num2);
    }   
    list1.sort();
    list2.sort();

    let mut output = 0;
    for i in 0..list1.len(){
        output += (list2[i]-list1[i]).abs();
    }
    output
}