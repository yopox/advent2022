#[test]
fn test() {
    println!("Day 22");

    let (map, commands) = parse_input();

    let p1 = part1(&map, &commands);
    println!("Part 1 -> {}", p1);
    assert_eq!(p1, 76332);

    let p2 = part2(&map, &commands);
    println!("Part 2 -> {}", p2);
    assert_eq!(p2, 144012);
}

#[derive(PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_move(&self) -> (i16, i16) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
}

enum Command {
    TurnR,
    TurnL,
    Forward(u16),
}

impl Command {
    fn from_str(str: &str) -> Self {
        match str {
            "L" => Command::TurnL,
            "R" => Command::TurnR,
            _ => Command::Forward(str.parse::<u16>().expect("Can't parse command number")),
        }
    }
}

fn parse_input() -> (Vec<Vec<char>>, Vec<Command>) {
    let (map, commands) = include_str!("data/day22")
        .split_once("\n\n")
        .expect("Can't find commands");

    let map = map.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let commands = commands
        .replace("L", " L ")
        .replace("R", " R ")
        .split_whitespace()
        .map(|c| Command::from_str(c))
        .collect::<Vec<Command>>();
    (map, commands)
}

fn part1(map: &Vec<Vec<char>>, commands: &Vec<Command>) -> usize {
    let mut direction = Direction::Right;
    let mut x = map[0].iter().enumerate().find(|&(i, c)| *c == '.').unwrap().0;
    let mut y = 0;

    for command in commands {
        match command {
            Command::TurnR => direction = direction.right(),
            Command::TurnL => direction = direction.left(),
            Command::Forward(n) => (x, y) = forward(x, y, map, &direction, n)
        }
    }

    1000 * (y + 1) + 4 * (x + 1) + match direction {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    }
}

fn forward(x: usize, y: usize, map: &Vec<Vec<char>>, dir: &Direction, amount: &u16) -> (usize, usize) {
    let (mut x, mut y) = (x, y);
    let line: Vec<&char> = match dir {
        Direction::Up | Direction::Down => map
            .iter()
            .map(|row| if row.len() > x { &row[x] } else { &' ' })
            .collect(),
        Direction::Left | Direction::Right => map[y]
            .iter()
            .collect(),
    };
    let wrap_pos = line.iter().enumerate().rev().find(|&(i, c)| **c != ' ').unwrap().0;
    let first_pos = line.iter().enumerate().find(|&(i, c)| **c != ' ').unwrap().0;
    for i in 0..*amount {
        match dir {
            Direction::Up => {
                if y > 0 && map[y-1][x] == '#' { return (x, y); }
                if y == first_pos && *line[wrap_pos] == '#' { return (x, y); }
                else if y == first_pos && *line[wrap_pos] == '.' { y = wrap_pos; }
                else { y -= 1; }
            }
            Direction::Down => {
                if y < wrap_pos && map[y+1][x] == '#' { return (x, y); }
                if y == wrap_pos && *line[first_pos] == '#' { return (x, y); }
                else if y == wrap_pos && *line[first_pos] == '.' { y = first_pos; }
                else { y += 1; }
            }
            Direction::Left => {
                if x > 0 && map[y][x-1] == '#' { return (x, y); }
                if x == first_pos && *line[wrap_pos] == '#' { return (x, y); }
                else if x == first_pos && *line[wrap_pos] == '.' { x = wrap_pos; }
                else { x -= 1; }
            }
            Direction::Right => {
                if x < wrap_pos && map[y][x+1] == '#' { return (x, y); }
                if x == wrap_pos && *line[first_pos] == '#' { return (x, y); }
                else if x == wrap_pos && *line[first_pos] == '.' { x = first_pos; }
                else { x += 1; }
            }
        }
    }
    (x, y)
}

fn part2(map: &Vec<Vec<char>>, commands: &Vec<Command>) -> usize {
    let mut direction = Direction::Right;
    let mut x = map[0].iter().enumerate().find(|&(i, c)| *c == '.').unwrap().0;
    let mut y = 0;

    for command in commands {
        match command {
            Command::TurnR => direction = direction.right(),
            Command::TurnL => direction = direction.left(),
            Command::Forward(n) => (x, y, direction) = forward_p2(x, y, map, &direction, *n)
        }
    }

    // let (x0, y0) = (50, 50);
    // println!("{:?}", (x0, y0));
    // let (_, _, x1, y1) = face_wrap(&Direction::Left, x0, y0).unwrap();
    // println!("{:?}", (x1, y1));

    1000 * (y + 1) + 4 * (x + 1) + match direction {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    }
}

fn face(x: usize, y: usize) -> usize {
    match (x / 50, y / 50) {
        (1, 0) => 0,
        (2, 0) => 1,
        (1, 1) => 2,
        (0, 2) => 3,
        (1, 2) => 4,
        (0, 3) => 5,
        _ => panic!("Not on the cube")
    }
}

fn face_wrap(direction: &Direction, x: usize, y: usize) -> Option<(usize, Direction, usize, usize)> {
    match (face(x, y), direction) {
        (0, Direction::Up) => Some((5, Direction::Right, 0, x + 100)),
        (5, Direction::Left) => Some((0, Direction::Down, y - 100, 0)),

        (0, Direction::Left) => Some((3, Direction::Right, 0, 49 - y + 100)),
        (3, Direction::Left) => Some((0, Direction::Right, 50, 49 - (y - 100))),

        (1, Direction::Down) => Some((2, Direction::Left, 99, x - 50)),
        (2, Direction::Right) => Some((1, Direction::Up, y + 50, 49)),

        (1, Direction::Up) => Some((5, Direction::Up, x - 100, 199)),
        (5, Direction::Down) => Some((1, Direction::Down, x + 100, 0)),

        (1, Direction::Right) => Some((4, Direction::Left, 99, 49 - y + 100)),
        (4, Direction::Right) => Some((1, Direction::Left, 149, 49 - (y - 100))),

        (2, Direction::Left) => Some((3, Direction::Down, y - 50, 100)),
        (3, Direction::Up) => Some((2, Direction::Right, 50, x + 50)),

        (4, Direction::Down) => Some((5, Direction::Left, 49, x + 100)),
        (5, Direction::Right) => Some((4, Direction::Up, y - 100, 149)),

        (_, _) => None
    }
}

fn forward_p2(x: usize, y: usize, map: &Vec<Vec<char>>, dir: &Direction, amount: u16) -> (usize, usize, Direction) {
    let (mut x, mut y) = (x, y);
    let line: Vec<&char> = match dir {
        Direction::Up | Direction::Down => map
            .iter()
            .map(|row| if row.len() > x { &row[x] } else { &' ' })
            .collect(),
        Direction::Left | Direction::Right => map[y]
            .iter()
            .collect(),
    };
    let wrap_pos = line.iter().enumerate().rev().find(|&(i, c)| **c != ' ').unwrap().0;
    let first_pos = line.iter().enumerate().find(|&(i, c)| **c != ' ').unwrap().0;

    for i in 0..amount {
        if *dir == Direction::Up && y == first_pos
            || *dir == Direction::Down && y == wrap_pos
            || *dir == Direction::Left && x == first_pos
            || *dir == Direction::Right && x == wrap_pos {
            let (_, new_dir, new_x, new_y) = face_wrap(dir, x, y).expect("Can't wrap");
            if map[new_y][new_x] == '#' { return (x, y, dir.clone()); }
            return forward_p2(new_x, new_y, map, &new_dir, amount - i - 1);
        }

        match dir {
            Direction::Up => {
                if y > 0 && map[y-1][x] == '#' { return (x, y, dir.clone()); }
                else { y -= 1; }
            }
            Direction::Down => {
                if y < wrap_pos && map[y+1][x] == '#' { return (x, y, dir.clone()); }
                else { y += 1; }
            }
            Direction::Left => {
                if x > 0 && map[y][x-1] == '#' { return (x, y, dir.clone()); }
                else { x -= 1; }
            }
            Direction::Right => {
                if x < wrap_pos && map[y][x+1] == '#' { return (x, y, dir.clone()); }
                else { x += 1; }
            }
        }
    }
    (x, y, dir.clone())
}