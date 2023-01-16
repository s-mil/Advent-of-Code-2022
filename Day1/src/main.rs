use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::num::ParseIntError;

fn main() {
    let numbers: Vec<i64> = Vec::new();

    let file = File::open("input").expect("Didnt find file");

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let num = line.parse::<i32>(:?);
    }

}
