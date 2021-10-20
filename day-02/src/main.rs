use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input").unwrap();

    // Split it by coma into vector

    // TODO : find another way, this is FUCKIN HELL.
    let data_to_parse: Vec<&str> = input.split(',').collect();
    let mut data: Vec<usize> = Vec::new();

    for item in data_to_parse {
        data.push(item.parse().unwrap());
    }

    println!("{:?}", &data);
}

fn process(mut program: Vec<usize>) -> Vec<usize> {
    // program is a collection of opcode
    // an opcode is a 4 index pattern, so jump 4 by 4
    // [    0    ,   1,     2,       3]
    // [operation, input, input, output pos to store operation's result]
    // 99 break execution and exit program
    let mut index = 0;

    //loop until the next 99 or end of vec
    while program[index] != 99 {
        let opcode = program[index];
        let input_1 = program[index + 1];
        let input_2 = program[index + 2];
        let result = program[index + 3];

        // 1 == ADD input_1 + input_2 and store in output position
        if opcode == 1 {
            program[result] = program[input_1] + program[input_2];
        } else if opcode == 2 {
            // 2 == MULTIPLY input_1 by input_2 and store in output position
            program[result] = program[input_1] * program[input_2];
        } else {
            println!("Something went wrong !");
        }

        // opcode is block of 4 index
        index += 4;
    }

    program // return processed program
}

#[test]
fn process_program() {
    assert_eq!(vec![2,0,0,0,99], process(vec![1,0,0,0,99]));
    // assert_eq!(vec![2,3,0,6,99], process(vec![2,3,0,3,99]));
    // assert_eq!(vec![2,4,4,5,99,0], process(vec![2,4,4,5,99,9801]));
    // assert_eq!(vec![1,1,1,4,99,5,6,0,99], process(vec![30,1,1,4,2,5,6,0,99]));
}