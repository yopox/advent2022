use std::collections::HashSet;
use crate::days::day17::Shape::{Corner, Cross, Horizontal, Square, Vertical};

#[test]
pub fn test() {
    println!("Day 17");

    let input = parse_input();

    let p1 = part1(&input);
    println!("Part 1 -> {}", p1);
    assert_eq!(p1, 3100);

    let p2 = part2(&input);
    println!("Part 2 -> {}", p2);
    assert_eq!(p2, 1540634005751);
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Shape {
    Horizontal,
    Cross,
    Corner,
    Vertical,
    Square,
}

impl Shape {
    fn colliders(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        match self {
            Horizontal => (0..=3).map(|dx| (x + dx, y)).collect(),
            Cross => vec![(x + 1, y + 2), (x, y + 1), (x + 1, y), (x + 2, y + 1)],
            Corner => vec![(x, y), (x + 1, y), (x + 2, y), (x + 2, y + 1), (x + 2, y + 2)],
            Vertical => (0..=3).map(|dy| (x, y + dy)).collect(),
            Square => vec![(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)],
        }
    }

    fn width(&self) -> usize {
        match self {
            Horizontal => 4,
            Cross => 3,
            Corner => 3,
            Vertical => 1,
            Square => 2,
        }
    }

    fn height(&self) -> usize {
        match self {
            Horizontal => 1,
            Cross => 3,
            Corner => 3,
            Vertical => 4,
            Square => 2,
        }
    }

    fn can_move_left(&self, x: usize, y: usize, rocks: &HashSet<(usize, usize)>) -> bool {
        x > 0 && match self {
            Horizontal => !rocks.contains(&(x - 1, y)),
            Corner => !rocks.contains(&(x - 1, y))
                && !rocks.contains(&(x + 1, y + 1))
                && !rocks.contains(&(x + 1, y + 2)),
            Cross => !rocks.contains(&(x - 1, y + 1))
                && !rocks.contains(&(x, y))
                && !rocks.contains(&(x, y + 2)),
            Square => (0..=1).all(|dy| !rocks.contains(&(x - 1, y + dy))),
            Vertical => (0..=3).all(|dy| !rocks.contains(&(x - 1, y + dy))),
        }
    }

    fn can_move_right(&self, x: usize, y: usize, width: usize, rocks: &HashSet<(usize, usize)>) -> bool {
        x + self.width() < width && match self {
            Horizontal => !rocks.contains(&(x + 4, y)),
            Cross => !rocks.contains(&(x + 3, y + 1))
                && !rocks.contains(&(x + 2, y))
                && !rocks.contains(&(x + 2, y + 2)),
            Vertical => (0..=3).all(|dy| !rocks.contains(&(x + 1, y + dy))),
            Square => (0..=1).all(|dy| !rocks.contains(&(x + 2, y + dy))),
            Corner => (0..=2).all(|dy| !rocks.contains(&(x + 3, y + dy))),
        }
    }

    fn can_move_down(&self, x: usize, y: usize, rocks: &HashSet<(usize, usize)>) -> bool {
        match self {
            Vertical => !rocks.contains(&(x, y - 1)),
            Cross => !rocks.contains(&(x + 1, y - 1))
                && !rocks.contains(&(x, y))
                && !rocks.contains(&(x + 2, y)),
            Square => (0..=1).all(|dx| !rocks.contains(&(x + dx, y - 1))),
            Corner => (0..=2).all(|dx| !rocks.contains(&(x + dx, y - 1))),
            Horizontal => (0..=3).all(|dx| !rocks.contains(&(x + dx, y - 1))),
        }
    }

    fn sequence() -> Vec<Shape> {
        vec![Horizontal, Cross, Corner, Vertical, Square]
    }
}

fn parse_input() -> Vec<char> {
    include_str!("data/day17")
        .chars()
        .collect()
}

fn part1(gas: &Vec<char>) -> usize {
    let mut rocks = HashSet::new();
    let width = 7;

    // |.......| 1
    // +-------+ 0
    //  0123456
    (0..=width).for_each(|x| { rocks.insert((x, 0)); });

    let mut max_y = 0;

    let order = Shape::sequence();
    let order_len = order.len();
    let gas_len = gas.len();
    let mut g = 0;
    for i in 0..2022 {
        let shape = &order[i % order_len];

        // Initial pos (bottom-left)
        let mut x = 2;
        let mut y = max_y + 5;

        'fall: loop {
            // Fall
            if shape.can_move_down(x, y, &rocks) { y -= 1; } else {
                shape.colliders(x, y).iter().for_each(|(cx, cy)| { rocks.insert((*cx, *cy)); });
                let shape_y_max = y + shape.height() - 1;
                if shape_y_max > max_y { max_y = shape_y_max; }
                break 'fall;
            }

            // Gas
            match gas[g % gas_len] {
                '>' => if shape.can_move_right(x, y, width, &rocks) { x += 1; },
                _ => if shape.can_move_left(x, y, &rocks) { x -= 1; }
            }
            g += 1;
        }
    }
    max_y
}

fn part2(gas: &Vec<char>) -> usize {
    let mut rocks = HashSet::new();
    let width = 7;

    // |.......| 1
    // +-------+ 0
    //  0123456
    (0..=width).for_each(|x| { rocks.insert((x, 0)); });

    let mut max_y = 0;
    let mut repeat_y = 0;
    let mut skipped = 0;

    let order = Shape::sequence();
    let order_len = order.len();
    let gas_len = gas.len();
    let mut g = 0;

    let mut i: usize = 0;
    let bound: usize = 1000000000000;
    let magic = order_len * 347; // I found the second factor manually
    while i <= bound {
        if i % magic == 0 {
            if repeat_y == 0 { repeat_y = max_y; }
            else {
                repeat_y = max_y - repeat_y;
                skipped = (bound - i) / magic;
                i += skipped * magic;
            }
        }

        let shape = &order[i % order_len];

        // Initial pos (bottom-left)
        let mut x = 2;
        let mut y = max_y + 5;

        'fall: loop {
            // Fall
            if shape.can_move_down(x, y, &rocks) { y -= 1; } else {
                shape.colliders(x, y).iter().for_each(|(cx, cy)| { rocks.insert((*cx, *cy)); });
                let shape_y_max = y + shape.height() - 1;
                if shape_y_max > max_y { max_y = shape_y_max; }
                break 'fall;
            }

            // Gas
            match gas[g % gas_len] {
                '>' => if shape.can_move_right(x, y, width, &rocks) { x += 1; },
                _ => if shape.can_move_left(x, y, &rocks) { x -= 1; }
            }
            g += 1;
        }
        i += 1;
    }
    max_y + skipped * repeat_y - 1
}