use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum LexError {
    UnexpectedChar(char),
    UnterminatedString,
    InvalidToken(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedToken(String),
    UnexpectedEOF,
    InvalidSyntax(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum EvalError {
    UnboundSymbol(String),
    TypeError(String),
    ArgumentError(String),
    EvalPanic(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LispError {
    Lex(LexError),
    Parse(ParseError),
    Eval(EvalError),
    // other: IOError, MacroError, FfiError
}

impl fmt::Display for LispError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LispError::Lex(e) => write!(f, "Lexer error: {:?}", e),
            LispError::Parse(e) => write!(f, "Parser error: {:?}", e),
            LispError::Eval(e) => write!(f, "Eval error: {:?}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_error() {
        let err = LexError::UnexpectedChar('!');
        assert_eq!(format!("{:?}", err), "UnexpectedChar('!')");
    }

    #[test]
    fn test_lisp_error_display() {
        let err = LispError::Eval(EvalError::UnboundSymbol("foo".into()));
        assert!(format!("{}", err).contains("Eval error"));
    }
}

