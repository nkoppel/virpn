use crate::modes::*;

use crate::modes::ops::helpers::*;
use crate::io::bind_from_str;

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

fn rotate(stack: &mut Stack) -> Res<()> {
    if stack.is_empty() {
        return Err("Too few arguments!".to_string());
    }

    let i = stack.pop().unwrap();
    stack.insert(0, i);
    Ok(())
}

fn range(stack: &mut Stack) -> Res<()> {
    if stack.len() < 2 {
        return Err("Too few arguments!".to_string());
    }

    match (stack.pop().unwrap(), stack.pop().unwrap()) {
        (Num(n2), Num(n1)) => {
            let (n1, n2) = (n1 as usize, n2 as usize);

            let iter: Box<dyn Iterator<Item = usize>> =
                if n1 > n2 {
                    Box::new((n2..n1+1).rev())
                } else {
                    Box::new((n1..n2+1))
                };

            let mut list = Vec::new();

            for i in iter {
                list.push(Num(i as f64));
            }
            stack.push(List(list));
        }
        (i1, i2) => {
            stack.push(i1);
            stack.push(i2);
        }
    }

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
        ("rotate", vec!["iso"           ], Box::new(rotate)),
        ("dup"   , vec!["isd"           ], Box::new(duplicate)),
        ("pop"   , vec!["isp"           ], basic(&|st| {st.pop();})),
        ("rev"   , vec!["isr"           ], basic(&|st| st.rev())),

        ("new_list" , vec!["iln"], basic(&|st| st.push(List(Vec::new())))),
        ("sum_list" , vec!["ilu"], list_fold_op(&|x, y| x + y, 0.)),
        ("msum_list", vec!["ilm"], list_fold_op(&|x, y| x * y, 1.)),

        ("range", vec!["ila"], Box::new(range)),

        ("down"     , vec!["ilj", "J"], basic(&|st| st.down())),
        ("up"       , vec!["ilk", "K"], basic(&|st| st.up())),
    ]
        .into_iter()
        .map(|(name, binds, op)| (
                name.to_string(),
                binds.into_iter().map(|bind| bind_from_str(bind)).collect(),
                op
            ))
        .collect()
}
