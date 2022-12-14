use std::collections::HashSet;

type Tree = i32;
type Forest = Vec<Vec<Tree>>;

enum Direction {
    Right,
    Left,
    Down,
    Up,
}

struct ForestIter<'a> {
    forest: &'a Forest,
    row: isize,
    col: isize,
    row_add: isize,
    col_add: isize,
}

impl<'a> ForestIter<'a> {
    fn new(forest: &'a Forest, row: usize, col: usize, direction: &Direction) -> Self {
        let mut row_add = 0;
        let mut col_add = 0;
        let row = row as isize;
        let col = col as isize;
        match direction {
            Direction::Right => col_add = 1,
            Direction::Left => col_add = -1,
            Direction::Down => row_add = 1,
            Direction::Up => row_add = -1,
        };
        ForestIter {
            forest,
            row,
            col,
            row_add,
            col_add,
        }
    }
}

impl<'a> Iterator for ForestIter<'a> {
    type Item = Tree;
    fn next(&mut self) -> Option<Self::Item> {
        self.row += self.row_add;
        self.col += self.col_add;
        let row = self.forest.get(self.row as usize);
        if let Some(row) = row {
            return row.get(self.col as usize).copied();
        }
        None
    }
}

fn parse_input(contents: &str) -> Result<Forest, String> {
    let mut forest: Forest = Vec::new();
    for line in contents.lines() {
        let trees: Vec<i32> = line
            .chars()
            .map(|c| i32::try_from(c.to_digit(10).unwrap_or_default()).unwrap_or_default())
            .collect();
        forest.push(trees);
    }
    Ok(forest)
}

pub fn solve_a(contents: &str) -> Result<String, String> {
    let trees = parse_input(contents)?;
    let mut visible: HashSet<(usize, usize)> = HashSet::new();
    for r in 0..trees.len() {
        let mut max = -1;
        let row = &trees[r];
        for c in 0..row.len() {
            let tree = row[c];
            // println!("++Checking {r},{c} ({})", tree);
            if tree > max {
                visible.insert((r, c));
                max = tree;
                // println!("Fits");
                if tree >= 9 {
                    break;
                }
            }
        }
        max = -1;
        for cp in 0..row.len() {
            let c = row.len() - cp - 1;
            let tree = row[c];
            // println!("+-Checking {r},{c} ({})", tree);

            if tree > max {
                visible.insert((r, c));
                max = tree;
                // println!("Fits");
                if tree >= 9 {
                    break;
                }
            }
        }
    }

    let row_len = trees[0].len();
    let col_len = trees.len();
    for c in 0..col_len {
        let mut max = -1;
        for r in 0..row_len {
            //r = row_len - rp;
            let tree = trees[r][c];
            // println!("-+Checking {r},{c} ({})", tree);

            if tree > max {
                visible.insert((r, c));
                max = tree;
                // println!("Fits");

                if tree >= 9 {
                    break;
                }
            }
        }

        max = -1;
        for rp in 0..row_len {
            let r = row_len - rp - 1;
            let tree = trees[r][c];
            // println!("--Checking {r},{c} ({})", tree);

            if tree > max {
                visible.insert((r, c));
                max = tree;
                // println!("Fits");

                if tree >= 9 {
                    break;
                }
            }
        }
    }
    Ok(format!("{}", visible.len()))
}

pub fn solve_b(contents: &str) -> Result<String, String> {
    let trees = parse_input(contents)?;
    let dirs = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];
    let mut max = 0;
    for r in 1..(trees.len() - 1) {
        let row = &trees[r];
        for (c, tree) in row.iter().enumerate().take(row.len() - 1).skip(1) {
            let mut total = 1;
            for direction in &dirs {
                let iter = ForestIter::new(&trees, r, c, direction);
                let mut count = 0;

                for seen_tree in iter {
                    count += 1;
                    if seen_tree >= *tree {
                        break;
                    }
                }
                total *= count;
            }
            max = std::cmp::max(max, total);
        }
    }
    Ok(format!("{}", max))
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
        assert_eq!(result.unwrap(), "8");
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
