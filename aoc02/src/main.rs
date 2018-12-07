use std::io::Read;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let filename = String::from("input/input.txt");
    println!("main: reading file {}", filename);

    let mut f = File::open(filename).expect("Error: file not found");

    let mut contents = String::new();
    let size = f.read_to_string(&mut contents).expect("Error: Something went wrong reading the file");
    println!("main: size of text is {} bytes", size);

    part1(&contents);
}

fn part1(input: &str) {
    let mut doubles = 0;
    let mut triples = 0;

    for line in input.lines() {
        let mut seen = HashMap::new();
        let mut has_double = false;
        let mut has_triple = false;
        let id: String = line.parse().expect("Unable to parse id");
        for c in id.chars() {
            if seen.contains_key(&c) {
                let mut count = *seen.get(&c).unwrap();
                count += 1;
                seen.insert(c, count);
            } else {
                seen.insert(c, 1);
            }
        }

        for (_ , v) in seen.iter() {
            match *v
                {
                    2 => has_double = true,
                    3 => has_triple = true,
                    _ => ()
                }
        }

        if has_double { doubles += 1; }
        if has_triple { triples += 1; }
    }

    println!("Found doubles: {}", doubles);
    println!("Found triples: {}", triples);
    println!("Checksum: {}", doubles * triples)
}
