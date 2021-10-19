use std::fs::read_to_string;

fn main() {
    let mut sum_fuel = 0;
    // read_to_string -> Result, so unwrap needed.
    // lines return every line.
    // Iterate over and display each item.
    // As item is a string, parse it and give it to func.
    for item in read_to_string("input").unwrap().lines() {
        sum_fuel += calculate(item.parse().unwrap());
    }

    println!("{}", sum_fuel);
}

fn calculate(val: i32) -> i32 {
    (val / 3) - 2
}

#[test]
fn ok() {
    assert_eq!(2, calculate(12));
    assert_eq!(2, calculate(14));
    assert_eq!(654, calculate(1969));
    assert_eq!(33583, calculate(100756));
}