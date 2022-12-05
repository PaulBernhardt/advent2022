use std::collections::{HashMap, HashSet};

pub fn solve_a(contents: &str) -> i32 {
    let item_map = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .zip(1..53)
        .collect::<HashMap<char, i32>>();

    let mut total = 0;
    for line in contents.lines() {
        let (left, right) = line.split_at(line.len() / 2);
        let left_index = left.chars().collect::<HashSet<char>>();

        for char in right.chars() {
            if left_index.contains(&char) {
                total += item_map.get(&char).unwrap();
                break;
            }
        }
    }

    total
}

pub fn solve_b(contents: &str) -> i32 {
    let item_map = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .zip(1..53)
        .collect::<HashMap<char, i32>>();

    let mut lines = contents.lines();
    let mut total = 0;

    while let Some(a) = lines.next() {
        if let Some(b) = lines.next() {
            if let Some(c) = lines.next() {
                let a_index = a.chars().collect::<HashSet<char>>();
                let b_index = b.chars().collect::<HashSet<char>>();

                for char in c.chars() {
                    if a_index.contains(&char) && b_index.contains(&char) {
                        total += item_map.get(&char).unwrap();
                        break;
                    }
                }
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let contents = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(solve_a(contents), 157);
    }

    #[test]
    fn test_b() {
        let contents = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(solve_b(contents), 70);
    }
}
