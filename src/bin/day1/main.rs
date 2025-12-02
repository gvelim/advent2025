use std::{ops::Add, str::FromStr};

use nom::combinator::Opt;

fn main() {}

type STEPS = u16;

#[derive(Debug)]
struct MyError(String);

#[derive(Debug)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug)]
struct Action {
    turn: Turn,
    steps: STEPS,
}

impl FromStr for Turn {
    type Err = MyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Turn::Left),
            "R" => Ok(Turn::Right),
            _ => Err(MyError("Input isn't \"L\" or \"R\"".to_string())),
        }
    }
}

impl FromStr for Action {
    type Err = MyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Action {
            turn: s[..1].parse::<Turn>()?,
            steps: STEPS::from_str(&s[1..]).map_err(|_| {
                MyError(
                    format!("cannot input {} to u8", &s[1..])
                        .as_str()
                        .to_owned(),
                )
            })?,
        })
    }
}

#[derive(Debug)]
struct RotaryDial {
    perimeter: STEPS,
    pointer: STEPS,
}

impl RotaryDial {
    fn new(perimeter: STEPS, pointer: STEPS) -> RotaryDial {
        RotaryDial { perimeter, pointer }
    }
    fn dial_left(&self, steps: STEPS) -> STEPS {
        self.perimeter + self.pointer - steps % self.perimeter
    }
    fn dial_right(&self, steps: STEPS) -> STEPS {
        self.perimeter - self.pointer + steps % self.perimeter
    }
    fn turn_and_listen(&mut self, act: Action) -> bool {
        self.pointer = match act.turn {
            Turn::Left => self.dial_left(act.steps),
            Turn::Right => self.dial_right(act.steps),
        };
        if self.pointer == 0 { true } else { false }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let mut dial = RotaryDial::new(100, 50);

        println!("L68 {:?}", dial.dial_left(68));
        println!("L30 {:?}", dial.dial_left(30));
        println!("R48 {:?}", dial.dial_right(48));
    }

    #[test]
    fn test_parse_input() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

        let out = input
            .lines()
            .map(|action| action.parse::<Action>())
            .collect::<Result<Vec<_>, _>>();
        println!("{:?}", out);
    }
}
