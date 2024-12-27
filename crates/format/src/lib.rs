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
    impl data::Text for Data {
        fn text(&self, _text: model::Text) -> &str {
            ""
        }
    }
    impl crate::data::GameData for Data {
        fn default_text_join_item(&self, _id: u8) -> &str {
            ""
        }
        fn has_extra_effect_config(&self, _name: &str) -> bool {
            false
        }
    }

    #[test]
    fn not_valid_tag() {
        let mut formatter = super::Formatter::new(&Data);
        assert_eq!(formatter.format("<(￣︶￣)>", &[]), "<(￣︶￣)>");
    }
}
