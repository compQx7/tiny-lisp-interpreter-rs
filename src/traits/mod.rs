use crate::ast::LispValue;
use crate::env::Env;
use crate::error::EvalError;
use std::rc::Rc;
use std::cell::RefCell;

/// Lisp function calling conventions
pub trait LispCallable {
    fn call(
        &self,
        args: &[LispValue],
        env: Rc<RefCell<Env>>,
    ) -> Result<LispValue, EvalError>;
}

/// Lisp macro expansion conventions
pub trait LispMacro {
    fn expand(
        &self,
        args: &[LispValue],
        env: Rc<RefCell<Env>>,
    ) -> Result<LispValue, EvalError>;
}

