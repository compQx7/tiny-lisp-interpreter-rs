use crate::lexer::tokenize;
use crate::parser::parse;
use crate::evaluator::Evaluator;
use crate::env::Env;
use std::io::{self, Write};
use std::rc::Rc;
use std::cell::RefCell;

pub struct REPL {
    evaluator: Evaluator,
}

impl REPL {
    pub fn new(evaluator: Evaluator) -> Self {
        Self { evaluator }
    }

    pub fn run(&mut self) {
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        let mut input = String::new();

        loop {
            input.clear();
            print!("> ");
            stdout.flush().unwrap();
            if stdin.read_line(&mut input).is_err() {
                println!("\nError reading input.");
                continue;
            }
            let line = input.trim();
            if line.is_empty() { continue; }
            if line == ":quit" || line == ":q" { break; }

            // lex
            let tokens = match tokenize(line) {
                Ok(tokens) => tokens,
                Err(e) => {
                    println!("Lex error: {:?}", e);
                    continue;
                }
            };
            // parse
            let ast = match parse(&tokens) {
                Ok(ast) => ast,
                Err(e) => {
                    println!("Parse error: {:?}", e);
                    continue;
                }
            };
            // evaluate
            match self.evaluator.eval(&ast) {
                Ok(val) => println!("{}", val),
                Err(e) => println!("Eval error: {:?}", e),
            }
        }
    }
}

