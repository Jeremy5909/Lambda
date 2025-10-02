use std::{collections::HashMap, fmt::Display};

use crate::tokens::Token;

pub type Environment = HashMap<String, Lambda>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Lambda {
    Variable(String),
    Function(String, Box<Lambda>),
    Application(Box<Lambda>, Box<Lambda>),
}

impl Display for Lambda {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lambda::Variable(name) => write!(f, "{name}"),
            Lambda::Function(param, arg) => write!(f, "\\{param}.{arg}"),
            Lambda::Application(lhs, rhs) => {
                match **lhs {
                    Lambda::Application(_, _) | Lambda::Function(_, _) => write!(f, "({lhs})")?,
                    _ => write!(f, "{lhs}")?,
                };

                write!(f, " ")?;

                match **rhs {
                    Lambda::Application(_, _) | Lambda::Function(_, _) => write!(f, "({rhs})"),
                    _ => write!(f, "{rhs}"),
                }
            }
        }
    }
}

impl Lambda {
    pub fn variable(name: &str) -> Self {
        Self::Variable(name.to_owned())
    }
    pub fn function(args: Vec<&str>, body: Lambda) -> Self {
        args.into_iter().rev().fold(body, |acc, arg| {
            Lambda::Function(arg.to_owned(), Box::new(acc))
        })
    }
    pub fn application(func: Lambda, args: Vec<Lambda>) -> Self {
        args.into_iter().fold(func, |acc, arg| {
            Lambda::Application(Box::new(acc), Box::new(arg))
        })
    }

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

    pub fn parse_definition(tokens: impl IntoIterator<Item = Token>, env: &mut Environment) {
        let mut tokens = tokens.into_iter();
        if let Some(Token::Var(name)) = tokens.next() {
            if let Some(Token::Define) = tokens.next() {
                let rhs: Vec<_> = tokens.collect();
                let rhs = Self::parse_tokens(rhs, Some(env));

                env.insert(
                    name.clone(),
                    rhs.expect(format!("Missing definition for {}", name).as_str()),
                );
            }
        }
    }
    pub fn parse_tokens(
        tokens: impl IntoIterator<Item = Token>,
        env: Option<&Environment>,
    ) -> Option<Lambda> {
        let tokens = tokens.into_iter();
        let mut lambdas = Vec::new();

        let mut iter = tokens.into_iter();
        while let Some(token) = iter.next() {
            match token {
                Token::Var(name) => {
                    if let Some(env) = env {
                        if let Some(value) = env.get(&name) {
                            lambdas.push(value.clone());
                            continue;
                        }
                    }
                    lambdas.push(Lambda::Variable(name))
                }
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

                    lambdas.push(Self::parse_tokens(inner_tokens, env)?);
                }
                Token::Lambda => {
                    let mut params_tokens = Vec::new();
                    while let Some(tok) = iter.next() {
                        if tok == Token::Dot {
                            break;
                        }
                        params_tokens.push(tok);
                    }

                    let params = Self::parse_variables(params_tokens);
                    let params_names: Vec<_> = params
                        .iter()
                        .map(|p| match p {
                            Lambda::Variable(name) => name.as_str(),
                            _ => panic!(),
                        })
                        .collect();

                    let body_tokens: Vec<_> = iter.collect();
                    let body = Self::parse_tokens(body_tokens, env);

                    return Some(Lambda::function(params_names, body?));
                }
                _ => {}
            }
        }
        let mut iter = lambdas.into_iter();
        let mut result = iter.next()?;
        for e in iter {
            result = Lambda::Application(Box::new(result), Box::new(e));
        }
        Some(result)
    }

    fn substitute(&self, var: &str, value: &Lambda) -> Self {
        match self {
            Lambda::Variable(name) if name == var => value.clone(),
            Lambda::Variable(_) => self.clone(),
            Lambda::Function(param, _) if param == var => self.clone(),
            Lambda::Function(param, body) => {
                Lambda::Function(param.clone(), Box::new(body.substitute(var, value)))
            }
            Lambda::Application(f, arg) => Lambda::Application(
                Box::new(f.substitute(var, value)),
                Box::new(arg.substitute(var, value)),
            ),
        }
    }
    pub fn reduce(&self) -> Self {
        match self {
            Lambda::Variable(_) => self.clone(),

            Lambda::Function(param, body) => {
                Lambda::Function(param.clone(), Box::new(body.reduce()))
            }
            Lambda::Application(f, arg) => match f.reduce() {
                Lambda::Function(param, body) => body.substitute(&param, &arg.reduce()).reduce(),
                reduced => Lambda::Application(Box::new(reduced), Box::new(arg.reduce())),
            },
        }
    }
}
