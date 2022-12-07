use std::iter::Skip;
use std::str::Lines;

#[test]
fn test() {
    println!("Day 7");
    let p1 = part1();
    println!("Part 1 -> {}", p1);
    assert_eq!(p1, 1367870);

    let p2 = part2();
    println!("Part 2 -> {}", p2);
    assert_eq!(p2, 549173);
}

enum Files {
    Folder(String, Vec<Files>),
    File(u32),
}

impl Files {
    fn add_file(&mut self, size: u32) {
        match self {
            Files::Folder(_, f) => { f.push(Files::File(size)) }
            Files::File(_) => panic!("Can't add file to file")
        }
    }

    fn add_folder(&mut self, name: String) {
        match self {
            Files::Folder(_, f) => { f.push(Files::Folder(name, vec![])) }
            Files::File(_) => panic!("Can't add folder to file")
        }
    }
}

fn cd(args: &str, position: &mut Vec<String>) {
    match &args[..args.len() - 1] {
        "/" => {}
        ".." => { position.pop(); }
        x => { position.push(x.to_string()); }
    };
}

fn ls(args: Skip<Lines>, tree: &mut Files, path: &mut Vec<String>) {
    let mut folder = tree;
    // Navigate in the tree from folder to folder
    for f in path.iter() {
        folder = match folder {
            Files::Folder(_, current_files) => {
                current_files.iter_mut().find(|x| match x {
                    Files::Folder(name, _) => name == f,
                    _ => false
                }).unwrap()
            },
            _ => panic!["Can't find folder"]
        };
    }
    // Add files and folder revealed in the current folder
    args
        .map(|line| line.split_once(" ").unwrap())
        .for_each(|(size, name)| {
            match size.parse::<u32>() {
                Ok(size) => if size > 0 { folder.add_file(size); },
                Err(..) => folder.add_folder(name.to_string()),
            }
        });
}

fn parse_files() -> Files {
    let mut position = vec![];
    let mut folders = Files::Folder("root".to_string(), vec![]);

    include_str!("data/day7")
        .split("$ ")
        .skip(1)
        .for_each(|s| {
            match &s[0..2] {
                "cd" => cd(s.split_once(" ").unwrap().1, &mut position),
                _ => ls(s.lines().skip(1), &mut folders, &mut position)
            }
        });

    folders
}

fn get_sizes(file: &Files) -> Vec<u32> {
    return match file {
        Files::Folder(_, f) => {
            let mut sizes: Vec<u32> = vec![];
            f.iter().for_each(|subf| sizes.append(&mut get_sizes(subf)));
            let mut size = get_size(file);
            sizes.push(size);
            sizes
        },
        Files::File(_) => vec![]
    }
}

fn get_size(file: &Files) -> u32 {
    return match file {
        Files::Folder(_, f) => f.iter().map(|f| get_size(f)).sum::<u32>(),
        Files::File(s) => *s
    }
}

fn part1() -> u32 {
    get_sizes(&parse_files())
        .iter()
        .filter(|s| **s <= 100000)
        .sum()
}

fn part2() -> u32 {
    let mut sizes = get_sizes(&parse_files());
    let available = 70000000 - sizes.last().unwrap();
    let missing = 30000000;
    sizes.sort();
    *sizes.iter().find(|s| **s + available >= missing).expect("No such folder")
}
