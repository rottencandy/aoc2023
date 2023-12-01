use aoc23::day1;
use std::{collections::HashMap, io};

fn main() {
    let table = HashMap::from([(1, day1::run)]);

    println!("Which day? ");
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read input");
    let problem = num.trim().parse::<i32>().unwrap_or(table.len() as i32);
    table
        .get(&problem)
        .expect("Solution might not be implemented")();
}
