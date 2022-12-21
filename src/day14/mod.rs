use std::{collections::HashMap, ops::Sub, str::FromStr};

enum Obstruction {
    Rock,
    Sand,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd)]
struct Point(isize, isize);

impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

enum Direction {
    X(isize),
    Y(isize),
    Neither,
    Both,
}

impl Point {
    fn compare_direction(self, rhs: Self) -> Direction {
        let delta = rhs - self;
        if delta.0 != 0 && delta.1 != 0 {
            Direction::Both
        } else if delta.0 != 0 {
            Direction::X(delta.0)
        } else if delta.1 != 0 {
            Direction::Y(delta.1)
        } else {
            Direction::Neither
        }
    }
    fn connect(self, rhs: Self) -> Vec<Self> {
        let mut points = Vec::new();
        let direction = self.compare_direction(rhs);
        match direction {
            Direction::X(x) => {
                let start = self.0.min(rhs.0);
                for i in 0..=x.abs() {
                    points.push(Point(start + i, self.1));
                }
            }
            Direction::Y(y) => {
                let start = self.1.min(rhs.1);
                for i in 0..=y.abs() {
                    points.push(Point(self.0, start + i));
                }
            }
            Direction::Neither => panic!("Can't handle neither direction"),
            Direction::Both => panic!("Can't handle both directions"),
        };
        points
    }
}
struct Rock {
    points: Vec<(Point, Point)>,
    floor: isize,
}

impl FromStr for Rock {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = Vec::new();
        let mut old_point: Option<Point> = None;
        let mut floor = 0;
        for step in s.split(" -> ") {
            let (x, y) = step
                .split_once(',')
                .ok_or_else(|| format!("Could not parse {}", step))?;
            let x = x
                .parse::<isize>()
                .map_err(|_| format!("Invalid number for x: {}", x))?;
            let y = y
                .parse::<isize>()
                .map_err(|_| format!("Invalid number for y: {}", y))?;

            if let Some(old_point) = old_point {
                floor = y.max(floor);
                points.push((old_point, Point(x, y)));
            }
            old_point = Some(Point(x, y));
        }

        Ok(Self { points, floor })
    }
}

pub fn solve_a(contents: &str) -> Result<String, String> {
    let rocks = contents.lines().flat_map(|s| s.parse::<Rock>());
    let mut occupied: HashMap<Point, Obstruction> = HashMap::new();
    let mut floor = 0;
    for rock in rocks {
        floor = floor.max(rock.floor);
        for (a, b) in rock.points {
            for point in a.connect(b) {
                occupied.insert(point, Obstruction::Rock);
            }
        }
    }
    let mut total = 0;
    let mut sand = Point(500, 0);
    while sand.1 <= floor {
        sand.1 += 1;
        if occupied.contains_key(&sand) {
            sand.0 -= 1;
            if occupied.contains_key(&sand) {
                sand.0 += 2;
                if occupied.contains_key(&sand) {
                    total += 1;

                    occupied.insert(sand - Point(1, 1), Obstruction::Sand);
                    sand = Point(500, 0);
                }
            }
        }
    }

    Ok(format!("{}", total))
}

pub fn solve_b(contents: &str) -> Result<String, String> {
    let rocks = contents.lines().flat_map(|s| s.parse::<Rock>());
    let mut occupied: HashMap<Point, Obstruction> = HashMap::new();
    let mut floor = 0;
    for rock in rocks {
        floor = floor.max(rock.floor);
        for (a, b) in rock.points {
            for point in a.connect(b) {
                occupied.insert(point, Obstruction::Rock);
            }
        }
    }

    for point in Point(0, floor + 2).connect(Point(10000, floor + 2)) {
        occupied.insert(point, Obstruction::Rock);
    }
    let mut total = 0;
    let mut sand = Point(500, 0);
    loop {
        sand.1 += 1;
        if occupied.contains_key(&sand) {
            sand.0 -= 1;
            if occupied.contains_key(&sand) {
                sand.0 += 2;
                if occupied.contains_key(&sand) {
                    total += 1;
                    if sand == Point(501, 1) {
                        break;
                    }

                    occupied.insert(sand - Point(1, 1), Obstruction::Sand);
                    sand = Point(500, 0);
                }
            }
        }
    }

    Ok(format!("{}", total))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let contents = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        let result = solve_a(contents);
        assert_eq!(result.unwrap(), "24");
    }

    #[test]
    fn test_b() {
        let contents = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        let result = solve_b(contents);
        assert_eq!(result.unwrap(), "93");
    }
}
