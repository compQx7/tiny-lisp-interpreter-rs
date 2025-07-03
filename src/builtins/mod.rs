use crate::ast::LispValue;
use crate::env::Env;
use crate::traits::LispCallable;
use crate::error::EvalError;
use std::rc::Rc;
use std::cell::RefCell;

pub struct AddFn;

impl LispCallable for AddFn {
    fn call(
        &self,
        args: &[LispValue],
        _env: Rc<RefCell<Env>>,
    ) -> Result<LispValue, EvalError> {
        let mut sum = 0.0;
        for v in args {
            match v {
                LispValue::Number(n) => sum += n,
                _ => return Err(EvalError::TypeError("Expected number".into())),
            }
        }
        Ok(LispValue::Number(sum))
    }
}

pub struct SubFn;
impl LispCallable for SubFn {
    fn call(
        &self,
        args: &[LispValue],
        _env: Rc<RefCell<Env>>,
    ) -> Result<LispValue, EvalError> {
        let mut iter = args.iter();
        let first = match iter.next() {
            Some(LispValue::Number(n)) => *n,
            _ => return Err(EvalError::TypeError("Expected number".into())),
        };
        let result = iter.fold(first, |acc, v| match v {
            LispValue::Number(n) => acc - n,
            _ => acc, // Type errors are detected at the beginning
        });
        Ok(LispValue::Number(result))
    }
}

pub fn register_builtins(env: &mut Env) {
    env.set("+".to_string(), LispValue::Function(Rc::new(AddFn)));
    env.set("-".to_string(), LispValue::Function(Rc::new(SubFn)));
    // env.set("car".to_string(), LispValue::Function(Rc::new(CarFn)));
    // env.set("cdr".to_string(), LispValue::Function(Rc::new(CdrFn)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::LispValue;
    use crate::env::Env;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn test_add() {
        let f = AddFn;
        let args = [LispValue::Number(1.0), LispValue::Number(2.0)];
        let env = Rc::new(RefCell::new(Env::new()));
        let result = f.call(&args, env);
        assert_eq!(result, Ok(LispValue::Number(3.0)));
    }

    #[test]
    fn test_sub() {
        let f = SubFn;
        let args = [LispValue::Number(5.0), LispValue::Number(3.0)];
        let env = Rc::new(RefCell::new(Env::new()));
        let result = f.call(&args, env);
        assert_eq!(result, Ok(LispValue::Number(2.0)));
    }
}

