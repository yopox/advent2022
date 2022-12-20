#[test]
pub fn test() {
    println!("Day 20");

    let input = parse_input();

    let p1 = part1(&input);
    println!("Part 1 -> {}", p1);
    assert_eq!(p1, 6640);

    let p2 = part2(&input);
    println!("Part 2 -> {}", p2);
    assert_eq!(p2, 11893839037215);
}

fn parse_input() -> Vec<i16> {
    include_str!("data/day20")
        .lines()
        .map(|l| l.parse::<i16>().expect("Can't parse number"))
        .collect()
}

struct Order {
    next: usize,
    previous: usize,
    first: bool,
}

impl Order {
    fn shift(vec: &mut Vec<Order>, i: usize, by: i64, len: usize) {
        if by == 0 { return; }
        // "Disconnect" i
        let (p, n) = (vec[i].previous, vec[i].next);
        vec[p].next = vec[i].next;
        vec[n].previous = vec[i].previous;
        if vec[i].first {
            vec[i].first = false;
            if by > 0 { vec[n].first = true; }
            else { vec[p].first = true; }
        }

        let new_i = if by > 0 {
            let diff = by as usize % (len - 1);
            Order::linked(&vec, p, diff)
            // Order::linked(&vec, i, by.abs() as usize)
        } else {
            let diff = by.abs() as usize % (len - 1);
            Order::linked_minus(&vec, p, diff)
            // Order::linked_minus(&vec, i, by.abs() as usize + 1)
        };
        // Connect i after new_i
        vec[i].previous = new_i;
        let n = vec[new_i].next;
        vec[i].next = n;
        vec[new_i].next = i;
        vec[n].previous = i;
    }

    fn linked(vec: &Vec<Order>, from: usize, steps: usize) -> usize {
        let mut i = from;
        for _ in 0..steps {
            i = vec[i].next;
        }
        // println!("linked({}, {}) -> {}", from, steps, i);
        i
    }

    fn linked_minus(vec: &Vec<Order>, from: usize, steps: usize) -> usize {
        let mut i = from;
        for _ in 0..steps {
            i = vec[i].previous;
        }
        // println!("linked_minus({}, {}) -> {}", from, steps, i);
        i
    }
}

fn part1(input: &Vec<i16>) -> i16 {
    let mut order = vec![];
    let len = input.len();

    for i in 0..input.len() {
        order.push(Order {
            next: (i + 1) % len,
            previous: (i + len - 1) % len,
            first: i == 0,
        })
    }

    let mut first = 0;

    for i in 0..input.len() {
        Order::shift(&mut order, i, input[i] as i64, len);
        first = order.iter().enumerate().find(|(i, o)| o.first).unwrap().0;
    }

    first = input.iter().enumerate().find(|(i, o)| **o == 0).unwrap().0;
    input[Order::linked(&order, first, 1000)]
    + input[Order::linked(&order, first, 2000)]
    + input[Order::linked(&order, first, 3000)]
}

fn part2(input: &Vec<i16>) -> i64 {
    let key = 811589153;
    let input: Vec<i64> = input.iter().map(|i| *i as i64 * key).collect();
    let mut order = vec![];
    let len = input.len();

    for i in 0..input.len() {
        order.push(Order {
            next: (i + 1) % len,
            previous: (i + len - 1) % len,
            first: i == 0,
        })
    }

    let mut first = 0;

    for _ in 0..10 {
        for i in 0..input.len() {
            Order::shift(&mut order, i, input[i], len);
            first = order.iter().enumerate().find(|(i, o)| o.first).unwrap().0;
        }
    }

    first = input.iter().enumerate().find(|(i, o)| **o == 0).unwrap().0;
    input[Order::linked(&order, first, 1000)]
        + input[Order::linked(&order, first, 2000)]
        + input[Order::linked(&order, first, 3000)]
}
