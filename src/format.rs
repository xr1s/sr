#[derive(Debug, Eq, PartialEq)]
enum FormatState {
    Literal,
    Hashbang,
    ArgNum,
    FormatArg,
    FmtArgEnd,
    Percent,
    XmlTag, // 按理来说，tag 是存在嵌套的，但是这里暂时不考虑了
}

#[derive(Debug)]
pub struct Formatter<'a> {
    template: &'a str,
    parameters: &'a [f32],

    keep_xml_tag: bool,

    state: FormatState,
    result: String,
    arg_num: usize,
    fmt_arg: String,
}

impl<'a> Formatter<'a> {
    pub fn new(template: &'a str, parameters: &'a [f32]) -> Self {
        Self {
            template,
            parameters,
            keep_xml_tag: true,
            state: FormatState::Literal,
            result: String::with_capacity(template.len()),
            arg_num: 0,
            fmt_arg: String::new(),
        }
    }

    pub fn keep_xml_tag(&mut self, keep: bool) -> &mut Self {
        self.keep_xml_tag = keep;
        self
    }

    fn feed(&mut self, char: char) {
        match self.state {
            FormatState::Literal => {
                if char == '#' {
                    self.state = FormatState::Hashbang;
                    return;
                }
                if char == '<' && !self.keep_xml_tag {
                    self.state = FormatState::XmlTag;
                    return;
                }
                self.result.push(char);
            }
            FormatState::Hashbang => {
                if char.is_ascii_digit() {
                    self.state = FormatState::ArgNum;
                    self.arg_num = char.to_digit(10).unwrap() as usize;
                    return;
                }
                self.result.push('#');
                self.result.push(char);
                self.state = FormatState::Literal;
            }
            FormatState::ArgNum => {
                if char.is_ascii_digit() {
                    self.arg_num = self.arg_num * 10 + char.to_digit(10).unwrap() as usize;
                    return;
                }
                if char == '[' {
                    self.state = FormatState::FormatArg;
                    return;
                }
                if char == '%' {
                    self.state = FormatState::Percent;
                    return;
                }
                self.format_single();
                self.feed(char);
            }
            FormatState::FormatArg => {
                if char == ']' {
                    self.state = FormatState::FmtArgEnd;
                    return;
                }
                self.fmt_arg.push(char);
            }
            FormatState::FmtArgEnd => {
                if char == '%' {
                    self.state = FormatState::Percent;
                    return;
                }
                self.format_single();
                self.feed(char);
            }
            FormatState::Percent => {
                self.format_single();
                self.feed(char);
            }
            FormatState::XmlTag => {
                if char == '>' {
                    self.state = FormatState::Literal;
                }
            }
        }
    }

    fn format_single(&mut self) {
        let percent = self.state == FormatState::Percent;
        if self.state == FormatState::Literal {
            return;
        }
        if self.arg_num > self.parameters.len() {
            self.state = FormatState::Literal;
            self.result.push('#');
            self.result.push_str(&self.arg_num.to_string());
            if !self.fmt_arg.is_empty() {
                self.result.push('[');
                self.result.push_str(&self.fmt_arg);
                self.result.push(']');
            }
            if percent {
                self.result.push('%');
            }
            return;
        }
        let mut arg = self.parameters[self.arg_num - 1];
        if percent {
            arg *= 100.;
        }
        if self.fmt_arg.is_empty() {
            self.result.push_str(&arg.to_string());
        }
        if self.fmt_arg.as_bytes().first() == Some(&b'f') {
            let prec = self.fmt_arg.as_bytes()[1] - b'0';
            self.result.push_str(&format!("{:.1$}", arg, prec as _));
        }
        if self.fmt_arg.as_bytes().first() == Some(&b'i') {
            use thousands::Separable;
            let arg = arg as i32;
            self.result.push_str(&arg.separate_with_commas());
        }
        if percent {
            self.result.push('%');
        }
        self.fmt_arg = String::new();
        self.state = FormatState::Literal;
    }

    fn format(&mut self) -> String {
        for char in self.template.chars() {
            self.feed(char);
        }
        self.format_single();
        std::mem::take(&mut self.result)
    }
}

pub fn format(template: &str, parameters: &[f32]) -> String {
    Formatter::new(template, parameters)
        .keep_xml_tag(false)
        .format()
}
