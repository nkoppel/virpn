use pancurses::{Input, Input::*};
use crate::stack::Stack;
use crate::op_helpers::*;
use std::collections::HashMap;

pub fn bind_from_str(s: &str) -> Vec<Input> {
    let mut out = Vec::new();

    for c in s.chars() {
        out.push(Character(c));
    }

    out
}

fn clear(stack: &mut Stack) -> Res<()> {
    stack.clear();
    Ok(())
}

fn swap(stack: &mut Stack) -> Res<()> {
    if stack.len() < 2 {
        return Err("Too few arguments!".to_string());
    }
    let i1 = stack.pop().unwrap();
    let i2 = stack.pop().unwrap();
    stack.push(i1);
    stack.push(i2);
    Ok(())
}

fn duplicate(stack: &mut Stack) -> Res<()> {
    if stack.is_empty() {
        return Err("Too few arguments!".to_string());
    }

    let i = stack.pop().unwrap();
    stack.push(i.clone());
    stack.push(i);
    Ok(())
}

pub fn gen_ops() -> Vec<(String, Vec<Input>, Op)> {
    vec![
        ("+".to_string(), bind_from_str("q"), op_2(&|x, y| x + y)),
        ("-".to_string(), bind_from_str("w"), op_2(&|x, y| x - y)),
        ("/".to_string(), bind_from_str("e"), op_2(&|x, y| x / y)),
        ("*".to_string(), bind_from_str("r"), op_2(&|x, y| x * y)),
        ("^".to_string(), bind_from_str("t"), op_2(&|x, y| x.powf(y))),

        ("square".to_string(), bind_from_str("uq") , op_1(&|x| x * x)),
        ("sqrt"  .to_string(), bind_from_str("ur") , op_1(&|x| x.sqrt())),
        ("negate".to_string(), bind_from_str("un") , op_1(&|x| -x)),

        ("sum"  .to_string(), bind_from_str("osu"), fold_op(&|x, y| x + y, 0.)),
        ("msum" .to_string(), bind_from_str("osm"), fold_op(&|x, y| x * y, 1.)),

        ("clear"    .to_string(), bind_from_str("c")  , Box::new(clear)),
        ("swap"     .to_string(), bind_from_str("osw"), Box::new(swap)),
        ("duplicate".to_string(), bind_from_str("osd"), Box::new(duplicate)),
    ]
}
