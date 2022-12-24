use std::collections::HashSet;

#[test]
fn test() {
    println!("Day 24");

    let input = parse_input();

    let p1 = part1(&input);
    println!("Part 1 -> {}", p1);
    assert_eq!(p1, 269);

    let p2 = part2(&input);
    println!("Part 2 -> {}", p2);
    assert_eq!(p2, 825);
}

#[derive(Clone, Debug)]
enum Direction { N, S, E, W }

#[derive(Clone)]
struct Blizzard {
    direction: Direction,
    x: usize,
    y: usize,
}

impl Blizzard {
    fn from_char(char: char, x: usize, y: usize) -> Option<Self> {
        match char {
            '<' => Some(Blizzard { direction: Direction::W, x, y }),
            '^' => Some(Blizzard { direction: Direction::N, x, y }),
            '>' => Some(Blizzard { direction: Direction::E, x, y }),
            'v' => Some(Blizzard { direction: Direction::S, x, y }),
            _ => None
        }
    }
}

fn parse_input() -> (Vec<(Blizzard)>, usize, usize) {
    let lines = include_str!("data/day24")
        .lines();
    let mut max_x = 0;
    let mut max_y = 0;
    let blizzard = lines
        .enumerate()
        .flat_map(|(y, l)| {
            if y > max_y { max_y = y }
            l
                .chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    if x > max_x { max_x = x }
                    if x > 0 && y > 0 {
                        Blizzard::from_char(c, x - 1, y - 1)
                    } else { None }
                })
                .collect::<Vec<(Blizzard)>>()
        })
        .collect();
    (blizzard, max_x - 1, max_y - 1)
}

fn part1(input: &(Vec<Blizzard>, usize, usize)) -> u32 {
    let mut blizzard: Vec<Blizzard> = input.0.clone();
    let (w, h) = (input.1 as i32, input.2 as i32);

    let mut turn = 0;
    let mut solutions: HashSet<(i32, i32)> = HashSet::new();
    solutions.insert((0, -1));
    return solve(&mut blizzard, w, h, turn, &mut solutions, true);
}

fn step(w: i32, h: i32, turn: u32, solutions: &mut HashSet<(i32, i32)>, occupied: HashSet<(i32, i32)>, down: bool) -> Option<u32> {
    let mut new_sol = HashSet::new();

    for &(x, y) in solutions.iter() {
        // Win
        if (down && x == w - 1 && y == h - 1)
            || (!down && x == 0 && y == 0) { return Some(turn) }
        // Wait
        if !occupied.contains(&(x, y)) { new_sol.insert((x, y)); }
        // Move
        if down {
            if y < h - 1 && !occupied.contains(&(x, y + 1)) { new_sol.insert((x, y + 1)); }
            if y == -1 { continue }
            if y > 0 && !occupied.contains(&(x, y - 1)) { new_sol.insert((x, y - 1)); }
        } else {
            if y > 0 && !occupied.contains(&(x, y - 1)) { new_sol.insert((x, y - 1)); }
            if y == h { continue }
            if y < h - 1 && !occupied.contains(&(x, y + 1)) { new_sol.insert((x, y + 1)); }
        }
        if x > 0 && !occupied.contains(&(x - 1, y)) { new_sol.insert((x - 1, y)); }
        if x < w - 1 && !occupied.contains(&(x + 1, y)) { new_sol.insert((x + 1, y)); }
    }
    solutions.clear();
    new_sol.iter().for_each(|s| { solutions.insert(*s); });
    return None;
}

fn update_blizzard(blizzard: &mut Vec<Blizzard>, w: usize, h: usize) -> HashSet<(i32, i32)> {
    let mut occupied = HashSet::new();
    for b in blizzard.iter_mut() {
        match b.direction {
            Direction::N => b.y = if b.y == 0 { h - 1 } else { b.y - 1 },
            Direction::S => b.y = (b.y + 1) % h,
            Direction::W => b.x = if b.x == 0 { w - 1 } else { b.x - 1 },
            Direction::E => b.x = (b.x + 1) % w,
        }
        occupied.insert((b.x as i32, b.y as i32));
    }
    occupied
}

fn part2(input: &(Vec<Blizzard>, usize, usize)) -> u32 {
    let mut blizzard: Vec<Blizzard> = input.0.clone();
    let (w, h) = (input.1 as i32, input.2 as i32);

    let mut turn = 0;
    let mut solution = 0;

    for pos in [(0, -1), (w - 1, h), (0, -1)] {
        turn = 0;
        let mut solutions = HashSet::new();
        solutions.insert(pos);
        solution += solve(&mut blizzard, w, h, turn, &mut solutions, pos == (0, -1));
    }
    solution
}

fn solve(mut blizzard: &mut Vec<Blizzard>, w: i32, h: i32, mut turn: u32, mut solutions: &mut HashSet<(i32, i32)>, down: bool) -> u32 {
    loop {
        turn += 1;
        let occupied = update_blizzard(&mut blizzard, w as usize, h as usize);
        let t = step(w, h, turn, &mut solutions, occupied, down);
        if t.is_some() { break t.unwrap() }
    }
}
