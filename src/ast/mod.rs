use std::rc::Rc;
use crate::traits::{LispCallable};
use std::fmt;

// Values handled in Lisp
#[derive(Clone)]
pub enum LispValue {
    Number(f64),
    Symbol(String),
    List(Vec<LispValue>),
    Function(Rc<dyn LispCallable>),
    // Macro(Rc<dyn LispMacro>),
    Bool(bool),
    String(String),
    Nil,
    // Vector(Vec<LispValue>),
    // Map(HashMap<String, LispValue>),
}

pub struct UserFunction {
    pub params: Vec<String>,
    pub body: Box<LispValue>,
    pub env: Rc<std::cell::RefCell<crate::env::Env>>, // Definition environment (closure)
}

impl fmt::Debug for LispValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LispValue::Number(n) => write!(f, "Number({})", n),
            LispValue::Symbol(s) => write!(f, "Symbol({})", s),
            LispValue::List(lst) => write!(f, "List({:?})", lst),
            LispValue::Function(_) => write!(f, "Function"),
            // LispValue::Macro(_) => write!(f, "Macro"),
            LispValue::Bool(b) => write!(f, "Bool({})", b),
            LispValue::String(s) => write!(f, "String({})", s),
            LispValue::Nil => write!(f, "Nil"),
        }
    }
}

impl fmt::Display for LispValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LispValue::Number(n) => write!(f, "{}", n),
            LispValue::Symbol(s) => write!(f, "{}", s),
            LispValue::List(lst) => {
                let items: Vec<String> = lst.iter().map(|v| format!("{}", v)).collect();
                write!(f, "({})", items.join(" "))
            }
            LispValue::Function(_) => write!(f, "<function>"),
            // LispValue::Macro(_) => write!(f, "<macro>"),
            LispValue::Bool(b) => write!(f, "{}", b),
            LispValue::String(s) => write!(f, "\"{}\"", s),
            LispValue::Nil => write!(f, "nil"),
        }
    }
}

impl PartialEq for LispValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LispValue::Number(a), LispValue::Number(b)) => a == b,
            (LispValue::Symbol(a), LispValue::Symbol(b)) => a == b,
            (LispValue::List(a), LispValue::List(b)) => a == b,
            (LispValue::Bool(a), LispValue::Bool(b)) => a == b,
            (LispValue::String(a), LispValue::String(b)) => a == b,
            (LispValue::Nil, LispValue::Nil) => true,
            // (LispValue::Function(a), LispValue::Function(b)) => Rc::ptr_eq(a, b),
            (LispValue::Function(_), LispValue::Function(_)) => false,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number() {
        let n = LispValue::Number(42.0);
        assert_eq!(format!("{}", n), "42");
    }

    #[test]
    fn test_symbol() {
        let s = LispValue::Symbol("foo".to_string());
        assert_eq!(format!("{}", s), "foo");
    }

    #[test]
    fn test_list() {
        let lst = LispValue::List(vec![LispValue::Number(1.0), LispValue::Symbol("x".to_string())]);
        assert_eq!(format!("{}", lst), "(1 x)");
    }

    #[test]
    fn test_nil() {
        let nil = LispValue::Nil;
        assert_eq!(format!("{}", nil), "nil");
    }

    #[test]
    fn test_string_value() {
        let s = LispValue::String("hello".to_string());
        assert_eq!(format!("{}", s), "\"hello\"");
        assert_eq!(s, LispValue::String("hello".to_string()));
    }
}

