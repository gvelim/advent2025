use std::str::FromStr;

fn main() -> Result<(), MyError> {
    let input = std::fs::read_to_string("src/bin/day1/sample.txt").expect("file not found");
    let mut dial = RotaryDial::new(100, 50);

    let out: usize = input
        .lines()
        .map(|action| {
            action
                .parse::<Action>()
                .map_err(|e| panic!("{:?}", e.0))
                .unwrap()
        })
        .inspect(|a| print!("{:?}", a))
        .map(|a| dial.turn(a))
        .inspect(|a| println!(" = {a}"))
        .filter(|a| *a == 0)
        .count();
    println!("{:?}", out);

    Ok(())
}

type Steps = i16;

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
    steps: Steps,
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
            steps: Steps::from_str(&s[1..]).map_err(|_| {
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
    fn turn(&mut self, act: Action) -> Steps {
        self.accum += match act.turn {
            Turn::Left => -act.steps,
            Turn::Right => act.steps,
        };
        print!(" ({}) ", self.accum);
        self.start + self.accum % self.perimeter
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let mut dial = RotaryDial::new(100, 50);

        println!(
            "L68 {:?}",
            dial.turn(Action {
                turn: Turn::Left,
                steps: 68
            })
        );
        println!(
            "L30 {:?}",
            dial.turn(Action {
                turn: Turn::Left,
                steps: 30
            })
        );
        println!(
            "L48 {:?}",
            dial.turn(Action {
                turn: Turn::Right,
                steps: 48
            })
        );
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
