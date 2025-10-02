use std::io::{Write, stdin, stdout};

use crate::lambda::Lambda;

mod lambda;
#[cfg(test)]
mod tests;
mod tokens;

fn main() {
    loop {
        let mut input = String::new();
        print!("> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let Some(lambda) = Lambda::from_string(input) else {
            continue;
        };

        println!("{}", lambda.reduce());
    }
}
