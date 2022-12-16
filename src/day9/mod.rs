use std::{collections::HashSet, ops::Add, str::FromStr};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point(i32, i32);

impl Point {
    fn compare(&self, other: Point) -> Point {
        let delta_x = other.0 - self.0;
        let delta_y = other.1 - self.1;
        if delta_x.abs() <= 1 && delta_y.abs() <= 1 {
            return Point(0, 0);
        }
        let x = if delta_x > 0 {
            -1
        } else {
            i32::from(delta_x < 0)
        };
        let y = if delta_y > 0 {
            -1
        } else {
            i32::from(delta_y < 0)
        };
        Point(x, y)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Step {
    point: Point,
    distance: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

impl FromStr for Step {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, distance) = s.split_once(' ').unwrap_or_default();
        if let Ok(distance) = distance.parse::<i32>() {
            let point = match dir {
                "R" => Point(0, 1),
                "L" => Point(0, -1),
                "D" => Point(1, 0),
                "U" => Point(-1, 0),
                _ => Point(0, 0),
            };
            Ok(Self { point, distance })
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
    let mut visited: HashSet<Point> = HashSet::new();

    let mut head = Point(0, 0);
    let mut tail = Point(0, 0);
    for step in steps {
        let movement = step.point;
        for _ in 0..step.distance {
            head = head + movement;
            tail = tail + head.compare(tail);
            visited.insert(tail);
        }
    }
    Ok(format!("{}", visited.len()))
}

pub fn solve_b(_contents: &str) -> Result<String, String> {
    let steps = parse_steps(contents)?;
    let mut visited: HashSet<Point> = HashSet::new();

    let knots = vec![ Point(0, 0); 10];
    for step in steps {
        let movement = step.point;
        for _ in 0..step.distance {
            head = head + movement;
            tail = tail + head.compare(tail);
            visited.insert(tail);
        }
    }
    Ok(format!("{}", visited.len()))
}
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
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        let result = solve_b(contents);
        assert_eq!(result.unwrap(), "36");
    }
}
