use std::io::Read;
use std::fs::File;

#[derive(Debug)]
struct Claim {
    id: u32,
    origin_x: u32,
    origin_y: u32,
    width: u32,
    height: u32,
    end_x: u32,
    end_y: u32,
}

const MAX_X: usize = 1000;
const MAX_Y: usize = 1000;


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

    // initialize matrix
    let matrix: [[i32; MAX_X]; MAX_Y] = [[-1; MAX_X]; MAX_Y];
    let mut res_matrix: [[i32; MAX_X]; MAX_Y] = [[-1; MAX_X]; MAX_Y];
    let mut pure_claim = 0; // claim that does not overlap
    let mut is_overlapping = false;

    let mut count: u32 = 1;
    for line in input.lines() {
        let claim_str: String = line.parse().expect("Unable to parse claim");
        let claim = parse_claim(count, &claim_str);
        println!("part1: found claim: {:?}, ", claim);
        count += 1;

        for (y, row) in matrix.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {

                //mark claim
                if (x as u32) >= claim.origin_x &&
                    (x as u32) < claim.end_x &&
                    (y as u32) >= claim.origin_y &&
                    (y as u32) < claim.end_y {
                    res_matrix[y][x] = if res_matrix[y][x] != -1 {
                        is_overlapping = true;
                        0 // 0 means multiple claims in one spot
                    } else {
                        claim.id as i32
                    }
                }
            }
        }

        // save if not overlapping
        if !is_overlapping {
            pure_claim = claim.id
        }
    }

    let mut overlaps = 0;
    // count 0s
    for (y, row) in res_matrix.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if res_matrix[y][x] == 0 {
                overlaps += 1;
            }
        }
    }

    println!("part1: overlaps: {}", overlaps);
    println!("part1: pure claim: {}", pure_claim);

}

fn parse_claim(id: u32, claim_str: &str) -> Claim {
    let claim_split: Vec<&str> = claim_str.split(" @ ").collect();
    let geometry: &str = claim_split.get(1).unwrap();
    let coordinates: Vec<&str> = geometry.split(": ").collect();
    let origins: Vec<&str> = coordinates.get(0)
        .unwrap()
        .split(",")
        .collect();
    let size: Vec<&str> = coordinates.get(1)
        .unwrap()
        .split("x")
        .collect();

    let origin_x = (origins.get(0).unwrap() as &str).parse::<u32>().unwrap();
    println!("origin_x is: {}", origin_x);
    let origin_y = (origins.get(1).unwrap() as &str).parse::<u32>().unwrap();
    println!("origin_y is: {}", origin_y);
    let width = (size.get(0).unwrap() as &str).parse::<u32>().unwrap();
    println!("width is: {}", width);
    let height = (size.get(1).unwrap() as &str).parse::<u32>().unwrap();
    println!("height is: {}", height);

    // calculate end x, y
    let end_x = origin_x + width;
    let end_y = origin_y + height;

    Claim { id, origin_x, origin_y, width, height, end_x, end_y }
}
