use regex::Captures;
use regex::Regex;

#[test]
pub fn test() {
    println!("Day 19");

    let input = parse_input();

    let p1 = part1(&input);
    println!("Part 1 -> {}", p1);
    assert_eq!(p1, 1147);

    let p2 = part2(&input);
    println!("Part 2 -> {}", p2);
    assert_eq!(p2, 3080);
}

struct Blueprint {
    ore_robot: Resources,
    clay_robot: Resources,
    obsidian_robot: Resources,
    geode_robot: Resources,
}

impl Blueprint {
    fn from_captures(regex: Captures) -> Self {
        Blueprint {
            ore_robot: Resources {
                ore: regex[1].parse::<usize>().unwrap(),
                ..Default::default()
            },
            clay_robot: Resources {
                ore: regex[2].parse::<usize>().unwrap(),
                ..Default::default()
            },
            obsidian_robot: Resources {
                ore: regex[3].parse::<usize>().unwrap(),
                clay: regex[4].parse::<usize>().unwrap(),
                ..Default::default()
            },
            geode_robot: Resources {
                ore: regex[5].parse::<usize>().unwrap(),
                obsidian: regex[6].parse::<usize>().unwrap(),
                ..Default::default()
            },
        }
    }
}

fn parse_input() -> Vec<Blueprint> {
    let regex = Regex::new(r"Blueprint (?:[0-9]*): Each ore robot costs ([0-9]*) ore. Each clay robot costs ([0-9]*) ore. Each obsidian robot costs ([0-9]*) ore and ([0-9]*) clay. Each geode robot costs ([0-9]*) ore and ([0-9]*) obsidian.").unwrap();

    include_str!("data/day19")
        .lines()
        .map(|l| Blueprint::from_captures(regex.captures(l).expect("Can't parse blueprint.")))
        .collect()
}

#[derive(Copy, Clone, Default)]
struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geodes: usize,
}

impl Resources {
    fn can_build(&self, unusable: &Resources, cost: &Resources) -> bool {
        if cost.ore > 0 && self.ore - unusable.ore < cost.ore { return false; }
        if cost.clay > 0 && self.clay - unusable.clay < cost.clay { return false; }
        if cost.obsidian > 0 && self.obsidian - unusable.obsidian < cost.obsidian { return false; }
        return true;
    }

    fn substract(&self, cost: &Resources) -> Resources {
        Resources {
            ore: self.ore - cost.ore,
            clay: self.clay - cost.clay,
            obsidian: self.obsidian - cost.obsidian,
            geodes: self.geodes - cost.geodes,
        }
    }
}

fn max_geodes_p1(blueprint: &Blueprint) -> usize {
    let mut paths = vec![(Resources { ore: 1, ..Default::default() }, Resources { ..Default::default() })];
    println!("blueprint");
    for i in 0..24 {
        let max_g = paths.iter().max_by_key(|&&(machines, resources)| resources.geodes).unwrap().1.geodes;
        let mut new_paths = vec![];
        for (machines, inventory) in paths.iter_mut() {
            if inventory.geodes < max_g { continue }

            // Collect
            inventory.ore += machines.ore;
            inventory.clay += machines.clay;
            inventory.obsidian += machines.obsidian;
            inventory.geodes += machines.geodes;

            // Skip branches where it's too late to build geodes
            if i == 22
                && machines.geodes == 0
                && machines.obsidian == 0
                && inventory.obsidian < blueprint.geode_robot.obsidian { continue }

            if i == 21
                && machines.geodes == 0
                && machines.obsidian == 0
                && inventory.clay < blueprint.obsidian_robot.clay { continue }

            new_paths.push((*machines, *inventory));

            // Build a robot
            if i == 23 { continue }
            if inventory.can_build(machines, &blueprint.geode_robot) {
                let mut new_robots = machines.clone();
                new_robots.geodes += 1;
                new_paths.push((new_robots, inventory.substract(&blueprint.geode_robot)));
            } else {
                if inventory.can_build(machines, &blueprint.obsidian_robot) {
                    let mut new_robots = machines.clone();
                    new_robots.obsidian += 1;
                    new_paths.push((new_robots, inventory.substract(&blueprint.obsidian_robot)));
                } else {
                    if inventory.can_build(machines, &blueprint.ore_robot) {
                        let mut new_robots = machines.clone();
                        new_robots.ore += 1;
                        new_paths.push((new_robots, inventory.substract(&blueprint.ore_robot)));
                    }
                    if inventory.can_build(machines, &blueprint.clay_robot) {
                        let mut new_robots = machines.clone();
                        new_robots.clay += 1;
                        new_paths.push((new_robots, inventory.substract(&blueprint.clay_robot)));
                    }
                }
            }
        }
        paths = new_paths;
    }
    paths.iter().max_by_key(|&&(machines, resources)| resources.geodes).unwrap().1.geodes
}

fn max_geodes_p2(blueprint: &Blueprint) -> usize {
    let mut paths = vec![(Resources { ore: 1, ..Default::default() }, Resources { ..Default::default() })];
    println!("blueprint");
    for i in 0..32 {
        let max_g = paths.iter().max_by_key(|&&(machines, resources)| resources.geodes).unwrap().1.geodes;
        let mut new_paths = vec![];
        for (machines, inventory) in paths.iter_mut() {
            if inventory.geodes < max_g { continue; }

            // Collect
            inventory.ore += machines.ore;
            inventory.clay += machines.clay;
            inventory.obsidian += machines.obsidian;
            inventory.geodes += machines.geodes;

            new_paths.push((*machines, *inventory));

            // Build a robot
            let build_ore_clay = i < 20;
            if inventory.can_build(machines, &blueprint.geode_robot) {
                let mut new_robots = machines.clone();
                new_robots.geodes += 1;
                new_paths.push((new_robots, inventory.substract(&blueprint.geode_robot)));
            }
            if inventory.can_build(machines, &blueprint.obsidian_robot) {
                let mut new_robots = machines.clone();
                new_robots.obsidian += 1;
                new_paths.push((new_robots, inventory.substract(&blueprint.obsidian_robot)));
            }
            if build_ore_clay && inventory.can_build(machines, &blueprint.ore_robot) {
                let mut new_robots = machines.clone();
                new_robots.ore += 1;
                new_paths.push((new_robots, inventory.substract(&blueprint.ore_robot)));
            }
            if build_ore_clay && inventory.can_build(machines, &blueprint.clay_robot) {
                let mut new_robots = machines.clone();
                new_robots.clay += 1;
                new_paths.push((new_robots, inventory.substract(&blueprint.clay_robot)));
            }
        }
        paths = new_paths;
    }
    let max_g = paths.iter().max_by_key(|&&(machines, resources)| resources.geodes).unwrap().1.geodes;
    max_g
}

fn part1(blueprints: &Vec<Blueprint>) -> usize {
    blueprints
        .iter()
        .enumerate()
        .map(|(i, b)| (i + 1) * max_geodes_p1(b))
        .sum()
}

fn part2(blueprints: &Vec<Blueprint>) -> usize {
    blueprints[0..3]
        .iter()
        .map(|b| max_geodes_p2(b))
        .product()
}
