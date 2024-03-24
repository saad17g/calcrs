//! # Evaluator Module
//!
//! This module provides functionality for evaluating the abstract syntax tree (AST) and computing the result.
//!
//! ## Errors
//!
//! The `EvaluationError` enum represents the possible errors that can occur during evaluation:
//!
//! - `DivisionByZero`: Indicates an attempt to divide by zero.
//! - `InvalidOperation`: Indicates an invalid mathematical operation.
//!
//! ## Functions
//!
//! - `evaluate(ast: Expression) -> Result<f64, EvaluationError>`: Evaluates the AST and computes the result.
use crate::lexer::Token;
use crate::parser::Expression;

#[derive(Debug, PartialEq)]
pub enum EvaluationError {
    DivisionByZero,
    InvalidOperation,
}

pub fn evaluate(ast: Expression) -> Result<f64, EvaluationError> {
    match ast {
        Expression::Number(val) => Ok(val),
        Expression::BinaryOp(left, op, right) => {
            let left_val = evaluate(*left)?;
            let right_val = evaluate(*right)?;
            match op {
                Token::Plus => Ok(left_val + right_val),
                Token::Minus => Ok(left_val - right_val),
                Token::Multiply => Ok(left_val * right_val),
                Token::Divide => {
                    if right_val == 0.0 {
                        Err(EvaluationError::DivisionByZero)
                    } else {
                        Ok(left_val / right_val)
                    }
                }
                Token::Pow => Ok(left_val.powf(right_val)),
                _ => Err(EvaluationError::InvalidOperation),
            }
        }
        Expression::UnaryOp(op, expr) => {
            let val = evaluate(*expr)?;
            match op {
                Token::Minus => Ok(-val),
                Token::Cos => Ok(val.cos()),
                Token::Acos => Ok(val.acos()),
                Token::Sin => Ok(val.sin()),
                Token::Asin => Ok(val.asin()),
                Token::Tan => Ok(val.tan()),
                Token::Atan => Ok(val.atan()),
                Token::Sqrt => Ok(val.sqrt()),
                _ => Err(EvaluationError::InvalidOperation),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::tokenize;
    use crate::parser::parse;

    use super::*;

    #[test]
    fn test_evaluate_number() {
        let input = "42";
        let tokens = tokenize(input);
        let ast = parse(&tokens).unwrap();
        let result = evaluate(ast);
        assert_eq!(result, Ok(42.0));
    }

    #[test]
    fn test_evaluate_binary_ops() {
        let input = "2 + 3 * 4 - 10 / 5";
        let tokens = tokenize(input);
        let ast = parse(&tokens).unwrap();
        let result = evaluate(ast);
        assert_eq!(result, Ok(12.0));
    }

    #[test]
    fn test_evaluate_unary_ops() {
        let input = "sin(0)";
        let tokens = tokenize(input);
        let ast = parse(&tokens).unwrap();
        let result = evaluate(ast);
        assert_eq!(result.unwrap(), 0.0);
    }

    #[test]
    fn test_evaluate_division_by_zero() {
        let input = "1 / (2 - 2)";
        let tokens = tokenize(input);
        let ast = parse(&tokens).unwrap();
        let result = evaluate(ast);
        assert_eq!(result, Err(EvaluationError::DivisionByZero));
    }

    #[test]
    fn test_evaluate_unary_op_minus() {
        let input = "-2.0";
        let tokens = tokenize(input);
        let ast = parse(&tokens).unwrap();
        let result = evaluate(ast);
        assert_eq!(result.unwrap(), -2.0);
    }
}
