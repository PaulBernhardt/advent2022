use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

type Terrain = Vec<Vec<i8>>;
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point(isize, isize);
impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

impl Point {
    fn dist(&self, target: &Point) -> i32 {
        //f64::from(((self.0 - target.0).pow(2) + (self.1 - target.1).pow(2)) as i32).sqrt()
        ((self.0 - target.0).abs() + (self.1 - target.1).abs()) as i32
    }
    fn adjacent(&self, max: &Point, visited: &HashSet<Point>) -> Vec<Point> {
        let mut adj: Vec<Point> = Vec::new();
        let moves = vec![Point(-1, 0), Point(1, 0), Point(0, -1), Point(0, 1)];
        for m in moves {
            let point = m + *self;
            if point.0 >= 0
                && point.0 < max.0
                && point.1 >= 0
                && point.1 < max.1
                && !visited.contains(&point)
            {
                adj.push(point);
            }
        }
        adj
    }
}

struct Map {
    terrain: Terrain,
    start: Point,
    end: Point,
    starts: Vec<Point>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Path {
    point: Point,
    steps: i32,
    dist: i32,
    huristic: i32,
}
impl Path {
    fn new(point: Point, steps: i32, dist: i32) -> Path {
        Path {
            point,
            steps,
            dist,
            huristic: steps + dist,
        }
    }

    fn to(&self, p: Point, end: &Point) -> Path {
        Path::new(p, self.steps + 1, p.dist(end))
    }
}

impl Eq for Path {}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.huristic.cmp(&other.huristic).reverse())
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        self.huristic.cmp(&other.huristic).reverse()
    }
}

fn parse_lines(contents: &str) -> Result<Map, String> {
    let item_map = "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .zip(0..26)
        .collect::<HashMap<char, i8>>();

    let mut terrain: Terrain = Vec::new();
    let lines = contents.lines();
    let mut start = Point(0, 0);
    let mut end = Point(0, 0);
    let mut starts: Vec<Point> = Vec::new();

    for (i, line) in lines.enumerate() {
        let mut row: Vec<i8> = Vec::new();

        for (j, char) in line.chars().enumerate() {
            row.push(match char {
                'S' => {
                    start = Point(i as isize, j as isize);
                    starts.push(start);
                    0
                }
                'E' => {
                    end = Point(i as isize, j as isize);
                    25
                }
                'a' => {
                    starts.push(Point(i as isize, j as isize));
                    0
                }
                _ => *item_map.get(&char).ok_or("Bad chracter")?,
            });
        }
        terrain.push(row);
    }
    Ok(Map {
        terrain,
        start,
        end,
        starts,
    })
}

pub fn solve_a(contents: &str) -> Result<String, String> {
    let map = parse_lines(contents)?;
    let terrain = map.terrain;
    let start = map.start;
    let end = map.end;

    let max = Point(terrain.len() as isize, terrain[0].len() as isize);

    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue: BinaryHeap<Path> = BinaryHeap::new();
    queue.push(Path::new(start, 0, start.dist(&end)));
    while let Some(path) = queue.pop() {
        let point = path.point;
        if point == end {
            return Ok(format! {"{}", path.steps});
        }
        visited.insert(point);
        let neighbours = point.adjacent(&max, &visited);
        for neighbour in neighbours {
            if terrain[neighbour.0 as usize][neighbour.1 as usize]
                - terrain[point.0 as usize][point.1 as usize]
                <= 1
            {
                queue.push(path.to(neighbour, &end));
            }
        }
    }

    Err("Didn't get to the end".to_string())
}

pub fn solve_b(contents: &str) -> Result<String, String> {
    let map = parse_lines(contents)?;
    let terrain = map.terrain;
    let starts = map.starts;
    let end = map.end;

    let max = Point(terrain.len() as isize, terrain[0].len() as isize);

    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue: BinaryHeap<Path> = BinaryHeap::new();
    for start in starts {
        queue.push(Path::new(start, 0, start.dist(&end)));
    }
    while let Some(path) = queue.pop() {
        let point = path.point;
        if point == end {
            return Ok(format! {"{}", path.steps});
        }
        visited.insert(point);
        let neighbours = point.adjacent(&max, &visited);
        for neighbour in neighbours {
            if terrain[neighbour.0 as usize][neighbour.1 as usize]
                - terrain[point.0 as usize][point.1 as usize]
                <= 1
            {
                queue.push(path.to(neighbour, &end));
            }
        }
    }

    Err("Didn't get to the end".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let contents = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        let result = solve_a(contents);
        assert_eq!(result.unwrap(), "31");
    }

    #[test]
    fn test_b() {
        let contents = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        let result = solve_b(contents);
        assert_eq!(result.unwrap(), "29");
    }
}
