use crate::modes::*;

use crate::modes::ops::helpers::*;
use crate::io::bind_from_str;

use std::f64::consts::PI;

fn swap(stack: &mut Stack) {
    if stack.len() < 2 {
        return;
    }
    let i1 = stack.pop().unwrap();
    let i2 = stack.pop().unwrap();
    stack.push(i1);
    stack.push(i2);
}

fn duplicate(stack: &mut Stack) {
    if stack.is_empty() {
        return;
    }

    let i = stack.pop().unwrap();
    stack.push(i.clone());
    stack.push(i);
}

fn rotate(stack: &mut Stack) {
    if stack.is_empty() {
        return;
    }

    let i = stack.pop().unwrap();
    stack.insert(0, i);
}

fn flatten_helper(item: Item) -> Vec<Item> {
    match item {
        List(l) => {
            let mut out = Vec::new();

            for i in l {
                out.append(&mut flatten_helper(i));
            }

            out
        }
        _ => vec![item]
    }
}

fn flatten(stack: &mut Stack) {
    if let Some(item) = stack.pop() {
        stack.push(List(flatten_helper(item)));
    }
}

fn range(stack: &mut Stack) {
    if stack.len() < 2 {
        return;
    }

    let mut new_stack = Stack::new();
    new_stack.push(stack.pop().unwrap());
    new_stack.push(stack.pop().unwrap());
    new_stack.rev();

    let f: Box<Fn(Vec<f64>) -> Item> =
        Box::new(|v| {
            let n1 = v[0] as i64;
            let n2 = v[1] as i64;

            let iter: Box<dyn Iterator<Item = i64>> =
                if n1 > n2 {
                    Box::new((n2..n1+1).rev())
                } else {
                    Box::new(n1..n2+1)
                };

            let mut list = Vec::new();

            for i in iter {
                list.push(Num(i as f64));
            }
            List(list)
        });

    stack.push(new_stack.apply_map(&f))
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
        ("cbrt"  , vec!["ob"            ], op_1(&|x| x.cbrt())),
        ("nth_rt", vec!["on"            ], op_2(&|x, y| x.powf(1. / y))),
        ("negate", vec!["oe"            ], op_1(&|x| -x)),
        ("abs"   , vec!["oab"           ], op_1(&|x| x.abs())),

        ("ln"    , vec!["oge"           ], op_1(&|x| x.ln())),
        ("log10" , vec!["oga", "og1"    ], op_1(&|x| x.log10())),
        ("log2"  , vec!["ogs", "og2"    ], op_1(&|x| x.log2())),
        ("log"   , vec!["ogg", "ogl"    ], op_2(&|x, y| x.log(y))),

        ("deg"   , vec!["oad"           ], op_1(&|x| x * 180. / PI)),
        ("rad"   , vec!["oar"           ], op_1(&|x| x * PI / 180.)),

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

        ("sum"   , vec!["isu", "ou"     ], fold_op(&|x, y| x + y, 0.)),
        ("msum"  , vec!["ism", "om"     ], fold_op(&|x, y| x * y, 1.)),

        ("clear" , vec!["C", "cc", "isc"], basic(&|st| st.clear())),
        ("swap"  , vec!["isw", "ow"     ], basic(&swap)),
        ("rotate", vec!["iso", "oo"     ], basic(&rotate)),
        ("dup"   , vec!["isd", "od"     ], basic(&duplicate)),
        ("pop"   , vec!["isp", "op"     ], basic(&|st| {st.pop();})),
        ("rev"   , vec!["isv", "ov"     ], basic(&|st| st.rev())),

        ("new_list" , vec!["iln"], basic(&|st| {st.push(List(Vec::new())); st.down()})),
        ("sum_list" , vec!["ilu"], list_fold_op(&|x, y| x + y, 0.)),
        ("msum_list", vec!["ilm"], list_fold_op(&|x, y| x * y, 1.)),
        ("rev_list" , vec!["ilv"], basic(&|st| {st.down(); st.rev(); st.up()})),

        ("range"  , vec!["ila"], basic(&range)),
        ("flatten", vec!["ilf"], basic(&flatten)),

        ("down"     , vec!["J", "oj"], basic(&|st| st.down())),
        ("up"       , vec!["K", "ok"], basic(&|st| st.up())),

        ("components", vec!["ivc"], vec2_op(&|l, a| (a.cos() * l, a.sin() * l))),
        ("heading"   , vec!["ivh"], vec2_op(&|x, y| ((x*x + y*y).sqrt(), y.atan2(x)))),
    ]
        .into_iter()
        .map(|(name, binds, op)| (
                name.to_string(),
                binds.into_iter().map(|bind| bind_from_str(bind)).collect(),
                op
            ))
        .collect()
}
