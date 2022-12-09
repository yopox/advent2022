use std::collections::HashSet;

#[test]
fn test() {
    println!("Day 9");

    let input = get_moves();

    let p1 = part1(&input);
    println!("Part 1 -> {}", p1);
    assert_eq!(p1, 6266);

    let p2 = part2(&input);
    println!("Part 2 -> {}", p2);
    assert_eq!(p2, 2369);
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn from_str(s: &str) -> Self {
        match s {
            "U" => Self::Up,
            "R" => Self::Right,
            "D" => Self::Down,
            "L" => Self::Left,
            _ => panic!("Can't parse direction")
        }
    }
}

struct Move {
    direction: Dir,
    amount: u8,
}

impl Move {
    fn from_str(s: &str) -> Self {
        let (d, a) = s.split_once(" ").expect("Malformed move");
        Move { direction: Dir::from_str(d), amount: a.parse::<u8>().expect("Can't parse move amount") }
    }
}

#[derive(Clone)]
struct Pos {
    x: i16,
    y: i16,
}

impl Pos {
    fn move_in_dir(&mut self, dir: &Dir) {
        match dir {
            Dir::Up => self.y += 1,
            Dir::Down => self.y -= 1,
            Dir::Left => self.x -= 1,
            Dir::Right => self.x += 1,
        }
    }

    fn catchup(&mut self, other: (i16, i16)) {
        match (other.0 - self.x, other.1 - self.y) {
            // Line movement
            (2, 0) => self.x += 1,
            (-2, 0) => self.x -= 1,
            (0, 2) => self.y += 1,
            (0, -2) => self.y -= 1,
            // Diagonal movement
            (2, 1) | (1, 2) | (2, 2) => { self.x += 1; self.y += 1; }
            (2, -1) | (1, -2) | (2, -2) => { self.x += 1; self.y -= 1; }
            (-2, 1) | (-1, 2) | (-2, 2) => { self.x -= 1; self.y += 1; }
            (-2, -1) | (-1, -2) | (-2, -2) => { self.x -= 1; self.y -= 1; }
            _ => {}
        }
    }
}

fn get_moves() -> Vec<Move> {
    include_str!("data/day9")
        .lines()
        .map(|l| Move::from_str(l))
        .collect()
}

fn follow_last(input: &Vec<Move>, size: usize) -> usize {
    let mut tail_pos = HashSet::new();
    let mut rope = vec![Pos { x: 0, y: 0 }; size];
    input
        .iter()
        .for_each(|Move { direction, amount }| {
            for _ in 0..*amount {
                rope[0].move_in_dir(direction);
                for i in 1..rope.len() {
                    let previous = (rope[i - 1].x, rope[i - 1].y);
                    rope[i].catchup(previous);
                }
                let last = rope.last().unwrap();
                tail_pos.insert((last.x, last.y));
            }
        });
    tail_pos.len()
}

fn part1(input: &Vec<Move>) -> usize {
    follow_last(input, 2)
}

fn part2(input: &Vec<Move>) -> usize {
    follow_last(input, 10)
}
