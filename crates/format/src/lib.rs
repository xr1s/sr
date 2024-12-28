mod argument;
mod data;
mod formattable;
mod formatter;

pub use argument::Argument;
pub use data::GameData;
pub use formatter::Formatter;

#[cfg(test)]
mod test {
    struct Data;
    impl crate::data::GameData for Data {
        fn default_text_join_item(&self, _id: u8) -> &str {
            ""
        }
        fn has_extra_effect_config(&self, _name: &str) -> bool {
            false
        }
    }

    use crate::{Argument::*, Formatter};
    #[test]
    fn basic_formatter() {
        let mut f = Formatter::new(&Data);
        assert_eq!(f.format("#2[i]", &[Floating(3.25), Floating(6.75)]), "7");
        assert_eq!(f.format("#3[i]", &[Floating(0.), Floating(0.475)]), "#3[i]");
        assert_eq!(
            f.format("2 的 16 次方是 <unbreak>#1[i]</unbreak>", &[Signed(65536)]),
            "2 的 16 次方是 65,536"
        );
        assert_eq!(
            f.format("大概有#1[f1]%的概率爆炸", &[Floating(0.6666)]),
            "大概有66.7%的概率爆炸"
        );
    }

    #[test]
    fn text_output() {
        let mut f = Formatter::new(&Data);
        assert_eq!(f.format("<i>斜体</i>", &[]), "斜体");
        assert_eq!(f.format("<u><i>斜体</i></u>", &[]), "斜体");
    }

    #[test]
    fn something_like_tag() {
        let mut f = Formatter::new(&Data).media_wiki_syntax(true);
        // 2.7 版本中三月七在主角冰箱上给主角留的便条
        assert_eq!(f.format("<(￣︶￣)>", &[]), "&lt;(￣︶￣)&gt;");
        // 2.1 版本黄泉实装后给主角发送的登录短信
        assert_eq!(
            f.format("<Grand\u{00A0}Melodie\u{00A0}黄金的时刻>", &[]),
            "&lt;Grand&nbsp;Melodie&nbsp;黄金的时刻&gt;"
        );
    }

    #[test]
    fn nested_tag_variable() {
        let mut f = Formatter::new(&Data).media_wiki_syntax(true);
        assert_eq!(f.format("<u>{NICKNAME}</u>", &[]), "<u>开拓者</u>");
        assert_eq!(
            f.format("<u>{RUBY_B#丰饶星神}<i>慈怀药师</i>{RUBY_E}</u>", &[]),
            "<u>{{注音|''慈怀药师''|丰饶星神}}</u>"
        );
    }

    #[test]
    fn text_join() {
        struct Data;
        impl crate::data::GameData for Data {
            fn default_text_join_item(&self, id: u8) -> &str {
                match id {
                    1 => "你好",
                    2 => "谢谢",
                    3 => "再见",
                    _ => "",
                }
            }
            fn has_extra_effect_config(&self, name: &str) -> bool {
                ["谢谢"].contains(&name)
            }
        }
        let mut f = Formatter::new(&Data).media_wiki_syntax(true);
        assert_eq!(f.format("<u>{TEXTJOIN#1}</u>", &[]), "<u>你好</u>");
        assert_eq!(f.format("<u>{TEXTJOIN#2}</u>", &[]), "{{效果说明|谢谢}}");
        assert_eq!(f.format("<u>{TEXTJOIN#3}</u>", &[]), "<u>再见</u>");
    }
}
