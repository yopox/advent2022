use std::collections::HashSet;

#[test]
fn test() {
    println!("Day 14");

    let walls = parse_input();

    let p1 = part1(&walls);
    println!("Part 1 -> {}", p1);
    assert_eq!(p1, 897);

    let p2 = part2(&walls);
    println!("Part 2 -> {}", p2);
    assert_eq!(p2, 26683);
}

fn add_wall(set: &mut HashSet<(u32, u32)>, p0: &(u32, u32), p1: &(u32, u32)) {
    if p0.0 == p1.0 {
        if p0.1 < p1.1 {
            (p0.1..=p1.1).for_each(|y| { set.insert((p0.0, y)); })
        } else {
            (p1.1..=p0.1).for_each(|y| { set.insert((p0.0, y)); })
        }
    } else if p0.1 == p1.1 {
        if p0.0 < p1.0 {
            (p0.0..=p1.0).for_each(|x| { set.insert((x, p0.1)); })
        } else {
            (p1.0..=p0.0).for_each(|x| { set.insert((x, p0.1)); })
        }
    } else { panic!("Non straight wall") }
}

fn parse_input() -> HashSet<(u32, u32)> {
    let mut walls = HashSet::new();
    include_str!("data/day14")
        .lines()
        .for_each(|l| l.split(" -> ")
            .map(|pos| pos.split_once(",").expect("Malformed position"))
            .map(|(p1, p2)| (p1.parse::<u32>().unwrap(), p2.parse::<u32>().unwrap()))
            .collect::<Vec<(u32, u32)>>()
            .windows(2)
            .for_each(|w| add_wall(&mut walls, &w[0], &w[1]))
        );
    walls
}

fn part1(walls: &HashSet<(u32, u32)>) -> usize {
    let max_y = walls.iter().map(|w| w.1).max().expect("No walls");
    let mut blocked: HashSet<(u32, u32)> = HashSet::new();

    loop {
        let (mut x, mut y) = (500, 0);

        'fall: loop {
            if y == max_y {
                return blocked.len();
            }

            // Move down
            if !walls.contains(&(x, y + 1)) && !blocked.contains(&(x, y + 1)) {
                y += 1;
            }
            // Move down-left
            else if !walls.contains(&(x - 1, y + 1)) && !blocked.contains(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
            }
            // Move down-right
            else if !walls.contains(&(x + 1, y + 1)) && !blocked.contains(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
            } else {
                blocked.insert((x, y));
                break 'fall;
            }
        }
    }
}

fn part2(walls: &HashSet<(u32, u32)>) -> usize {
    let max_y = walls.iter().map(|w| w.1).max().expect("No walls");
    let mut blocked: HashSet<(u32, u32)> = HashSet::new();

    loop {
        let (mut x, mut y) = (500, 0);

        'fall: loop {
            // Move down
            if y < max_y + 1 && !walls.contains(&(x, y + 1)) && !blocked.contains(&(x, y + 1)) {
                y += 1;
            }
            // Move down-left
            else if y < max_y + 1 && !walls.contains(&(x - 1, y + 1)) && !blocked.contains(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
            }
            // Move down-right
            else if y < max_y + 1 && !walls.contains(&(x + 1, y + 1)) && !blocked.contains(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
            } else {
                blocked.insert((x, y));
                if x == 500 && y == 0 {
                    return blocked.len();
                }
                break 'fall;
            }
        }
    }
}
