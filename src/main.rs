
use std::io;

// Wait for user's input, parse the entered input as number in a loop. It breaks from loop if the entered number is valid.
fn read_line_number() -> i32 {
    let result: i32;
    let stdin = io::stdin();

    loop {
        let mut number = String::new();
        stdin.read_line(&mut number)
            .expect("Failed to read input !");
        result = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break;
    }
    result
}

// Analyse the number based on specific rules, returns the analysis result in a tuple of booleans.
fn analyse_number(number: i32) -> (bool, bool, bool, bool) {
    let mut result: (bool, bool, bool, bool) = (false, false, false, false);

    // Is number an odd or even number ?
    if number % 2 == 0 {
        result.1 = true;
    } else {
        result.0 = true;
    }

    // Is number less than 10
    result.2 = if number < 10 { true } else { false };
    // Is number higher than 25
    result.3 = if number > 25 { true } else { false };

    result
}

fn print_number_analysis_result(number: i32, analysis_result: (bool, bool, bool, bool)) {
    println!("Number {}: ", number);
    println!("is an odd number: {}", analysis_result.0);
    println!("is an even number: {}", analysis_result.1);
    println!("is less than 10: {}", analysis_result.2);
    println!("is higher than 25: {}", analysis_result.3);
}

// Main function
fn main() {
    println!("\nRust Control Flow's Demo.");
    println!("=====================\n");

    println!("1. 'if' control flow's demo:");
    println!("----------------------------\n");

    println!("Enter a number: ");
    let number: i32 = read_line_number();

    // Is the entered a number: odd, even, less than 10, higher than 25
    let analyse_result: (bool, bool, bool, bool) = analyse_number(number);
    print_number_analysis_result(number, analyse_result);

    println!("\n2. 'while' loop's demo");
    println!("-----------------------\n");

    let mut index = 0;
    while index < number {
        println!("{} is less than {}", index, number);        
        index = index+1;
    }

    println!("\n3. 'for' loop's demo");
    println!("-----------------------\n");

    let mut array_nums = [20, 30, 40, 50, 60];
    println!("Given, an array of numbers as follow: {:?}", array_nums);
    for i in {0..array_nums.len()} {
        array_nums[i] = array_nums[i] * number;
    }
    println!("When multiplied by the number you've entered, they are shown as follow: {:?} \n", array_nums);
}
