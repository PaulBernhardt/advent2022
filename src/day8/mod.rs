use std::collections::HashSet;

type Forest = Vec<Vec<i32>>;

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
