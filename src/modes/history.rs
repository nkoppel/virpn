use crate::modes::*;

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct History_mode {
    index: usize,
    lines: Vec<String>,
    undos: Vec<(String, Stack)>
}

impl History_mode {
    pub fn new() -> Self {
        History_mode {
            index: 0,
            lines: Vec::new(),
            undos: vec![(String::new(), Stack::new())]
        }
    }
}

fn ret_history_add(s: &str, res: Option<Vec<Input>>)
    -> ModeRes<(String, usize)>
{
    return ((format!("history_add {}", s), s.len() + 11), res);
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
            self.index += 1;
            self.undos.resize_with(self.index, ||(String::new(), Stack::new()));

            ops = &ops[spc + 1..];

            ui.eval(ops.to_string());

            self.undos.push((ops.to_string(), ui.get_stack().clone()));
            self.lines.push(ops.to_string());
        } else if op == "undo" && self.index > 0 {
            self.index -= 1;

            let (line, stack) = &self.undos[self.index];

            print_command(&ui.window, line, line.len());
            *ui.get_stack() = stack.clone();

        } else if op == "redo" && self.index < self.undos.len() - 1 {
            self.index += 1;

            let (line, stack) = &self.undos[self.index];

            print_command(&ui.window, line, line.len());
            *ui.get_stack() = stack.clone();
        }

        ui.insert_mode(
            "history".to_string(),
            Box::new(mem::replace(self, History_mode::new()))
        );
    }

    fn eval_bindings(&self, mut ui: Ui_helper, _: HashMap<&str, &str>)
        -> ModeRes<(String, usize)>
    {
        ui.add_escape_binding(vec![KeyUp]);
        ui.add_escape_binding(bind_from_str("u"));
        ui.add_escape_binding(bind_from_str("R"));
        ui.add_escape_binding(bind_from_str(" "));
        ui.add_escape_binding(bind_from_str("\n"));

        let out = ui.call_mode_by_next_binding(Vec::new());

        let ((_, op, _, _), res) = out;

        if let Some(b) = res.clone() {
            if b == bind_from_str(" ") || b == bind_from_str("\n") {
                if op.is_empty() {
                    let s =
                        self.lines.last().unwrap_or(&String::new()).to_string();
                    let len = s.len();

                    return ((s, len), None);
                } else {
                    return ret_history_add(&op, None);
                }
            } else if b == vec![KeyUp] && self.lines.len() > 1 {
                let mut key;
                let mut line;
                let mut loc = self.lines.len() - 2;

                loop {
                    line = self.lines[loc].to_string();
                    ui.print_output(&line, line.len());

                    key = ui.get_next_key();

                    match key {
                        KeyUp   => if loc > 0                    {loc -= 1},
                        KeyDown => if loc < self.lines.len() - 1 {loc += 1},
                        Character(' ') | Character('\n') => {
                            return ret_history_add(&line, None);
                        }
                        _ => {
                            ui.print_output("", 0);
                            return ((String::new(), 0), None)
                        }
                    }
                }
            } else if b == bind_from_str("u") {
                ui.print_output("undo", 4);
                return (("undo".to_string(), 4), None);
            } else if b == bind_from_str("R") {
                ui.print_output("redo", 4);
                return (("redo".to_string(), 4), None);
            }
        }
        if !op.is_empty() {
            return ret_history_add(&op, res);
        } else {
            ((String::new(), 0), res)
        }
    }
}
