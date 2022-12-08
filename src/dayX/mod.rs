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
        let contents = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let result = solve_a(contents);
        assert_eq!(result.unwrap(), "CMZ");
    }

    #[test]
    fn test_b() {
        let contents = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let result = solve_b(contents);
        assert_eq!(result.unwrap(), "CMZ");
    }
}
