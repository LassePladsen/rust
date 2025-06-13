use std::{
    f64,
    io::{self, Write},
    num::{ParseFloatError, ParseIntError},
};

#[allow(dead_code)]
fn read_line() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .unwrap();
    String::from(buffer.trim())
}

#[allow(dead_code)]
fn read_integer() -> Result<i32, ParseIntError> {
    let line = read_line();
    line.parse()
}

#[allow(dead_code)]
fn read_float() -> Result<f64, ParseFloatError> {
    let line = read_line();
    line.parse()
}

fn main() {
    print!("Enter a number: $ ");
    io::stdout().flush().expect("Could not flush stdout");
    let input = read_float();
    match input {
        Ok(number) => println!("You entered {number}"),
        Err(_) => println!("Please enter a valid number!"),
    }
}

