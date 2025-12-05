use std::str::FromStr;
use thiserror::Error;

fn main() -> Result<(), MyError> {
    let input = std::fs::read_to_string("src/bin/day1/sample.txt").expect("file not found");
    let mut dial = RotaryDial::new(100, 50);
    let actions = input
        .lines()
        .map(|action| action.parse::<Action>())
        .collect::<Result<Vec<_>, _>>()?;

    let out = actions
        .iter()
        .inspect(|a| print!("{:?}", a))
        .map(|a| dial.turn(a))
        .inspect(|a| println!(" = {a}"))
        .filter(|a| *a == 0)
        .count();
    println!("Part 1: {:?}", out);
    // assert_eq!(out, 969);

    let mut dial = RotaryDial::new(100, 50);
    let out = actions
        .iter()
        .inspect(|a| print!("{:?}", a))
        .map(|a| dial.zeros(a))
        .inspect(|a| println!(" = {a}"))
        .sum::<Steps>();
    println!("Part 2: {:?}", out);
    // assert_eq!(out, 969);
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

#[derive(Debug, PartialEq, Clone, Copy)]
enum Turn {
    Left = -1,
    Right = 1,
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
    needle: Steps,
}

impl RotaryDial {
    fn new(perimeter: Steps, start: Steps) -> RotaryDial {
        RotaryDial {
            perimeter,
            needle: start,
        }
    }
    fn turn(&mut self, act: &Action) -> Steps {
        self.needle = (self.needle + (act.turn as Steps) * act.steps) % self.perimeter;
        self.needle
    }
    fn zeros(&mut self, act: &Action) -> Steps {
        let init = self.needle;
        let n = self.turn(act);
        print!(" {init} -> {n} ");
        match (n, init.signum(), n.signum()) {
            (0, _, _) => 1,
            (_, -1, 1) => 1,
            (_, 1, -1) => 1,
            (_, _, _) => 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_zeros() {
        let mut dial = RotaryDial::new(100, 50);

        assert_eq!(
            dial.zeros(&Action {
                turn: Turn::Left,
                steps: 1000,
            }),
            10
        );
        assert_eq!(
            dial.zeros(&Action {
                turn: Turn::Right,
                steps: 950,
            }),
            10
        );
    }

    #[test]
    fn test_turn() {
        let mut dial = RotaryDial::new(100, 50);

        assert_eq!(
            dial.turn(&Action {
                turn: Turn::Left,
                steps: 68,
            }),
            82
        );
        assert_eq!(
            dial.turn(&Action {
                turn: Turn::Left,
                steps: 30,
            }),
            52
        );
        assert_eq!(
            dial.turn(&Action {
                turn: Turn::Right,
                steps: 48,
            }),
            0
        );
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
