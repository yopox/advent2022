use std::cmp::max;
use itertools::Itertools;

#[test]
fn test() {
    println!("Day 25");

    let input = parse_input();

    let p1 = part1(&input);
    println!("Part 1 -> {}", p1);
    assert_eq!(p1, "122-2=200-0111--=200");
}

fn parse_input() -> Vec<String> {
    include_str!("data/day25")
        .lines()
        .map(|l| l.to_string())
        .collect()
}

fn to_dec(number: &String) -> i64 {
    let mut power = 0;
    let mut nb: i64 = 0;
    for c in number.chars().rev() {
        let pow = 5_i64.pow(power);
        match c {
            '1' => nb += pow,
            '2' => nb += 2 * pow,
            '-' => nb -= pow,
            '=' => nb -= 2 * pow,
            _ => {}
        }
        power += 1;
    }
    nb
}

fn to_snafu(nb: u128) -> String {
    let mut nb = nb;
    let powers = (0..20)
        .rev()
        .map(|n| {
            let pow = 5_u128.pow(n);
            let p = nb / pow;
            nb -= pow * p;
            (n as usize, p)
        })
        .collect::<Vec<(usize, u128)>>();
    let first = powers.iter().find(|(_, p)| *p != 0).unwrap().0;
    let mut retain = 0;
    let powers = powers
        .iter()
        .skip(max(20 - first, 2) - 2)
        .rev()
        .map(|(i, p)| {
            let digit = match p + retain {
                0|1|2 => { let r = p + retain; retain = 0; r },
                3|4|5 => { let r = p + retain; retain = 1; r % 5 },
                _ => panic!()
            };
            digit
        })
        .collect::<Vec<u128>>();
    let result = powers
        .iter()
        .rev()
        .map(|p| p.to_string())
        .join("")
        .replace("3", "=")
        .replace("4", "-");
    result
}

fn part1(input: &Vec<String>) -> String {
    let sum = input.iter()
        .map(|n| to_dec(n))
        .sum::<i64>();
    to_snafu(sum as u128)
}