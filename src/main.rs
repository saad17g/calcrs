//! # Command Line Calculator
//!
//! This is a command line calculator application written in Rust.
//! It evaluates mathematical expressions provided as command line arguments.
//!
//! ## Usage
//!
//! ```
//! calcrs <expression>
//! ```
//!
//! - `<expression>`: The mathematical expression to evaluate.
//!
//! ## Examples
//!
//! ```
//! calcrs "2 + 3 * 4"
//! calcrs "sin(0.5) * (1 + 2)"
//! calcrs "sqrt(16) / 2"
//! ```
//!
//! ## Error Handling
//!
//! The application handles the following error cases:
//!
//! - Invalid number of command line arguments
//! - Parsing errors in the expression
//! - Division by zero during evaluation
//! - Invalid mathematical operations
//!
//! In case of an error, an appropriate error message is displayed, and the application exits with a non-zero status code.
//!
//! ## Modules
//!
//! The application consists of the following modules:
//!
//! - `lexer`: Tokenizes the input expression into individual tokens.
//! - `parser`: Parses the tokens into an abstract syntax tree (AST).
//! - `evaluator`: Evaluates the AST and computes the result.

use std::env;
use std::process;

mod evaluator;
mod lexer;
mod parser;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: calcrs <expression>");
        process::exit(1);
    }

    let expression = &args[1];

    let tokens = lexer::tokenize(expression);
    let ast = match parser::parse(&tokens) {
        Ok(ast) => ast,
        Err(err) => {
            eprintln!("Parsing error: {}", err);
            process::exit(1);
        }
    };

    let result = match evaluator::evaluate(ast) {
        Ok(val) => val,
        Err(err) => match err {
            evaluator::EvaluationError::DivisionByZero => {
                eprintln!("Evaluation error: Division by zero");
                process::exit(1);
            }
            evaluator::EvaluationError::InvalidOperation => {
                eprintln!("Evaluation error: Invalid operation");
                process::exit(1);
            }
        },
    };

    println!("{}", result);
}
