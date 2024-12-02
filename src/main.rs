use std::{fs::File, io::Read, path::Path};

fn main() {
    let path = Path::new("/home/mirage/dev/historian_hysteria/src/puzzle_input.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {
            let puzzle_one_solution = solve_puzzle_one(&s);
            let puzzle_two_solution = solve_puzzle_two(&s);
            println!("Puzzle one solution: {}", puzzle_one_solution);
            println!("Puzzle two solution: {}", puzzle_two_solution);
        }
    }
}

fn solve_puzzle_one(input: &str) -> i32 {
    let mut distance_sum = 0;

    let lines = input.lines();
    let mut left_pair: Vec<i32> = Vec::new();
    let mut right_pair: Vec<i32> = Vec::new();
    for line in lines {
        let mut numbers = line.split_whitespace();
        let left_number = numbers.next().unwrap().parse::<i32>().unwrap();
        let right_number = numbers.next().unwrap().parse::<i32>().unwrap();
        left_pair.push(left_number);
        right_pair.push(right_number);
    }

    // Sort both pairsd
    left_pair.sort();
    right_pair.sort();

    // Calculate the distance between the two pairs
    for i in 0..left_pair.len() {
        distance_sum += (left_pair[i] - right_pair[i]).abs();
    }

    distance_sum
}

fn solve_puzzle_two(input: &str) -> i32 {
    let mut similarity_score = 0;

    let lines = input.lines();
    let mut left_pair: Vec<i32> = Vec::new();
    let mut right_pair: Vec<i32> = Vec::new();
    for line in lines {
        let mut numbers = line.split_whitespace();
        let left_number = numbers.next().unwrap().parse::<i32>().unwrap();
        let right_number = numbers.next().unwrap().parse::<i32>().unwrap();
        left_pair.push(left_number);
        right_pair.push(right_number);
    }

    for number in left_pair.iter() {
        for other_number in right_pair.iter() {
            if number == other_number {
                similarity_score += number;
            }
        }
    }

    similarity_score
}
