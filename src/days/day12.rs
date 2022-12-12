#[test]
fn test() {
    println!("Day 12");

    let input = parse_input();

    let p1 = part1(&input);
    println!("Part 1 -> {}", p1);
    assert_eq!(p1, 361);

    let p2 = part2(&input);
    println!("Part 2 -> {}", p2);
    assert_eq!(p2, 354);
}

struct Grid {
    values: Vec<Vec<u8>>,
    starting: Vec<(usize, usize)>,
    from: (usize, usize),
    to: (usize, usize),
}

impl Grid {
    fn can_move(&self, x1: usize, y1: usize, x2: usize, y2: usize, up: bool) -> bool {
        let v2 = self.values[y2][x2];
        let v1 = self.values[y1][x1];
        match up {
            true => v1 <= v2 || v2 < v1 && v1 - v2 == 1,
            false => v2 <= v1 || v1 < v2 && v2 - v1 == 1
        }
    }
}

fn parse_input() -> Grid {
    let mut from = (0, 0);
    let mut to = (0, 0);
    let mut starting = vec![];
    let values = include_str!("data/day12")
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars().enumerate().map(|(x, c)| {
                match c {
                    'S' => { from = (x, y); starting.push((x, y)); 'a' as u8 },
                    'E' => { to = (x, y); 'z' as u8 },
                    'a' => { starting.push((x, y)); 'a' as u8 }
                    _   => c as u8
                }
            })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    Grid {
        values,
        starting,
        from,
        to,
    }
}

fn distances(grid: &Grid, start: (usize, usize), up: bool) -> Vec<Vec<u16>> {
    let y_max = grid.values.len();
    let x_max = grid.values[0].len();
    let mut dist = vec![vec![u16::MAX - 1; x_max]; y_max];
    dist[start.1][start.0] = 0;
    let mut updated = dist.clone();
    let mut changed = true;
    let mut changelist = vec![];
    while changed {
        changed = false;
        for y in 0..y_max {
            for x in 0..x_max {
                let mut possible = vec![];

                // Just pretend I don't recompute the same values each time 🙏
                if x > 0 && grid.can_move(x, y, x - 1, y, up) { possible.push(dist[y][x - 1] + 1); }
                if y > 0 && grid.can_move(x, y, x, y - 1, up) { possible.push(dist[y - 1][x] + 1); }
                if x < x_max - 1 && grid.can_move(x, y, x + 1, y, up) { possible.push(dist[y][x + 1] + 1); }
                if y < y_max - 1 && grid.can_move(x, y, x, y + 1, up) { possible.push(dist[y + 1][x] + 1); }

                if let Some(&n) = possible.iter().min() {
                    if n < dist[y][x] {
                        changed = true;
                        updated[y][x] = n;
                        changelist.push((x, y, n));
                    }
                }
            }
        }
        changelist.iter().for_each(|(x, y, val)| dist[*y][*x] = *val);
        changelist.clear();
    }
    dist
}

fn part1(input: &Grid) -> u16 {
    distances(input, input.from, true)[input.to.1][input.to.0]
}

fn part2(input: &Grid) -> u16 {
    let distances = distances(input, input.to, false);
    input
        .starting
        .iter()
        .map(|&(x, y)| distances[y][x])
        .min()
        .unwrap()
}
