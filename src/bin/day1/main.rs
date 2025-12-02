use std::str::FromStr;

fn main() {}

#[derive(Debug)]
struct MyError(String);

type STEPS = i16;

#[derive(Debug)]
enum Turn {
    Left,
    Right,
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
#[derive(Debug)]
struct Action {
    turn: Turn,
    steps: STEPS,
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

struct RotaryDial {
    perimeter: STEPS,
    pointer: STEPS,
}

impl RotaryDial {
    fn new(perimeter: STEPS, pointer: STEPS) -> RotaryDial {
        RotaryDial { perimeter, pointer }
    }
    fn turn_and_listen(&mut self, act: Action) -> bool {
        match act.turn {
            Turn::Left => todo!(),
            Turn::Right => todo!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_input() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

        let out = input
            .lines()
            .map(|action| action.parse::<Action>())
            .collect::<Result<Vec<_>, _>>();
        println!("{:?}", out);
    }
}
