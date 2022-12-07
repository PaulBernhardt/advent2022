use std::collections::HashMap;

fn first_unique_set(contents: &str, size: usize) -> Result<String, String> {
    let chars: Vec<char> = contents.chars().collect();
    let mut current_chars = "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .zip(vec![0; 26])
        .collect::<HashMap<char, i32>>();

    let mut unique_chars = 0;
    for i in 0..chars.len() {
        let char = chars[i];
        let count = current_chars.get(&char).unwrap_or(&0);
        if count == &0 {
            unique_chars += 1;
        }
        // println!("Adding {char} (was {count}), unique: {unique_chars}");
        if unique_chars == size {
            // for (key, val) in current_chars.iter() {
            //     if val != &0 {
            //         println!("key: {key} val: {val}");
            //     }
            // }
            return Ok((i + 1).to_string());
        }
        current_chars.insert(char, count + 1);
        if i >= size - 1 {
            let char = chars[i - (size - 1)];
            let count = current_chars.get(&char).unwrap_or(&0);
            // println!("Removing {char} (was {count})");
            if count == &1 {
                unique_chars -= 1;
            }
            current_chars.insert(char, count - 1);
        }
    }
    Err("No starting packet".to_string())
}

pub fn solve_a(contents: &str) -> Result<String, String> {
    first_unique_set(contents, 4)
}

pub fn solve_b(contents: &str) -> Result<String, String> {
    first_unique_set(contents, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let contents = "mjqjpqmgbljsphdztnvjfqwrcgsmlb2";
        let result = solve_a(contents);
        assert!(result.is_ok());
        assert_eq!(result.unwrap_or_default(), "7");

        let contents = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let result = solve_a(contents);
        assert!(result.is_ok());
        assert_eq!(result.unwrap_or_default(), "5");

        let contents = "nppdvjthqldpwncqszvftbrmjlhg";
        let result = solve_a(contents);
        assert!(result.is_ok());
        assert_eq!(result.unwrap_or_default(), "6");
    }

    #[test]
    fn test_b() {
        let contents = "mjqjpqmgbljsphdztnvjfqwrcgsmlb2";
        let result = solve_b(contents);
        assert!(result.is_ok());
        assert_eq!(result.unwrap_or_default(), "19");
    }
}
