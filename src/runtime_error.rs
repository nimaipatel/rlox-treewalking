use std::{error::Error, fmt::Display, rc::Rc};

use crate::{lox_type::LoxType, token::Token};

// TODO: runtime errors should be reported using Stmt and Expr not Token
#[derive(Debug)]
pub enum RunTimeError<'a> {
    ReturnNotInAFunc,
    WrongNumArgs {
        paren: &'a Token<'a>,
        expected: usize,
        actual: usize,
    },
    NotCallable {
        paren: &'a Token<'a>,
    },
    OperandShouldBeNumber {
        operator: &'a Token<'a>,
        operand: Rc<LoxType<'a>>,
    },
    OperandsShouldBeNumber {
        op: &'a Token<'a>,
        left: Rc<LoxType<'a>>,
        right: Rc<LoxType<'a>>,
    },
    UndefinedVariable(&'a Token<'a>),
}

impl<'a> Error for RunTimeError<'a> {}

impl<'a> Display for RunTimeError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunTimeError::OperandShouldBeNumber { operator, operand } => write!(
                f,
                "Operand for the unary operator {} on line {} must be number but found {}",
                operator.lexeme, operator.line, operand
            ),
            RunTimeError::OperandsShouldBeNumber {
                op: operator,
                left,
                right,
            } => write!(
                f,
                "Operands for the binary operator {} on line {} must be numbers but found {:?} and {:?}",
                operator.lexeme, operator.line, left, right
            ),
            RunTimeError::UndefinedVariable(name) =>
                write!(
                    f,
                    "Found undefined variable {} on line {}",
                    name.lexeme, name.line
            ),
            RunTimeError::NotCallable { paren } =>
                write!(
                    f,
                    "Can't call {} on line {} as it is not callable",
                    paren.lexeme, paren.line
            ),
            RunTimeError::WrongNumArgs { paren, expected, actual } =>
                write!(
                    f,
                    "Expected {} arguments but got {} for {} on line {}", 
                    expected, actual, paren.lexeme, paren.line
            ),
            RunTimeError::ReturnNotInAFunc => write!(f, "Can't use return, not in a statment"),
        }
    }
}
