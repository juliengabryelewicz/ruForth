use crate::forth::{Forth, ForthResult};

type BinOperator = fn(i32, i32) -> bool;

fn binary_operator(name: &str, bin_op: BinOperator, forth: &mut Forth) -> ForthResult<()> {
    let a = forth.pop(format!(
        "Empty stack: for first argument for {}",
        name.to_string()
    ))?;
    let b = forth.pop(format!(
        "Empty stack: for second argument for {}",
        name.to_string()
    ))?;
    if bin_op(a, b) {
        forth.push(-1);
    } else {
        forth.push(0);
    }
    Ok(())
}

pub fn equals(forth: &mut Forth) -> ForthResult<()> {
    binary_operator("=", |a, b| a == b, forth)
}

pub fn not_equals(forth: &mut Forth) -> ForthResult<()> {
    binary_operator("=", |a, b| a != b, forth)
}

pub fn greater_than(forth: &mut Forth) -> ForthResult<()> {
    binary_operator(">", |a, b| b > a, forth)
}

pub fn less_than(forth: &mut Forth) -> ForthResult<()> {
    binary_operator("<", |a, b| b < a, forth)
}

pub fn greater_than_equals(forth: &mut Forth) -> ForthResult<()> {
    binary_operator(">=", |a, b| b >= a, forth)
}

pub fn less_than_equals(forth: &mut Forth) -> ForthResult<()> {
    binary_operator("<=", |a, b| b <= a, forth)
}