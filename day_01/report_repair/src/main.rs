use std::fs;
use std::error::Error;

/// The purpose of this exercise is to find two numbers in a 
/// given list that add up to 2020 and find the product of those numbers
fn main() {

    let input_values = import_values().unwrap();
    let result_value = find_2020_product(input_values);

    println!("The missing value is {}", result_value);
}

fn import_values() -> Result<Vec<u32>, Box<dyn Error>> {

    println!("Importing values");
    let input_file = "input.txt";
    let contents = fs::read_to_string(input_file)?;

    let mut input_values = Vec::new();
    for line in contents.lines() {
        let val: u32 = line.parse().unwrap();
        input_values.push(val);
    }

    Ok(input_values)
}

fn find_2020_product(input_values: Vec<u32>) -> u32 {
    println!("Beginning evaluation");

    // I'm in a time bind. I know there has to be a more elegant way to do this:
    let mut i = 0;
    let mut j = 1;

    while i < input_values.len() {
        while j < input_values.len() {
            let lhs = input_values[i];
            let rhs = input_values[j];

            if lhs + rhs == 2020 {
                return lhs * rhs;
            }

            j += 1;
        }
        i += 1;
        j = i + 1;
    }
    
    0
}
