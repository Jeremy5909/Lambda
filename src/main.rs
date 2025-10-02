use std::{
    collections::HashMap,
    fs,
    io::{Write, stdin, stdout},
};

use crate::{
    lambda::{Environment, Lambda},
    tokens::tokenize,
};

mod lambda;
mod tokens;

#[cfg(test)]
mod tests;

fn run(input: &str, env: &mut Environment) {
    let tokens = tokenize(input);

    Lambda::parse_definition(tokens.clone(), env);

    Lambda::parse_tokens(tokens, Some(&env)).inspect(|l| println!("{}", l.reduce()));
}

fn main() {
    let mut environment = HashMap::new();

    if let Some(path) = std::env::args().skip(1).next() {
        let input = fs::read_to_string(path).expect("File not found");
        for line in input.lines() {
            run(line, &mut environment);
        }
    } else {
        loop {
            let mut input = String::new();
            print!("> ");
            stdout().flush().unwrap();
            stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            run(input, &mut environment);
        }
    }
}
