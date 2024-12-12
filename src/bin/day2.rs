use itertools::Itertools;
use std::time::Instant;

fn is_safe<F: Fn(u32, u32) -> bool>(arr: &[u32], f: F) -> bool {
    for i in 0..arr.len() - 1 {
        if !f(arr[i], arr[i + 1]) {
            return false;
        }
        if f(arr[i], arr[i + 1]) {
            let diff = arr[i].abs_diff(arr[i + 1]);
            if !(diff > 0 && diff <= 3) {
                return false;
            }
        }
    }
    true
}

fn check(line: &[u32]) -> bool {
    is_safe(line, |i, j| i < j) || is_safe(line, |i, j| i > j)
}

fn part1(rows: &[Vec<u32>]) -> usize {
    rows.iter().filter(|line| check(line)).count()
}

fn part2(rows: &[Vec<u32>]) -> usize {
    rows.iter().filter(|line| check2(line)).count()
}

fn check2(arr: &[u32]) -> bool {
    for i in 0..arr.len() {
        let removed = [&arr[0..i], &arr[i + 1..]].concat();
        if check(&removed) {
            return true;
        }
    }
    false
}

fn main() -> anyhow::Result<()> {
    let lines = aoc::read_lines()?;
    let rows = lines
        .map(|line| {
            line.unwrap()
                .split_ascii_whitespace()
                .map(|line| line.parse::<u32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let before = Instant::now();

    let part1 = part1(&rows);
    println!("Part 1 = {part1}, {}us taken", before.elapsed().as_micros());
    let before = Instant::now();
    let part2 = part2(&rows);
    println!("Part 2 = {part2}, {}us taken", before.elapsed().as_micros());
    Ok(())
}
