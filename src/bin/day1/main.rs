use std::str::FromStr;
use thiserror::Error;

fn main() -> Result<(), MyError> {
    let input = std::fs::read_to_string("src/bin/day1/input.txt").expect("file not found");
    let mut dial = RotaryDial::new(100, 50);
    let actions = input
        .lines()
        .map(|action| action.parse::<Action>())
        .collect::<Result<Vec<_>, _>>()?;

    // let out = actions
    //     .iter()
    //     .inspect(|a| print!("{:?}", a))
    //     .map(|a| dial.turn(a))
    //     .inspect(|a| println!(" = {a}"))
    //     .filter(|a| *a == 0)
    //     .count();
    // println!("Part 1: {:?}", out);
    // // Expected: 969

    let mut dial = RotaryDial::new(100, 50);
    let out = actions
        .iter()
        .inspect(|a| print!("{:?}", a))
        .map(|a| dial.zeros(a))
        .inspect(|a| println!(" = {a}"))
        .sum::<Steps>();
    println!("Part 2: {:?}", out);
    // Expected: 969
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
    cursor: Steps,
    needle: Steps,
}

impl RotaryDial {
    fn new(perimeter: Steps, start: Steps) -> RotaryDial {
        RotaryDial {
            perimeter,
            cursor: start,
            needle: start,
        }
    }
    fn turn(&mut self, act: &Action) -> Steps {
        self.cursor = (self.cursor + (act.turn as Steps) * act.steps) % self.perimeter;
        self.needle = self.cursor + if self.cursor < 0 { 100 } else { 0 };
        self.needle
    }
    fn zeros(&mut self, act: &Action) -> Steps {
        let RotaryDial {
            perimeter,
            cursor: last,
            ..
        } = *self;

        self.turn(act);

        let new = self.cursor;
        let z_rounds = act.steps / perimeter;

        print!(
            " {last} -> {} -> {new} (rounds: {z_rounds})",
            act.turn as Steps * act.steps
        );
        match (last.signum(), new.signum()) {
            // we've landed on zero going over 0..* cycles
            (0, 0) if z_rounds > 0 => z_rounds,
            (_, 0) if z_rounds > 0 => 1 + z_rounds,
            (_, 0) => 1,
            (0, _) if z_rounds > 0 => z_rounds,
            (0, _) => 0,
            // we've crossed zero in 0..* cycles
            (-1, 1) | (1, -1) if z_rounds > 0 => z_rounds,
            (-1, 1) | (1, -1) => 1,
            // we've travelled more than a dial's perimeter length without crossing zero
            (-1, -1) | (1, 1) if z_rounds > 0 => z_rounds,
            (-1, -1) | (1, 1) => 0,
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_zeros() {
        let mut dial = RotaryDial::new(100, 50);
        let mut res = dial.zeros(&Action {
            turn: Turn::Left,
            steps: 1000,
        });
        assert_eq!(res, 10, "got:{} - expected:{}\n", res, 10);
        res = dial.zeros(&Action {
            turn: Turn::Right,
            steps: 950,
        });
        assert_eq!(res, 10, "got:{} - expected:{}\n", res, 10);
        res = dial.zeros(&Action {
            turn: Turn::Right,
            steps: 1000,
        });
        assert_eq!(res, 10, "got:{} - expected:{}\n", res, 10);
        res = dial.zeros(&Action {
            turn: Turn::Right,
            steps: 1,
        });
        assert_eq!(res, 0, "got:{} - expected:{}\n", res, 10);
        res = dial.zeros(&Action {
            turn: Turn::Left,
            steps: 2,
        });
        assert_eq!(res, 1, "got:{} - expected:{}\n", res, 1);
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
