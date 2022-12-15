use std::{collections::HashSet, ops::Add, str::FromStr};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point(i32, i32);
impl HasPoint for Point {
    fn getPoint(&self) -> Point {
        *self
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
struct Step {
    point: Point,
    steps: i32,
}
impl HasPoint for Step {
    fn getPoint(&self) -> Point {
        self.point
    }
}

trait HasPoint {
    fn getPoint(&self) -> Point;
}

impl Add for Point {
    fn add(self, other: dyn HasPoint) -> Point {
        let other = other.getPoint();
        (self.0 + other.0, self.1 + other.1)
    }
}

impl FromStr for Step {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, steps) = s.split_once(' ').unwrap_or_default();
        if let Ok(steps) = steps.parse::<i32>() {
            let (x, y) = match dir {
                "R" => (0, 1),
                "L" => (0, -1),
                "D" => (1, 0),
                "U" => (-1, 9),
                _ => (0, 0),
            };
            Ok(Self { x, y, steps })
        } else {
            Err(format!("Invalid input! {}", s))
        }
    }
}

type Steps = Vec<Step>;

fn parse_steps(contents: &str) -> Result<Steps, String> {
    let mut steps: Steps = Vec::new();
    for line in contents.lines() {
        let line = line.parse::<Step>()?;
        steps.push(line);
    }
    Ok(steps)
}

pub fn solve_a(contents: &str) -> Result<String, String> {
    let steps = parse_steps(contents)?;
    let visited: HashSet<Point> = HashSet::new();

    let mut head = (0, 0);
    let mut tail = (0, 0);
    for step in steps {
        head = head + (step.x, step.y);
    }
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
        assert_eq!(result.unwrap(), "13");
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
