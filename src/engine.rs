#[allow(unused)]
pub struct Engine
{
    p_code: String,
    js_code: String,
    current_indent: usize,
    open_brackets: usize,
    close_brackets: usize,
}
impl Engine
{
    pub fn new(p_code: String) -> Self {
        Engine {
            p_code,
            js_code: String::new(),
            current_indent: 0,
            open_brackets: 0,
            close_brackets: 0,
        }
    }
    pub fn run(&mut self) {
        let lines = self.p_code.clone();
        for line in lines.lines() {
            let line = line.to_string();
            self.check_indent(line.clone());
            self.parse_line(line.clone());
        }
        if self.open_brackets > self.close_brackets {
            self.js_code.push_str("\n");
            for _ in 0..self.open_brackets - self.close_brackets {
                self.js_code.push_str("}\n");
            }
        }
        println!("{}", self.js_code);
    }
    fn get_ident(&mut self, line: String) -> usize {
        let mut indent = 0;
        for c in line.chars() {
            if c == ' ' {
                indent += 1;
            } else {
                break;
            }
        }
        indent
    }
    fn check_indent(&mut self, line: String) {
        let indent = self.get_ident(line);
        if self.current_indent < indent
        {
            self.js_code.push_str("{\n");
            self.open_brackets += 1;
        }
        else if self.current_indent > indent
        {
            self.js_code.push_str("}");
            self.close_brackets += 1;
        }
        self.current_indent = indent;
    }
    fn parse_line(&mut self, line: String) {
        let line = line.trim();
        if line.is_empty() {
            self.js_code.push_str("");
        }
        self.js_code.push_str(&format!("{}\n", line));
    }
}
