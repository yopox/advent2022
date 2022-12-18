use std::collections::HashSet;

#[test]
fn test() {
    println!("Day 18");

    let input = parse_input();

    let p1 = part1(&input);
    println!("Part 1 -> {}", p1);
    assert_eq!(p1, 3390);

    let p2 = part2(&input);
    println!("Part 2 -> {}", p2);
    assert_eq!(p2, 2058);
}

fn parse_input() -> HashSet<(i32, i32, i32)> {
    include_str!("data/day18")
        .lines()
        .map(|l| {
            let split: Vec<i32> = l.split(",").map(|n| n.parse::<i32>().unwrap()).collect();
            (split[0], split[1], split[2])
        })
        .collect()
}

fn part1(input: &HashSet<(i32, i32, i32)>) -> i32 {
    input
        .iter()
        .map(|&(x, y, z)| {
            let mut connected = 0;
            if input.contains(&(x + 1, y, z)) { connected += 1; }
            if input.contains(&(x - 1, y, z)) { connected += 1; }
            if input.contains(&(x, y + 1, z)) { connected += 1; }
            if input.contains(&(x, y - 1, z)) { connected += 1; }
            if input.contains(&(x, y, z + 1)) { connected += 1; }
            if input.contains(&(x, y, z - 1)) { connected += 1; }
            6 - connected
        })
        .sum()
}

fn part2(input: &HashSet<(i32, i32, i32)>) -> i32 {
    let mut water = HashSet::new();
    let mut faces = 0;

    let min_x = input.iter().min_by_key(|&(x, y, z)| x).unwrap().0;
    let max_x = input.iter().max_by_key(|&(x, y, z)| x).unwrap().0;
    let min_y = input.iter().min_by_key(|&(x, y, z)| y).unwrap().1;
    let max_y = input.iter().max_by_key(|&(x, y, z)| y).unwrap().1;
    let min_z = input.iter().min_by_key(|&(x, y, z)| z).unwrap().2;
    let max_z = input.iter().max_by_key(|&(x, y, z)| z).unwrap().2;

    let mut to_log = vec![(min_x - 1, min_y - 1, min_z - 1)];

    while to_log.len() > 0 {
        let mut next_log = vec![];
        for (lx, ly, lz) in to_log {
            if water.contains(&(lx, ly, lz)) { continue }
            water.insert((lx, ly, lz));

            if input.contains(&(lx + 1, ly, lz)) {
                faces += 1;
            } else if lx + 1 <= max_x + 1 { next_log.push((lx + 1, ly, lz)); }
            if input.contains(&(lx - 1, ly, lz)) {
                faces += 1;
            } else if lx - 1 >= min_x - 1 { next_log.push((lx - 1, ly, lz)); }

            if input.contains(&(lx, ly + 1, lz)) {
                faces += 1;
            } else if ly + 1 <= max_y + 1 { next_log.push((lx, ly + 1, lz)); }
            if input.contains(&(lx, ly - 1, lz)) {
                faces += 1;
            } else if ly - 1 >= min_y - 1 { next_log.push((lx, ly - 1, lz)); }


            if input.contains(&(lx, ly, lz + 1)) {
                faces += 1;
            } else if lz + 1 <= max_z + 1 { next_log.push((lx, ly, lz + 1)); }
            if input.contains(&(lx, ly, lz - 1)) {
                faces += 1;
            } else if lz - 1 >= min_z - 1 { next_log.push((lx, ly, lz - 1)); }
        }
        to_log = next_log;
    }

    faces
}
