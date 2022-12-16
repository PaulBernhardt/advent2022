use std::{collections::HashSet, str::FromStr};

enum Instruction {
    NOOP,
    ADDNOOP,
    ADDX(i32),
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = s.split_once(" ");
        match res {
            Some((_, count)) => Ok(Instruction::ADDX(count.parse::<i32>().unwrap_or_default())),
            None => Ok(Instruction::NOOP),
        }
    }
}

type Instructions = Vec<Instruction>;

fn parse_lines(contents: &str) -> Result<Instructions, String> {
    let mut instructions: Instructions = Vec::new();
    for line in contents.lines() {
        let instruction = line.parse::<Instruction>()?;
        match instruction {
            Instruction::NOOP => instructions.push(instruction),
            Instruction::ADDX(_) => {
                instructions.push(Instruction::ADDNOOP);
                instructions.push(instruction)
            }
            Instruction::ADDNOOP => (),
        };
    }
    Ok(instructions)
}

pub fn solve_a(contents: &str) -> Result<String, String> {
    let instructions = parse_lines(contents)?;
    let mut total = 0;
    let mut signal = 1;
    let checks: HashSet<usize> = HashSet::from_iter(vec![20, 60, 100, 140, 180, 220]);
    for i in 1..=instructions.len() {
        if checks.contains(&(i)) {
            total += signal * i as i32;
        }
        if let Instruction::ADDX(c) = instructions[i - 1] {
            signal += c;
        }
    }
    Ok(format!("{}", total))
}

pub fn solve_b(contents: &str) -> Result<String, String> {
    let instructions = parse_lines(contents)?;
    let mut signal = 1;
    let mut buffer = String::new();
    for i in 1..=instructions.len() {
        if (((i as i32 - 1) % 40) - signal).abs() <= 1 {
            buffer.push('#');
        } else {
            buffer.push('.');
        }
        if i % 40 == 0 {
            buffer.push('\n');
        }
        if let Instruction::ADDX(c) = instructions[i - 1] {
            signal += c;
        }
    }
    Ok(format!("\n{}", buffer))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let contents = "\
        addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        let result = solve_a(contents);
        assert_eq!(result.unwrap(), "13140");
    }

    #[test]
    fn test_b() {
        let contents = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        let result = solve_b(contents);
        assert_eq!(
            result.unwrap(),
            "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        );
    }
}
