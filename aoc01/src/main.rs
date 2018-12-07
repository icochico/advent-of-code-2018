use std::io::{self, Read, Write};
use std::fs::File;
use std::collections::HashSet;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;


fn main() -> Result<()> {

    let filename = String::from("input/input.txt");
    println!("main: reading file {}", filename);

    let mut f = File::open(filename).expect("Error: file not found");

    let mut contents = String::new();
    let size = f.read_to_string(&mut contents).expect("Error: Something went wrong reading the file");
    println!("main: size of text is {} bytes", size);

    part1(&contents)?;
    part2(&contents)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut freq = 0;

    for line in input.lines() {
        let change: i32 = line.parse()?;
        freq += change;
    }

    writeln!(io::stdout(), "part1: frequency is {}", freq)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut freq = 0;
    let mut seen = HashSet::new();
    seen.insert(0);

    loop {
        for line in input.lines() {
            let change: i32 = line.parse()?;
            freq += change;

            // check if frequency has been seen already
            if seen.contains(&freq) {
                writeln!(io::stdout(), "part2: first frequency seen twice is {}", freq)?;
                return Ok(());
            }
            seen.insert(freq);
        }
    }
}

