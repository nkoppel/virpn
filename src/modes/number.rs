use crate::modes::*;

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct Number_mode {}

impl Mode for Number_mode {
    fn get_bindings(&self) -> Vec<Vec<Input>> {
        vec![
            vec![Character('a')], vec![Character('A')],
            vec![Character('s')], vec![Character('S')],
            vec![Character('d')], vec![Character('D')],
            vec![Character('f')], vec![Character('F')],
            vec![Character('g')], vec![Character('G')],
            vec![Character('h')], vec![Character('H')],
            vec![Character('j')], vec![Character('J')],
            vec![Character('k')], vec![Character('K')],
            vec![Character('l')], vec![Character('L')],
            vec![Character(';')], vec![Character(':')],
            vec![Character('n')],
            vec![Character('m')],
        ]
    }

    fn get_operator_regex(&self) -> Regex {
        Regex::new(r"^-?\d*\.?\d+").unwrap()
    }

    fn get_name(&self) -> String {
        "number".to_string()
    }

    fn eval_operators(&mut self, ui: &mut Ui, op: &str) {
        match op.parse::<f64>() {
            Ok(f) => ui.get_stack().push(Num(f)),
            Err(_) => ()
        }

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
                Character(c) => {
                    match c.to_ascii_lowercase() {
                        'a' => {buffer.insert(loc, '1'); loc += 1}
                        's' => {buffer.insert(loc, '2'); loc += 1}
                        'd' => {buffer.insert(loc, '3'); loc += 1}
                        'f' => {buffer.insert(loc, '4'); loc += 1}
                        'g' => {buffer.insert(loc, '5'); loc += 1}
                        'h' => {buffer.insert(loc, '6'); loc += 1}
                        'j' => {buffer.insert(loc, '7'); loc += 1}
                        'k' => {buffer.insert(loc, '8'); loc += 1}
                        'l' => {buffer.insert(loc, '9'); loc += 1}
                        ';' => {buffer.insert(loc, '0'); loc += 1}
                        ':' => {buffer.insert(loc, '0'); loc += 1}
                        'n' => {
                            if loc == 0 {
                                buffer.insert(loc, '-');
                                loc = 1
                            }
                        }
                        'm' => {
                            if !buffer.contains('.') {
                                buffer.insert(loc, '.');
                                loc += 1;
                            }
                        }
                        _ => return ((buffer, loc), res)
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

            if let Character(c) = bind[0] {
                if c.is_ascii_uppercase() || c == ':' {
                    ui.print_output(&buffer, loc);

                    return ((buffer, loc), res);
                }
            }
        }
    }
}
