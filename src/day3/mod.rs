pub fn solve_a(contents: &str) -> i32 {
    let mut totals = Vec::new();
    let mut current = 0;
    for line in contents.lines() {
        let result = line.parse::<i32>();
        match result {
            Ok(num) => {
                // println!("{}", num);
                current = current + num
            }
            Err(_) => {
                totals.push(current);
                current = 0
            }
        }
    }
    totals.sort_unstable_by(|a, b| b.cmp(a));

    return totals[0];
}

pub fn solve_b(contents: &str) -> i32 {
    let mut totals = Vec::new();
    let mut current = 0;
    for line in contents.lines() {
        let result = line.parse::<i32>();
        match result {
            Ok(num) => {
                // println!("{}", num);
                current = current + num
            }
            Err(_) => {
                totals.push(current);
                current = 0
            }
        }
    }
    totals.sort_unstable_by(|a, b| b.cmp(a));

    return totals[0] + totals[1] + totals[2];
}
