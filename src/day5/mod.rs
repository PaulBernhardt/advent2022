use std::str::{FromStr, Lines};

#[derive(Debug)]
struct Instruction {
    count: i32,
    from: usize,
    to: usize,
}

#[derive(Debug)]
pub struct StackPuzzle {
    stacks: Vec<Stack>,
    instructions: Vec<Instruction>,
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split_whitespace().collect();
        let count = words[1]
            .parse::<i32>()
            .map_err(|_| format!("Invalid 'count' ({:?}) in '{s}", words[1]))?;
        let from = words[3]
            .parse::<usize>()
            .map_err(|_| format!("Invalid 'from' ({:?}) in '{s}", words[3]))?;
        let to = words[5]
            .parse::<usize>()
            .map_err(|_| format!("Invalid 'to' ({:?}) in '{s}", words[5]))?;
        Ok(Self { count, to, from })
    }
}

type Stack = Vec<char>;

enum State {
    Stacks,
    Instructions,
}

pub fn parse_stack_input(lines: Lines) -> Result<StackPuzzle, String> {
    let mut stacks: Vec<Stack> = vec![Vec::new()];
    let mut instructions: Vec<Instruction> = Vec::new();

    let mut state = State::Stacks;
    for line in lines {
        match state {
            State::Stacks => {
                if line.is_empty() {
                    state = State::Instructions;
                    for stack in &mut stacks {
                        stack.reverse();
                    }
                    continue;
                }

                let chars: Stack = line.chars().collect();
                let mut pos = 0;
                while pos <= chars.len() / 4 {
                    let char = chars[pos * 4 + 1];
                    if char == '1' {
                        // We've hit the numbers on the bottom of the stack, but we don't care
                        break;
                    } else {
                        if pos + 1 >= stacks.len() {
                            let stacks_mut = &mut stacks;
                            stacks_mut.push(Vec::new());
                        }
                        if char != ' ' {
                            let stack = &mut stacks[pos + 1];
                            stack.push(char);
                        }
                    }
                    pos += 1;
                }
            }
            State::Instructions => {
                let instruction = line.parse::<Instruction>();
                if let Ok(instruction) = instruction {
                    instructions.push(instruction);
                } else {
                    println!("{:?}", instruction);
                }
            }
        }
    }
    Ok(StackPuzzle {
        stacks,
        instructions,
    })
}

pub fn solve_a(contents: &str) -> Result<String, String> {
    let mut puzzle = parse_stack_input(contents.lines())?;
    for instruction in puzzle.instructions {
        for _ in 0..instruction.count {
            if let Some(item) = puzzle.stacks[instruction.from].pop() {
                puzzle.stacks[instruction.to].push(item);
            }
        }
    }
    let mut answer = String::new();
    for mut stack in puzzle.stacks {
        if let Some(item) = stack.pop() {
            answer.push(item)
        }
    }
    Ok(answer)
}

pub fn solve_b(contents: &str) -> Result<String, String> {
    let mut puzzle = parse_stack_input(contents.lines())?;
    for instruction in puzzle.instructions {
        let mut temp: Vec<char> = Vec::new();
        for _ in 0..instruction.count {
            if let Some(item) = puzzle.stacks[instruction.from].pop() {
                temp.push(item);
            }
        }
        temp.reverse();
        for item in temp {
            puzzle.stacks[instruction.to].push(item);
        }
    }
    let mut answer = String::new();
    for mut stack in puzzle.stacks {
        if let Some(item) = stack.pop() {
            answer.push(item)
        }
    }
    Ok(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let contents = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let result = solve_a(contents);
        assert!(result.is_ok());
        assert_eq!(result.unwrap_or_default(), "CMZ");
    }

    #[test]
    fn test_b() {
        //         let contents = "\
        // 2-4,6-8
        // 2-3,4-5
        // 5-7,7-9
        // 2-8,3-7
        // 6-6,4-6
        // 2-6,4-8";
        //         let result = solve_b(contents);
        //         assert!(result.is_ok());
        //         assert_eq!(result.unwrap_or_default(), 4);
    }

    // fn test_parse_stack_input() {
    //     let teststr = "    [D]    ";
    //     let teststr = "[N] [C]    ";
    //     let teststr = "[Z] [M] [P]";
    //     assert_eq!(parse_stack_input(teststr).unwrap(), vec!['X', 'M', 'P']);

    //     let teststr = " 1   2   3 ";
    //     assert_eq!(parse_stack_input(teststr).unwrap(), vec!['1', '2', '3']);
    // }
}
