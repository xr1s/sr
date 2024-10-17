trait FormatArgs {
    /// #1% 形式
    fn write_raw(&self, f: &mut Formatter<'_>, percent: bool);
    /// #1[i]% 形式
    fn write_int(&self, f: &mut Formatter<'_>, percent: bool);
    /// #1[f1]% 形式
    fn write_float(&self, f: &mut Formatter<'_>, prec: usize, percent: bool);
}

impl FormatArgs for &'_ str {
    fn write_raw(&self, f: &mut Formatter<'_>, percent: bool) {
        f.result.push_str(self);
        if percent {
            f.result.push('%');
        }
    }

    fn write_int(&self, f: &mut Formatter<'_>, percent: bool) {
        f.result.push_str(self);
        f.result.push_str("[i]");
        if percent {
            f.result.push('%');
        }
    }

    fn write_float(&self, f: &mut Formatter<'_>, prec: usize, percent: bool) {
        f.result.push_str(self);
        f.result.push_str("[f");
        if prec != 0 {
            f.result.push_str(&prec.to_string());
            f.result.push(']');
        }
        if percent {
            f.result.push('%');
        }
    }
}

impl FormatArgs for u32 {
    fn write_raw(&self, f: &mut Formatter<'_>, percent: bool) {
        self.write_int(f, percent);
    }

    fn write_int(&self, f: &mut Formatter<'_>, percent: bool) {
        use thousands::Separable;
        let value = if percent { *self * 100 } else { *self };
        f.result.push_str(&value.separate_with_commas());
        if percent {
            f.result.push('%');
        }
    }

    fn write_float(&self, f: &mut Formatter<'_>, prec: usize, percent: bool) {
        let value = *self as f32;
        FormatArgs::write_float(&value, f, prec, percent);
    }
}

impl FormatArgs for f32 {
    fn write_raw(&self, f: &mut Formatter<'_>, percent: bool) {
        let value = if percent { *self * 100. } else { *self };
        f.result.push_str(&format!("{}", value));
    }

    fn write_int(&self, f: &mut Formatter<'_>, percent: bool) {
        let value = if percent { *self * 100. } else { *self } as u32;
        FormatArgs::write_int(&value, f, false);
        if percent {
            f.result.push('%');
        }
    }

    fn write_float(&self, f: &mut Formatter<'_>, prec: usize, percent: bool) {
        let value = if percent { *self * 100. } else { *self };
        f.result.push_str(&format!("{value:.0$}", prec));
        if percent {
            f.result.push('%');
        }
    }
}

pub struct Formattable<'a>(&'a dyn FormatArgs);

impl<'a> From<&'a f32> for Formattable<'a> {
    fn from(value: &'a f32) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a &str> for Formattable<'a> {
    fn from(value: &'a &str) -> Self {
        Self(value)
    }
}

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

pub struct Formatter<'a> {
    template: &'a str,
    parameters: &'a [Formattable<'a>],

    keep_xml_tag: bool,

    state: FormatState,
    result: String,
    arg_num: usize,
    fmt_arg: String,
}

impl<'a> Formatter<'a> {
    pub fn new(template: &'a str, parameters: &'a [Formattable]) -> Self {
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
        if self.state == FormatState::Literal {
            return;
        }
        let percent = self.state == FormatState::Percent;
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
        let arg = &self.parameters[self.arg_num - 1];
        if self.fmt_arg.is_empty() {
            arg.0.write_raw(self, percent);
        }
        if self.fmt_arg.as_bytes().first() == Some(&b'f') {
            let prec = self.fmt_arg.as_bytes()[1] - b'0';
            arg.0.write_float(self, prec as usize, percent);
        }
        if self.fmt_arg.as_bytes().first() == Some(&b'i') {
            arg.0.write_int(self, percent);
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

pub fn format(template: &str, parameters: &[Formattable]) -> String {
    Formatter::new(template, parameters)
        .keep_xml_tag(false)
        .format()
}
