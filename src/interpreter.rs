use std::collections::HashMap;
use std::slice::Iter;

use crate::forth::{Forth, ForthResult, Operators};
use crate::operators;

pub struct Interpreter<'a> {
    pub commands: HashMap<String, &'a Operators>,
}

impl<'a> Interpreter<'a> {
    pub fn eval(&self, forth: &mut Forth, instruction: &str) {
        let tokens: Vec<_> = instruction.split(' ').map(|s| s.trim().to_string()).collect();
        self.eval_tokens(forth, &mut tokens.iter());
    }

    pub fn new() -> Self {
        let mut intr = Interpreter {
            commands: HashMap::new(),
        };

        intr.init();
        intr
    }

    fn init(&mut self) {
        self.commands.insert("+".to_owned(), &operators::add);
        self.commands.insert("-".to_owned(), &operators::sub);
        self.commands.insert("*".to_owned(), &operators::mul);
        self.commands.insert("/".to_owned(), &operators::div);
        self.commands.insert("mod".to_owned(), &operators::modulus);
        self.commands.insert("negate".to_owned(), &operators::negate);
        self.commands.insert("abs".to_owned(), &operators::abs);

        self.commands.insert("dup".to_owned(), &operators::dup);
        self.commands.insert("swap".to_owned(), &operators::swap);
    }

    fn eval_commands(&self, op: &str, forth: &mut Forth) -> Option<ForthResult<()>> {
        if self.commands.contains_key(op) {
            let opr = self.commands.get(op).unwrap();
            Some(opr(forth))
        } else {
            None
        }
    }

    pub fn eval_tokens(&self, forth: &mut Forth, tokens: &mut Iter<String>) {
        while let Some(s) = tokens.next() {
            if s.trim().is_empty() {
                continue;
            }

            //Check for default commands
            match self.eval_commands(s, forth) {
                None => (),
                Some(Ok(())) => continue,
                Some(Err(e)) => {
                    println!("Error: {}", e);
                    break;
                }
            }

            //Check for number
            match s.parse::<i32>() {
                Ok(num) => forth.push(num),
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
            }
        }

        print!("> ");
        forth.print_stack();
    }
}