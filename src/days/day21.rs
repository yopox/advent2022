use std::collections::HashMap;
use crate::days::day21::RefNumber::{Number, Ref};

#[test]
pub fn test() {
    println!("Day 21");

    let input = parse_input();

    let p1 = part1(&input);
    println!("Part 1 -> {}", p1);
    assert_eq!(p1, 299983725663456);

    let p2 = part2(&input);
    println!("Part 2 -> {}", p2);
    assert_eq!(p2, 3093175982595);
}

#[derive(Clone)]
enum RefNumber {
    Number(usize),
    Ref(String),
}

impl RefNumber {
    fn from_str(s: &str) -> Self {
        let nb = s.parse::<usize>();
        return if nb.is_ok() { Number(nb.unwrap()) } else { Ref(s.to_string()) }
    }

    fn resolve(&self, monkeys: &HashMap<String, Operation>) -> usize {
        return match self {
            Number(n) => *n,
            Ref(r) => {
                monkeys.get(r).expect("Reference to unknown monkey.").resolve(monkeys)
            }
        }
    }

    fn has_humn(&self, monkeys: &HashMap<String, Operation>) -> bool {
        match self {
            Number(_) => false,
            Ref(s) => s == "humn" || monkeys.get(s).expect("Reference to unknown monkey.").has_human(monkeys)
        }
    }

    fn solve(&self, target: i64, monkeys: &HashMap<String, Operation>) -> i64 {
        match self {
            Number(_) => panic!("RefNumber shouldn't have human"),
            Ref(s) => {
                return
                    if s == "humn" { target }
                    else { monkeys.get(s).unwrap().solve(target, monkeys) }
            }
        }
    }
}

#[derive(Clone)]
enum Operation {
    Number(RefNumber),
    Add(RefNumber, RefNumber),
    Subtract(RefNumber, RefNumber),
    Multiply(RefNumber, RefNumber),
    Divide(RefNumber, RefNumber),
}

impl Operation {
    fn from_str(s: &str) -> Self {
        let parts = s.split(" ").collect::<Vec<&str>>();
        if parts.len() == 1 { return Operation::Number(RefNumber::from_str(parts[0])); }
        let n1 = RefNumber::from_str(parts[0]);
        let n2 = RefNumber::from_str(parts[2]);
        return match parts[1] {
            "+" => Operation::Add(n1, n2),
            "-" => Operation::Subtract(n1, n2),
            "*" => Operation::Multiply(n1, n2),
            "/" => Operation::Divide(n1, n2),
            _ => panic!("Can't parse operation.")
        }
    }

    fn resolve(&self, monkeys: &HashMap<String, Operation>) -> usize {
        match self {
            Operation::Number(n1) => n1.resolve(monkeys),
            Operation::Add(n1, n2) => n1.resolve(monkeys) + n2.resolve(monkeys),
            Operation::Subtract(n1, n2) => {
                let nb1 = n1.resolve(monkeys);
                let nb2 = n2.resolve(monkeys);
                if nb1 > nb2 { nb1 - nb2 } else { 0 }
            },
            Operation::Multiply(n1, n2) => n1.resolve(monkeys) * n2.resolve(monkeys),
            Operation::Divide(n1, n2) => {
                let nb1 = n1.resolve(monkeys);
                let nb2 = n2.resolve(monkeys);
                if nb2 != 0 { nb1 / nb2 } else { 0 }
            },
        }
    }

    fn get_numbers(&self) -> Option<(&RefNumber, &RefNumber)> {
        match self {
            Operation::Number(_) => None,
            Operation::Add(n1, n2) => Some((n1, n2)),
            Operation::Subtract(n1, n2) => Some((n1, n2)),
            Operation::Multiply(n1, n2) => Some((n1, n2)),
            Operation::Divide(n1, n2) => Some((n1, n2)),
        }
    }

    fn has_human(&self, monkeys: &HashMap<String, Operation>) -> bool {
        match self {
            Operation::Number(n1) => n1.has_humn(monkeys),
            Operation::Add(n1, n2) => n1.has_humn(monkeys) || n2.has_humn(monkeys),
            Operation::Subtract(n1, n2) => n1.has_humn(monkeys) || n2.has_humn(monkeys),
            Operation::Multiply(n1, n2) => n1.has_humn(monkeys) || n2.has_humn(monkeys),
            Operation::Divide(n1, n2) => n1.has_humn(monkeys) || n2.has_humn(monkeys),
        }
    }

    fn solve(&self, target: i64, monkeys: &HashMap<String, Operation>) -> i64 {
        match self {
            Operation::Number(n1) => n1.solve(target, monkeys),
            Operation::Add(n1, n2) => {
                if n1.has_humn(monkeys) { n1.solve(target - n2.resolve(monkeys) as i64, monkeys) }
                else { n2.solve(target - n1.resolve(monkeys) as i64, monkeys) }
            },
            Operation::Subtract(n1, n2) => {
                if n1.has_humn(monkeys) { n1.solve(target + n2.resolve(monkeys) as i64, monkeys) }
                else { n2.solve(n1.resolve(monkeys) as i64 - target, monkeys) }
            },
            Operation::Multiply(n1, n2) => {
                if n1.has_humn(monkeys) { n1.solve(target / n2.resolve(monkeys) as i64, monkeys) }
                else { n2.solve(target / n1.resolve(monkeys) as i64, monkeys) }
            },
            Operation::Divide(n1, n2) => {
                if n1.has_humn(monkeys) { n1.solve(target * n2.resolve(monkeys) as i64, monkeys) }
                else { n2.solve(n1.resolve(monkeys) as i64 / target, monkeys) }
            },
        }
    }
}

fn parse_input() -> HashMap<String, Operation> {
    let mut map = HashMap::new();
    include_str!("data/day21")
        .lines()
        .for_each(|l| {
            let (name, operation) = l.split_once(": ").expect("Can't parse monkey ID.");
            map.insert(name.to_string(), Operation::from_str(operation));
        });
    map
}

fn part1(monkeys: &HashMap<String, Operation>) -> usize {
    return monkeys.get("root").expect("Can't find root.").resolve(&monkeys);
}

fn part2(monkeys: &HashMap<String, Operation>) -> i64 {
    let (n1, n2) = monkeys.get("root").expect("Can't find root").get_numbers().unwrap();
    let n1 = n1.clone();
    let n2 = n2.clone();

    let (n, target) =
        if n1.has_humn(monkeys) { (n1, n2.resolve(monkeys)) }
        else { (n2, n1.resolve(monkeys)) };

    return n.solve(target as i64, monkeys);
}
