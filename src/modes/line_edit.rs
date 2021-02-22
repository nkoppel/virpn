use crate::modes::*;

#[allow(non_camel_case_types)]
pub struct Line_edit_mode {
    loc: usize,
    strs: Vec<String>,
    strs_hist: Vec<(usize, Vec<String>)>,
} 

impl Line_edit_mode {
    pub fn new() -> Self {
        Self {
            loc: 0,
            strs: Vec::new(),
            strs_hist: Vec::new(),
        }
    }
}

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

fn tokenize_rec(ui: &mut Ui, ops: &str) -> Vec<(String, String)> {
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
        Regex::new(r"^\(.*\)|^\[.*\]|^tokenize_rec .*").unwrap()
    }

    fn get_name(&self) -> String {
        "line edit".to_string()
    }

    fn eval_operators(&mut self, ui: &mut Ui, op: &str) {
        ui.insert_mode(self.get_name(), Box::new(Line_edit_mode::new()));

        if op.len() >= 13 && op[0..12] == *"tokenize_rec" {
            self.strs = tokenize_rec(ui, &op[13..])
                .into_iter().map(|(_, x)| x).collect();

            self.strs_hist.clear();
            self.loc = self.strs.len();
        } else if let Some(m) = find_matching_paren(op) {
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

    fn eval_binding(&mut self, state: &mut State, bind: Vec<Input>)
        -> Vec<Message>
    {
        let mut msg = Vec::new();
        let mut ret = false;

        msg.push(AllowReplace(false));

        msg.push(EscBind(vec![KeyLeft]));
        msg.push(EscBind(vec![KeyRight]));
        msg.push(EscBind(vec![KeyBackspace]));
        msg.push(EscBind(vec![KeyDC]));
        msg.push(EscBind(vec![Character('\n')]));
        msg.push(EscBind(vec![Character(' ')]));

        msg.push(EscBind(bind_from_str("I")));
        msg.push(EscBind(bind_from_str("u")));
        msg.push(EscBind(bind_from_str("ili")));
        msg.push(EscBind(bind_from_str("ifi")));
        msg.push(EscBind(bind_from_str("(")));
        msg.push(EscBind(bind_from_str("[")));
        msg.push(EscBind(bind_from_str(")")));
        msg.push(EscBind(bind_from_str("]")));

        if let Some(Str(i)) = state.remove("init") {
            msg.push(Eval(format!("tokenize_rec {}", i)));
            
            if bind.is_empty() {
                msg.push(PressKeys(bind_from_str("I")));
            } else {
                msg.push(PressKeys(bind));
            }

            return msg;
        }

        if  bind.is_empty() ||
            bind == bind_from_str(" ") ||
            bind == bind_from_str("\n")
        {
            if let Some(Str(op)) = state.remove("return") {
                self.strs_hist.push((self.loc, self.strs.clone()));
                self.strs.insert(self.loc, op);
                self.loc += 1;
            }
        }

        if bind == bind_from_str("(") || bind == bind_from_str("ifi") {
            self.strs_hist.push((self.loc, self.strs.clone()));
            self.strs.insert(self.loc, "(".to_string());
            self.loc += 1;
            self.strs.insert(self.loc, ")".to_string());
        }

        if bind == bind_from_str("[") || bind == bind_from_str("ili") {
            self.strs_hist.push((self.loc, self.strs.clone()));
            self.strs.insert(self.loc, "[".to_string());
            self.loc += 1;
            self.strs.insert(self.loc, "]".to_string());
        }

        if bind.len() == 1 {
            match bind[0] {
                KeyLeft => {
                    if self.loc > 0 {
                        self.loc -= 1;
                    }
                }
                KeyRight => {
                    if self.loc < self.strs.len() {
                        self.loc += 1;
                    }
                }
                KeyDC => {
                    if self.loc < self.strs.len() {
                        self.strs.remove(self.loc);
                        self.strs_hist.push((self.loc, self.strs.clone()));
                    } else if self.strs.is_empty() {
                        ret = true;
                    }
                }
                KeyBackspace => {
                    if self.loc > 0 {
                        self.loc -= 1;
                        let tmp = self.strs.remove(self.loc);

                        if (&tmp[..] == "(" &&
                            self.strs.get(self.loc) == Some(&")".to_string())) ||
                           (&tmp[..] == "[" &&
                            self.strs.get(self.loc) == Some(&"]".to_string()))
                        {
                            self.strs.remove(self.loc);
                        }

                        self.strs_hist.push((self.loc, self.strs.clone()));
                    } else if self.strs.is_empty() {
                        ret = true;
                    }
                }
                Character('\n') => {
                    ret = true;
                }
                Character(')') => {
                    self.strs_hist.push((self.loc, self.strs.clone()));
                    self.strs.insert(self.loc, ")".to_string());
                    self.loc += 1;
                    self.strs_hist.push((self.loc, self.strs.clone()));
                }
                Character(']') => {
                    self.strs_hist.push((self.loc, self.strs.clone()));
                    self.strs.insert(self.loc, "]".to_string());
                    self.loc += 1;
                    self.strs_hist.push((self.loc, self.strs.clone()));
                }
                Character('u') => {
                    if let Some((l, s)) = self.strs_hist.pop() {
                        self.loc = l;
                        self.strs = s;
                    }
                }
                _ => {}
            }
        }

        let before = self.strs[..self.loc].join(" ");
        let after  = self.strs[self.loc..].join(" ");

        msg.push(Print(
            format!("{} {}", before, after),
            if before.len() == 0 {
                0
            } else {
                before.len() + 1
            }
        ));
        msg.push(WrapText(before + " ", " ".to_string() + &after));

        if ret {
            msg.push(Return);
        }

        msg
    }

    fn ret(&mut self, _: &mut State) -> String {
        let out = self.strs.join(" ");

        self.loc = 0;
        self.strs.clear();
        self.strs_hist.clear();

        out
    }
}
