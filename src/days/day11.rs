use itertools::Itertools;
use regex::{Captures, Regex};

#[test]
fn test() {
    println!("Day 11");

    let p1 = part1(process_input());
    println!("Part 1 -> {}", p1);
    assert_eq!(p1, 151312);

    let p2 = part2(process_input());
    println!("Part 2 -> {}", p2);
    assert_eq!(p2, 51382025916);
}

enum Operation {
    Add(usize),
    Mul(usize),
    Squared,
}

impl Operation {
    fn from_str(o: &char, n: &str) -> Self {
        let n = n.parse::<usize>();
        if n.is_err() { return Operation::Squared; }
        let n = n.unwrap();
        match o {
            '+' => Operation::Add(n),
            '*' => Operation::Mul(n),
            _ => panic!["Unknown operation"]
        }
    }

    fn apply(&self, to: &usize) -> usize {
        match self {
            Operation::Add(n) => to + n,
            Operation::Mul(n) => to * n,
            Operation::Squared => to.pow(2)
        }
    }
}

struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: usize,
    send_true: usize,
    send_false: usize,
    inspected: usize,
}

impl Monkey {
    fn from_capture(capture: &Captures) -> Self {
        Monkey {
            items: capture[1]
                .split(", ")
                .map(|i| i.parse::<usize>().expect("Can't parse item"))
                .collect::<Vec<usize>>(),
            operation: Operation::from_str(
                &capture[2].parse::<char>().expect("Can't parse operation"),
                &capture[3]
            ),
            test: capture[4].parse::<usize>().expect("Can't parse test"),
            send_true: capture[5].parse::<usize>().expect("Can't parse send true"),
            send_false: capture[6].parse::<usize>().expect("Can't parse send false"),
            inspected: 0,
        }
    }
}

fn process_input() -> Vec<Monkey> {
    let input = include_str!("data/day11");
    let regex = Regex::new(r"  Starting items: ([0-9 ,]+)\n  Operation: new = old ([+\-*/]) ([0-9]+|old)\n  Test: divisible by ([0-9]+)\n    If true: throw to monkey ([0-9]+)\n    If false: throw to monkey ([0-9]+)").unwrap();
    input
        .split("\n\n")
        .map(|m| {
            let matches = regex.captures(
                m.split_once("\n").expect("Malformed input").1
            ).expect("Can't parse monkey.");
            Monkey::from_capture(&matches)
        })
        .collect()
}

fn round(monkeys: &mut Vec<Monkey>, bound: usize, stress_op: fn(usize) -> usize) {
    for i in 0..monkeys.len() {
        let monkey = &mut monkeys[i];
        let items = monkey
            .items
            .iter()
            .map(|j| {
                let stress = stress_op(monkey.operation.apply(j)) % bound;
                monkey.inspected += 1;
                match &stress % monkey.test == 0 {
                    true => (stress, monkey.send_true),
                    false => (stress, monkey.send_false),
                }
            })
            .collect::<Vec<(usize, usize)>>();
        monkey.items.clear();
        items
            .iter()
            .for_each(|(item, m)| monkeys[*m].items.push(*item));
    }
}

fn part1(mut input: Vec<Monkey>) -> usize {
    let bound = usize::MAX;
    for _ in 0..20 {
        round(&mut input, bound, |n| n / 3);
    }
    input
        .iter()
        .map(|m| m.inspected)
        .sorted().rev()
        .take(2)
        .product()
}

fn part2(mut input: Vec<Monkey>) -> usize {
    let bound = input.iter().map(|m| m.test).product();
    for _ in 0..10000 {
        round(&mut input, bound, |n| n);
    }
    input
        .iter()
        .map(|m| m.inspected)
        .sorted().rev()
        .take(2)
        .product()
}
