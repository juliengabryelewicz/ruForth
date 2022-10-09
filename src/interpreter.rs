use std::collections::HashMap;
use std::slice::Iter;

use crate::forth::{Forth, ForthResult, ForthWord, Operators};
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
        self.commands.insert("max".to_owned(), &operators::max);
        self.commands.insert("min".to_owned(), &operators::min);

        self.commands.insert("dup".to_owned(), &operators::dup);
        self.commands.insert("swap".to_owned(), &operators::swap);
        self.commands.insert("rot".to_owned(), &operators::rot);
        self.commands.insert("drop".to_owned(), &operators::drop);
        self.commands.insert("nip".to_owned(), &operators::nip);
        self.commands.insert("tuck".to_owned(), &operators::tuck);

        self.commands.insert("clearstack".to_owned(), &operators::clearstack);
    }

    fn eval_commands(&self, op: &str, forth: &mut Forth) -> Option<ForthResult<()>> {
        if self.commands.contains_key(op) {
            let opr = self.commands.get(op).unwrap();
            Some(opr(forth))
        } else {
            None
        }
    }

    fn eval_word(&self, name: &str, forth: &mut Forth) -> Option<ForthResult<()>> {
        match forth.get_word(name) {
            Some(ref word) => {
                self.eval_tokens(forth, &mut word.1.iter());
                Some(Ok(()))
            }
            None => None,
        }
    }

    fn eval_constants(&self, name: &str, forth: &mut Forth) -> Option<ForthResult<()>> {
        if let Some(a) = forth.get_constant(&name) {
            forth.push(a);
            Some(Ok(()))
        } else {
            None
        }
    }

    pub fn eval_tokens(&self, forth: &mut Forth, tokens: &mut Iter<String>) {
        while let Some(s) = tokens.next() {
            if s.trim().is_empty() {
                continue;
            }

            // new word to collect
            if s.trim() == ":" {
                match self.check_new_word(tokens) {
                    Ok(word) => {
                        println!("New word defined: {:?}", word);
                        let name = word.0.clone();
                        forth.add_word(&name, word);
                        continue;
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        break;
                    }
                }
            }

            // Print stack
            if s.trim() == "." {
                print!("> ");
                forth.print_stack();
                continue;
            }

            // Print all words available
            if s.trim() == "words" {

                for command in &self.commands {
                    let name = command.0.clone();
                    print!("{} ", name);
                }

                for word in forth.get_words() {
                    let name = word.0.clone();
                    print!("{} ", name);
                }
                continue;
            }

            // Create constant
            if s.trim() == "constant" {
                match self.create_constant(forth, tokens) {
                    Ok(_) => {
                        continue;
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        break;
                    }
                }
            }

            //write string
            if s.trim() == ".\"" {
                match self.check_string(tokens) {
                    Ok(sentence) => {
                        println!("{}", sentence);
                        continue
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        break;
                    }
                }
            }

            // Check for new word created
            match self.eval_word(s, forth) {
                None => (),
                Some(Ok(())) => continue,
                Some(Err(e)) => {
                    println!("Error: {}", e);
                    break;
                }
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

            //Check for constants
            match self.eval_constants(s, forth) {
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
    }


    fn check_new_word(&self, tokens: &mut Iter<String>) -> ForthResult<ForthWord> {
        if let Some(name) = tokens.next() {
            if !Self::valid_word_name(name) {
                return Err(format!("Invalid name for function: {}", name));
            }
            let mut definition: Vec<String> = vec![];
            for s in tokens {
                if s == ";" {
                    return Ok((name.to_string(), definition));
                } else {
                    definition.push(s.to_string());
                }
            }
        }
        Err("Invalid function".to_string())
    }

    fn check_string(&self, tokens: &mut Iter<String>) -> ForthResult<String> {
        let mut sentence = String::new();
        for s in tokens {
            if s == "\"" {
                return Ok(sentence);
            }
            if s.chars().last().unwrap() == '"' {
                let mut temp = s.clone();
                temp.pop();
                sentence.push_str(&temp.clone());
                return Ok(sentence);
            }
            sentence.push_str(&format!("{} ", s));
        }

        Err("Invalid string".to_string())
    }

    fn create_constant(&self, forth: &mut Forth, tokens: &mut Iter<String>) -> ForthResult<()> {
        if let Some(const_name) = tokens.next() {
            let a = forth.pop(format!("Stack empty to set constant {}", const_name))?;
            forth.add_constant(const_name, a);
            Ok(())
        } else {
            Err("Const name not found".to_string())
        }
    }

    fn valid_word_name(name: &str) -> bool {
        name.parse::<i32>().is_err()
    }
    
}