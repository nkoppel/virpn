use pancurses::{Input, Input::*};
use crate::stack::Stack;
use crate::modes::ops::helpers::*;
use std::collections::HashMap;

pub fn bind_from_str(s: &str) -> Vec<Input> {
    let mut out = Vec::new();

    for c in s.chars() {
        out.push(Character(c));
    }

    out
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

use std::f64::consts;

pub fn gen_ops() -> Vec<(String, Vec<Input>, Op)> {
    vec![
        ("+"     , "q"  , op_2(&|x, y| x + y)),
        ("-"     , "w"  , op_2(&|x, y| x - y)),
        ("/"     , "e"  , op_2(&|x, y| x / y)),
        ("*"     , "r"  , op_2(&|x, y| x * y)),
        ("^"     , "t"  , op_2(&|x, y| x.powf(y))),

        ("square", "oq" , op_1(&|x| x * x)),
        ("sqrt"  , "or" , op_1(&|x| x.sqrt())),
        ("negate", "on" , op_1(&|x| -x)),

        ("ln"    , "oge", op_1(&|x| x.ln())),
        ("log10" , "oga", op_1(&|x| x.log10())),
        ("log2"  , "ogs", op_1(&|x| x.log2())),
        ("log"   , "ogg", op_2(&|x, y| x.log(y))),

        ("sin"   , "os" , op_1(&|x| x.sin())),
        ("cos"   , "oc" , op_1(&|x| x.cos())),
        ("tan"   , "ot" , op_1(&|x| x.tan())),

        ("asin"  , "oas", op_1(&|x| x.asin())),
        ("acos"  , "oac", op_1(&|x| x.acos())),
        ("atan"  , "oat", op_1(&|x| x.atan())),

        ("pi"    , "cp" , constant(consts::PI)),
        ("e"     , "ce" , constant(consts::E)),
        ("sqrt_2", "cq" , constant(consts::SQRT_2)),
        ("nan"   , "cn" , constant(consts::SQRT_2)),
        ("inf"   , "cip", constant(consts::SQRT_2)),
        ("-inf"  , "cin", constant(consts::SQRT_2)),

        ("sum"   , "isu", fold_op(&|x, y| x + y, 0.)),
        ("msum"  , "ism", fold_op(&|x, y| x * y, 1.)),

        ("clear" , "C"  , basic(&|st| st.clear())),
        ("swap"  , "isw", Box::new(swap)),
        ("dup"   , "isd", Box::new(duplicate)),
        ("pop"   , "isp", basic(&|st| {st.pop();})),
        ("rev"   , "isr", basic(&|st| st.rev())),
    ]
        .into_iter()
        .map(|(x, y, z)| (x.to_string(), bind_from_str(y), z))
        .collect()
}
