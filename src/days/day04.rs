#[test]
fn test() {
    println!("Day 4");
    let p1 = part1();
    println!("Part 1 -> {}", p1);
    assert_eq!(p1, 500);

    let p2 = part2();
    println!("Part 2 -> {}", p2);
    assert_eq!(p2, 815);
}

struct Range {
    from: u8,
    to: u8,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        return other.from >= self.from && other.to <= self.to;
    }

    fn partially_overlaps(&self, other: &Range) -> bool {
        return other.from >= self.from && other.from <= self.to
        || other.to <= self.to && other.to >= self.from;
    }

    fn from_str(s: &str) -> Self {
        let (from, to) = s.split_once('-').unwrap();
        Range {
            from: from.parse::<u8>().unwrap(),
            to: to.parse::<u8>().unwrap()
        }
    }
}

fn get_lines() -> Vec<&'static str> {
    include_str!("data/day4")
        .lines()
        .collect()
}

fn part1() -> u32 {
    get_lines()
        .iter()
        .map(|l| l.split_once(',').unwrap())
        .map(|(s1, s2)| (Range::from_str(s1), Range::from_str(s2)))
        .map(|(r1, r2)| r1.contains(&r2) || r2.contains(&r1))
        .filter(|b| *b)
        .count() as u32
}

fn part2() -> u32 {
    get_lines()
        .iter()
        .map(|l| l.split_once(',').unwrap())
        .map(|(s1, s2)| (Range::from_str(s1), Range::from_str(s2)))
        .map(|(r1, r2)| r1.partially_overlaps(&r2) || r1.contains(&r2) || r2.contains(&r1))
        .filter(|b| *b)
        .count() as u32
}
