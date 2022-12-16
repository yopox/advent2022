use regex::{Captures, Regex};

#[test]
fn test() {
    println!("Day 16");

    let input = parse_input();

    let p1 = part1(&input);
    println!("Part 1 -> {}", p1);
    assert_eq!(p1, 1474);

    let p2 = part2(&input);
    println!("Part 2 -> {}", p2);
    assert_eq!(p2, 2100);
}

struct Valve {
    name: String,
    flow: u32,
    connections: Vec<String>,
}

impl Valve {
    fn from_captures(regex: Captures) -> Self {
        Valve {
            name: regex[1].parse::<String>().expect("Can't parse valve ID"),
            flow: regex[2].parse::<u32>().expect("Can't parse valve flow"),
            connections: regex[3].parse::<String>().expect("Can't parse valve connections").split(", ")
                .map(|s| s.to_string()).collect(),
        }
    }
}

fn parse_input() -> Vec<Valve> {
//     Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    let regex = Regex::new(r"Valve ([A-Z]+) has flow rate=([0-9]*); tunnels? leads? to valves? ([A-Z ,]+)").unwrap();

    include_str!("data/day16")
        .lines()
        .map(|l| Valve::from_captures(regex.captures(l).expect("Can't parse valve data")))
        .collect()
}

fn part1(valves: &Vec<Valve>) -> u32 {
    let mut time = 30;
    let mut solutions: Vec<(String, String, u32, Vec<(String, u32)>)> = vec![("AA".to_string(), "".to_string(), 0, vec![])];

    while time >= 1 {
        let mut new_solutions = vec![];
        time -= 1;
        for (current, before, score, opened) in solutions.iter_mut() {
            let current_valve = valves.iter().find(|v| v.name.eq(current)).unwrap();
            // Open current valve
            if opened.iter().find(|(v, _)| current == v).is_none() && current_valve.flow > 0 {
                let mut new_opened = opened.clone();
                new_opened.push((current.to_string(), time));
                new_solutions.push((current.to_string(), "".to_string(), *score + time * current_valve.flow, new_opened));
            }
            // Move to a new valve
            else {
                for connection in current_valve.connections.iter() {
                    if current_valve.connections.len() == 1 || connection != before {
                        new_solutions.push((connection.to_string(), current.to_string(), *score, opened.clone()));
                    }
                }
            }
        }
        solutions = new_solutions;
    }
    let solution = solutions.iter().max_by_key(|(_, _, o, _)| *o).unwrap();
    // solution.3.iter().for_each(|(s, t)| println!("Open {} at t={}", s, t));
    solution.2
}

fn part2(valves: &Vec<Valve>) -> u32 {
    let mut time = 26;
    let mut solutions: Vec<(String, String, String, String, u32, Vec<(String, u32)>)> = vec![("AA".to_string(), "".to_string(), "AA".to_string(), "".to_string(), 0, vec![])];

    while time >= 1 {
        let mut new_solutions: Vec<(String, String, String, String, u32, Vec<(String, u32)>)> = vec![];
        time -= 1;
        for (current, before, current_e, before_e, score, opened) in solutions.iter_mut() {
            if time == 13 && *score < 800 { continue }
            if time == 10 && *score < 1500 { continue }
            let current_valve = valves.iter().find(|v| v.name.eq(current)).unwrap();
            let current_valve_e = valves.iter().find(|v| v.name.eq(current_e)).unwrap();

            let mut open_valve = false;
            let mut open_valve_e = false;

            // Open current valve
            if opened.iter().find(|(v, _)| current == v).is_none() && current_valve.flow > 0 {
                open_valve = true;
                opened.push((current.to_string(), time));
                *score += time * current_valve.flow;
            }

            // Open current valve (elephant)
            if opened.iter().find(|(v, _)| current_e == v).is_none() && current_valve_e.flow > 0 {
                open_valve_e = true;
                opened.push((current_e.to_string(), time));
                *score += time * current_valve_e.flow;
            }

            if open_valve && open_valve_e {
                new_solutions.push((
                    current.to_string(), "".to_string(),
                    current_e.to_string(), "".to_string(),
                    *score, opened.clone()
                ));
            } else if open_valve {
                for connection in current_valve_e.connections.iter() {
                    if current_valve_e.connections.len() == 1 || connection != before_e {
                        new_solutions.push((
                            current.to_string(), "".to_string(),
                            connection.to_string(), current_e.to_string(),
                            *score, opened.clone()
                        ));
                    }
                }
            } else if open_valve_e {
                for connection in current_valve.connections.iter() {
                    if current_valve.connections.len() == 1 || connection != before {
                        new_solutions.push((
                            connection.to_string(), current.to_string(),
                            current_e.to_string(), "".to_string(),
                            *score, opened.clone()
                        ));
                    }
                }
            } else {
                for connection in current_valve.connections.iter() {
                    for connection_e in current_valve_e.connections.iter() {
                        if (current_valve.connections.len() == 1 || connection != before)
                        && (current_valve_e.connections.len() == 1 || connection_e != before_e) {
                            new_solutions.push((
                                connection.to_string(), current.to_string(),
                                connection_e.to_string(), current_e.to_string(),
                                *score, opened.clone()
                            ));
                        }
                    }
                }
            }
        }
        solutions = new_solutions;
    }
    let solution = solutions.iter().max_by_key(|(_, _, _, _, o, _)| *o).unwrap();
    // solution.5.iter().for_each(|(s, t)| println!("Open {} at t={}", s, t));
    solution.4
}
