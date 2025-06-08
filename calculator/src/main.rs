use std::{
    f64,
    io::{Write, stdin, stdout},
    num::{ParseFloatError, ParseIntError},
};

fn main() {
    print!("Enter a number: $ ");
    stdout().flush().expect("Could not flush stdout");
    let input = read_float();
    match input {
        Ok(number) => println!("You entered {number}"),
        Err(_) => println!("Please enter a valid number!"),
    }
}

fn read_line() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("Failed to read line");
    String::from(buffer.trim())
}

#[allow(dead_code)]
fn read_integer() -> Result<i32, ParseIntError> {
    read_line().parse()
}

#[allow(dead_code)]
fn read_float() -> Result<f64, ParseFloatError> {
    read_line().parse()
}
