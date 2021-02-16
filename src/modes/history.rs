use crate::modes::*;

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct History_mode {
    line_id: usize,
    undo_id: usize,
    op: String,
    lines: Vec<String>,
    undos: Vec<(String, Stack)>
}

impl History_mode {
    pub fn new() -> Self {
        History_mode {
            line_id: 0,
            undo_id: 0,
            op: String::new(),
            lines: Vec::new(),
            undos: vec![(String::new(), Stack::new())]
        }
    }
}

impl Mode for History_mode {
    fn get_bindings(&self) -> Vec<Vec<Input>> {
        vec![
            bind_from_str("u"),
            bind_from_str("R"),
            bind_from_str("Q"),
        ]
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
            self.undos.resize_with(self.undo_id, ||(String::new(), Stack::new()));
            self.undo_id += 1;

            ops = &ops[spc + 1..];

            self.undos.push((ops.to_string(), ui.get_stack().clone()));
            self.lines.push(ops.to_string());
            self.line_id = self.lines.len() - 1;

            ui.insert_mode(
                "history".to_string(),
                Box::new(mem::replace(self, History_mode::new()))
            );

            ui.eval(ops.to_string());
            return;
        } else if op == "undo" && self.undo_id > 0 {
            self.undo_id -= 1;

            let (line, stack) = &self.undos[self.undo_id];

            self.op = line.clone();

            *ui.get_stack() = stack.clone();
        } else if op == "redo" && self.undo_id < self.undos.len() - 1 {
            self.undo_id += 1;

            let (line, stack) = &self.undos[self.undo_id];

            self.op = line.clone();

            *ui.get_stack() = stack.clone();
        }

        ui.insert_mode(
            "history".to_string(),
            Box::new(mem::replace(self, History_mode::new()))
        );
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
            let op =
                Data::unwrap_string_or(
                    state.remove("return").as_ref(),
                    String::new()
                );

            self.op = op;

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
                if self.op.is_empty() {
                    self.op =
                        if let Some(r) = state.remove("return") {
                            r.into_string()
                        } else if let Some(l) = self.lines.get(self.line_id) {
                            l.clone()
                        } else {
                            String::new()
                        };
                }

                msg.push(Return);
            }
            Character('u') => {
                self.op = "undo".to_string();
                msg.push(Return);

                if self.undo_id > 0 {
                    let line = &self.undos[self.undo_id - 1].0;
                    msg.push(Print(line.clone(), line.len()));
                }
            }
            Character('R') => {
                self.op = "redo".to_string();
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
        let op = mem::replace(&mut self.op, String::new());

        if op == "undo" || op == "redo" {
            op.clone()
        } else {
            format!("history_add {}", op)
        }
    }
}
