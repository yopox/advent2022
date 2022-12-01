pub fn main() {
    part1();
    part2();
}

fn part1() {
    let calories: u64 = include_str!("data/day1")
        .split("\n\n")
        .map(|group| group.lines().map(|line| line.to_string().parse::<u64>().unwrap()).sum())
        .max()
        .unwrap();
    println!("{}", calories);
}

fn part2() {
    let mut groups: Vec<u64> = include_str!("data/day1")
        .split("\n\n")
        .map(|group| group.lines().map(|line| line.to_string().parse::<u64>().unwrap()).sum())
        .collect();
    groups.sort();
    println!("{}", groups.iter().rev().take(3).sum::<u64>());
}