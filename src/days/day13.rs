use std::cmp::Ordering;
use itertools::Itertools;
use crate::days::day13::Token::{ClosingBracket, Number, OpeningBracket};

#[test]
fn test() {
    println!("Day 13");

    let input = process_input();

    let p1 = part1(&input);
    println!("Part 1 -> {}", p1);
    assert_eq!(p1, 6478);

    let p2 = part2(&input);
    println!("Part 2 -> {}", p2);
    assert_eq!(p2, 21922);
}

#[derive(Clone, Eq, PartialEq)]
enum Token {
    Number(u8),
    OpeningBracket,
    ClosingBracket,
}

impl Token {
    fn parse(line: &str) -> Vec<Token> {
        if line.len() == 0 { return vec![] }
        let char_0 = line.chars().nth(0).unwrap();
        let char_1 = line.chars().nth(1);
        let to_insert = match char_0 {
            '1' => {
                if line.len() > 1 && char_1 == Some('0') { Some(Number(10)) }
                else { Some(Number(1)) }
            },
            '0'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => Some(Number(char_0.to_digit(10).unwrap() as u8)),
            '[' => Some(OpeningBracket),
            ']' => Some(ClosingBracket),
            ','|'\n' => None,
            _ => panic!("Unexpected token")
        };
        let mut rest = Token::parse(&line[1..]);
        if let Some(t) = to_insert { rest.insert(0, t); }
        rest
    }
}

fn process_input() -> Vec<Vec<Token>> {
    include_str!("data/day13")
        .replace("\n\n", "\n")
        .split("\n")
        .map(|l| Token::parse(&l.to_string()[..]))
        .collect()
}

fn compare(l1: &[Token], l2: &[Token]) -> Ordering {
    if l1.len() == 0 { return if l2.len() == 0 { Ordering::Equal } else { Ordering::Less } }
    if l2.len() == 0 { return Ordering::Greater }
    match (&l1[0], &l2[0]) {
        (Number(n1), Number(n2)) =>
            if n1 < n2 { return Ordering::Less }
            else if n2 < n1 { return Ordering::Greater },
        (OpeningBracket, ClosingBracket) => return Ordering::Greater,
        (Number(_), ClosingBracket) => return Ordering::Greater,
        (ClosingBracket, Number(_)) => return Ordering::Less,
        (ClosingBracket, OpeningBracket) => return Ordering::Less,
        (Number(_), OpeningBracket) => return compare(&l1[0..], &l2[1..]),
        (OpeningBracket, Number(_)) => return compare(&l1[1..], &l2[0..]),
        _ => ()
    }
    return compare(&l1[1..], &l2[1..])
}

fn part1(input: &Vec<Vec<Token>>) -> usize {
    input
        .iter()
        .chunks(2)
        .into_iter()
        .enumerate()
        .map(|(i, mut c)| {
            let l1 = c.nth(0).unwrap();
            let l2 = c.nth(0).unwrap();
            if compare(&l1.as_slice(), &l2.as_slice()) != Ordering::Greater { i + 1 } else { 0 }
        })
        .sum()
}

fn part2(input: &Vec<Vec<Token>>) -> usize {
    let d1 = &[OpeningBracket, OpeningBracket, Number(2), ClosingBracket, ClosingBracket];
    let d2 = &[OpeningBracket, OpeningBracket, Number(6), ClosingBracket, ClosingBracket];
    input
        .iter()
        .chain(vec![d1.to_vec(), d2.to_vec()].iter())
        .sorted_by(|l1, l2| compare(&l1.as_slice(), &l2.as_slice()))
        .enumerate()
        .filter(|(i, l)|
            compare(&l.as_slice(), d1) == Ordering::Equal
            || compare(&l.as_slice(), d2) == Ordering::Equal
        )
        .map(|(i, _)| i + 1)
        .product()
}