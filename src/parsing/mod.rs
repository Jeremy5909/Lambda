use crate::{lambda::Lambda, parsing::tokens::Token};

pub mod tokens;

pub fn parse_variables(tokens: Vec<Token>) -> Vec<Lambda> {
    let mut vars = Vec::new();
    for token in tokens {
        match token {
            Token::Var(name) => vars.push(Lambda::Variable(name)),
            _ => {}
        }
    }
    vars
}

pub fn parse(tokens: Vec<Token>) -> Lambda {
    let mut lambdas = Vec::new();

    let mut iter = tokens.into_iter();
    while let Some(token) = iter.next() {
        match token {
            Token::Var(name) => lambdas.push(Lambda::Variable(name.clone())),
            Token::LParen => {
                // Parse everything inside parentheses
                let mut inner_tokens = Vec::new();
                let mut depth = 1;
                while let Some(tok) = iter.next() {
                    match tok {
                        Token::LParen => depth += 1,
                        Token::RParen => {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }
                        _ => {}
                    }
                    inner_tokens.push(tok);
                }
                lambdas.push(parse(inner_tokens));
            }
            Token::Lambda => {
                let mut params_tokens = Vec::new();
                while let Some(tok) = iter.next() {
                    if tok == Token::Dot {
                        break;
                    }
                    params_tokens.push(tok);
                }

                let params = parse_variables(params_tokens);
                let params_names: Vec<_> = params
                    .iter()
                    .map(|p| match p {
                        Lambda::Variable(name) => name.as_str(),
                        _ => panic!(),
                    })
                    .collect();

                let body_tokens: Vec<_> = iter.collect();
                let body = parse(body_tokens);

                return Lambda::function(params_names, body);
            }
            _ => {}
        }
    }
    let mut iter = lambdas.into_iter();
    let mut result = iter.next().unwrap();
    for e in iter {
        result = Lambda::Application(Box::new(result), Box::new(e));
    }
    result
}
