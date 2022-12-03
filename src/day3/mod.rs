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
                total = total + item_map.get(&char).unwrap();
                break;
            }
        }
    }

    return total;
}

pub fn solve_b(contents: &str) -> i32 {
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
                total = total + item_map.get(&char).unwrap();
                break;
            }
        }
    }

    return total;
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
