use std::fs;

fn main() {
    let input = fs::read_to_string("src/input").unwrap();

    println!("part one: {:?}", part1(&input));
    println!("part two: {:?}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut floor = 0;

    for i in input.chars() {
        floor += if i == '(' { 1 } else { -1 };
    }

    floor
}

fn part2(input: &str) -> i32 {
    let mut floor = 0;
    let mut counter = 0;

    for i in input.chars() {
        floor += if i == '(' { 1 } else { -1 };
        counter += 1;

        if floor == -1 {
            break;
        }
    }

    counter
}