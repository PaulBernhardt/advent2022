type Forest = Vec<Vec<u32>>;

fn parse_input(contents: &str) -> Result<Forest, String> {
    let mut forest: Forest = Vec::new();
    for line in contents.lines() {
        let trees: Vec<u32> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap_or_default())
            .collect();
        forest.push(trees);
    }
    Ok(forest)
}

pub fn solve_a(_contents: &str) -> Result<String, String> {
    Err("Not implemented".to_string())
}

pub fn solve_b(_contents: &str) -> Result<String, String> {
    Err("Not implemented".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let contents = "\
30373
25512
65332
33549
35390";
        let result = solve_a(contents);
        assert_eq!(result.unwrap(), "21");
    }

    #[test]
    fn test_b() {
        let contents = "\
30373
25512
65332
33549
35390";
        let result = solve_b(contents);
        assert_eq!(result.unwrap(), "CMZ");
    }

    #[test]
    fn test_parse() {
        let contents = "\
302
253";
        let result = parse_input(contents);
        assert_eq!(result.unwrap(), vec![vec![3, 0, 2], vec![2, 5, 3]]);
    }
}
