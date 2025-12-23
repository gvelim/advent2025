use std::str::FromStr;
use thiserror::Error;

type Steps = i16;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Turn {
    Left = -1,
    Right = 1,
}

#[derive(Debug, PartialEq)]
pub struct Action {
    turn: Turn,
    steps: Steps,
}

#[derive(Debug)]
pub struct RotaryDial {
    perimeter: Steps,
    cursor: Steps,
    pub needle: Steps,
}

impl RotaryDial {
    pub fn new(perimeter: Steps, start: Steps) -> RotaryDial {
        RotaryDial {
            perimeter,
            cursor: start,
            needle: start,
        }
    }

    // we could take a mutable reference
    // but instead we consume self and return it
    // which is a more of a functional paradigm
    pub fn turn(mut self, act: &Action) -> Self {
        self.cursor = (self.cursor + (act.turn as Steps) * act.steps) % self.perimeter;
        self.needle = self.cursor + if self.cursor < 0 { 100 } else { 0 };
        self
    }

    pub fn count_zero_crossings(&self, act: &Action) -> Steps {
        let RotaryDial {
            perimeter,
            needle: last,
            ..
        } = *self;

        let residual_steps = act.steps % perimeter;
        let x_zone = match act.turn {
            Turn::Left | Turn::Right if last == 0 => perimeter,
            Turn::Left => last,
            Turn::Right => perimeter - last,
        };

        // total zero crossings = full circles + has_residual_steps_crossed()
        // ====
        // full circles = steps / perimeter, i.e. 130 / 100 = 1
        // residual_steps = steps % perimeter, i.e. 130 % 100 = 30
        // crossing_zone (x_zone) is
        // - Right: perimeter - residual steps, e.g 10 -> 130, x_zone >= 90
        // - Left: residual steps, e.g. e.g 10 -> 130, x_zone <= 10
        //
        // insight: you cannot cross zero if "residual_steps" < "x_zone"
        //
        act.steps / perimeter + if residual_steps >= x_zone { 1 } else { 0 }
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum MyError {
    #[error("Cannot parse turn letter")]
    InvalidTurn = 0,
    #[error("Cannot parse step count")]
    InvalidStep = 1,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_zeros() {
        let count_zeros = |start, turn, steps| -> Steps {
            RotaryDial::new(100, start).count_zero_crossings(&Action { turn, steps })
        };
        let tests = [
            (count_zeros(50, Turn::Right, 25), 0),
            (count_zeros(50, Turn::Right, 50), 1),
            (count_zeros(50, Turn::Right, 100), 1),
            (count_zeros(50, Turn::Right, 150), 2),
            (count_zeros(50, Turn::Right, 75), 1),
            (count_zeros(50, Turn::Right, 175), 2),
            (count_zeros(50, Turn::Left, 50), 1),
            (count_zeros(50, Turn::Left, 150), 2),
            (count_zeros(50, Turn::Left, 75), 1),
            (count_zeros(10, Turn::Left, 110), 2),
            (count_zeros(10, Turn::Left, 215), 3),
            (count_zeros(76, Turn::Left, 46), 0),
            (count_zeros(0, Turn::Left, 305), 3),
        ];

        for (res, exp) in tests {
            assert_eq!(res, exp, "Got {res}, expected {exp}")
        }
    }

    #[test]
    fn test_turn() {
        let new_action = |turn, steps| -> Action { Action { turn, steps } };

        let mut dial = RotaryDial::new(100, 50);
        let turns = [
            (new_action(Turn::Left, 68), 82),
            (new_action(Turn::Left, 30), 52),
            (new_action(Turn::Right, 48), 0),
        ];

        for (act, res) in turns {
            dial = dial.turn(&act);
            assert_eq!(dial.needle, res);
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
