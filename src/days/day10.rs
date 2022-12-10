use crate::days::day10::Instruction::Noop;

#[test]
fn test() {
    println!("Day 10");

    let input = parse_input();

    let p1 = part1(&input);
    println!("Part 1 -> {}", p1);
    assert_eq!(p1, 11960);

    let p2 = part2(&input);
    // println!("Part 2 -> {}", p2);
    // assert_eq!(p2, 0);
}

enum Instruction {
    Noop,
    AddX(i32),
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        if s == "noop" { Instruction::Noop }
        else {
            match s.split_once(" ") {
                Some((i, j)) => Instruction::AddX(j.parse::<i32>().expect("Couldn't parse value")),
                _ => panic!("Couldn't parse instruction")
            }
        }
    }
}

fn parse_input() -> Vec<Instruction> {
    include_str!("data/day10")
        .lines()
        .map(|l| Instruction::from_str(l))
        .collect()
}

struct State {
    step: usize,
    total: i32,
}

fn increase_counter(state: &mut State, x: &i32) {
    state.step += 1;
    if (state.step as i32 - 20) % 40 == 0 && state.step <= 220 {
        println!("{} x {}", state.step, state.total);
        state.total += x * state.step as i32;
    }
}

/// Hardcore, un album, ouais, encore, hardcore
/// J'ai des choses à dire, hardcore, un album, ouais, encore
/// J'fais un McDo, j'prends un fish à mon pote parce qu'il mange pas d'porc
fn part1(input: &Vec<Instruction>) -> i32 {
    let mut x: i32 = 1;
    let mut state = State { step: 0, total: 0 };
    input
        .iter()
        .for_each(|i| {
            increase_counter(&mut state, &x);
            match i {
                Noop => {}
                Instruction::AddX(amount) => {
                    increase_counter(&mut state, &x);
                    x += amount;
                }
            }
        });
    state.total
}

fn draw(state: &mut State, x: &i32) {
    state.step += 1;
    if (state.step - 1) % 40 == 0 { print!("\n"); }
    let dx = ((state.step - 1) % 40) as i32 - x;
    if dx == -1 || dx == 0 || dx == 1 { print!("XX"); }
    else { print!("  "); }
}

/// S'il m'arrive quelque chose, moi, j'veux que toute la Terre y sache
/// Que la vie, c'est pas facile, que t'es pas forcé de vendre des kil'
/// Que, la vie c'est face ou pile, aujourd'hui par terre, demain sur une île
fn part2(input: &Vec<Instruction>) {
    let mut x: i32 = 1;
    let mut state = State { step: 0, total: 0 };
    input
        .iter()
        .for_each(|i| {
            draw(&mut state, &x);
            match i {
                Noop => {}
                Instruction::AddX(amount) => {
                    draw(&mut state, &x);
                    x += amount;
                }
            }
        });
}
