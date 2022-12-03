use std::str::FromStr;

#[derive(Copy, Clone)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for RPS {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(RPS::Rock),
            "B" => Ok(RPS::Paper),
            "C" => Ok(RPS::Scissors),

            "X" => Ok(RPS::Rock),
            "Y" => Ok(RPS::Paper),
            "Z" => Ok(RPS::Scissors),

            _ => Err(format!("String '{s}' is not of allowed type")),
        }
    }
}

impl RPS {
    fn build(num: i32) -> Result<RPS, &'static str> {
        match num {
            1 => Ok(RPS::Rock),
            2 => Ok(RPS::Paper),
            3 => Ok(RPS::Scissors),
            _ => Err("Invalid RPS"),
        }
    }
}

#[derive(Copy, Clone)]
enum RoundResult {
    Win = 6,
    Loss = 0,
    Tie = 3,
}

impl FromStr for RoundResult {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(RoundResult::Loss),
            "Y" => Ok(RoundResult::Tie),
            "Z" => Ok(RoundResult::Win),

            _ => Err(format!("String '{s}' is not of allowed type")),
        }
    }
}

struct RPSRound {
    a: i32,
    b: i32,
}

impl RPSRound {
    fn build(opponent: RPS, you: RPS, goal: RoundResult) -> Result<RPSRound, &'static str> {
        let delta = you as i32 - opponent as i32;
        let result = match delta {
            1 => RoundResult::Win,
            -2 => RoundResult::Win,
            0 => RoundResult::Tie,
            _ => RoundResult::Loss,
        };
        let a = you as i32 + result as i32;

        let opp_num = opponent as i32;
        let target_move = match goal {
            RoundResult::Win => {
                let mut player_num = opp_num + 1;
                if player_num == 4 {
                    player_num = 1;
                }
                player_num
            }
            RoundResult::Tie => opp_num,
            RoundResult::Loss => {
                let mut player_num = opp_num - 1;
                if player_num == 0 {
                    player_num = 3;
                }
                player_num
            }
        };
        let target_move = RPS::build(target_move)?;

        let b = target_move as i32 + goal as i32;

        Ok(RPSRound { a, b })
    }
}

impl FromStr for RPSRound {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((a, b)) = s.split_once(' ') {
            let opponent = a.parse::<RPS>()?;
            let you = b.parse::<RPS>()?;
            let result = b.parse::<RoundResult>()?;
            let rps = RPSRound::build(opponent, you, result)?;
            Ok(rps)
        } else {
            return Err(format!("Invalid line: {s}"));
        }
    }
}

pub fn solve_a(contents: &str) -> i32 {
    let mut score = 0;
    for line in contents.lines() {
        let result = line.parse::<RPSRound>();
        match result {
            Ok(round) => {
                score = score + round.a;
            }
            Err(_) => {
                panic!("This shouldn't happen");
            }
        }
    }

    return score;
}

pub fn solve_b(contents: &str) -> i32 {
    let mut score = 0;
    for line in contents.lines() {
        let result = line.parse::<RPSRound>();
        match result {
            Ok(round) => {
                score = score + round.b;
            }
            Err(_) => {
                panic!("This shouldn't happen");
            }
        }
    }

    return score;
}
