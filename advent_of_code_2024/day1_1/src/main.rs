use std::fs;

fn main() -> std::io::Result<()> {
    let content = fs::read_to_string("input.txt")?;
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
    set1.sort();
    set2.sort();
    let mut sum = 0;
    for i in 0..set1.len() {
        match set1[i] > set2[i] {
            true => sum += set1[i] - set2[i],
            false => sum += set2[i] - set1[i],
        }
    }
    println!("Sum: {}", sum);
    Ok(())
}
