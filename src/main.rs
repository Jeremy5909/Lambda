use crate::{
    lambda::Lambda,
    parsing::{parse, tokens::tokenize},
};

mod lambda;
mod parsing;
#[cfg(test)]
mod tests;

fn main() {
    let one = parse(tokenize(r"(\f x.f x)"));
    let two = parse(tokenize(r"(\f x.f (f x))"));
    let successor = parse(tokenize(r"(\n f x.f (n f x))"));

    let result = Lambda::application(successor, vec![two]);
    println!("{:#?}", result.reduce());
}
