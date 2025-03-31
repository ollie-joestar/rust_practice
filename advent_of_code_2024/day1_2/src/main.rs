use std::fs;

fn main() {
    let content = match fs::read_to_string("input.txt") {
        Ok(content) => content,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    let lines: Vec<String> = content.lines().map(String::from).collect();
    let mut set1: Vec<i32> = Vec::new();
    let mut set2: Vec<i32> = Vec::new();

    for line in lines.iter() {
        let num: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        set1.push(num[0]);
        set2.push(num[1]);
    }
    let mut sum = 0;
    for i in 0..set1.len() {
        for j in 0..set2.len() {
            let mut multiplier = 0;
            match set1[i] == set2[j] {
                true => multiplier += 1,
                false => multiplier += 0,
            }
            sum += set1[i] * multiplier;
        }
    }
    println!("Sum: {}", sum);
}
