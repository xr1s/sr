use std::borrow::Cow;

mod sealed {
    use super::Formatter;

    pub(super) trait Formattable {
        /// #1% 形式
        fn write_raw(&self, f: &mut Formatter<'_>, percent: bool);
        /// #1[i]% 形式
        fn write_int(&self, f: &mut Formatter<'_>, percent: bool);
        /// #1[f1]% 形式
        fn write_float(&self, f: &mut Formatter<'_>, prec: usize, percent: bool);
    }

    impl Formattable for &'_ str {
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

    impl Formattable for u32 {
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
            Formattable::write_float(&value, f, prec, percent);
        }
    }

    impl Formattable for f32 {
        fn write_raw(&self, f: &mut Formatter<'_>, percent: bool) {
            let value = if percent { *self * 100. } else { *self };
            f.result.push_str(&format!("{}", value));
        }

        fn write_int(&self, f: &mut Formatter<'_>, percent: bool) {
            let value = if percent { *self * 100. } else { *self } as u32;
            Formattable::write_int(&value, f, false);
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

    impl Formattable for u16 {
        fn write_raw(&self, f: &mut Formatter<'_>, percent: bool) {
            <u32 as Formattable>::write_raw(&(*self as u32), f, percent);
        }
        fn write_int(&self, f: &mut Formatter<'_>, percent: bool) {
            <u32 as Formattable>::write_int(&(*self as u32), f, percent);
        }
        fn write_float(&self, f: &mut Formatter<'_>, prec: usize, percent: bool) {
            <u32 as Formattable>::write_float(&(*self as u32), f, prec, percent);
        }
    }
}

// TODO: 试着改造成抛弃所有权的版本
pub struct Argument<'a>(&'a dyn sealed::Formattable);

impl<'a> From<&'a f32> for Argument<'a> {
    fn from(value: &'a f32) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a &str> for Argument<'a> {
    fn from(value: &'a &str) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a u16> for Argument<'a> {
    fn from(value: &'a u16) -> Self {
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
    UnityEscape,
    UnityTagKey,
    UnityTagVal,
}

pub struct Formatter<'a> {
    template: &'a str,
    arguments: &'a [Argument<'a>],

    keep_xml_tag: bool,
    wiki: bool,

    state: FormatState,
    result: String,
    arg_num: usize,
    fmt_arg: String,
    tag_key: String,
    tag_val: String,
}

impl<'a> Formatter<'a> {
    pub fn new(template: &'a str, arguments: &'a [Argument]) -> Self {
        Self {
            template,
            arguments,
            keep_xml_tag: true,
            wiki: false,
            state: FormatState::Literal,
            result: String::with_capacity(template.len()),
            arg_num: 0,
            fmt_arg: String::new(),
            tag_key: String::new(),
            tag_val: String::new(),
        }
    }

    pub fn keep_xml_tag(mut self, keep: bool) -> Self {
        self.keep_xml_tag = keep;
        self
    }

    pub fn wiki(mut self, wiki: bool) -> Self {
        self.wiki = wiki;
        self
    }

    fn feed(&mut self, char: char) {
        match self.state {
            FormatState::Literal => {
                if char == '#' {
                    self.state = FormatState::Hashbang;
                    return;
                }
                if char == '<' && (!self.keep_xml_tag || self.wiki) {
                    self.state = FormatState::UnityTagKey;
                    return;
                }
                if char == '\\' {
                    self.state = FormatState::UnityEscape;
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
            FormatState::UnityEscape => {
                self.state = FormatState::Literal;
                self.result.push_str(&match char {
                    'n' => Cow::Borrowed(if self.wiki { "<br />" } else { "\n" }),
                    _ => Cow::Owned("\\".to_string() + &char.to_string()),
                });
            }
            FormatState::UnityTagKey => {
                if char == '=' {
                    self.state = FormatState::UnityTagVal;
                    return;
                }
                if char == '>' {
                    self.unity_to_wiki();
                    return;
                }
                self.tag_key.push(char);
            }
            FormatState::UnityTagVal => {
                if char == '>' {
                    self.unity_to_wiki();
                    return;
                }
                self.tag_val.push(char);
            }
        }
    }

    fn format_single(&mut self) {
        if self.state == FormatState::Literal {
            return;
        }
        let percent = self.state == FormatState::Percent;
        if self.arg_num > self.arguments.len() {
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
        let arg = &self.arguments[self.arg_num - 1];
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

    /// 将 Unity XML 格式转为 MediaWiki 需要的格式
    /// 目前就碰到几种固定的 Tag，硬编码就完事儿了
    fn unity_to_wiki(&mut self) {
        self.result.push_str(&match self.tag_key.as_str() {
            "u" => Cow::Borrowed("{{效果说明"),
            "i" | "/i" => Cow::Borrowed("''"),
            "color" => {
                let mut color = String::from("{{颜色|");
                color += match &self.tag_val.to_lowercase()[1..] {
                    "e47d00" | "e47d00ff" => "描述",
                    "88785a" | "88785aff" => "描述1",
                    "f29e38" | "f29e38ff" => "描述2",
                    _ => &self.tag_val[1..],
                };
                Cow::Owned(color + "|")
            }
            // 闭合标签
            "/u" | "/color" => Cow::Borrowed("}}"),
            _ => Cow::Borrowed(""),
        });
        self.state = FormatState::Literal;
        self.tag_key = String::new();
        self.tag_val = String::new();
    }

    pub fn format(&mut self) -> String {
        for char in self.template.chars() {
            self.feed(char);
        }
        self.format_single();
        std::mem::take(&mut self.result)
    }
}

pub fn format(template: &str, arguments: &[Argument]) -> String {
    Formatter::new(template, arguments).format()
}

pub fn format_wiki(template: &str) -> String {
    Formatter::new(template, &[]).wiki(true).format()
}
