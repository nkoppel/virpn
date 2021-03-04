use crate::modes::*;

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct Number_mode {}

const NUM_CHRS: &str = ";asdfghjkl";

impl Mode for Number_mode {
    fn get_bindings(&self) -> Vec<Vec<Input>> {
        let mut out: Vec<_> = NUM_CHRS
            .chars()
            .map(|c| vec![Character(c)])
            .collect();

        out.push(vec![Character('n')]);
        out.push(vec![Character('m')]);

        out
    }

    fn get_operator_regex(&self) -> Regex {
        Regex::new(r"^-?\d*\.?\d+").unwrap()
    }

    fn get_name(&self) -> String {
        "number".to_string()
    }

    fn eval_operators(&mut self, ui: &mut Ui, op: &str) {
        if let Ok(f) = op.parse::<f64>() {
             ui.get_stack().push(Num(f));
        }

        ui.insert_mode(
            "number".to_string(),
            Box::new(Number_mode{})
        );
    }

    fn eval_binding(&mut self, state: &mut State, bind: Vec<Input>)
        -> Vec<Message>
    {
        let mut buffer = Data::unwrap_string_or(state.get("buffer"), String::new());
        let mut loc = Data::unwrap_usize_or(state.get("loc"), 0);
        let mut msg = Vec::new();
        let mut ret = false;

        msg.push(EscBind(vec![KeyBackspace]));
        msg.push(EscBind(vec![KeyLeft]));
        msg.push(EscBind(vec![KeyRight]));
        msg.push(EscBind(bind_from_str(" ")));

        match bind[0] {
            Character('n') => {
                if loc == 0 && !buffer.contains('-') {
                    buffer.insert(0, '-');
                    loc = 1
                }
            }
            Character('m') => {
                if !buffer.contains('.') {
                    buffer.insert(loc, '.');
                    loc += 1;
                }
            }

            Character(' ') => {
                ret = true;
            }
            KeyLeft        => loc = loc.saturating_sub(1),
            KeyRight       => if loc < buffer.len() {loc += 1},
            KeyBackspace   => {
                if loc != 0 {
                    loc -= 1;
                    buffer.remove(loc);
                }
            }
            Character(c) => {
                if let Some(i) = NUM_CHRS.find(c) {
                    buffer.insert_str(loc, &i.to_string());
                    loc += 1;
                }
            }
            _ => {}
        }

        msg.push(Print(buffer.clone(), loc));

        state.insert("buffer".to_string(), Str(buffer));
        state.insert("loc".to_string(), Uint(loc as u64));

        if ret {
            msg.push(Return);
        }

        msg
    }

    fn ret(&mut self, state: &mut State) -> String {
        Data::unwrap_string_or(state.get("buffer"), String::new())
    }
}
