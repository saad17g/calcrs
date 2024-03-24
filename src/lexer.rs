//! # Lexer Module
//!
//! This module provides functionality for tokenizing the input expression.
//!
//! ## Tokens
//!
//! The following tokens are supported:
//!
//! - `Number`: Represents a numeric value.
//! - `Plus`: Represents the addition operator (`+`).
//! - `Minus`: Represents the subtraction operator (`-`).
//! - `Multiply`: Represents the multiplication operator (`*`).
//! - `Divide`: Represents the division operator (`/`).
//! - `LeftParen`: Represents a left parenthesis (`(`).
//! - `RightParen`: Represents a right parenthesis (`)`).
//! - `Cos`: Represents the cosine function.
//! - `Acos`: Represents the arccosine function.
//! - `Sin`: Represents the sine function.
//! - `Asin`: Represents the arcsine function.
//! - `Tan`: Represents the tangent function.
//! - `Atan`: Represents the arctangent function.
//! - `Sqrt`: Represents the square root function.
//! - `Pow`: Represents the exponentiation function.
//! - `Comma`: Represents a comma separator (`,`).
//!
//! ## Functions
//!
//! - `tokenize(input: &str) -> Vec<Token>`: Tokenizes the input expression into a vector of tokens.

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParen,
    RightParen,
    Cos,
    Acos,
    Sin,
    Asin,
    Tan,
    Atan,
    Sqrt,
    Pow,
    Comma,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '0'..='9' => {
                let mut number = String::from(c);
                while let Some(&next) = chars.peek() {
                    if next.is_digit(10) || next == '.' {
                        number.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(number.parse().unwrap()));
            }
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Multiply),
            '/' => tokens.push(Token::Divide),
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            ',' => tokens.push(Token::Comma),
            'a'..='z' => {
                let mut identifier = String::from(c);
                while let Some(&next) = chars.peek() {
                    if next.is_alphabetic() {
                        identifier.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }

                match identifier.as_str() {
                    "cos" => tokens.push(Token::Cos),
                    "sin" => tokens.push(Token::Sin),
                    "tan" => tokens.push(Token::Tan),
                    "acos" => tokens.push(Token::Acos),
                    "asin" => tokens.push(Token::Asin),
                    "atan" => tokens.push(Token::Atan),
                    "sqrt" => tokens.push(Token::Sqrt),
                    "pow" => tokens.push(Token::Pow),
                    _ => panic!("Unknown identifier: {}", identifier),
                }
            }
            ' ' => continue,
            _ => panic!("Invalid character: {}", c),
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_tokenization() {
        let input = "1 + (2 * 3 - 10.5) / sin(0.5)";
        let expected_tokens = vec![
            Token::Number(1.0),
            Token::Plus,
            Token::LeftParen,
            Token::Number(2.0),
            Token::Multiply,
            Token::Number(3.0),
            Token::Minus,
            Token::Number(10.5),
            Token::RightParen,
            Token::Divide,
            Token::Sin,
            Token::LeftParen,
            Token::Number(0.5),
            Token::RightParen,
        ];

        let tokens = tokenize(input);
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    #[should_panic(expected = "Unknown identifier: saad")]
    fn test_panic_incorrect_identifier() {
        let input = "saad(10)";
        tokenize(input);
    }

    #[test]
    #[should_panic(expected = "Invalid character: #")]
    fn test_panic_invalid_character() {
        let input = "10 # 5";
        tokenize(input);
    }
}
