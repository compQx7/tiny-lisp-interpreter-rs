// pub mod lexer;
// pub mod parser;
// pub mod ast;
// pub mod eval;
pub mod repl;

/// Evaluate a single Lisp expression from string input
// pub fn eval_str(input: &str) -> Result<eval::value::LispValue, String> {
//     let tokens = lexer::tokenize(input).map_err(|e| format!("Lexer error: {}", e))?;
//     let expr = parser::parse(&tokens).map_err(|e| format!("Parser error: {}", e))?;
//     let mut env = eval::env::default_env();
//     eval::eval(&expr, &mut env)
// }

/// Start interactive REPL
pub fn start_repl() {
    repl::start();
}

