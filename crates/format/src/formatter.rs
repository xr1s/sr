use std::borrow::Cow;

#[derive(Debug, Eq, PartialEq)]
enum FormatState {
    Literal,
    Hashbang,
    ArgNum,
    FormatArg,
    FmtArgEnd,
    Percent,
    UnityEscape,
    UnityTagKey(String),
    UnityTagVal(String, String),
    UnityVarKey(String),
    UnityVarVal(String, String),
}

pub struct Formatter<'a> {
    template: &'a str,
    arguments: &'a [crate::argument::Argument<'a>],

    keep_xml_tag: bool,
    output_wiki: bool,

    state: FormatState,
    pub(crate) result: String,
    arg_num: usize,
    fmt_arg: String,
    // 性别内容，出现于 {F#她}{M#他} 形式中
    f_content: String,
    m_content: String,
    // 注音内容，出现于 {RUBY_B#顶部文字}底部文字{RUBY_E#}
    ruby: String,
    // 对于 <align="..."> 这种输出 display: block，需要省略一次 <br>
    omit_br_once: bool,
}

impl<'a> Formatter<'a> {
    pub fn new(template: &'a str, arguments: &'a [crate::argument::Argument]) -> Self {
        Self {
            template,
            arguments,
            keep_xml_tag: true,
            output_wiki: false,
            state: FormatState::Literal,
            result: String::with_capacity(template.len()),
            arg_num: 0,
            fmt_arg: String::new(),
            f_content: String::new(),
            m_content: String::new(),
            ruby: String::new(),
            omit_br_once: false,
        }
    }

    pub fn keep_xml_tag(mut self, keep: bool) -> Self {
        self.keep_xml_tag = keep;
        self
    }

    pub fn output_wiki(mut self, wiki: bool) -> Self {
        self.output_wiki = wiki;
        self
    }

    fn feed(&mut self, char: char) {
        match &mut self.state {
            FormatState::Literal => {
                if char != '<' && char != '\\' {
                    self.omit_br_once = false;
                }
                match char {
                    '#' => self.state = FormatState::Hashbang,
                    '<' if !self.keep_xml_tag || self.output_wiki => {
                        self.state = FormatState::UnityTagKey(String::new());
                    }
                    '\\' if self.output_wiki => self.state = FormatState::UnityEscape,
                    '{' if self.output_wiki => self.state = FormatState::UnityVarKey(String::new()),
                    '\u{00A0}' => {
                        self.result
                            .push_str(if self.output_wiki { "&nbsp;" } else { " " })
                    }
                    '|' if self.output_wiki => self.result.push_str("&#x7c;"),
                    '=' if self.output_wiki => self.result.push_str("{{=}}"),
                    _ => self.result.push(char),
                };
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
                    'n' => Cow::Borrowed(if !self.output_wiki {
                        "\n"
                    } else if !self.omit_br_once {
                        "<br />"
                    } else {
                        ""
                    }),
                    _ => Cow::Owned("\\".to_string() + &char.to_string()),
                });
                self.omit_br_once = false;
            }
            FormatState::UnityTagKey(tag) => {
                let mut tag = std::mem::take(tag);
                if char == '=' {
                    self.omit_br_once = false;
                    self.state = FormatState::UnityTagVal(tag, String::new());
                    return;
                }
                if char == '>' {
                    if !tag.starts_with('/') {
                        // 因为可能存在 <size=50><align="center">！！！警告！！！</align></size>
                        // 所以闭合标签继续省略一次 br
                        self.omit_br_once = false;
                    }
                    self.unity_tag_to_wiki(tag, None);
                    return;
                }
                tag.push(char);
                self.state = FormatState::UnityTagKey(tag);
            }
            FormatState::UnityTagVal(tag, val) => {
                let (tag, mut val) = (std::mem::take(tag), std::mem::take(val));
                if char == '>' {
                    self.unity_tag_to_wiki(tag, Some(val));
                    return;
                }
                val.push(char);
                self.state = FormatState::UnityTagVal(tag, val);
            }
            FormatState::UnityVarKey(var) => {
                let mut var = std::mem::take(var);
                if char == '}' {
                    self.unity_var_to_wiki(var, None);
                    return;
                }
                if char == '#' {
                    self.state = FormatState::UnityVarVal(var, String::new());
                    return;
                }
                var.push(char);
                self.state = FormatState::UnityVarKey(var);
            }
            FormatState::UnityVarVal(var, val) => {
                let (var, mut val) = (std::mem::take(var), std::mem::take(val));
                if char == '}' {
                    self.unity_var_to_wiki(var, Some(val));
                    return;
                }
                val.push(char);
                self.state = FormatState::UnityVarVal(var, val);
            }
        }
    }

    fn format_single(&mut self) {
        if !std::matches!(
            self.state,
            FormatState::Hashbang
                | FormatState::ArgNum
                | FormatState::FormatArg
                | FormatState::FmtArgEnd
                | FormatState::Percent
        ) {
            return;
        }
        let percent = self.state == FormatState::Percent;
        if self.arg_num > self.arguments.len() || self.arguments.is_empty() {
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
        use crate::formattable::Formattable;
        let arg = &self.arguments[self.arg_num - 1];
        if self.fmt_arg.is_empty() {
            arg.write_raw(self, percent);
        }
        if self.fmt_arg.as_bytes().first() == Some(&b'f') {
            let prec = self.fmt_arg.as_bytes()[1] - b'0';
            arg.write_float(self, prec as _, percent);
        }
        if self.fmt_arg.as_bytes().first() == Some(&b'i') {
            arg.write_int(self, percent);
        }
        self.fmt_arg = String::new();
        self.state = FormatState::Literal;
    }

    /// 将 Unity XML 格式转为 MediaWiki 需要的格式
    /// 目前就碰到几种固定的 Tag，硬编码就完事儿了
    fn unity_tag_to_wiki(&mut self, tag: String, val: Option<String>) {
        if !std::matches!(
            self.state,
            FormatState::UnityTagKey(_) | FormatState::UnityTagVal(_, _)
        ) {
            return;
        }
        match tag.as_str() {
            // TODO: <u> 标签需要通过内容判断输出什么
            // 根据是否存在于 game.extra_effect_config 中判断是否输出 {{效果说明}}
            // 目前都转成 HTML 下划线 <u> 好了
            "u" => self.result.push_str("<u>"),
            "/u" => self.result.push_str("</u>"),
            "i" | "/i" => self.result.push_str("''"),
            "color" => {
                self.result.push_str("{{颜色|");
                let color = val.unwrap();
                self.result.push_str(match &color[1..] {
                    "e47d00" | "e47d00ff" => "描述",
                    "88785a" | "88785aff" => "描述1",
                    "f29e38" | "f29e38ff" => "描述2",
                    _ => &color[1..],
                });
                self.result.push('|');
            }
            "/color" => self.result.push_str("}}"),
            "unbreak" | "/unbreak" => (),
            "align" => {
                // 三种 <align="center"> <align="left"> <align="right">
                self.result.push_str("<p style=\"text-align: ");
                let align = val
                    .as_deref()
                    .map(|val| val.strip_prefix('"').unwrap_or(val))
                    .map(|val| val.strip_suffix('"').unwrap_or(val))
                    .unwrap();
                self.result.push_str(align);
                self.result.push_str("\">");
            }
            "/align" => {
                self.result.push_str("</p>");
                self.omit_br_once = true;
            }
            "size" => {
                // 两种形式
                // 1. <size=32> <size=18px> 直接指定字号
                // 2. <size=+2> <size=-2>   指定相对字号
                //
                // 游戏内基础字号是 20，Wiki 基础字号为 14
                // 因此需要按比例缩放，不能直接用游戏内的大小
                // 考虑到 Wiki 字号可能会变，最好用 em 而非 px
                self.result.push_str("<span style=\"font-size: ");
                let val = val.unwrap();
                if val.starts_with('+') || val.starts_with('-') {
                    // grep 了一遍没有格式问题，不会 panic
                    let relative = val.parse::<i32>().unwrap();
                    let font_size = format!("{}", 1. + relative as f32 / 20.);
                    self.result.push_str(&font_size);
                } else {
                    let game_size = val
                        .strip_suffix("px")
                        .unwrap_or(&val)
                        .parse::<i32>()
                        .unwrap();
                    let font_size = format!("{}", game_size as f32 / 20.);
                    self.result.push_str(&font_size);
                }
                self.result.push_str("em\">");
            }
            "/size" => self.result.push_str("</span>"),
            _ => {
                // 不认识的标签原样填回去
                self.result.push('<');
                self.result.push_str(&tag);
                if let Some(val) = val {
                    self.result.push('=');
                    self.result.push_str(&val);
                }
                self.result.push('>');
            }
        }
        self.state = FormatState::Literal;
    }

    fn unity_var_to_wiki(&mut self, var: String, val: Option<String>) {
        if !std::matches!(
            self.state,
            FormatState::UnityVarKey(_) | FormatState::UnityVarVal(_, _)
        ) {
            return;
        }
        match var.as_str() {
            "NICKNAME" => self.result.push_str("开拓者"),
            "F" => {
                if !self.m_content.is_empty() {
                    self.result.push_str(&val.unwrap());
                    self.result.push('/');
                    self.result.push_str(&self.m_content);
                    self.m_content = String::new();
                } else {
                    self.f_content = val.unwrap();
                }
            }
            "M" => {
                if !self.f_content.is_empty() {
                    self.result.push_str(&self.f_content);
                    self.result.push('/');
                    self.result.push_str(&val.unwrap());
                    self.f_content = String::new();
                } else {
                    self.m_content = val.unwrap();
                }
            }
            "RUBY_B" => {
                self.result.push_str("{{注音|");
                self.ruby = val.unwrap();
            }
            "RUBY_E" => {
                self.result.push('|');
                self.result.push_str(&self.ruby);
                self.result.push_str("}}");
                self.ruby = String::new();
            }
            _ => {
                self.result.push('{');
                self.result.push_str(&var);
                if let Some(val) = val {
                    self.result.push('#');
                    self.result.push_str(&val);
                }
                self.result.push('}');
            }
        }
        self.state = FormatState::Literal;
    }

    pub fn flush(&mut self) {
        self.format_single();
        if let FormatState::UnityTagKey(tag) = &self.state {
            self.result.push('<');
            self.result.push_str(tag);
            // 特地不闭合 >
        }
        if let FormatState::UnityTagVal(tag, val) = &self.state {
            self.result.push('<');
            self.result.push_str(tag);
            self.result.push('=');
            self.result.push_str(val);
            // 特地不闭合 >
        }
        if let FormatState::UnityVarKey(var) = &self.state {
            self.result.push('{');
            self.result.push_str(var);
            // 特地不闭合 }
        }
        if let FormatState::UnityVarKey(var) = &self.state {
            self.result.push('{');
            self.result.push_str(var);
            self.result.push('#');
            self.result.push_str(var);
            // 特地不闭合 }
        }
    }

    pub fn format(&mut self) -> String {
        for char in self.template.chars() {
            self.feed(char);
        }
        self.flush();
        std::mem::take(&mut self.result)
    }
}
