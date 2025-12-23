mod dial;

use dial::{Action, MyError, RotaryDial};

fn main() -> Result<(), MyError> {
    let input = std::fs::read_to_string("src/bin/day1/input.txt").expect("file not found");
    let actions = input
        .lines()
        .map(|action| action.parse::<Action>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut dial = RotaryDial::new(100, 50);
    let (zero_crossing, zero_count) =
        actions
            .iter()
            .fold((0, 0), |(crossing_count, zero_count), action| {
                let crossings = dial.count_zero_crossings(action);
                let needle = dial.turn(action);
                (
                    crossing_count + crossings,
                    zero_count + if needle == 0 { 1 } else { 0 },
                )
            });

    println!("Part 1: {:?}", zero_count);
    assert_eq!(zero_count, 969);
    println!("Part 2: {:?}", zero_crossing);
    assert_eq!(zero_crossing, 5887);

    Ok(())
}
