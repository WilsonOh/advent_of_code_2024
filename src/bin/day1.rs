use itertools::Itertools;
use std::collections::HashMap;

fn get_lists(input: Vec<String>) -> (Vec<u32>, Vec<u32>) {
    let (mut l, mut r): (Vec<u32>, Vec<u32>) = input
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|line| line.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip();
    l.sort_unstable();
    r.sort_unstable();
    (l, r)
}

fn part1(l: &[u32], r: &[u32]) -> u32 {
    l.iter()
        .zip(r.iter())
        .fold(0, |acc, (&i, &j)| acc + i.abs_diff(j))
}

fn part2(l: &[u32], r: &[u32]) -> u32 {
    let mut counts: HashMap<u32, u32> = HashMap::new();
    for &i in r {
        counts.entry(i).and_modify(|count| *count += 1).or_insert(1);
    }
    l.iter()
        .fold(0, |acc, i| acc + (i * counts.get(i).unwrap_or(&0)))
}

fn main() -> anyhow::Result<()> {
    let input: Vec<_> = aoc::read_lines()?.map_while(|line| line.ok()).collect();
    let (l, r) = get_lists(input);
    let ans1 = part1(&l, &r);
    println!("Part1 = {ans1}");
    let ans2 = part2(&l, &r);
    println!("Part2 = {ans2}");
    Ok(())
}
