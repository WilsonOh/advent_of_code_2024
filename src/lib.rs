use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

pub fn read_lines() -> Result<std::io::Lines<BufReader<File>>> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}
