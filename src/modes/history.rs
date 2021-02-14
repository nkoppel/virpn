use crate::modes::*;

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct History_mode {
    line_id: usize,
    undo_id: usize,
    lines: Vec<String>,
    undos: Vec<(String, Stack)>
}

impl History_mode {
    pub fn new() -> Self {
        History_mode {
            line_id: 0,
            undo_id: 0,
            lines: Vec::new(),
            undos: vec![(String::new(), Stack::new())]
        }
    }
}

impl Mode for History_mode {
    fn get_bindings(&self) -> Vec<Vec<Input>> {
        Vec::new()
    }

    fn get_operator_regex(&self) -> Regex {
        Regex::new(r"^history_add.*|^undo|^redo").unwrap()
    }

    fn get_name(&self) -> String {
        "history".to_string()
    }

    fn eval_operators(&mut self, ui: &mut Ui, mut ops: &str) {
        let spc = ops.find(' ').unwrap_or(ops.len());
        let op = &ops[0..spc];

        if op == "history_add" {
            self.undo_id += 1;
            self.undos.resize_with(self.undo_id, ||(String::new(), Stack::new()));

            ops = &ops[spc + 1..];

            ui.eval(ops.to_string());

            self.undos.push((ops.to_string(), ui.get_stack().clone()));
            self.lines.push(ops.to_string());
            self.line_id = self.lines.len() - 1;
        } else if op == "undo" && self.undo_id > 0 {
            self.undo_id -= 1;

            let (line, stack) = &self.undos[self.undo_id];

            *ui.get_stack() = stack.clone();
        } else if op == "redo" && self.undo_id < self.undos.len() - 1 {
            self.undo_id += 1;

            let (line, stack) = &self.undos[self.undo_id];

            *ui.get_stack() = stack.clone();
        }
    }

    fn eval_binding(&mut self, state: &mut State, bind: Vec<Input>)
        -> Vec<Message>
    {
        let mut msg = Vec::new();

        msg.push(EscBind(vec![KeyUp]));
        msg.push(EscBind(vec![KeyDown]));
        msg.push(EscBind(bind_from_str("u")));
        msg.push(EscBind(bind_from_str("R")));
        msg.push(EscBind(bind_from_str(" ")));
        msg.push(EscBind(bind_from_str("\n")));
        msg.push(EscBind(bind_from_str("Q")));

        msg.push(AllowReplace(false));

        if bind.is_empty() {
            state.insert(
                "op".to_string(),
                state.get("return").unwrap_or(&Str(String::new())).clone()
            );
            msg.push(Return);
            return msg;
        }

        match bind[0] {
            KeyUp => {
                if self.line_id > 0 {
                    self.line_id -= 1;
                    let line = self.lines[self.line_id].clone();
                    let len = line.len();
                    msg.push(Print(line, len));
                }
            }
            KeyDown => {
                if self.line_id < self.lines.len() - 1 {
                    self.line_id += 1;
                    let line = self.lines[self.line_id].clone();
                    let len = line.len();
                    msg.push(Print(line, len));
                }
            }
            Character(' ') | Character('\n') => {
                state.insert(
                    "op".to_string(),
                    state
                        .get("return")
                        .unwrap_or(&Str(
                            self.lines
                                .get(self.line_id)
                                .unwrap_or(&String::new())
                                .clone()
                        ))
                        .clone()
                );
                msg.push(Return);
            }
            Character('u') => {
                state.insert("op".to_string(), Str("undo".to_string()));
                msg.push(Return);

                if self.undo_id > 0 {
                    let line = &self.undos[self.undo_id - 1].0;
                    msg.push(Print(line.clone(), line.len()));
                }
            }
            Character('R') => {
                state.insert("op".to_string(), Str("redo".to_string()));
                msg.push(Return);

                if self.undo_id < self.undos.len() - 1 {
                    let line = &self.undos[self.undo_id + 1].0;
                    msg.push(Print(line.clone(), line.len()));
                }
            }
            Character('Q') => {
                msg.push(Exit);
            }
            _ => panic!()
        }
        msg
    }

    fn ret(&mut self, state: &mut State) -> String {
        let op =
            Data::unwrap_string_or(
                state.get("op"),
                Data::unwrap_string_or(state.get("return"), String::new())
            );

        if op == "undo" || op == "redo" {
            op
        } else {
            format!("history_add {}", op)
        }
    }
}
