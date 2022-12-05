use std::{
    fmt::{self, Display},
    str::FromStr,
};

struct Range {
    high: i8,
    low: i8,
}

impl Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(low: {}, high: {})", self.low, self.high)
    }
}
impl FromStr for Range {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((low, high)) = s.split_once('-') {
            let low = low
                .parse::<i8>()
                .map_err(|_| format!("Low side of range invalid: '{low}' of '{s}'"))?;
            let high = high
                .parse::<i8>()
                .map_err(|_| format!("High side of range invalid: '{high}' of '{s}'"))?;
            Ok(Self { high, low })
        } else {
            Err(format!("Invalid range: {s}"))
        }
    }
}

pub fn solve_a(contents: &str) -> Result<i32, String> {
    let mut total = 0;
    for line in contents.lines() {
        if let Some((left, right)) = line.split_once(',') {
            let left = left.parse::<Range>()?;
            let right = right.parse::<Range>()?;

            let left_contains_right = left.low <= right.low && left.high >= right.high;
            let right_contains_left = left.low >= right.low && left.high <= right.high;
            if right_contains_left || left_contains_right {
                total += 1;
            }
        } else {
            return Err(format!("Couldn't split '{line}'"));
        }
    }

    Ok(total)
}

pub fn solve_b(contents: &str) -> Result<i32, String> {
    let mut total = 0;
    for line in contents.lines() {
        if let Some((left, right)) = line.split_once(',') {
            let left = left.parse::<Range>()?;
            let right = right.parse::<Range>()?;

            let overlap = left.low <= right.low && left.high >= right.low
                || right.low <= left.low && right.high >= left.low;
            if overlap {
                total += 1;
            }
        } else {
            return Err(format!("Couldn't split '{line}'"));
        }
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let contents = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let result = solve_a(contents);
        assert!(result.is_ok());
        assert_eq!(result.unwrap_or_default(), 2);
    }

    #[test]
    fn test_b() {
        let contents = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let result = solve_b(contents);
        assert!(result.is_ok());
        assert_eq!(result.unwrap_or_default(), 4);
    }
}
