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

pub fn negate(forth: &mut Forth) -> ForthResult<()> {
    let a = forth.pop("Empty stack for negate".to_string())?;
    forth.push(-a);
    Ok(())
}

pub fn abs(forth: &mut Forth) -> ForthResult<()> {
    let a = forth.pop("Empty stack for abs".to_string())?;
    forth.push(a.abs());
    Ok(())
}

pub fn dup(forth: &mut Forth) -> ForthResult<()> {
    let a = forth.pop("Empty stack for dup".to_string())?;
    forth.push(a);
    forth.push(a);
    Ok(())
}

pub fn swap(forth: &mut Forth) -> ForthResult<()> {
    let a = forth.pop("Empty stack for first element in swap".to_string())?;
    let b = forth.pop("Empty stack for second element in swap".to_string())?;
    forth.push(a);
    forth.push(b);
    Ok(())
}

pub fn rot(forth: &mut Forth) -> ForthResult<()> {
    let a = forth.pop("Empty stack for first element in rot".to_string())?;
    let b = forth.pop("Empty stack for second element in rot".to_string())?;
    let c = forth.pop("Empty stack for third element in rot".to_string())?;
    forth.push(b);
    forth.push(a);
    forth.push(c);
    Ok(())
}

pub fn drop(forth: &mut Forth) -> ForthResult<()> {
    forth.pop("Empty stack for drop".to_string())?;
    Ok(())
}