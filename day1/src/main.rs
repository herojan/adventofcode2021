use std::fs::read_to_string;

fn main() -> () {
    let ints: Vec<usize> = read_to_string("input.txt").unwrap().lines().map(|l| l.parse().unwrap()).collect();

    println!("part1: {}", count_increases(&ints, 1));
    println!("part2: {}", count_increases(&ints, 3));
}

fn count_increases(ints: &Vec<usize>, window_size: usize) -> usize {
    let tail: &Vec<&[usize]> = &ints[1..].windows(window_size).collect();
    ints.windows(window_size)
        .zip(tail)
        .filter(|(a, &b)| b.iter().sum::<usize>() > a.iter().sum())
        .count()
}