#[test]
fn test() {
    part1();
    part2();
}

#[derive(Eq, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Move {
    fn score(&self) -> u16 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn wins_against(&self, other: &Move) -> Outcome {
        return if self == other { Outcome::Draw }
        else if *other == self.losing() { Outcome::Win }
        else { Outcome::Lose }
    }

    fn from_str(str: &str) -> Self {
        match str {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => panic!("Letter not recognized")
        }
    }

    fn losing(&self) -> Self {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    fn winning(&self) -> Self {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }
}

impl Outcome {
    fn score(&self) -> u16 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }

    fn from_str(str: &str) -> Outcome {
        match str {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Outcome not recognized")
        }
    }
}

fn part1() {
    let rounds: Vec<(Move, Move)> = include_str!("data/day2")
        .lines()
        .map(|line| {
            let (c1, c2) = line.split_once(' ').unwrap();
            (Move::from_str(c1), Move::from_str(c2))
        })
        .collect();

    let score: u16 = rounds.iter()
        .map(|(m1, m2)| m2.wins_against(m1).score() + m2.score())
        .sum();

    println!("{}", score);
}

fn part2() {
    let rounds: Vec<(Move, Outcome)> = include_str!("data/day2")
        .lines()
        .map(|line| {
            let (c1, c2) = line.split_once(' ').unwrap();
            (Move::from_str(c1), Outcome::from_str(c2))
        })
        .collect();

    let score: u16 = rounds.iter()
        .map(|(m, o)| {
           o.score() + match o {
               Outcome::Lose => m.losing().score(),
               Outcome::Win => m.winning().score(),
               Outcome::Draw => m.score(),
           }
        })
        .sum();

    println!("{}", score);
}
