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

    pub fn turn(&mut self, act: &Action) -> Steps {
        self.cursor = (self.cursor + (act.turn as Steps) * act.steps) % self.perimeter;
        self.needle = self.cursor + if self.cursor < 0 { 100 } else { 0 };
        self.needle
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

        // 50 -> 25 = 75, 0
        let mut res = count_zeros(50, Turn::Right, 25);
        assert_eq!(res, 0, "got:{} - expected:{}\n", res, 0);
        // 50 -> 50 = 0, 1
        res = count_zeros(50, Turn::Right, 50);
        assert_eq!(res, 1, "got:{} - expected:{}\n", res, 1);
        // 50 -> 100 = 50, 1
        res = count_zeros(50, Turn::Right, 100);
        assert_eq!(res, 1, "got:{} - expected:{}\n", res, 1);
        // 50 -> 150 = 0, 2
        res = count_zeros(50, Turn::Right, 150);
        assert_eq!(res, 2, "got:{} - expected:{}\n", res, 2);
        // 50 -> 75 = 15, 1
        res = count_zeros(50, Turn::Right, 75);
        assert_eq!(res, 1, "got:{} - expected:{}\n", res, 1);
        // 50 -> 175 = 15, 2
        res = count_zeros(50, Turn::Right, 175);
        assert_eq!(res, 2, "got:{} - expected:{}\n", res, 2);
        // 50 <- 50 = 0, 1
        res = count_zeros(50, Turn::Left, 50);
        assert_eq!(res, 1, "got:{} - expected:{}\n", res, 1);
        // 50 <- 150 = 0, 2
        res = count_zeros(50, Turn::Left, 150);
        assert_eq!(res, 2, "got:{} - expected:{}\n", res, 2);
        // 50 <- 75 = 15, 1
        res = count_zeros(50, Turn::Left, 75);
        assert_eq!(res, 1, "got:{} - expected:{}\n", res, 1);
        // 50 <- 175 = 15, 2
        res = count_zeros(10, Turn::Left, 110);
        assert_eq!(res, 2, "got:{} - expected:{}\n", res, 2);
        // 10 <- 115 = 0, 2
        res = count_zeros(10, Turn::Left, 115);
        assert_eq!(res, 2, "got:{} - expected:{}\n", res, 2);
        // 76 <- 46/46 -> 30/30
        res = count_zeros(76, Turn::Left, 46);
        assert_eq!(res, 0, "got:{} - expected:{}\n", res, 0);
        // 0 <- 305 -> 95 (r:3, %:5, d:0) = 4
        res = count_zeros(0, Turn::Left, 305);
        assert_eq!(res, 3, "got:{} - expected:{}\n", res, 3);
    }

    #[test]
    fn test_turn() {
        let new_action = |turn, steps| -> Action { Action { turn, steps } };

        let mut dial = RotaryDial::new(100, 50);
        let turns = [
            new_action(Turn::Left, 68),
            new_action(Turn::Left, 30),
            new_action(Turn::Right, 48),
        ];

        assert_eq!(dial.turn(&turns[0]), 82);
        assert_eq!(dial.turn(&turns[1]), 52);
        assert_eq!(dial.turn(&turns[2]), 0);
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
