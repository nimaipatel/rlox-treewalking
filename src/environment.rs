use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::lox_type::{FunctionType, LoxType};
use crate::runtime_error::RunTimeError;
use crate::token::Token;
use crate::token_type::TokenType;

pub struct Environment<'a> {
    pub map: HashMap<String, Rc<LoxType<'a>>>,
    pub enclosing: Option<Rc<RefCell<Environment<'a>>>>,
}

impl<'a> Environment<'a> {
    pub fn fresh() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Environment::new(None)))
    }

    pub fn new(enclosing: Option<Rc<RefCell<Environment<'a>>>>) -> Self {
        if let Some(enclosing) = enclosing {
            Self {
                map: HashMap::new(),
                enclosing: Some(enclosing),
            }
        } else {
            let mut globals = Self {
                map: HashMap::new(),
                enclosing: None,
            };
            let clock = LoxType::Function {
                call: Box::new(|_, _| {
                    let time = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    Ok(LoxType::Number(time as f64).into())
                }),
                arity: 0,
                function_type: FunctionType::NativeFunction("clock".into()),
                closure: None,
            };
            globals.define("clock".into(), clock.into());
            globals
        }
    }

    pub fn define(&mut self, name: String, value: Rc<LoxType<'a>>) {
        self.map.insert(name, value);
    }

    pub fn get(&self, name: &'a Token) -> Result<Rc<LoxType<'a>>, RunTimeError<'a>> {
        match &name.token_type {
            TokenType::Identifier => match self.map.get(name.lexeme) {
                Some(lox_val) => Ok(Rc::clone(lox_val)),
                None => match &self.enclosing {
                    Some(enclosing) => enclosing.borrow_mut().get(name),
                    None => Err(RunTimeError::UndefinedVariable(name)),
                },
            },
            _ => unreachable!(),
        }
    }

    pub fn assign(
        &mut self,
        name: &'a Token<'a>,
        value: Rc<LoxType<'a>>,
    ) -> Result<Rc<LoxType<'a>>, RunTimeError<'a>> {
        if let Some(old_val) = self.map.get_mut(name.lexeme) {
            *old_val = value;
            Ok(LoxType::Nil.into())
        } else {
            match &mut self.enclosing {
                Some(enclosing) => enclosing.borrow_mut().assign(name, value),
                None => Err(RunTimeError::UndefinedVariable(name)),
            }
        }
    }
}
