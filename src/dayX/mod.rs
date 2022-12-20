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
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let result = solve_a(contents);
        assert_eq!(result.unwrap(), "CMZ");
    }

    #[test]
    fn test_b() {
        let contents = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let result = solve_b(contents);
        assert_eq!(result.unwrap(), "CMZ");
    }
}
