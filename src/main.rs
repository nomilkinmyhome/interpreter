#![feature(exact_size_is_empty)]

use std::io::{stdin, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TokenType {
    Plus,
    Minus,
    Mul,
    Div,
    Integer
}

struct Token {
    token_type: TokenType,
    token_value: char,
}

fn tokenize(expression: &str) -> impl Iterator<Item = Token> {
    let mut tokens: Vec<Token> = Vec::with_capacity(expression.len());
    for token in expression.chars() {
        if token.is_digit(10) {
            tokens.push(Token {
                token_type: TokenType::Integer,
                token_value: token,
            });
        } else {
            match token {
                '+' => {
                    tokens.push(
                        Token {
                            token_type: TokenType::Plus,
                            token_value: token,
                        }
                    );
                },
                '-' => {
                    tokens.push(
                        Token {
                            token_type: TokenType::Minus,
                            token_value: token,
                        }
                    );
                },
                '*' => {
                    tokens.push(
                        Token {
                            token_type: TokenType::Mul,
                            token_value: token,
                        }
                    );
                },
                '/' => {
                    tokens.push(
                        Token {
                            token_type: TokenType::Div,
                            token_value: token,
                        }
                    );
                },
                _ => { }
            };
        }
    }
    tokens.into_iter()
}

fn cast_token_to_u32(token: char) -> u32 {
    token.to_digit(10).unwrap()
}

fn execute(mut tokens: impl Iterator<Item = Token>) {
    let tokens = tokens.collect::<Vec::<_>>();
    let operator = tokens[1].token_type;

    let first = cast_token_to_u32(tokens[0].token_value);
    let second = cast_token_to_u32(tokens[2].token_value);
    match operator {
        TokenType::Plus => {
            println!("{}", first + second);
        },
        TokenType::Minus => {
            println!("{}", first - second);
        },
        TokenType::Mul => {
            println!("{}", first * second);
        },
        TokenType::Div => {
            if second == 0 {
                println!("< zero division error >");
            } else {
                println!("{}", first / second);
            }
        },
        _ => { }
    };
}

fn main() {
    loop {
        println!("-> ");
        let mut expression = String::new();
        stdin().read_line(&mut expression).unwrap();

        if let "exit!" = expression.trim() {
            break;
        }

        let expression = expression.as_str();
        execute(tokenize(expression));
    }
}
