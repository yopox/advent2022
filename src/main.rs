use std::time::Instant;

mod days;

///```
///              _               _
///           _|  |   AOC      |  | _
///         |  |  |     2022   |  |  |
///   ____  |  |  |__        __|  |  |   ____
///  \_   \ |  |  |  |––..––|  |  |  | /   _/
///     \  \|  |  |  |  ||  |  |  |  |/  /
///      \              /\              /
///       \            /  \            /
///```

fn main() {}

pub fn benchmark<T, U>(input: T, p1: fn(&T) -> U, p2: fn(&T) -> U) {
    const N: u128 = 2000;
    let t0 = Instant::now();
    for _ in 0..N {
        p1(&input);
    }
    println!("Part 1 -> {}µs", t0.elapsed().as_micros() / N);

    let t0 = Instant::now();
    for _ in 0..N {
        p2(&input);
    }
    println!("Part 2 -> {}µs", t0.elapsed().as_micros() / N);
}