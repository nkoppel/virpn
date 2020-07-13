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

        ("square".to_string(), bind_from_str("oq") , op_1(&|x| x * x)),
        ("sqrt"  .to_string(), bind_from_str("or") , op_1(&|x| x.sqrt())),
        ("negate".to_string(), bind_from_str("on") , op_1(&|x| -x)),

        ("ln"   .to_string(), bind_from_str("oge"), op_1(&|x| x.ln())),
        ("log10".to_string(), bind_from_str("oga"), op_1(&|x| x.log10())),
        ("log2" .to_string(), bind_from_str("ogs"), op_1(&|x| x.log2())),
        ("log"  .to_string(), bind_from_str("ogg"), op_2(&|x, y| x.log(y))),

        ("sin".to_string(), bind_from_str("os") , op_1(&|x| x.sin())),
        ("cos".to_string(), bind_from_str("oc") , op_1(&|x| x.cos())),
        ("tan".to_string(), bind_from_str("ot") , op_1(&|x| x.tan())),

        ("asin".to_string(), bind_from_str("oas") , op_1(&|x| x.asin())),
        ("acos".to_string(), bind_from_str("oac") , op_1(&|x| x.acos())),
        ("atan".to_string(), bind_from_str("oat") , op_1(&|x| x.atan())),


        ("sum" .to_string(), bind_from_str("isu"), fold_op(&|x, y| x + y, 0.)),
        ("msum".to_string(), bind_from_str("ism"), fold_op(&|x, y| x * y, 1.)),

        ("clear".to_string(), bind_from_str("c")  , basic(&|st| st.clear())),
        ("swap" .to_string(), bind_from_str("isw"), Box::new(swap)),
        ("dup"  .to_string(), bind_from_str("isd"), Box::new(duplicate)),
        ("pop"  .to_string(), bind_from_str("isp"), basic(&|st| {st.pop();})),
        ("rev"  .to_string(), bind_from_str("isr"), basic(&|st| st.rev())),
    ]
}
