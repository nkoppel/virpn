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
            bind_from_str("E"),
        ]
    }

    fn get_operator_regex(&self) -> Regex {
        Regex::new(r"^history_add.*|^undo|^redo").unwrap()
    }

    fn get_name(&self) -> String {
        "history".to_string()
    }

    fn eval_operators(&mut self, ui: &mut Ui, ops: &str) {
        let spc = ops.find(' ').unwrap_or_else(|| ops.len());
        let op = &ops[0..spc];

        if op == "history_add" {
            let ops = ops[spc + 1..].to_string();

            self.undos.resize_with(self.undo_id, ||(String::new(), Stack::new()));
            self.undo_id += 1;
            self.undos.push((ops.clone(), ui.get_stack().clone()));

            let tokens = ui.tokenize(&ops);

            if Some(&ops) != self.lines.last() {
                if let Some((mode, _)) = tokens.get(0) {
                    if tokens.len() > 1 || mode == "line edit" {
                        self.lines.push(ops.clone());
                    }
                }
            }

            if self.lines.len() > 0 {
                self.line_id = self.lines.len() - 1;
            }

            ui.insert_mode(
                "history".to_string(),
                Box::new(mem::replace(self, History_mode::new()))
            );

            ui.eval(ops);
            return;
        } else if op == "undo" && self.undo_id > 0 {
            self.undo_id -= 1;
            self.lines.push("undo".to_string());

            let (line, stack) = &self.undos[self.undo_id];

            self.op = line.clone();

            *ui.get_stack() = stack.clone();
        } else if op == "redo" && self.undo_id < self.undos.len() - 1 {
            self.undo_id += 1;
            self.lines.push("redo".to_string());

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
        let mut msg = vec![
            EscBind(vec![KeyUp]),
            EscBind(vec![KeyDown]),
            EscBind(vec![Character('\u{1b}')]),
            EscBind(vec![KeyDC]),
            EscBind(bind_from_str("u")),
            EscBind(bind_from_str("R")),
            EscBind(bind_from_str(" ")),
            EscBind(bind_from_str("\n")),
            EscBind(bind_from_str("Q")),

            EscBind(vec![KeyLeft]),
            EscBind(vec![KeyRight]),
            EscBind(vec![KeyBackspace]),

            AllowReplace(false),
        ];

        if bind.is_empty() {
            self.op =
                Data::unwrap_string_or(
                    state.remove("return").as_ref(),
                    String::new()
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
                if self.lines.len() > 0 && self.line_id < self.lines.len() - 1 {
                    self.line_id += 1;
                    let line = self.lines[self.line_id].clone();
                    let len = line.len();
                    msg.push(Print(line, len));
                }
            }
            Character(' ') | Character('\n') => {
                self.op =
                    if let Some(r) = state.remove("return") {
                        r.into_string()
                    } else if let Some(l) = self.lines.get(self.line_id) {
                        l.clone()
                    } else {
                        String::new()
                    };

                msg.push(Return);
            }
            KeyLeft | KeyRight | KeyBackspace | Character('E') => {
                let line =
                    if let Some(r) = state.remove("return") {
                        r.into_string()
                    } else if let Some(l) = self.lines.get(self.line_id) {
                        l.clone()
                    } else {
                        String::new()
                    };

                let mut state = HashMap::new();

                state.insert("init".to_string(), Str(line));

                msg.push(Call("line edit".to_string(), state));

                msg.push(PressKeys(bind_from_str("I")));

                if bind[0] != Character('E') {
                    msg.push(PressKeys(vec![bind[0]]));
                }
            }
            Character('\u{1b}') | KeyDC => {
                self.line_id = self.lines.len().saturating_sub(1);

                self.op.clear();
                state.remove("return");

                msg.push(Print(String::new(), 0));
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

    fn ret(&mut self, _: &mut State) -> String {
        let op = mem::replace(&mut self.op, String::new());

        if op == "undo" || op == "redo" {
            op
        } else {
            format!("history_add {}", op)
        }
    }
}
