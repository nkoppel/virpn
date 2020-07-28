use crate::modes::*;

#[derive(Clone, Debug)]
pub struct Number_mode {}

impl Mode for Number_mode {
    fn get_bindings(&self) -> Vec<Vec<Input>> {
        vec![
            vec![Character('a')],
            vec![Character('s')],
            vec![Character('d')],
            vec![Character('f')],
            vec![Character('g')],
            vec![Character('h')],
            vec![Character('j')],
            vec![Character('k')],
            vec![Character('l')],
            vec![Character(';')],
            vec![Character('n')],
            vec![Character('m')],
        ]
    }

    fn get_operator_regex(&self) -> Regex {
        Regex::new(r"^-?\d*.?\d+").unwrap()
    }

    fn get_name(&self) -> String {
        "number".to_string()
    }

    fn eval_operators(&mut self, ui: &mut Ui, ops: &mut String) {
        let spc = ops.find(' ').unwrap_or(ops.len());
        let op = &ops[0..spc];

        match op.parse::<f64>() {
            Ok(f) => ui.get_stack().push(Num(f)),
            Err(_) => ()
        }

        *ops = ops[(spc + 1).min(ops.len())..].to_string();

        ui.insert_mode("number".to_string(), Box::new(Number_mode{}));
    }

    fn eval_bindings(&self, mut ui: Ui_helper, init: HashMap<&str, &str>)
        -> ModeRes<(String, usize)>
    {
        let mut buffer = init.get("text").unwrap_or(&"").to_string();
        let mut loc = buffer.len();

        ui.add_escape_binding(vec![KeyBackspace]);
        ui.add_escape_binding(vec![KeyLeft]);
        ui.add_escape_binding(vec![KeyRight]);

        loop {
            ui.print_output(&buffer, loc);

            let (bind, res) = ui.get_next_binding();

            match bind[0] {
                Character('a') => {buffer.insert(loc, '1'); loc += 1}
                Character('s') => {buffer.insert(loc, '2'); loc += 1}
                Character('d') => {buffer.insert(loc, '3'); loc += 1}
                Character('f') => {buffer.insert(loc, '4'); loc += 1}
                Character('g') => {buffer.insert(loc, '5'); loc += 1}
                Character('h') => {buffer.insert(loc, '6'); loc += 1}
                Character('j') => {buffer.insert(loc, '7'); loc += 1}
                Character('k') => {buffer.insert(loc, '8'); loc += 1}
                Character('l') => {buffer.insert(loc, '9'); loc += 1}
                Character(';') => {buffer.insert(loc, '0'); loc += 1}
                Character('n') => {
                    if loc == 0 {
                        buffer.insert(loc, '-');
                        loc = 1
                    }
                }
                Character('m') => {
                    if !buffer.contains('.') {
                        buffer.insert(loc, '.');
                        loc += 1;
                    }
                }

                KeyLeft        => loc = loc.saturating_sub(1),
                KeyRight       => if loc < buffer.len() {loc += 1},
                KeyBackspace   => {
                    if loc != 0 {
                        loc -= 1;
                        buffer.remove(loc);
                    }
                }
                _ => return ((buffer, loc), res)
            }
        }
    }
}
