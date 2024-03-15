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
