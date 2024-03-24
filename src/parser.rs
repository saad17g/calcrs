//! # Parser Module
//!
//! This module provides functionality for parsing the tokenized expression into an abstract syntax tree (AST).
//!
//! ## Expression
//!
//! The `Expression` enum represents the different types of expressions in the AST:
//!
//! - `Number`: Represents a numeric value.
//! - `BinaryOp`: Represents a binary operation with a left operand, an operator, and a right operand.
//! - `UnaryOp`: Represents a unary operation with an operator and an operand.
//!
//! ## Functions
//!
//! - `parse(tokens: &Vec<Token>) -> Result<Expression, String>`: Parses the tokens into an AST.
//! - `parse_expression(iter: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Result<Expression, String>`: Parses an expression.
//! - `parse_term(iter: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Result<Expression, String>`: Parses a term.
//! - `parse_factor(iter: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Result<Expression, String>`: Parses a factor.
//! - `parse_unary_op(iter: &mut std::iter::Peekable<std::slice::Iter<Token>>, op: Token) -> Result<Expression, String>`: Parses a unary operation.
//! - `parse_binary_op(iter: &mut std::iter::Peekable<std::slice::Iter<Token>>, op: Token) -> Result<Expression, String>`: Parses a binary operation.

use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(f64),
    BinaryOp(Box<Expression>, Token, Box<Expression>),
    UnaryOp(Token, Box<Expression>),
}

pub fn parse(tokens: &Vec<Token>) -> Result<Expression, String> {
    let mut iter = tokens.iter().peekable();
    parse_expression(&mut iter)
}

fn parse_expression(
    iter: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<Expression, String> {
    let mut left = parse_term(iter)?;

    while let Some(&token) = iter.peek() {
        match token {
            Token::Plus | Token::Minus => {
                iter.next();
                let right = parse_term(iter)?;
                left = Expression::BinaryOp(Box::new(left), token.clone(), Box::new(right));
            }
            _ => break,
        }
    }

    Ok(left)
}

fn parse_term(
    iter: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<Expression, String> {
    let mut left = parse_factor(iter)?;

    while let Some(&token) = iter.peek() {
        match token {
            Token::Multiply | Token::Divide => {
                iter.next();
                let right = parse_factor(iter)?;
                left = Expression::BinaryOp(Box::new(left), token.clone(), Box::new(right));
            }
            _ => break,
        }
    }

    Ok(left)
}

fn parse_factor(
    iter: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<Expression, String> {
    match iter.next() {
        Some(Token::Number(val)) => Ok(Expression::Number(*val)),
        Some(Token::Minus) => {
            let expr = parse_factor(iter)?;
            Ok(Expression::UnaryOp(Token::Minus, Box::new(expr)))
        }

        Some(Token::LeftParen) => {
            let expr = parse_expression(iter)?;
            match iter.next() {
                Some(Token::RightParen) => Ok(expr),
                _ => Err(String::from("Expected right parenthese")),
            }
        }
        Some(Token::Cos) => parse_unary_op(iter, Token::Cos),
        Some(Token::Acos) => parse_unary_op(iter, Token::Acos),
        Some(Token::Sin) => parse_unary_op(iter, Token::Sin),
        Some(Token::Asin) => parse_unary_op(iter, Token::Asin),
        Some(Token::Tan) => parse_unary_op(iter, Token::Tan),
        Some(Token::Atan) => parse_unary_op(iter, Token::Atan),
        Some(Token::Sqrt) => parse_unary_op(iter, Token::Sqrt),
        Some(Token::Pow) => parse_binary_op(iter, Token::Pow),
        Some(_) => Err(String::from("Unexpected token")),
        None => Err(String::from("Unexpected end of input")),
    }
}

fn parse_unary_op(
    iter: &mut std::iter::Peekable<std::slice::Iter<Token>>,
    op: Token,
) -> Result<Expression, String> {
    match op {
        Token::Minus => {
            let expr = parse_factor(iter)?;
            Ok(Expression::UnaryOp(op, Box::new(expr)))
        }
        _ => match iter.next() {
            Some(Token::LeftParen) => {
                let expr = parse_expression(iter)?;
                match iter.next() {
                    Some(Token::RightParen) => Ok(Expression::UnaryOp(op, Box::new(expr))),
                    _ => Err(String::from("Expected right parenthese")),
                }
            }
            _ => Err(String::from("Expected left parenthese")),
        },
    }
}

fn parse_binary_op(
    iter: &mut std::iter::Peekable<std::slice::Iter<Token>>,
    op: Token,
) -> Result<Expression, String> {
    match iter.next() {
        Some(Token::LeftParen) => {
            let left = parse_expression(iter)?;
            match iter.next() {
                Some(Token::Comma) => {
                    let right = parse_expression(iter)?;
                    match iter.next() {
                        Some(Token::RightParen) => {
                            Ok(Expression::BinaryOp(Box::new(left), op, Box::new(right)))
                        }
                        _ => Err(String::from("Expected right parenthese")),
                    }
                }
                _ => Err(String::from("Expected comma")),
            }
        }
        _ => Err(String::from("Expected left parenthese")),
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::tokenize;

    use super::*;

    #[test]
    fn test_parse_expression() {
        let input = "1 + (2 * 3 - 10.5) / sin(0.5)";
        let tokens = tokenize(input);
        let expected_ast = Expression::BinaryOp(
            Box::new(Expression::Number(1.0)),
            Token::Plus,
            Box::new(Expression::BinaryOp(
                Box::new(Expression::BinaryOp(
                    Box::new(Expression::BinaryOp(
                        Box::new(Expression::Number(2.0)),
                        Token::Multiply,
                        Box::new(Expression::Number(3.0)),
                    )),
                    Token::Minus,
                    Box::new(Expression::Number(10.5)),
                )),
                Token::Divide,
                Box::new(Expression::UnaryOp(
                    Token::Sin,
                    Box::new(Expression::Number(0.5)),
                )),
            )),
        );

        let ast = parse(&tokens).unwrap();
        assert_eq!(ast, expected_ast);
    }

    #[test]
    fn test_parse_unary_op() {
        let input = "sin(0.5)";
        let tokens = tokenize(input);
        let expected_ast = Expression::UnaryOp(Token::Sin, Box::new(Expression::Number(0.5)));

        let ast = parse(&tokens).unwrap();
        assert_eq!(ast, expected_ast);
    }

    #[test]
    fn test_parse_binary_op_pow() {
        let input = "pow(2, 3)";
        let tokens = tokenize(input);
        let expected_ast = Expression::BinaryOp(
            Box::new(Expression::Number(2.0)),
            Token::Pow,
            Box::new(Expression::Number(3.0)),
        );

        let ast = parse(&tokens).unwrap();
        assert_eq!(ast, expected_ast);
    }

    #[test]
    fn test_parse_nested_expressions() {
        let input = "((1 + 2) * 3 - (4 / 2)) * (5 + 6)";
        let tokens = tokenize(input);
        let expected_ast = Expression::BinaryOp(
            Box::new(Expression::BinaryOp(
                Box::new(Expression::BinaryOp(
                    Box::new(Expression::BinaryOp(
                        Box::new(Expression::Number(1.0)),
                        Token::Plus,
                        Box::new(Expression::Number(2.0)),
                    )),
                    Token::Multiply,
                    Box::new(Expression::Number(3.0)),
                )),
                Token::Minus,
                Box::new(Expression::BinaryOp(
                    Box::new(Expression::Number(4.0)),
                    Token::Divide,
                    Box::new(Expression::Number(2.0)),
                )),
            )),
            Token::Multiply,
            Box::new(Expression::BinaryOp(
                Box::new(Expression::Number(5.0)),
                Token::Plus,
                Box::new(Expression::Number(6.0)),
            )),
        );

        let ast = parse(&tokens).unwrap();
        assert_eq!(ast, expected_ast);
    }

    #[test]
    fn test_parse_multiple_unary_ops() {
        let input = "sin(cos(tan(0.5)))";
        let tokens = tokenize(input);
        let expected_ast = Expression::UnaryOp(
            Token::Sin,
            Box::new(Expression::UnaryOp(
                Token::Cos,
                Box::new(Expression::UnaryOp(
                    Token::Tan,
                    Box::new(Expression::Number(0.5)),
                )),
            )),
        );

        let ast = parse(&tokens).unwrap();
        assert_eq!(ast, expected_ast);
    }

    #[test]
    fn test_parse_binary_op_pow_with_expressions() {
        let input = "pow(2 + 3, 4 - 1)";
        let tokens = tokenize(input);
        let expected_ast = Expression::BinaryOp(
            Box::new(Expression::BinaryOp(
                Box::new(Expression::Number(2.0)),
                Token::Plus,
                Box::new(Expression::Number(3.0)),
            )),
            Token::Pow,
            Box::new(Expression::BinaryOp(
                Box::new(Expression::Number(4.0)),
                Token::Minus,
                Box::new(Expression::Number(1.0)),
            )),
        );

        let ast = parse(&tokens).unwrap();
        assert_eq!(ast, expected_ast);
    }

    #[test]
    fn test_parse_invalid_expression() {
        let input = "1 + (2 * 3";
        let tokens = tokenize(input);
        let result = parse(&tokens);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_empty_input() {
        let input = "";
        let tokens = tokenize(input);
        let result = parse(&tokens);
        assert!(result.is_err());
    }
}
