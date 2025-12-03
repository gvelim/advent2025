use std::str::FromStr;
use thiserror::Error;

fn main() -> Result<(), MyError> {
    let input = std::fs::read_to_string("src/bin/day1/sample.txt").expect("file not found");
    let mut dial = RotaryDial::new(100, 50);

    let out: usize = input
        .lines()
        .map(|action| {
            action
                .parse::<Action>()
                .map_err(|e| panic!("{e:?}"))
                .unwrap()
        })
        .inspect(|a| print!("{:?}", a))
        .map(|a| dial.turn(&a))
        .inspect(|a| println!(" = {a}"))
        .filter(|a| *a == 0)
        .count();
    println!("{:?}", out);

    Ok(())
}

type Steps = i16;

#[derive(Debug, Error, PartialEq)]
enum MyError {
    #[error("Cannot parse turn letter")]
    InvalidTurn = 0,
    #[error("Cannot parse step count")]
    InvalidStep = 1,
}

#[derive(Debug, PartialEq)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct Action {
    turn: Turn,
    steps: Steps,
}

impl FromStr for Turn {
    type Err = MyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Turn::Left),
            "R" => Ok(Turn::Right),
            _ => Err(MyError::InvalidTurn),
        }
    }
}

impl FromStr for Action {
    type Err = MyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Action {
            turn: s[..1].parse::<Turn>()?,
            steps: Steps::from_str(&s[1..]).map_err(|_| MyError::InvalidStep)?,
        })
    }
}

#[derive(Debug)]
struct RotaryDial {
    perimeter: Steps,
    start: Steps,
    accum: Steps,
}

impl RotaryDial {
    fn new(perimeter: Steps, start: Steps) -> RotaryDial {
        RotaryDial {
            perimeter,
            start,
            accum: 0,
        }
    }
    fn turn(&mut self, act: &Action) -> Steps {
        self.accum += match act.turn {
            Turn::Left => -act.steps,
            Turn::Right => act.steps,
        };
        self.start + self.accum % self.perimeter
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let mut dial = RotaryDial::new(100, 50);
        let data = [
            Action {
                turn: Turn::Left,
                steps: 68,
            },
            Action {
                turn: Turn::Left,
                steps: 30,
            },
            Action {
                turn: Turn::Right,
                steps: 48,
            },
        ];
        for a in data {
            println!("{:?}{:?} {:?}", a.turn, a.steps, dial.turn(&a));
        }
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            "L68".parse::<Action>(),
            Ok(Action {
                turn: Turn::Left,
                steps: 68
            })
        );
        assert_eq!("E68".parse::<Action>(), Err(MyError::InvalidTurn));
        assert_eq!("LA8".parse::<Action>(), Err(MyError::InvalidStep));
    }
}
