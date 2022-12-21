use std::{collections::HashSet, ops::Sub, str::FromStr};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd)]
struct Point(isize, isize);

impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Point {
    fn dist(self, rhs: Self) -> isize {
        let delta = rhs - self;
        delta.0.abs() + delta.1.abs()
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(", ")
            .ok_or_else(|| format!("Bad input, missing ', ' in '{}", s))?;

        let x = x
            .strip_prefix("x=")
            .ok_or_else(|| format!("Bad x, missing prefix: '{}'", s))?
            .parse::<isize>()
            .map_err(|_| "Couldn't parse x".to_string())?;

        let y = y
            .strip_prefix("y=")
            .ok_or_else(|| format!("Bad y, missing prefix: '{}'", s))?
            .parse::<isize>()
            .map_err(|_| "Couldn't parse y".to_string())?;

        Ok(Point(y, x))
    }
}

struct Sensor {
    point: Point,
    beacon: Point,
    range: isize,
}

impl FromStr for Sensor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sensor, beacon) = s
            .split_once(": ")
            .ok_or_else(|| format!("Bad input, missing ':' in '{}", s))?;

        let sensor = sensor
            .strip_prefix("Sensor at ")
            .ok_or_else(|| format!("Bad input, missing prefix: '{}'", sensor))?;

        let point = sensor.parse::<Point>()?;

        let beacon = beacon
            .strip_prefix("closest beacon is at ")
            .ok_or_else(|| format!("Bad input, missing prefix: '{}'", beacon))?;

        let beacon = beacon.parse::<Point>()?;

        let range = point.dist(beacon);

        Ok(Sensor {
            point,
            beacon,
            range,
        })
    }
}

pub fn solve_a(contents: &str) -> Result<String, String> {
    let sensors: Vec<Sensor> = contents.lines().flat_map(|l| l.parse::<Sensor>()).collect();
    let mut min = isize::MAX;
    let mut max = isize::MIN;

    for sensor in sensors.iter() {
        min = min.min(sensor.point.1 - sensor.range);
        max = max.max(sensor.point.1 + sensor.range);
    }

    let beacons: HashSet<Point> = sensors.iter().map(|s| s.beacon).collect();

    let target_row = if max < 50 { 10 } else { 2000000 }; // Switch between test row and real row

    let mut total = 0;
    for i in min..=max {
        let point = Point(target_row, i);
        if beacons.contains(&point) {
            continue;
        }
        for sensor in sensors.iter() {
            if point.dist(sensor.point) <= sensor.range {
                total += 1;
                break;
            }
        }
    }
    Ok(format!("{}", total))
}

pub fn solve_b(contents: &str) -> Result<String, String> {
    let sensors: Vec<Sensor> = contents.lines().flat_map(|l| l.parse::<Sensor>()).collect();
    let mut min = isize::MAX;
    let mut max = isize::MIN;

    for sensor in sensors.iter() {
        min = min.min(sensor.point.1 - sensor.range);
        max = max.max(sensor.point.1 + sensor.range);
    }

    let beacons: HashSet<Point> = sensors.iter().map(|s| s.beacon).collect();

    let max = if max < 50 { 20 } else { 4000000 }; // Switch between test row and real row

    for x in 0..=max {
        for y in 0..=max {
            let point = Point(x, y);
            if beacons.contains(&point) {
                continue;
            }
            let mut possible = true;
            for sensor in sensors.iter() {
                if point.dist(sensor.point) <= sensor.range {
                    possible = false;
                    break;
                }
            }
            if possible {
                return Ok(format!("{}", point.1 * 4000000 + point.0));
            }
        }
    }
    Err("Couldn't find point".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let contents = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        let result = solve_a(contents);
        assert_eq!(result.unwrap(), "26");
    }

    #[test]
    fn test_b() {
        let contents = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        let result = solve_b(contents);
        assert_eq!(result.unwrap(), "56000011");
    }
}
