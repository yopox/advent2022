use itertools::Itertools;
use crate::benchmark;

#[test]
fn test() {
    println!("Day 8");

    let input = get_trees();

    let p1 = part1(&input);
    println!("Part 1 -> {}", p1);
    assert_eq!(p1, 1823);

    let p2 = part2(&input);
    println!("Part 2 -> {}", p2);
    assert_eq!(p2, 211680);

    // benchmark(input, |i| part1(i), |i| part2(i));
}

#[derive(Debug)]
enum Directions {
    North,
    East,
    South,
    West,
}

impl Directions {
    fn get_index(&self, n: usize, m: usize, size: usize) -> (usize, usize) {
        match self {
            Directions::North => (n, m),
            Directions::West => (m, n),
            Directions::South => (n, size - m - 1),
            Directions::East => (size - m - 1, n),
        }
    }
}

fn visible_trees(trees: &Vec<Vec<u32>>, direction: &Directions) -> Vec<(usize, usize)> {
    let mut visible = vec![];
    let mut tallest: i32;
    let size = trees.len();
    for n in 0..size {
        tallest = -1;
        for m in 0..size {
            let (x, y) = direction.get_index(n, m, size);
            let tree = trees[y][x];
            if tree as i32 > tallest { visible.push((x, y)); tallest = tree as i32; }
        }
    }
    visible
}

fn get_trees() -> Vec<Vec<u32>> {
    include_str!("data/day8")
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
        .collect()
}

fn part1(input: &Vec<Vec<u32>>) -> usize {
    [Directions::North, Directions::East, Directions::South, Directions::West]
        .iter()
        .flat_map(|d| visible_trees(input, d))
        .unique()
        .count()
}

fn score(trees: &Vec<Vec<u32>>, direction: &Directions) -> Vec<(usize, usize, usize)> {
    let mut score = vec![];
    let size = trees.len();
    for n in 0..size {
        let mut before = vec![];
        for m in 0..size {
            let (x, y) = direction.get_index(n, m, size);
            let tree = trees[y][x];
            before.push(tree);

            // Tree on the border -> null score
            if x == 0 || y == 0 || x == size - 1 || y == size - 1 {
                score.push((x, y, 0));
                continue
            }

            // Find blocking tree in the direction
            let tree_score = before
                .iter()
                .rev()
                .enumerate()
                .skip(1)
                .find(|(_, t)| **t >= tree)
                .unwrap_or((before.len() - 1, &0))
                .0;
            score.push((x, y, tree_score));
        }
    }
    score
}

fn part2(input: &Vec<Vec<u32>>) -> usize {
    let len = input.len();
    let mut scores = vec![vec![1 as usize; len]; len];
    [Directions::North, Directions::East, Directions::South, Directions::West]
        .iter()
        // Get scores for all trees for each direction
        .flat_map(|d| score(input, d))
        // Multiply scores from all directions
        .for_each(|(x, y, score)| scores[y][x] *= score);
    *scores
        .iter()
        .flatten()
        .max()
        .unwrap()
}
