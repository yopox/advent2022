use std::collections::{HashMap, HashSet};

#[test]
fn test() {
    println!("Day 23");

    let positions = parse_input();

    let p1 = part1(&positions);
    println!("Part 1 -> {}", p1);
    assert_eq!(p1, 4247);

    let p2 = part2(&positions);
    println!("Part 2 -> {}", p2);
    // assert_eq!(p2, 0);
}

fn parse_input() -> Vec<(i32, i32)> {
    include_str!("data/day23")
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(x, _)| (x as i32, y as i32 * -1))
                .collect::<Vec<(i32, i32)>>()
        })
        .collect()
}

enum Direction { N, S, E, W }

fn part1(initial_pos: &Vec<(i32, i32)>) -> i32 {
    let mut positions: HashSet<(i32, i32)> = initial_pos.iter().map(|p| *p).collect();
    let mut priorities = vec![Direction::N, Direction::S, Direction::W, Direction::E];

    for _ in 0..10 {
        let mut destinations: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
        positions.iter().for_each(|&(x, y)| {
            let (dx, dy, _) = get_destination(x, y, &positions, &priorities);
            if destinations.contains_key(&(dx, dy)) { destinations.get_mut(&(dx, dy)).unwrap().push((x, y)); }
            else { destinations.insert((dx, dy), vec![(x, y)]); }
        });
        destinations.iter().for_each(|(new_pos, candidates)| {
            if candidates.len() == 1 {
                positions.remove(&candidates.first().unwrap());
                positions.insert(new_pos.clone());
            }
        });
        let dir1 = priorities.remove(0);
        priorities.push(dir1);
    }

    let min_x = positions.iter().min_by_key(|(x, _)| x).unwrap().0;
    let max_x = positions.iter().max_by_key(|(x, _)| x).unwrap().0;
    let min_y = positions.iter().min_by_key(|(_, y)| y).unwrap().1;
    let max_y = positions.iter().max_by_key(|(_, y)| y).unwrap().1;
    (max_x - min_x + 1) * (max_y - min_y + 1) - positions.len() as i32
}

fn print_board(positions: &HashSet<(i32, i32)>) {
    let min_x = positions.iter().min_by_key(|(x, _)| x).unwrap().0;
    let max_x = positions.iter().max_by_key(|(x, _)| x).unwrap().0;
    let min_y = positions.iter().min_by_key(|(_, y)| y).unwrap().1;
    let max_y = positions.iter().max_by_key(|(_, y)| y).unwrap().1;
    for dy in 0..=(max_y - min_y) {
        for x in min_x..=max_x {
            if positions.contains(&(x, max_y - dy)) { print!("# ") } else { print!(". ") }
        }
        println!();
    }
    println!();
}

fn get_destination(x: i32, y: i32, positions: &HashSet<(i32, i32)>, priorities: &Vec<Direction>) -> (i32, i32, bool) {
    if !positions.contains(&(x - 1, y - 1))
        && !positions.contains(&(x, y - 1))
        && !positions.contains(&(x + 1, y - 1))
        && !positions.contains(&(x - 1, y))
        && !positions.contains(&(x + 1, y))
        && !positions.contains(&(x - 1, y + 1))
        && !positions.contains(&(x, y + 1))
        && !positions.contains(&(x + 1, y + 1)) { return (x, y, true) }
    for dir in priorities.iter() {
        match dir {
            Direction::N => {
                if !positions.contains(&(x - 1, y + 1))
                    && !positions.contains(&(x, y + 1))
                    && !positions.contains(&(x + 1, y + 1))
                { return (x, y + 1, false); }
            }
            Direction::S => {
                if !positions.contains(&(x - 1, y - 1))
                    && !positions.contains(&(x, y - 1))
                    && !positions.contains(&(x + 1, y - 1))
                { return (x, y - 1, false); }
            }
            Direction::E => {
                if !positions.contains(&(x + 1, y - 1))
                    && !positions.contains(&(x + 1, y))
                    && !positions.contains(&(x + 1, y + 1))
                { return (x + 1, y, false); }
            }
            Direction::W => {
                if !positions.contains(&(x - 1, y - 1))
                    && !positions.contains(&(x - 1, y))
                    && !positions.contains(&(x - 1, y + 1))
                { return (x - 1, y, false); }
            }
        }
    }
    return (x, y, true);
}

fn part2(initial_pos: &Vec<(i32, i32)>) -> u32 {
    let mut positions: HashSet<(i32, i32)> = initial_pos.iter().map(|p| *p).collect();
    let mut priorities = vec![Direction::N, Direction::S, Direction::W, Direction::E];
    let mut turn = 0;

    loop {
        let mut fixed = true;
        let mut destinations: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
        positions.iter().for_each(|&(x, y)| {
            let (dx, dy, f) = get_destination(x, y, &positions, &priorities);
            if !f { fixed = false; }
            if destinations.contains_key(&(dx, dy)) { destinations.get_mut(&(dx, dy)).unwrap().push((x, y)); }
            else { destinations.insert((dx, dy), vec![(x, y)]); }
        });
        destinations.iter().for_each(|(new_pos, candidates)| {
            if candidates.len() == 1 {
                positions.remove(&candidates.first().unwrap());
                positions.insert(new_pos.clone());
            }
        });
        let dir1 = priorities.remove(0);
        priorities.push(dir1);
        turn += 1;
        if fixed { return turn; }
    }
}
