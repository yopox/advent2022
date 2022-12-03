#[test]
fn test() {
    println!("Day 3");
    let p1 = part1();
    println!("Part 1 -> {}", p1);
    assert_eq!(p1, 8088);

    let p2 = part2();
    println!("Part 2 -> {}", p2);
    assert_eq!(p2, 2522);
}

fn common_letter(slice: &[&str]) -> char {
    slice
        .first()
        .unwrap()
        .chars()
        .into_iter()
        .find(|c| slice.iter().skip(1).all(|line| line.contains(*c)))
        .unwrap()
}

fn score(c: char) -> u32 {
    return if c.is_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 27
    }
}

fn get_lines() -> Vec<&'static str> {
    include_str!("data/day3")
        .lines()
        .collect()
}

fn part1() -> u32 {
    get_lines()
        .iter()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(s1, s2)| common_letter(&[s1, s2]))
        .map(|c| score(c))
        .sum::<u32>()
}

fn part2() -> u32 {
    let lines = get_lines();
    (0..lines.len()/3)
        .map(|i| common_letter(&lines[3*i..3*i+3]))
        .map(|c| score(c))
        .sum::<u32>()
}
