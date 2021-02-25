use crate::modes::*;

use crate::modes::ops::helpers::*;
use crate::io::bind_from_str;

fn add(stack: &mut Stack) {op_2(&|x, y| x + y)(stack)}
fn sub(stack: &mut Stack) {op_2(&|x, y| x - y)(stack)}
fn mul(stack: &mut Stack) {op_2(&|x, y| x * y)(stack)}
fn div(stack: &mut Stack) {op_2(&|x, y| x / y)(stack)}

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

    let f: Box<dyn Fn(Vec<f64>) -> Item> =
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

fn repeat(stack: &mut Stack) {
    if stack.len() < 2 {
        return;
    }

    let mut new_stack = Stack::new();
    new_stack.push(stack.pop().unwrap());
    let item = stack.pop().unwrap();
    new_stack.rev();

    let f: Box<dyn Fn(Vec<f64>) -> Item> =
        Box::new(|v| {
            let n2 = v[0].abs() as u64;

            let mut list = Vec::new();

            for _ in 0..n2 {
                list.push(item.clone());
            }
            List(list)
        });

    stack.push(new_stack.apply_map(&f))
}

fn list_len(stack: &mut Stack) {
    if let Some(l) = stack.pop_as_list() {
        stack.push(Num(l.len() as f64));
    }
}

fn round_digits(mut x: f64, digits: f64) -> f64 {
    let digits = digits as i32;
    let base: f64 = 10.;
    let scale = base.powi(digits);

    x *= scale;
    x = x.round();
    x /= scale;

    x
}

fn clean_errors(mut x: f64) -> f64 {
    if x == 0. {
        return 0.;
    }

    let base: f64 = 10.;
    let digits = 14 - x.abs().log10() as i32;
    let scale = base.powi(digits);

    x *= scale;
    x = x.round();
    x /= scale;

    x
}

fn synth_div(stack: &mut Stack) {
    let mut l = if let Some(l) = stack.pop_as_list() {l} else {return};
    let x =
        match stack.pop() {
            Some(Func(f)) => {
                stack.push(Func(f));
                stack.push(List(l));
                return
            },
            Some(x) => x,
            _ => {
                stack.push(List(l));
                return
            }
        };

    for i in l.iter() {
        if let Func(_) = i {
            stack.push(x);
            stack.push(List(l));
            return
        }
    }

    l.reverse();

    let mut tmp_stack = Stack::from_vec(l);
    let mut res = Vec::new();

    while tmp_stack.len() > 1 {
        res.push(tmp_stack.last().unwrap().clone());
        tmp_stack.push(x.clone());
        mul(&mut tmp_stack);
        add(&mut tmp_stack);
    }

    stack.push(List(res));
    stack.push(tmp_stack.pop().unwrap());
}

fn synth_sub(stack: &mut Stack) {
    synth_div(stack);

    let tmp = if let Some(i) = stack.pop() {i} else {return};
    stack.pop();
    stack.push(tmp);
}

fn poly_div(stack: &mut Stack) {
    let mut poly2 = if let Some(l) = stack.pop_as_list() {l} else {return};
    let mut poly1 = if let Some(l) = stack.pop_as_list() {l} else {
        stack.push(List(poly2));
        return
    };

    if poly2.len() > poly1.len() {
        stack.push(List(poly1));
        stack.push(List(poly2));
        return;
    }

    poly1.reverse();
    poly2.reverse();

    let rep = poly1.len() - poly2.len();
    let mut tmp = poly2;

    poly2 = vec![Num(0.); rep];
    poly2.append(&mut tmp);

    let mut out = Vec::new();
    let mut tmp_stack = Stack::new();

    for _ in 0..rep + 1 {
        tmp_stack.push(List(poly1.clone()));

        tmp_stack.push(poly1.last().unwrap().clone());
        tmp_stack.push(poly2.last().unwrap().clone());

        div(&mut tmp_stack);

        out.push(tmp_stack.last().unwrap().clone());
        tmp_stack.push(List(poly2.clone()));

        mul(&mut tmp_stack);
        sub(&mut tmp_stack);

        poly1 = tmp_stack.pop_as_list().unwrap();

        poly1.pop();
        poly2.remove(0);
    }

    stack.push(List(out));
    poly1.reverse();
    stack.push(List(poly1));
}

fn poly_mul(stack: &mut Stack) {
    let poly1 = if let Some(l) = stack.pop_as_list() {l} else {return};
    let poly2 = if let Some(l) = stack.pop_as_list() {l} else {
        stack.push(List(poly1));
        return
    };

    let len = poly1.len();
    let mut tmp_stack = Stack::new();

    for (i, item) in poly1.into_iter().enumerate() {
        tmp_stack.push(item);
        tmp_stack.push(List(poly2.clone()));

        mul(&mut tmp_stack);

        let mut tmp = vec![Num(0.); i];
        tmp.append(&mut tmp_stack.pop_as_list().unwrap());
        tmp.append(&mut vec![Num(0.); len - 1 - i]);

        tmp_stack.push(List(tmp.clone()));

        add(&mut tmp_stack);
    }

    stack.push(tmp_stack.pop().unwrap());
}

fn transpose(stack: &mut Stack) {
    let l = if let Some(l) = stack.pop_as_list() {l} else {return};

    let new_stack = Stack::from_vec(l);
    stack.push(
        new_stack.apply_map(&|l| List(l.into_iter().map(|x| Num(x)).collect()))
    );
}

fn cumsum(stack: &mut Stack) {
    let l = if let Some(l) = stack.pop_as_list() {l} else {return};

    let mut tmp_stack = Stack::new();

    for i in l {
        tmp_stack.push(tmp_stack.last().unwrap_or(&Num(0.)).clone());
        tmp_stack.push(i);
        add(&mut tmp_stack);
    }

    stack.push(List(tmp_stack.into_vec()));
}

use std::f64::consts;
use std::f64;

pub fn gen_ops() -> Vec<(String, Vec<Vec<Input>>, Op)> {
    vec![
        ("+"     , vec!["q", "+"        ], op_2(&|x, y| x + y)),
        ("-"     , vec!["w", "-"        ], op_2(&|x, y| x - y)),
        ("/"     , vec!["e", "/"        ], op_2(&|x, y| x / y)),
        ("%"     , vec!["D", "%"        ], op_2(&|x, y| x % y)),
        ("*"     , vec!["r", "*"        ], op_2(&|x, y| x * y)),
        ("^"     , vec!["t", "^"        ], op_2(&|x, y| x.powf(y))),

        ("square", vec!["oq"            ], op_1(&|x| x * x)),
        ("sqrt"  , vec!["or"            ], op_1(&|x| x.sqrt())),
        ("cbrt"  , vec!["ob"            ], op_1(&|x| x.cbrt())),
        ("nth_rt", vec!["on"            ], op_2(&|x, y| x.powf(1. / y))),
        ("negate", vec!["oe"            ], op_1(&|x| -x)),
        ("invert", vec!["oi"            ], op_1(&|x| x.recip())),
        ("abs"   , vec!["oab"           ], op_1(&|x| x.abs())),

        ("pow"   , vec!["iwe"              ], op_1(&|x| x.exp())),
        ("pow2"  , vec!["iws", "iw2", "iww"], op_1(&|x| x.exp2())),
        ("pow10" , vec!["iwa", "iw1"       ], op_1(&|x| (10f64).powf(x))),

        ("ln"    , vec!["oge"           ], op_1(&|x| x.ln())),
        ("log10" , vec!["oga", "og1"    ], op_1(&|x| x.log10())),
        ("log2"  , vec!["ogs", "og2"    ], op_1(&|x| x.log2())),
        ("log"   , vec!["ogg", "ogl"    ], op_2(&|x, y| x.log(y))),

        ("deg"   , vec!["oad"           ], op_1(&|x| x.to_degrees())),
        ("rad"   , vec!["oar"           ], op_1(&|x| x.to_radians())),

        ("sin"   , vec!["os"            ], op_1(&|x| x.sin())),
        ("cos"   , vec!["oc"            ], op_1(&|x| x.cos())),
        ("tan"   , vec!["ot"            ], op_1(&|x| x.tan())),

        ("asin"  , vec!["oas"           ], op_1(&|x| x.asin())),
        ("acos"  , vec!["oac"           ], op_1(&|x| x.acos())),
        ("atan"  , vec!["oat"           ], op_1(&|x| x.atan())),

        ("sinh"  , vec!["ohs"           ], op_1(&|x| x.sinh())),
        ("cosh"  , vec!["ohc"           ], op_1(&|x| x.cosh())),
        ("tanh"  , vec!["oht"           ], op_1(&|x| x.tanh())),

        ("asinh" , vec!["ohas"          ], op_1(&|x| x.asinh())),
        ("acosh" , vec!["ohac"          ], op_1(&|x| x.acosh())),
        ("atanh" , vec!["ohat"          ], op_1(&|x| x.atanh())),

        ("pi"     , vec!["cp"            ], constant(consts::PI)),
        ("e"      , vec!["ce"            ], constant(consts::E)),
        ("sqrt_2" , vec!["cq"            ], constant(consts::SQRT_2)),
        ("nan"    , vec!["cn"            ], constant(f64::NAN)),
        ("inf"    , vec!["cip"           ], constant(f64::INFINITY)),
        ("-inf"   , vec!["cin"           ], constant(f64::NEG_INFINITY)),
        ("epsilon", vec!["cs"            ], constant(f64::EPSILON)),

        ("sum"   , vec!["isu", "ou"     ], fold_op(&|x, y| x + y, 0.)),
        ("msum"  , vec!["ism", "om"     ], fold_op(&|x, y| x * y, 1.)),

        ("clear" , vec!["C", "cc", "isc"], basic(&|st| st.clear())),
        ("swap"  , vec!["isw", "ow"     ], basic(&swap)),
        ("rotate", vec!["iso", "oo"     ], basic(&rotate)),
        ("dup"   , vec!["isd", "od"     ], basic(&duplicate)),
        ("pop"   , vec!["isp", "op"     ], basic(&|st| {st.pop();})),
        ("rev"   , vec!["isv", "ov"     ], basic(&|st| st.rev())),

        ("round"       , vec!["ior"      ], op_1(&|x| x.round())),
        ("floor"       , vec!["iof"      ], op_1(&|x| x.floor())),
        ("ceil"        , vec!["ioc"      ], op_1(&|x| x.ceil())),
        ("round_digits", vec!["iodr"     ], op_2(&round_digits)),
        ("clean_errors", vec!["ioe", "ol"], op_1(&clean_errors)),

        ("new_list" , vec!["iln"], basic(&|st| {st.push(List(Vec::new())); st.down()})),
        ("sum_list" , vec!["ilu"], list_fold_op(&|x, y| x + y, 0.)),
        ("msum_list", vec!["ilm"], list_fold_op(&|x, y| x * y, 1.)),
        ("rev_list" , vec!["ilv"], basic(&|st| {st.down(); st.rev(); st.up()})),

        ("range"  , vec!["ila"], basic(&range)),
        ("repeat" , vec!["ilr"], basic(&repeat)),
        ("len"    , vec!["ill"], basic(&list_len)),
        ("flatten", vec!["ilf"], basic(&flatten)),
        ("cumsum" , vec!["ilc"], basic(&cumsum)),
        ("transpose", vec!["ilt"], basic(&transpose)),

        ("synth_sub", vec!["ipp", "ilp"], basic(&synth_sub)),
        ("synth_div", vec!["ips"       ], basic(&synth_div)),
        ("poly_div" , vec!["ipd", "ipe"], basic(&poly_div)),
        ("poly_mul" , vec!["ipm", "ipr"], basic(&poly_mul)),

        ("poly_square" , vec!["ipq"], chain(vec![basic(&duplicate), basic(&poly_mul)])),

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
