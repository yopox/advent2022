use regex::{Captures, Regex};

#[test]
fn test() {
    println!("Day 15");

    let input = parse_input();

    let p1 = part1(&input);
    println!("Part 1 -> {}", p1);
    assert_eq!(p1, 5125700);

    let p2 = part2(&input);
    println!("Part 2 -> {}", p2);
    assert_eq!(p2, 11379394658764);
}

struct Sensor {
    pos: (i64, i64),
    nearest: (i64, i64),
    distance: i64,
}

impl Sensor {
    fn from_captures(regex: Captures) -> Self {
        let p_sensor = (regex[1].parse::<i64>().expect("Can't parse sensor x"),
                        regex[2].parse::<i64>().expect("Can't parse sensor y"));
        let p_beacon = (regex[3].parse::<i64>().expect("Can't parse beacon x"),
                        regex[4].parse::<i64>().expect("Can't parse beacon y"));

        Sensor {
            pos: p_sensor,
            nearest: p_beacon,
            distance: manhattan(p_sensor, p_beacon),
        }
    }
}

fn parse_input() -> Vec<Sensor> {
    let regex = Regex::new(r"Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)").unwrap();

    include_str!("data/day15")
        .lines()
        .map(|l| Sensor::from_captures(regex.captures(l).expect("Can't parse sensor data")))
        .collect()
}

fn manhattan(p1: (i64, i64), p2: (i64, i64)) -> i64 {
    return (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn part1(input: &Vec<Sensor>) -> usize {
    let y = 2000000;

    let max_d = input.iter().map(|s| s.distance).max().unwrap();
    let min_x = input.iter().map(|s| s.pos.0).min().unwrap();
    let max_x = input.iter().map(|s| s.pos.0).max().unwrap();
    ((min_x - max_d)..(max_x + max_d))
        .filter(|x| {
            let pos = (*x, y);
            let impossible = input.iter().find(|s| manhattan(s.pos, pos) <= s.distance).is_some();
            impossible
        })
        .filter(|x| input.iter().find(|s| s.nearest.0 == *x && s.nearest.1 == y).is_none())
        .collect::<Vec<i64>>()
        .len()
}

fn part2(input: &Vec<Sensor>) -> usize {
    let size = 4000000;

    let mut y = 0;
    let mut x = 0;

    loop {
        let pos = (x, y);
        let distances = input.iter().map(|s| (s, manhattan(s.pos, pos))).collect::<Vec<(&Sensor, i64)>>();
        if distances.iter().find(|(s, d)| d <= &s.distance).is_none() {
            // println!("{} ; {}", x, y);
            return x as usize * 4000000 + y as usize;
        }
        let constraint = distances.iter().map(|(s, d)| if d < &s.distance { &s.distance - d + 1 } else { 1 }).max().unwrap();
        x += constraint;
        if x > size {
            x = 0;
            y += 1;
        }
        if y > size {
            panic!("Couldn't find a solution")
        }
    }
}
