use std::fs::read_to_string;

fn main() -> () {
    let commands: Vec<Command> = read_to_string("input.txt").unwrap().lines().map(|l| {
        let (dir, dist) = l.split_once(" ").unwrap();
        Command { dir: dir.to_string(), dist: dist.parse().unwrap() }
    }).collect();

    part1(&commands);
    part2(&commands)
}

fn part1(commands: &Vec<Command>) {
    let mut depth = 0;
    let mut pos = 0;

    for command in commands {
        match command.dir.as_ref() {
            "forward" => pos += command.dist,
            "up" => depth -= command.dist,
            "down" => depth += command.dist,
            d => panic!("Invalid command {}", d),
        }
    }

    println!("Part 1: {}", depth * pos)
}

fn part2(commands: &Vec<Command>) {
    let mut depth = 0;
    let mut pos = 0;
    let mut aim = 0;

    for command in commands {
        match command.dir.as_ref() {
            "forward" => {
                pos += command.dist;
                depth += aim * command.dist
            }
            "up" => aim -= command.dist,
            "down" => aim += command.dist,
            d => panic!("Invalid command {}", d),
        }
    }

    println!("Part 2: {}", depth * pos)
}

#[derive(Debug)]
struct Command {
    dir: String,
    dist: usize,
}