use std::collections::HashMap;
use std::result;

pub type ForthResult<T> = result::Result<T, String>;
pub type ForthWord = (String, Vec<String>);
pub type Operators = dyn Fn(&mut Forth) -> ForthResult<()>;

pub struct Forth {
    stack: Vec<i32>,
    words: HashMap<String, ForthWord>
}

impl Forth {
    pub fn empty() -> Forth {
        Forth {
            stack: vec![],
            words: HashMap::new()
        }
    }

    pub fn pop(&mut self, msg: String) -> ForthResult<i32> {
        match self.stack.pop() {
            Some(n) => Ok(n),
            None => Err(msg),
        }
    }

    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
    }

    pub fn print_stack(&self) {
        println!("{:?}", self.stack);
    }

    pub fn get_stack(&self) -> Vec<i32> {
        self.stack.clone()
    }

    pub fn add_word(&mut self, name: &str, word: ForthWord) -> Option<ForthWord> {
        self.words.insert(name.to_string(), word)
    }

    pub fn get_word(&self, name: &str) -> Option<ForthWord> {
        match self.words.get(name) {
            Some(word) => Some(word.clone()),
            None => None,
        }
    }

}
