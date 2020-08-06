use crate::modes::*;

#[allow(non_camel_case_types)]
pub struct Line_edit_mode {} 

fn find_matching_paren(s: &str) -> Option<usize> {
    let mut chars = s.char_indices();

    match chars.next() {
        Some((0, '(')) | Some((0, '[')) => {
            let mut paren_count = 1isize;

            for (i, c) in chars {
                match c {
                    '(' | '[' => paren_count += 1,
                    ')' | ']' => paren_count -= 1,
                    _ => {}
                }

                if paren_count == 0 {
                    return Some(i);
                }
            }
        }
        _ => {}
    }

    None
}

fn tokenize_rec(ui: &mut Ui_helper, ops: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();

    for (mode, op) in ui.tokenize(ops) {
        if &op[0..1] == "[" || &op[0..1] == "(" {
            out.append(&mut tokenize_rec(ui, &op[..]));
        } else {
            out.push((mode, op));
        }
    }

    out
}

impl Mode for Line_edit_mode {
    fn get_bindings(&self) -> Vec<Vec<Input>> {
        vec![
            bind_from_str("I"),
            bind_from_str("ili"),
            bind_from_str("ifi"),
            bind_from_str("("),
            bind_from_str("["),
        ]
    }

    fn get_operator_regex(&self) -> Regex {
        Regex::new(r"^\(.*\)|^\[.*\]").unwrap()
    }

    fn get_name(&self) -> String {
        "line edit".to_string()
    }

    fn eval_operators(&mut self, ui: &mut Ui, op: &str) {
        ui.insert_mode(self.get_name(), Box::new(Line_edit_mode{}));

        if let Some(m) = find_matching_paren(op) {
            if &op[0..1] == "(" {
                ui.get_stack().push(Func(op[1..m - 1].trim().to_string()));

                ui.eval(
                    op[(m + 1).min(op.len())..]
                        .trim()
                        .to_string()
                );
            } else {
                ui.get_stack().push(List(Vec::new()));

                ui.get_stack().down();
                ui.eval(op[1..m].trim().to_string());
                ui.get_stack().up();
            }
        }
    }

    fn eval_bindings(&self, mut ui: Ui_helper, init: HashMap<&str, &str>)
        -> ModeRes<(String, usize)>
    {
        ui.add_escape_binding(vec![KeyLeft]);
        ui.add_escape_binding(vec![KeyRight]);
        ui.add_escape_binding(vec![KeyBackspace]);
        ui.add_escape_binding(vec![KeyDC]);
        ui.add_escape_binding(vec![Character('\n')]);
        ui.add_escape_binding(vec![Character(' ')]);

        ui.add_escape_binding(bind_from_str("I"));
        ui.add_escape_binding(bind_from_str("ili"));
        ui.add_escape_binding(bind_from_str("ifi"));
        ui.add_escape_binding(bind_from_str("("));
        ui.add_escape_binding(bind_from_str("["));

        let strs =
            if let Some(i) = init.get("text") {
                tokenize_rec(&mut ui, &i[..])
            } else {
                Vec::new()
            };

        let mut strs: Vec<_> = strs.into_iter().map(|(_, x)| x).collect();
        let mut idx = strs.len();
        let mut inputs = Vec::new();

        let (bind, _) = ui.get_next_binding();

        if strs.is_empty() {
            if bind == bind_from_str("[") || bind == bind_from_str("ili") {
                strs = vec!["[".to_string(), "]".to_string()];
                idx = 1;
            }
            else if bind == bind_from_str("(") || bind == bind_from_str("ifi") {
                strs = vec!["(".to_string(), ")".to_string()];
                idx = 1;
            }
        }

        loop {
            let mut before = strs[..idx].join(" ");
            let mut after  = strs[idx..].join(" ");
            let full = strs.join(" ");

            if !before.is_empty() {before.push(' ')}
            if !after .is_empty() {after .insert(0, ' ')}

            ui.print_output(&full, before.len());
            ui.set_surrounding_text((before, after));

            let out = ui.call_mode_by_next_binding(inputs);
            inputs = Vec::new();

            let ((_, s, _, _), _) = &out;

            if !s.is_empty() {
                strs.insert(idx, s.clone());
                idx += 1;
            }

            match out {
                ((.., true), Some(b)) => {
                    if b == vec![KeyLeft] {
                        if idx > 0 {
                            idx -= 1;
                        }
                    } else if b == vec![KeyRight] {
                        if idx < strs.len() {
                            idx += 1;
                        }
                    } else if b == vec![KeyBackspace] {
                        if idx > 0 {
                            idx -= 1;
                            let tmp = strs.remove(idx);

                            if (&tmp[..] == "(" &&
                                strs.get(idx) == Some(&")".to_string())) ||
                               (&tmp[..] == "[" &&
                                strs.get(idx) == Some(&"]".to_string()))
                            {
                                strs.remove(idx);
                            }
                        } else if strs.is_empty() {
                            ui.print_output("", 0);
                            return ((String::new(), 0), None);
                        }
                    } else if b == vec![KeyDC] {
                        if idx < strs.len() {
                            strs.remove(idx);
                        } else if strs.is_empty() {
                            ui.print_output("", 0);
                            return ((String::new(), 0), None);
                        }
                    } else if b == bind_from_str("\n") {
                        let tmp  = strs.join(" ");
                        let len = tmp.len();

                        return ((tmp, len), None);
                    } else
                    if b == bind_from_str("(") || b == bind_from_str("ifi") {
                        strs.insert(idx, "(".to_string());
                        idx += 1;        
                        strs.insert(idx, ")".to_string());
                    } else
                    if b == bind_from_str("[") || b == bind_from_str("ili") {
                        strs.insert(idx, "[".to_string());
                        idx += 1;        
                        strs.insert(idx, "]".to_string());
                    } else
                    if b != bind_from_str(" ") && b != bind_from_str("I") {
                        return ((String::new(), 0), Some(b));
                    }
                }
                ((.., true), res) => return ((String::new(), 0), res),
                (_, Some(binds)) => inputs = binds.clone(),
                _ => {}
            }
        }
    }
}
