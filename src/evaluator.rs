use crate::lexer::Token;
use crate::parser::Expression;

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
