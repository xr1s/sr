#[derive(Debug, Eq, PartialEq)]
enum State {
    Literal,
    Hashbang,
    ArgNum,
    FormatArg,
    FmtArgEnd,
    Percent,
    UnityEscape,

    // <u> <i> <color=#000000> <size=20> <size=+2>
    UnityTagKey(String),
    UnityTagVal(String, String),

    // {BIRTH} {F#女性开拓者分支} {M#男性开拓者分支}
    // {RUBY_B#注释内容}对话内容{RUBY_E}
    // {TEXTJOIN#编号}
    UnityVarKey(String),
    UnityVarVal(String, String),
}

enum Syntax {
    Raw,
    MediaWiki,
    AnsiSequence,
}

impl Syntax {
    fn is_raw(&self) -> bool {
        std::matches!(self, Self::Raw)
    }
    fn is_media_wiki(&self) -> bool {
        std::matches!(self, Self::MediaWiki)
    }
}

pub struct Formatter<'a, Data: crate::data::GameData + ?Sized> {
    data: &'a Data,

    syntax: Syntax,
    newline_after_block: bool,

    state: State,
    result: String,
    arg_num: usize,
    fmt_arg: String,
    // 为 <u> 判断 extra_effect 准备的
    // 非常巨大的逻辑改动，相当于是在 result 上面再加了一层
    // 在出现 <u> 之后进入 underlining 状态，此状态下所有数据都会被写入 underline
    // 直到处理完 underlining 才会重新往 result 写入数据
    underlining: bool,
    underline: String,
    // 性别内容，出现于 {F#她}{M#他} 形式中
    f_content: String,
    m_content: String,
    // 注音内容，出现于 {RUBY_B#顶部文字}底部文字{RUBY_E#}
    ruby: String,
    // 对于 <align="..."> 这种输出 display: block，需要省略一次 <br>
    omit_br_once: bool,
    // 在 </p> 或者 <br /> 之后需要写一个 \n，但是避免后续仍是闭合标签的情况
    need_write_newline: bool,
}

impl<'a, Data: crate::data::GameData> Formatter<'a, Data> {
    pub fn new(data: &'a Data) -> Self {
        Self {
            data,
            syntax: Syntax::Raw,
            newline_after_block: false,
            state: State::Literal,
            result: String::new(),
            arg_num: 0,
            fmt_arg: String::new(),
            underlining: false,
            underline: String::new(),
            f_content: String::new(),
            m_content: String::new(),
            ruby: String::new(),
            omit_br_once: false,
            need_write_newline: false,
        }
    }
}

impl<Data: crate::data::GameData> Formatter<'_, Data> {
    pub fn media_wiki_syntax(mut self, set: bool) -> Self {
        if set {
            self.syntax = Syntax::MediaWiki;
        }
        self
    }

    pub fn ansi_sequence_syntax(mut self, set: bool) -> Self {
        if set {
            self.syntax = Syntax::AnsiSequence;
        }
        self
    }

    pub fn newline_after_block(mut self, set: bool) -> Self {
        self.newline_after_block = set;
        self
    }

    pub(crate) fn push(&mut self, ch: char) {
        if self.underlining {
            &mut self.underline
        } else {
            &mut self.result
        }
        .push(ch);
    }

    pub(crate) fn push_str(&mut self, string: &str) {
        if self.underlining {
            &mut self.underline
        } else {
            &mut self.result
        }
        .push_str(string);
    }

    fn feed(&mut self, char: char, arguments: &[crate::Argument]) {
        match &mut self.state {
            State::Literal => {
                if char != '<' && char != '\\' {
                    self.omit_br_once = false;
                }
                if char != '<' && self.need_write_newline {
                    self.push('\n');
                    self.need_write_newline = false;
                }
                match char {
                    '#' => self.state = State::Hashbang,
                    '<' => self.state = State::UnityTagKey(String::new()),
                    '{' => self.state = State::UnityVarKey(String::new()),
                    '\\' => self.state = State::UnityEscape,
                    '\u{00A0}' => self.push_str(if self.syntax.is_media_wiki() {
                        "&nbsp;"
                    } else {
                        " "
                    }),
                    '|' if self.syntax.is_media_wiki() => self.push_str("&#x7c;"),
                    '=' if self.syntax.is_media_wiki() => self.push_str("{{=}}"),
                    _ => self.push(char),
                };
            }
            State::Hashbang => {
                if char.is_ascii_digit() {
                    self.state = State::ArgNum;
                    self.arg_num = char.to_digit(10).unwrap() as usize;
                    return;
                }
                self.push('#');
                self.push(char);
                self.state = State::Literal;
            }
            State::ArgNum => {
                if char.is_ascii_digit() {
                    self.arg_num = self.arg_num * 10 + char.to_digit(10).unwrap() as usize;
                    return;
                }
                if char == '[' {
                    self.state = State::FormatArg;
                    return;
                }
                if char == '%' {
                    self.state = State::Percent;
                    return;
                }
                self.format_single(arguments);
                self.feed(char, arguments);
            }
            State::FormatArg => {
                if char == ']' {
                    self.state = State::FmtArgEnd;
                    return;
                }
                self.fmt_arg.push(char);
            }
            State::FmtArgEnd => {
                if char == '%' {
                    self.state = State::Percent;
                    return;
                }
                self.format_single(arguments);
                self.feed(char, arguments);
            }
            State::Percent => {
                self.format_single(arguments);
                self.feed(char, arguments);
            }
            State::UnityEscape => {
                self.state = State::Literal;
                match char {
                    'n' => {
                        if !self.syntax.is_media_wiki() {
                            self.push('\n');
                        } else if !self.omit_br_once {
                            self.push_str("<br />");
                            if self.newline_after_block {
                                self.need_write_newline = true;
                            }
                        }
                    }
                    _ => self.push_str(&("\\".to_string() + &char.to_string())),
                }
                self.omit_br_once = false;
            }
            State::UnityTagKey(tag) => {
                let mut tag = std::mem::take(tag);
                if char == '=' {
                    self.omit_br_once = false;
                    self.state = State::UnityTagVal(tag, String::new());
                    return;
                }
                if char == '>' {
                    if !tag.starts_with('/') {
                        // 因为可能存在 <size=50><align="center">！！！警告！！！</align></size>
                        // 所以闭合标签继续省略一次 br
                        self.omit_br_once = false;
                    }
                    if !tag.starts_with('/') && self.need_write_newline {
                        self.push('\n');
                        self.need_write_newline = false;
                    }
                    self.unity_tag_to_wiki(tag, None);
                    return;
                }
                tag.push(char);
                self.state = State::UnityTagKey(tag);
            }
            State::UnityTagVal(tag, val) => {
                let (tag, mut val) = (std::mem::take(tag), std::mem::take(val));
                if char == '>' {
                    self.unity_tag_to_wiki(tag, Some(val));
                    return;
                }
                val.push(char);
                self.state = State::UnityTagVal(tag, val);
            }
            State::UnityVarKey(var) => {
                let mut var = std::mem::take(var);
                if char == '}' {
                    self.unity_var_to_wiki(var, None);
                    return;
                }
                if char == '#' {
                    self.state = State::UnityVarVal(var, String::new());
                    return;
                }
                var.push(char);
                self.state = State::UnityVarKey(var);
            }
            State::UnityVarVal(var, val) => {
                let (var, mut val) = (std::mem::take(var), std::mem::take(val));
                if char == '}' {
                    self.unity_var_to_wiki(var, Some(val));
                    return;
                }
                val.push(char);
                self.state = State::UnityVarVal(var, val);
            }
        }
    }

    fn format_single(&mut self, arguments: &[crate::Argument]) {
        if !std::matches!(
            self.state,
            State::Hashbang | State::ArgNum | State::FormatArg | State::FmtArgEnd | State::Percent
        ) {
            return;
        }
        let percent = self.state == State::Percent;
        if self.arg_num > arguments.len() || arguments.is_empty() {
            self.state = State::Literal;
            self.push('#');
            self.push_str(&self.arg_num.to_string());
            if !self.fmt_arg.is_empty() {
                self.push('[');
                let fmt_arg = std::mem::take(&mut self.fmt_arg);
                self.push_str(&fmt_arg);
                self.push(']');
            }
            if percent {
                self.push('%');
            }
            return;
        }
        use crate::formattable::Formattable;
        let arg = &arguments[self.arg_num - 1];
        if self.fmt_arg.is_empty() {
            self.write_raw(arg, percent);
        }
        if self.fmt_arg.as_bytes().first() == Some(&b'f') {
            let prec = self.fmt_arg.as_bytes()[1] - b'0';
            self.write_float(arg, prec as _, percent);
        }
        if self.fmt_arg.as_bytes().first() == Some(&b'i') {
            self.write_int(arg, percent);
        }
        self.fmt_arg = String::new();
        self.state = State::Literal;
    }

    /// 将 Unity XML 格式转为 MediaWiki 需要的格式
    /// 目前就碰到几种固定的 Tag，硬编码就完事儿了
    fn unity_tag_to_wiki(&mut self, tag: String, val: Option<String>) {
        if !std::matches!(self.state, State::UnityTagKey(_) | State::UnityTagVal(_, _)) {
            return;
        }
        if self.syntax.is_raw()
            && ["u", "i", "color", "align", "size"].contains(&tag.strip_prefix('/').unwrap_or(&tag))
        {
            self.state = State::Literal;
            return;
        }
        match tag.as_str() {
            "u" => self.underlining = true,
            "/u" => match self.syntax {
                Syntax::Raw => unreachable!(),
                Syntax::MediaWiki => {
                    if self.data.has_extra_effect_config(&self.underline) {
                        self.result.push_str("{{效果说明|");
                        self.result.push_str(&self.underline);
                        self.result.push_str("}}");
                    } else {
                        self.result.push_str("<u>");
                        self.result.push_str(&self.underline);
                        self.result.push_str("</u>");
                    }
                    self.underlining = false;
                    self.underline = String::new();
                }
                Syntax::AnsiSequence => {
                    self.result.push_str("\x1B[4m");
                    self.result.push_str(&self.underline);
                    self.result.push_str("\x1B[24m");
                    self.underlining = false;
                    self.underline = String::new();
                }
            },
            "s" => self.push_str(match self.syntax {
                Syntax::Raw => "",
                Syntax::MediaWiki => "<s>",
                Syntax::AnsiSequence => "\x1B[9m",
            }),
            "/s" => self.push_str(match self.syntax {
                Syntax::Raw => "",
                Syntax::MediaWiki => "</s>",
                Syntax::AnsiSequence => "\x1B[29m",
            }),
            "i" => self.push_str(match self.syntax {
                Syntax::Raw => "",
                Syntax::MediaWiki => "''",
                Syntax::AnsiSequence => "\x1B[3m",
            }),
            "/i" => self.push_str(match self.syntax {
                Syntax::Raw => "",
                Syntax::MediaWiki => "''",
                Syntax::AnsiSequence => "\x1B[23m",
            }),
            "b" => self.push_str(match self.syntax {
                Syntax::Raw => "",
                Syntax::MediaWiki => "'''",
                Syntax::AnsiSequence => "\x1B[1m",
            }),
            "/b" => self.push_str(match self.syntax {
                Syntax::Raw => "",
                Syntax::MediaWiki => "'''",
                Syntax::AnsiSequence => "\x1B[m",
            }),
            "color" => match self.syntax {
                Syntax::Raw => (),
                Syntax::MediaWiki => {
                    self.push_str("{{颜色|");
                    let color = val.unwrap();
                    self.push_str(match &color[1..] {
                        "e47d00" | "e47d00ff" => "描述",
                        "88785a" | "88785aff" => "描述1",
                        "f29e38" | "f29e38ff" => "描述2",
                        _ => &color[1..],
                    });
                    self.push('|');
                }
                Syntax::AnsiSequence => {
                    use std::str::FromStr;
                    if let Some(val) = val {
                        if let Ok(color) = base::serde::Color::from_str(&val) {
                            self.push_str("\x1B[38;2;");
                            self.push_str(&color.0.to_string());
                            self.push(';');
                            self.push_str(&color.1.to_string());
                            self.push(';');
                            self.push_str(&color.2.to_string());
                            self.push('m');
                        }
                    }
                }
            },
            "/color" => self.push_str(match self.syntax {
                Syntax::Raw => "",
                Syntax::MediaWiki => "}}",
                Syntax::AnsiSequence => "\x1B[39m",
            }),
            "unbreak" | "/unbreak" => (),
            "align" => {
                // 三种 <align="center"> <align="left"> <align="right">
                match self.syntax {
                    Syntax::Raw => (),
                    Syntax::MediaWiki => {
                        self.push_str("<p style=\"text-align: ");
                        let align = val
                            .as_deref()
                            .map(|val| val.strip_prefix('"').unwrap_or(val))
                            .map(|val| val.strip_suffix('"').unwrap_or(val))
                            .unwrap();
                        self.push_str(align);
                        self.push_str("\">");
                    }
                    Syntax::AnsiSequence => {
                        let align = val
                            .as_deref()
                            .map(|val| val.strip_prefix('"').unwrap_or(val))
                            .map(|val| val.strip_suffix('"').unwrap_or(val))
                            .unwrap();
                        self.push_str(match align {
                            "left" => "",
                            "center" => "                   ",
                            "right" => "                                        ",
                            _ => unreachable!(),
                        });
                    }
                }
            }
            "/align" => match self.syntax {
                Syntax::Raw => (),
                Syntax::MediaWiki => {
                    self.push_str("</p>");
                    if self.newline_after_block {
                        self.need_write_newline = true;
                    }
                    self.omit_br_once = true;
                }
                Syntax::AnsiSequence => {
                    self.push('\n');
                    self.omit_br_once = true;
                }
            },
            "size" if self.syntax.is_media_wiki() => {
                // 两种形式
                // 1. <size=32> <size=18px> 直接指定字号
                // 2. <size=+2> <size=-2>   指定相对字号
                //
                // 游戏内基础字号是 20，Wiki 基础字号为 14
                // 因此需要按比例缩放，不能直接用游戏内的大小
                // 考虑到 Wiki 字号可能会变，最好用 em 而非 px
                self.push_str("<span style=\"font-size: ");
                let val = val.unwrap();
                if val.starts_with('+') || val.starts_with('-') {
                    // grep 了一遍没有格式问题，不会 panic
                    let relative = val.parse::<i32>().unwrap();
                    let font_size = format!("{}", 1. + relative as f32 / 20.);
                    self.push_str(&font_size);
                } else {
                    let game_size = val
                        .strip_suffix("px")
                        .unwrap_or(&val)
                        .parse::<i32>()
                        .unwrap();
                    let font_size = format!("{}", game_size as f32 / 20.);
                    self.push_str(&font_size);
                }
                self.push_str("em\">");
            }
            "/size" if self.syntax.is_media_wiki() => self.push_str("</span>"),
            _ => {
                // 不认识的标签原样填回去
                if self.syntax.is_media_wiki() {
                    self.push_str("&lt;");
                    self.push_str(&tag.replace('\u{00A0}', "&nbsp;"));
                    if let Some(val) = val {
                        self.push('=');
                        self.push_str(&val.replace('\u{00A0}', "&nbsp;"));
                    }
                    self.push_str("&gt;");
                } else {
                    self.push('<');
                    self.push_str(&tag.replace('\u{00A0}', " "));
                    if let Some(val) = val {
                        self.push('=');
                        self.push_str(&val.replace('\u{00A0}', " "));
                    }
                    self.push('>');
                }
            }
        }
        self.state = State::Literal;
    }

    fn unity_var_to_wiki(&mut self, var: String, val: Option<String>) {
        if !std::matches!(self.state, State::UnityVarKey(_) | State::UnityVarVal(_, _)) {
            return;
        }
        match var.as_str() {
            "NICKNAME" => self.push_str("开拓者"),
            "F" => {
                if !self.m_content.is_empty() {
                    self.push_str(&val.unwrap());
                    self.push('/');
                    let m_content = std::mem::take(&mut self.m_content);
                    self.push_str(&m_content);
                } else {
                    self.f_content = val.unwrap();
                }
            }
            "M" => {
                if !self.f_content.is_empty() {
                    let f_content = std::mem::take(&mut self.f_content);
                    self.push_str(&f_content);
                    self.push('/');
                    self.push_str(&val.unwrap());
                } else {
                    self.m_content = val.unwrap();
                }
            }
            "RUBY_B" => {
                self.push_str("{{注音|");
                self.ruby = val.unwrap();
            }
            "RUBY_E" => {
                self.push('|');
                let ruby = std::mem::take(&mut self.ruby);
                self.push_str(&ruby);
                self.push_str("}}");
                self.ruby = String::new();
            }
            "TEXTJOIN" => {
                let id: u8 = val.unwrap().parse().unwrap();
                let item = self.data.default_text_join_item(
                    id,
                    self.syntax.is_media_wiki(),
                    self.newline_after_block,
                );
                self.push_str(&item);
            }
            _ => {
                self.push('{');
                self.push_str(&var);
                if let Some(val) = val {
                    self.push('#');
                    self.push_str(&val);
                }
                self.push('}');
            }
        }
        self.state = State::Literal;
    }

    pub fn flush(&mut self) {
        if self.underlining {
            self.push_str("<u>");
            let underline = std::mem::take(&mut self.underline);
            self.push_str(&underline);
        }
        if let State::UnityTagKey(tag) = &mut self.state {
            let tag = std::mem::take(tag);
            self.push('<');
            self.push_str(&tag);
            // 特地不闭合 >
        }
        if let State::UnityTagVal(tag, val) = &mut self.state {
            let tag = std::mem::take(tag);
            let val = std::mem::take(val);
            self.push('<');
            self.push_str(&tag);
            self.push('=');
            self.push_str(&val);
            // 特地不闭合 >
        }
        if let State::UnityVarKey(var) = &mut self.state {
            let var = std::mem::take(var);
            self.push('{');
            self.push_str(&var);
            // 特地不闭合 }
        }
        if let State::UnityVarVal(var, val) = &mut self.state {
            let var = std::mem::take(var);
            let val = std::mem::take(val);
            self.push('{');
            self.push_str(&var);
            self.push('#');
            self.push_str(&val);
            // 特地不闭合 }
        }
        if self.need_write_newline {
            self.push('\n');
            self.need_write_newline = false;
        }
        self.state = State::Literal;
    }

    pub fn format(&mut self, format: &str, arguments: &[crate::Argument]) -> String {
        for char in format.chars() {
            self.feed(char, arguments);
        }
        self.format_single(arguments);
        self.flush();
        std::mem::take(&mut self.result)
    }
}
