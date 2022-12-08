use crate::benchmark;

#[test]
fn test() {
    println!("Day 6");
    let input = input();

    let p1 = part1(input.as_slice());
    println!("Part 1 -> {}", p1);
    assert_eq!(p1, 1848);

    let p2 = part2(input.as_slice());
    println!("Part 2 -> {}", p2);
    assert_eq!(p2, 2308);

    benchmark(input.as_slice(), |i| part1(i), |i| part2(i));
}

fn distinct(slice: &[char], size: usize) -> usize {
    let mut i = 0;
    'outer: loop {
        let s = &slice[i..i + size];
        for j in 1..size {
            if slice[i + j..i + size].contains(&s[j - 1]) { i += j; continue 'outer; }
        }
        return i + size;
    }
}

fn input() -> Vec<char> {
    include_str!("data/day6").chars().collect()
}

fn part1(input: &[char]) -> usize {
    distinct(input, 4)
}

fn part2(input: &[char]) -> usize {
    distinct(input, 14)
}
