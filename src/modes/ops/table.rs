use pancurses::{Input, Input::*};
use crate::stack::Stack;
use crate::modes::ops::helpers::*;
use crate::io::bind_from_str;
use std::collections::HashMap;

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
use std::f64;

pub fn gen_ops() -> Vec<(String, Vec<Vec<Input>>, Op)> {
    vec![
        ("+"     , vec!["q", "+"        ], op_2(&|x, y| x + y)),
        ("-"     , vec!["w", "-"        ], op_2(&|x, y| x - y)),
        ("/"     , vec!["e", "/"        ], op_2(&|x, y| x / y)),
        ("%"     , vec!["D", "E", "%"   ], op_2(&|x, y| x % y)),
        ("*"     , vec!["r", "*"        ], op_2(&|x, y| x * y)),
        ("^"     , vec!["t", "^"        ], op_2(&|x, y| x.powf(y))),

        ("square", vec!["oq"            ], op_1(&|x| x * x)),
        ("sqrt"  , vec!["or"            ], op_1(&|x| x.sqrt())),
        ("negate", vec!["on"            ], op_1(&|x| -x)),

        ("ln"    , vec!["oge"           ], op_1(&|x| x.ln())),
        ("log10" , vec!["oga", "og1"    ], op_1(&|x| x.log10())),
        ("log2"  , vec!["ogs", "og2"    ], op_1(&|x| x.log2())),
        ("log"   , vec!["ogg", "ogl"    ], op_2(&|x, y| x.log(y))),

        ("sin"   , vec!["os"            ], op_1(&|x| x.sin())),
        ("cos"   , vec!["oc"            ], op_1(&|x| x.cos())),
        ("tan"   , vec!["ot"            ], op_1(&|x| x.tan())),

        ("asin"  , vec!["oas"           ], op_1(&|x| x.asin())),
        ("acos"  , vec!["oac"           ], op_1(&|x| x.acos())),
        ("atan"  , vec!["oat"           ], op_1(&|x| x.atan())),

        ("pi"    , vec!["cp"            ], constant(consts::PI)),
        ("e"     , vec!["ce"            ], constant(consts::E)),
        ("sqrt_2", vec!["cq"            ], constant(consts::SQRT_2)),
        ("nan"   , vec!["cn"            ], constant(f64::NAN)),
        ("inf"   , vec!["cip"           ], constant(f64::INFINITY)),
        ("-inf"  , vec!["cin"           ], constant(f64::NEG_INFINITY)),

        ("sum"   , vec!["isu"           ], fold_op(&|x, y| x + y, 0.)),
        ("msum"  , vec!["ism"           ], fold_op(&|x, y| x * y, 1.)),

        ("clear" , vec!["C", "cc", "isc"], basic(&|st| st.clear())),
        ("swap"  , vec!["isw"           ], Box::new(swap)),
        ("dup"   , vec!["isd"           ], Box::new(duplicate)),
        ("pop"   , vec!["isp"           ], basic(&|st| {st.pop();})),
        ("rev"   , vec!["isr"           ], basic(&|st| st.rev())),
    ]
        .into_iter()
        .map(|(x, y, z)| (
                x.to_string(),
                y.into_iter().map(|x| bind_from_str(x)).collect(),
                z
            ))
        .collect()
}
