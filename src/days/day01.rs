#[test]
fn test() {
    println!("Day 1");
    let p1 = part1();
    println!("Part 1 -> {}", p1);
    assert_eq!(p1, 72511);

    let p2 = part2();
    println!("Part 2 -> {}", p2);
    assert_eq!(p2, 212117);
}

fn part1() -> u32 {
    include_str!("data/day1")
        .split("\n\n")
        .map(|group| group.lines().map(|line| line.to_string().parse::<u32>().unwrap()).sum())
        .max()
        .unwrap()
}

fn part2() -> u32 {
    let mut groups: Vec<u32> = include_str!("data/day1")
        .split("\n\n")
        .map(|group| group.lines().map(|line| line.to_string().parse::<u32>().unwrap()).sum())
        .collect();
    groups.sort();
    groups.iter().rev().take(3).sum::<u32>()
}