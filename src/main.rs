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

impl Token {
    fn new(t_type: TokenType, t_value: char) -> Token {
        Token{
            token_type: t_type,
            token_value: t_value,
        }
    }

    fn cast_to_u32(t_value: char) -> u32 {
        t_value.to_digit(10).unwrap()
    }
}

fn tokenize(expression: &str) -> impl Iterator<Item = Token> {
    let mut tokens: Vec<Token> = Vec::with_capacity(expression.len());
    for token in expression.chars() {
        if token.is_digit(10) {
            tokens.push(Token::new(TokenType::Integer, token));
        } else {
            match token {
                '+' => { tokens.push(Token::new(TokenType::Plus, token)); },
                '-' => { tokens.push(Token::new(TokenType::Minus, token)); },
                '*' => { tokens.push(Token::new(TokenType::Mul, token)); },
                '/' => { tokens.push(Token::new(TokenType::Div, token)); },
                _ => { }
            };
        }
    }
    tokens.into_iter()
}

fn execute(mut tokens: impl Iterator<Item = Token>) {
    let tokens = tokens.collect::<Vec::<_>>();
    let operator = tokens[1].token_type;

    let first = Token::cast_to_u32(tokens[0].token_value);
    let second = Token::cast_to_u32(tokens[2].token_value);
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
