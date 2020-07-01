use crate::stack::Stack;
use crate::op_helpers::*;
use std::collections::HashMap;

pub fn gen_ops() -> HashMap<String, Op> {
    vec![
        ("+".to_string(), op_2(&|x, y| x + y)),
        ("-".to_string(), op_2(&|x, y| x - y)),
        ("*".to_string(), op_2(&|x, y| x * y)),
        ("/".to_string(), op_2(&|x, y| x / y)),
        ("sum".to_string(), fold_op(&|x, y| x + y, 0.)),
        ("msum".to_string(), fold_op(&|x, y| x * y, 1.)),
    ].into_iter().collect()
}
