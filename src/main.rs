enum TokenType {
    Number(i64),
    Plus,
    Minus,
    EOF,
}

struct Token {
    t: TokenType,
    input: String,
}

fn tokenize(program: String) -> Vec<Token> {
    let mut chars = program.chars().peekable();
    let mut tokens = vec![];
    while let Some(c) = chars.next() {
        match c {
            ' ' => continue,
            '+' => {
                tokens.push(Token{
                    t: TokenType::Plus,
                    input: c.to_string()
                });
            },
            '-' => {
                tokens.push(Token{
                    t: TokenType::Minus,
                    input: c.to_string()
                });
            },
            '0'...'9' => {
                let mut num = c.to_string();
                loop {
                    if let Some(c) = chars.peek() {
                        match c {
                            '0'...'9' => {
                                num.push(*c);
                            },
                            _ => break,
                        }
                        chars.next();
                    } else {
                        break;
                    }
                }
                let val = num.parse::<i64>().unwrap();
                tokens.push(Token{
                    t: TokenType::Number(val),
                    input: num,
                });
            },
            _ => panic!("Unexpected char {}", c)
        }
    }
    tokens
}

fn main() {
    let mut args = std::env::args();
    if let Some(program) = args.nth(1) {
        let tokens = tokenize(program);

        // Print the prologue
        println!(".intel_syntax noprefix");
        println!(".global main");
        println!("main:");

        // Verify that the given expression starts with a number,
        // and then emit the first `mov` instruction.
        match tokens[0].t {
            TokenType::Number(value) => {
                println!("  mov rax, {}", value);
            },
            _ => panic!("Unexpected token"),
        }

        let mut tokens = tokens.iter();
        tokens.next();
        while let Some(token) = tokens.next() {
            match token.t {
                TokenType::Plus => {
                    let token = tokens.next().unwrap();
                    match token.t {
                        TokenType::Number(value) => {
                            println!("  add rax, {}", value);
                        },
                        _ => panic!("Unexpected token"),
                    }
                },
                TokenType::Minus => {
                    let token = tokens.next().unwrap();
                    match token.t {
                        TokenType::Number(value) => {
                            println!("  sub rax, {}", value);
                        },
                        _ => panic!("Unexpected token"),
                    }
                },
                _ => panic!("Unexpected token"),
            }
        }

        println!("  ret");
    }
}
