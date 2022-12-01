use crate::aoc_utils;

static FILEPATH: &str = "../data/day1_input.txt";

fn get_total_per_elf() -> Vec<u32> {
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

    return cals;
}

pub fn part_1() {
    let cals = get_total_per_elf();
    let max = *(cals.iter().max().unwrap());
    println!("Part 1: {}", max);
}

pub fn part_2() {
    let mut cals = get_total_per_elf();
    cals.sort();
    let sum: u32 = cals.iter().rev().take(3).sum();
    println!("Part 2: {}", sum);
}
