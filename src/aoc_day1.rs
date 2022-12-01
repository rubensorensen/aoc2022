use crate::aoc_utils;

static FILEPATH: &str = "../data/day1_input.txt";

pub fn part_1() {
    let file_content = aoc_utils::slurp_file(FILEPATH);

    let mut cals: Vec<u32> = Vec::new();
    let mut cal: u32 = 0;
    for line in file_content.lines() {
        if line.is_empty() {
            cals.push(cal);
            cal = 0;
            continue;
        }
        cal += line.parse::<u32>().unwrap();
    }
    cals.push(cal);
    println!("Part 1: {}", cals.iter().max().unwrap());
}

pub fn part_2() {
    let file_content = aoc_utils::slurp_file(FILEPATH);

    let mut cals: Vec<u32> = Vec::new();
    let mut cal: u32 = 0;
    for line in file_content.lines() {
        if line.is_empty() {
            cals.push(cal);
            cal = 0;
            continue;
        }
        cal += line.parse::<u32>().unwrap();
    }
    cals.push(cal);

    cals.sort();
    let sum: u32 = cals.iter().rev().take(3).sum();
    println!("Part 2: {}", sum);
}
