use std::result;

pub type ForthResult<T> = result::Result<T, String>;
pub type Operators = dyn Fn(&mut Forth) -> ForthResult<()>;

pub struct Forth {
    stack: Vec<i32>,
}

impl Forth {
    pub fn empty() -> Forth {
        Forth {
            stack: vec![]
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

}
