use regex::Regex;

#[test]
fn test() {
    println!("Day 5");
    let p1 = part1();
    println!("Part 1 -> {}", p1);
    assert_eq!(p1, "FCVRLMVQP");

    let p2 = part2();
    println!("Part 2 -> {}", p2);
    assert_eq!(p2, "RWLWGJGFD");
}

fn parse_crates(line: &str) -> String {
    line
        .replace("     ", "-")
        .replace("    ", "-")
        .replace("   ", "-")
        .replace(" ", "")
        .replace("[", "")
        .replace("]", "")
}

fn get_stacks(initial: &str) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = vec![];

    initial
        .lines()
        .rev()
        .skip(1)
        .map(|line| parse_crates(line))
        .for_each(|crates| crates.chars().enumerate()
            .for_each(|(i, c)|
                if c != '-' {
                    while stacks.len() <= i {
                        stacks.push(vec![]);
                    }
                    stacks.get_mut(i).unwrap().push(c);
                })
        );
    stacks
}

fn get_moves(moves: &str) -> Vec<(usize, usize, usize)> {
    let move_regex = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
    moves
        .lines()
        .map(|line| move_regex.captures(line).unwrap())
        .map(|cap| (
            cap[1].parse::<usize>().unwrap(),
            cap[2].parse::<usize>().unwrap() - 1,
            cap[3].parse::<usize>().unwrap() - 1
        ))
        .collect()
}

fn pop_stacks(stacks: &mut Vec<Vec<char>>) -> String {
    stacks
        .iter_mut()
        .map(|i| i.pop().unwrap())
        .collect()
}

fn part1() -> String {
    let (initial, moves) = include_str!("data/day5")
        .split_once("\n\n")
        .unwrap();

    let mut stacks = get_stacks(initial);

    get_moves(moves)
        .iter()
        .for_each(|(n, from, to)|
            for _ in 0..*n {
                if let Some(moved) = stacks.get_mut(*from).unwrap().pop() {
                    stacks.get_mut(*to).unwrap().push(moved);
                }
            }
        );

    return pop_stacks(&mut stacks);
}

fn part2() -> String {
    let (initial, moves) = include_str!("data/day5")
        .split_once("\n\n")
        .unwrap();

    let mut stacks = get_stacks(initial);

    get_moves(moves)
        .iter()
        .for_each(|(n, from, to)| {
            let mut picked = vec![];
            for _ in 0..*n {
                if let Some(moved) = stacks.get_mut(*from).unwrap().pop() {
                    picked.push(moved);
                }
            }
            picked.iter().rev().for_each(|c|
                stacks.get_mut(*to).unwrap().push(*c)
            )
        });

    return pop_stacks(&mut stacks);
}
