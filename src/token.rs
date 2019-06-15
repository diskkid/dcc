#[derive(Debug)]
pub enum TokenType {
    Number(i64),
    Plus,
    Minus,
    Mul,
    Div,
    EOF,
}

#[derive(Debug)]
pub struct Token {
    pub t: TokenType,
    pub input: String,
}

pub fn tokenize(program: String) -> Vec<Token> {
    let mut chars = program.chars().peekable();
    let mut tokens = vec![];
    while let Some(c) = chars.next() {
        match c {
            ' ' => continue,
            '+' => {
                tokens.push(Token {
                    t: TokenType::Plus,
                    input: c.to_string(),
                });
            }
            '-' => {
                tokens.push(Token {
                    t: TokenType::Minus,
                    input: c.to_string(),
                });
            }
            '*' => {
                tokens.push(Token {
                    t: TokenType::Mul,
                    input: c.to_string(),
                });
            }
            '/' => {
                tokens.push(Token {
                    t: TokenType::Div,
                    input: c.to_string(),
                });
            }
            '0'...'9' => {
                let mut num = c.to_string();
                loop {
                    if let Some(c) = chars.peek() {
                        match c {
                            '0'...'9' => {
                                num.push(*c);
                            }
                            _ => break,
                        }
                        chars.next();
                    } else {
                        break;
                    }
                }
                let val = num.parse::<i64>().unwrap();
                tokens.push(Token {
                    t: TokenType::Number(val),
                    input: num,
                });
            }
            _ => panic!("Unexpected char {}", c),
        }
    }
    tokens.push(Token {
        t: TokenType::EOF,
        input: String::from(""),
    });
    tokens
}
