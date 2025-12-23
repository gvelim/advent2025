mod dial;

use dial::{Action, MyError, RotaryDial};
use std::time;

fn main() -> Result<(), MyError> {
    let input = std::fs::read_to_string("src/bin/day1/input.txt").expect("file not found");
    let actions = input
        .lines()
        .map(|action| action.parse::<Action>())
        .collect::<Result<Vec<_>, _>>()?;

    let t = time::Instant::now();
    let (zero_crossing, zero_count, _) = actions.iter().fold(
        (0, 0, RotaryDial::new(100, 50)),
        |(mut crossing_count, mut zero_count, mut dial), action| {
            crossing_count += dial.count_zero_crossings(action);
            dial = dial.turn(action);
            zero_count += if dial.needle == 0 { 1 } else { 0 };
            (crossing_count, zero_count, dial)
        },
    );

    println!("Time: {:?}", t.elapsed());
    println!("Part 1: {:?}", zero_count);
    assert_eq!(zero_count, 969);
    println!("Part 2: {:?}", zero_crossing);
    assert_eq!(zero_crossing, 5887);

    Ok(())
}
