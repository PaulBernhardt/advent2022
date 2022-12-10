use std::{collections::HashMap, str::FromStr};
#[derive(Debug)]
struct CD {
    target: String,
}
// struct LS {}
// struct DIR {}
#[allow(dead_code)]
struct Directory {
    size: i32,
    // parents: Vec<Directory>,
    name: String,
    // files: Vec<File>,
}
impl Directory {
    fn new(name: String) -> Self {
        Directory {
            size: 0,
            name,
            // parents,
            // files: Vec::new(),
        }
    }
}
#[derive(Debug)]
struct DirectoryDetails {}
#[derive(Debug)]
#[allow(dead_code)]
struct FileDetails {
    name: String,
    size: i32,
}

#[derive(Debug)]
enum Command {
    CD(CD),
    LS,
}
#[derive(Debug)]
enum ConsoleLine {
    Command(Command),
    Directory(DirectoryDetails),
    File(FileDetails),
}

impl FromStr for ConsoleLine {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((first, rest)) = s.split_once(' ') {
            match first {
                "$" => {
                    // println!("{rest}");
                    if let Some((_command, path)) = rest.split_once(' ') {
                        return Ok(ConsoleLine::Command(Command::CD(CD {
                            target: path.to_string(),
                        })));
                    } else {
                        match rest {
                            "ls" => {
                                return Ok(ConsoleLine::Command(Command::LS));
                            }
                            _ => return Err(format!("Invalid command: {rest}")),
                        }
                    }
                }
                "dir" => {
                    return Ok(ConsoleLine::Directory(DirectoryDetails {
                        // name: rest.to_string(),
                    }));
                }
                _ => {
                    return Ok(ConsoleLine::File(FileDetails {
                        size: first.parse::<i32>().unwrap_or(-1),
                        name: rest.to_string(),
                    }))
                }
            }
        }
        Err(format!("Invalid string: '{s}'"))
    }
}

fn parse_commands(contents: &str) -> Result<Vec<ConsoleLine>, String> {
    let mut commands: Vec<ConsoleLine> = Vec::new();
    for line in contents.lines() {
        let line = line.parse::<ConsoleLine>()?;
        commands.push(line);
    }
    Ok(commands)
}

fn calc_directory_size(contents: &str) -> Result<HashMap<String, Directory>, String> {
    let commands = parse_commands(contents)?;
    let mut directories: HashMap<String, Directory> = HashMap::new();
    let mut current_directory_chain: Vec<String> = Vec::new();
    for line in commands {
        // println!("{:?}", line);
        match line {
            ConsoleLine::Command(command) => match command {
                Command::CD(cd) => {
                    let target = cd.target.as_str();
                    // println!("{target}");
                    match target {
                        ".." => {
                            current_directory_chain.pop();
                        }
                        _ => {
                            current_directory_chain.push(target.to_string());
                            let name = current_directory_chain.join("/");
                            if directories.get(&name).is_none() {
                                let directory = Directory::new(name.clone());
                                directories.insert(name, directory);
                            }
                        }
                    }
                }
                Command::LS => {}
            },
            ConsoleLine::Directory(_dir) => {}
            ConsoleLine::File(file_details) => {
                let mut dirs = current_directory_chain.to_vec();
                dirs.push("".to_string());

                while let Some(_dir) = dirs.pop() {
                    let name = dirs.join("/");
                    directories
                        .entry(name)
                        .or_insert_with_key(|name| Directory::new(name.clone()))
                        .size += file_details.size;
                }
            }
        }
    }
    Ok(directories)
}

pub fn solve_a(contents: &str) -> Result<String, String> {
    let directories = calc_directory_size(contents)?;
    let mut total = 0;
    for dir in directories.values() {
        // println!("Checking {} at {}", dir.name, dir.size);
        if dir.size <= 100000 {
            total += dir.size;
        }
    }
    Ok(total.to_string())
}

pub fn solve_b(contents: &str) -> Result<String, String> {
    let directories = calc_directory_size(contents)?;
    let total_space = 70000000;
    let needed_space = 30000000;
    let root = directories.get("/").unwrap();
    let used_space = total_space - root.size;
    let target = needed_space - used_space;

    let mut best = root.size;
    for dir in directories.values() {
        if dir.size < best && target - dir.size <= 0 {
            best = dir.size;
        }
    }
    Ok(format!("{best}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let contents = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k2";
        let result = solve_a(contents);
        assert_eq!(result.unwrap(), "95437");
    }

    #[test]
    fn test_b() {
        let contents = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k2";
        let result = solve_b(contents);
        assert_eq!(result.unwrap(), "24933642");
    }

    #[test]
    fn test_console_parsing() {
        let input = "$ cd /";
        let result = input.parse::<ConsoleLine>().unwrap();
        assert!(matches!(result, ConsoleLine::Command(Command::CD(_))));

        let input = "$ ls";
        let result = input.parse::<ConsoleLine>().unwrap();
        assert!(matches!(result, ConsoleLine::Command(Command::LS)));

        let input = "dir a";
        let _name = "a".to_string();
        let result = input.parse::<ConsoleLine>().unwrap();
        assert!(matches!(
            result,
            ConsoleLine::Directory(DirectoryDetails {})
        ));

        let input = "8033020 d.log";
        let _name = "d.log".to_string();
        let result = input.parse::<ConsoleLine>().unwrap();
        assert!(matches!(
            result,
            ConsoleLine::File(FileDetails {
                name: _name,
                size: 8033020
            })
        ));
    }
}
