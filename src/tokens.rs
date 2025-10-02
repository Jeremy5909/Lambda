#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    LParen,
    RParen,
    Lambda,
    Dot,
    Var(String),
    Define,
}

pub fn tokenize(input: impl ToString) -> Vec<Token> {
    let mut tokens = Vec::new();
    let input = input.to_string();

    let mut chars = input.chars().peekable();
    while let Some(next) = chars.next() {
        match next {
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '.' => tokens.push(Token::Dot),
            '\\' => tokens.push(Token::Lambda),
            ':' => {
                if chars.peek() == Some(&'=') {
                    tokens.push(Token::Define)
                } else {
                    continue;
                }
            }
            next if next.is_whitespace() => {}
            _ => {
                let mut var = next.to_string();
                while let Some(&next) = chars.peek() {
                    if next == '('
                        || next == ')'
                        || next == '\\'
                        || next == '.'
                        || next == ':'
                        || next.is_whitespace()
                    {
                        break;
                    }
                    var.push(next);
                    chars.next();
                }
                tokens.push(Token::Var(var));
            }
        }
    }

    tokens
}
