use crate::benchmark;

#[test]
fn test() {
    println!("Day 6");
    let p1 = part1();
    println!("Part 1 -> {}", p1);
    assert_eq!(p1, 1848);

    let p2 = part2();
    println!("Part 2 -> {}", p2);
    assert_eq!(p2, 2308);
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

fn part1() -> usize {
    distinct(include_str!("data/day6").chars().collect::<Vec<char>>().as_slice(), 4)
}

fn part2() -> usize {
    distinct(include_str!("data/day6").chars().collect::<Vec<char>>().as_slice(), 14)
}
