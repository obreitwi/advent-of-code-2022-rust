use anyhow::Result;
use nom::character::complete::char;
use nom::character::complete::newline;
use nom::character::complete::one_of;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::Finish;
use nom::IResult;

#[derive(Debug, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn outcome(&self, other: &Self) -> Outcome {
        use Hand::*;
        use Outcome::*;
        match (self, other) {
            (Rock, Paper) => Loss,
            (Paper, Scissors) => Loss,
            (Scissors, Rock) => Loss,

            (Paper, Rock) => Win,
            (Scissors, Paper) => Win,
            (Rock, Scissors) => Win,

            (_, _) => Draw,
        }
    }

    fn need_to_choose(&self, outcome: &Outcome) -> Hand {
        use Hand::*;
        use Outcome::*;
        match (self, outcome) {
            (hand, Draw) => *hand,

            (Rock, Loss) => Scissors,
            (Paper, Loss) => Rock,
            (Scissors, Loss) => Paper,

            (Rock, Win) => Paper,
            (Paper, Win) => Scissors,
            (Scissors, Win) => Rock,
        }
    }

    fn score(&self) -> u64 {
        use Hand::*;
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn score(&self) -> u64 {
        use Outcome::*;
        match self {
            Win => 6,
            Draw => 3,
            Loss => 0,
        }
    }
}

#[derive(Debug)]
struct GameOutcome {
    opponent: Hand,
    outcome: Outcome,
}

impl GameOutcome {
    fn parse(i: &str) -> IResult<&str, Self> {
        let (i, (opponent, _, outcome)) = tuple((one_of("ABC"), char(' '), one_of("XYZ")))(i)?;

        use Hand::*;
        use Outcome::*;

        Ok((
            i,
            GameOutcome {
                opponent: match opponent {
                    'A' => Rock,
                    'B' => Paper,
                    'C' => Scissors,
                    _ => unreachable!(),
                },
                outcome: match outcome {
                    'X' => Loss,
                    'Y' => Draw,
                    'Z' => Win,
                    _ => unreachable!(),
                },
            },
        ))
    }

    fn score(&self) -> u64 {
        self.opponent.need_to_choose(&self.outcome).score() + self.outcome.score()
    }
}

#[derive(Debug)]
struct Game {
    opponent: Hand,
    played: Hand,
}

impl Game {
    fn parse(i: &str) -> IResult<&str, Self> {
        let (i, (opponent, _, played)) = tuple((one_of("ABC"), char(' '), one_of("XYZ")))(i)?;

        use Hand::*;

        Ok((
            i,
            Game {
                opponent: match opponent {
                    'A' => Rock,
                    'B' => Paper,
                    'C' => Scissors,
                    _ => unreachable!(),
                },
                played: match played {
                    'X' => Rock,
                    'Y' => Paper,
                    'Z' => Scissors,
                    _ => unreachable!(),
                },
            },
        ))
    }

    fn score(&self) -> u64 {
        self.played.score() + self.played.outcome(&self.opponent).score()
    }
}

fn read_games(path: &std::path::Path) -> Result<Vec<Game>> {
    let content = std::fs::read_to_string(path)?;
    let games = separated_list1(newline, Game::parse)(&content)
        .finish()
        .unwrap()
        .1;
    Ok(games)
}

fn read_game_outcomes(path: &std::path::Path) -> Result<Vec<GameOutcome>> {
    let content = std::fs::read_to_string(path)?;
    let games = separated_list1(newline, GameOutcome::parse)(&content)
        .finish()
        .unwrap()
        .1;
    Ok(games)
}

fn main() -> Result<()> {
    let games = read_games(&std::path::PathBuf::from("input.txt"))?;
    let total = games.iter().map(|g| g.score()).sum::<u64>();
    println!("total score: {total}");

    let game_outcomes = read_game_outcomes(&std::path::PathBuf::from("input.txt"))?;
    let total = game_outcomes.iter().map(|g| g.score()).sum::<u64>();
    println!("total score with outcomes: {total}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_games() -> Result<()> {
        let games = read_games(&std::path::PathBuf::from("debug.txt"))?;
        let total = games.iter().map(|g| g.score()).sum::<u64>();
        assert_eq!(total, 15);
        Ok(())
    }

    #[test]
    fn test_games_outcome() -> Result<()> {
        let games = read_game_outcomes(&std::path::PathBuf::from("debug.txt"))?;
        let total = games.iter().map(|g| g.score()).sum::<u64>();
        assert_eq!(total, 12);
        Ok(())
    }
}
