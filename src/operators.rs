use crate::forth::{Forth, ForthResult};

type ArOperator = fn(i32, i32) -> i32;

fn arithmetic_operator(name: &str, op: ArOperator, forth: &mut Forth) -> ForthResult<()> {
    let x = forth.pop(format!(
        "Empty stack: for first argument for {}",
        name.to_string()
    ))?;
    let y = forth.pop(format!(
        "Empty stack: for second argument for {}",
        name.to_string()
    ))?;
    forth.push(op(x, y));
    Ok(())
}

pub fn add(forth: &mut Forth) -> ForthResult<()> {
    arithmetic_operator("+", |x, y| x + y, forth)
}

pub fn sub(forth: &mut Forth) -> ForthResult<()> {
    arithmetic_operator("-", |x, y| y - x, forth)
}

pub fn mul(forth: &mut Forth) -> ForthResult<()> {
    arithmetic_operator("*", |x, y| x * y, forth)
}

pub fn div(forth: &mut Forth) -> ForthResult<()> {
    arithmetic_operator("/", |x, y| y / x, forth)
}

pub fn modulus(forth: &mut Forth) -> ForthResult<()> {
    arithmetic_operator("mod", |x, y| y % x, forth)
}