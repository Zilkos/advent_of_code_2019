use std::fs::read_to_string;

fn main() {
    let mut sum_fuel = 0;
    let mut sum_total_fuel = 0;
    // read_to_string -> Result, so unwrap needed.
    // lines return every line.
    // Iterate over and display each item.
    // As item is a string, parse it and give it to func.
    for item in read_to_string("input").unwrap().lines() {
        sum_fuel += calculate(item.parse().unwrap());
    }

    for item in read_to_string("input").unwrap().lines() {
        sum_total_fuel += fuel_of_fuel(item.parse().unwrap());
        println!("Item : {} -> {}",item, sum_total_fuel);
    }

    println!("Answer part 01 : {}", sum_fuel);
    println!("Answer part 02 : {}", sum_total_fuel);
}

// i32 ? Because Rust natively do entire division on it so no floor() required <3
fn calculate(val: i32) -> i32 {
    (val / 3) - 2
}

fn fuel_of_fuel(val: i32) -> i32 {
    let mut total_fuel = 0;
    let mut fuel = calculate(val);

    while fuel > 0 {
        total_fuel += fuel;
        fuel = calculate(fuel);
    }

    total_fuel
}

// recursive, fk dat shit
// fn fuel_of_fuel(val: i32) -> i32 {
//     let mut total = 0;
//     let mut fuel = calculate(val);
//     if fuel > 0 {
//         total += fuel;
//         fuel_of_fuel(fuel)
//     } else {
//         total
//     }
// }

#[test]
fn ok() {
    assert_eq!(2, calculate(12));
    assert_eq!(2, calculate(14));
    assert_eq!(654, calculate(1969));
    assert_eq!(33583, calculate(100756));
}