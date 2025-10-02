use std::{
    collections::HashMap,
    io::{Write, stdin, stdout},
};

use crate::{lambda::Lambda, tokens::tokenize};

mod lambda;
#[cfg(test)]
mod tests;
mod tokens;

fn main() {
    let mut environment: HashMap<String, Lambda> = HashMap::new();

    loop {
        let mut input = String::new();
        print!("> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let tokens = tokenize(input);

        Lambda::parse_definition(tokens.clone(), &mut environment);

        let Some(lambda) = Lambda::parse_tokens(tokens, Some(&environment)) else {
            continue;
        };

        println!("{}", lambda.reduce());
    }
}
