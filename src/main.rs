mod forth;
mod interpreter;
mod operators;
mod operators_binary;

use std::io::{self, BufRead};

fn exit_ruforth(cmd: &str) -> bool {
    match cmd {
        "exit" => true,
        _ => false,
    }
}

fn start_ruforth() {

    let mut forth = forth::Forth::empty();
    let intr = interpreter::Interpreter::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let input_user = line.unwrap().trim().to_string();

        if exit_ruforth(&input_user) {
            println!("Bye!");
            return;
        } else {
            intr.eval(&mut forth, &input_user);
            println!("ok");
        }
    }
}

fn main() {
    start_ruforth();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let mut vec = Vec::new();
        vec.push(5);
        intr.eval(&mut forth, "2 3 +");
        assert_eq!(vec, forth.get_stack());
    }

    #[test]
    fn test_sub() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let mut vec = Vec::new();
        vec.push(-1);
        intr.eval(&mut forth, "2 3 -");
        assert_eq!(vec, forth.get_stack());
    }

    #[test]
    fn test_mul() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let mut vec = Vec::new();
        vec.push(6);
        intr.eval(&mut forth, "2 3 *");
        assert_eq!(vec, forth.get_stack());
    }

    #[test]
    fn test_div() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let mut vec = Vec::new();
        vec.push(2);
        intr.eval(&mut forth, "6 3 /");
        assert_eq!(vec, forth.get_stack());
    }

    #[test]
    fn test_mod() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let mut vec = Vec::new();
        vec.push(0);
        intr.eval(&mut forth, "3 3 mod");
        assert_eq!(vec, forth.get_stack());
    }

    #[test]
    fn test_dup() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let mut vec = Vec::new();
        vec.push(3);
        vec.push(3);
        intr.eval(&mut forth, "3 dup");
        assert_eq!(vec, forth.get_stack());
    }

    #[test]
    fn test_swap() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let mut vec = Vec::new();
        vec.push(2);
        vec.push(3);
        intr.eval(&mut forth, "3 2 swap");
        assert_eq!(vec, forth.get_stack());
    }

    #[test]
    fn test_negate() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let mut vec = Vec::new();
        vec.push(-99);
        intr.eval(&mut forth, "99 negate");
        assert_eq!(vec, forth.get_stack());
    }

    #[test]
    fn test_abs_negative() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let mut vec = Vec::new();
        vec.push(99);
        intr.eval(&mut forth, "-99 abs");
        assert_eq!(vec, forth.get_stack());
    }

    #[test]
    fn test_abs_positive() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let mut vec = Vec::new();
        vec.push(99);
        intr.eval(&mut forth, "99 abs");
        assert_eq!(vec, forth.get_stack());
    }

    #[test]
    fn test_rot() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let mut vec = Vec::new();
        vec.push(4);
        vec.push(5);
        vec.push(6);
        intr.eval(&mut forth, "6 4 5 rot");
        assert_eq!(vec, forth.get_stack());
    }

    #[test]
    fn test_drop() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let mut vec = Vec::new();
        vec.push(2);
        intr.eval(&mut forth, "2 3 drop");
        assert_eq!(vec, forth.get_stack());
    }

    #[test]
    fn test_nip() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(3);
        intr.eval(&mut forth, "1 2 3 nip");
        assert_eq!(vec, forth.get_stack());
    }

    #[test]
    fn test_tuck() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(2);
        vec.push(4);
        vec.push(3);
        vec.push(4);
        intr.eval(&mut forth, "1 2 3 4 tuck");
        assert_eq!(vec, forth.get_stack());
    }

    #[test]
    fn test_over() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        vec.push(3);
        intr.eval(&mut forth, "1 2 3 4 over");
        assert_eq!(vec, forth.get_stack());
    }

    #[test]
    fn test_max() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let mut vec = Vec::new();
        vec.push(3);
        intr.eval(&mut forth, "1 3 max");
        assert_eq!(vec, forth.get_stack());
    }

    #[test]
    fn test_min() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let mut vec = Vec::new();
        vec.push(1);
        intr.eval(&mut forth, "1 3 min");
        assert_eq!(vec, forth.get_stack());
    }

    #[test]
    fn test_clearstack() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let vec : Vec<i32> = Vec::new();
        intr.eval(&mut forth, "1 3 4 + clearstack");
        assert_eq!(vec, forth.get_stack());
    }

    #[test]
    fn test_add_word() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let mut vec = Vec::new();
        vec.push(25);
        intr.eval(&mut forth, ": square dup * ;");
        intr.eval(&mut forth, "5 square");
        assert_eq!(vec, forth.get_stack());
    }

    #[test]
    fn test_add_constant() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let mut vec = Vec::new();
        vec.push(100);
        intr.eval(&mut forth, "100 constant HUNDRED");
        intr.eval(&mut forth, "HUNDRED");
        assert_eq!(vec, forth.get_stack());
    }

    #[test]
    fn test_use_constant() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let mut vec = Vec::new();
        vec.push(125);
        intr.eval(&mut forth, "100 constant HUNDRED");
        intr.eval(&mut forth, "HUNDRED 25 +");
        assert_eq!(vec, forth.get_stack());
    }

    #[test]
    fn test_multiple_commands() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let mut vec = Vec::new();
        vec.push(14);
        intr.eval(&mut forth, "2 3 4 * +");
        assert_eq!(vec, forth.get_stack());
    }

    #[test]
    fn test_few_stack() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let vec : Vec<i32> = Vec::new();
        intr.eval(&mut forth, "2 +");
        assert_eq!(vec, forth.get_stack());
    }

    #[test]
    fn test_equals_1() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let mut vec = Vec::new();
        vec.push(-1);
        intr.eval(&mut forth, "2 2 =");
        assert_eq!(vec, forth.get_stack());
    }

    #[test]
    fn test_equals_2() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let mut vec = Vec::new();
        vec.push(0);
        intr.eval(&mut forth, "2 3 =");
        assert_eq!(vec, forth.get_stack());
    }

    #[test]
    fn test_not_equals_1() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let mut vec = Vec::new();
        vec.push(-1);
        intr.eval(&mut forth, "2 3 !=");
        assert_eq!(vec, forth.get_stack());
    }

    #[test]
    fn test_not_equals_2() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let mut vec = Vec::new();
        vec.push(0);
        intr.eval(&mut forth, "2 2 !=");
        assert_eq!(vec, forth.get_stack());
    }

    #[test]
    fn test_greater() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let mut vec = Vec::new();
        vec.push(-1);
        intr.eval(&mut forth, "4 2 >");
        assert_eq!(vec, forth.get_stack());
    }

    #[test]
    fn test_less() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let mut vec = Vec::new();
        vec.push(-1);
        intr.eval(&mut forth, "2 4 <");
        assert_eq!(vec, forth.get_stack());
    }

    #[test]
    fn test_greater_equals() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let mut vec = Vec::new();
        vec.push(-1);
        intr.eval(&mut forth, "2 2 >=");
        assert_eq!(vec, forth.get_stack());
    }

    #[test]
    fn test_less_equals() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let mut vec = Vec::new();
        vec.push(-1);
        intr.eval(&mut forth, "4 4 <=");
        assert_eq!(vec, forth.get_stack());
    }

    #[test]
    fn test_invert() {
        let mut forth = forth::Forth::empty();
        let intr = interpreter::Interpreter::new();
        let mut vec = Vec::new();
        vec.push(-1);
        intr.eval(&mut forth, "0 invert");
        assert_eq!(vec, forth.get_stack());
    }

}