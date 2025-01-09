use data::Text;

use data::SealedGameData;

pub trait GameData {
    fn default_text_join_item(
        &self,
        id: u8,
        media_wiki_syntax: bool,
        newline_after_block: bool,
    ) -> String;
    fn has_extra_effect_config(&self, name: &str) -> bool;
}

impl GameData for data::GameData {
    fn default_text_join_item(
        &self,
        id: u8,
        media_wiki_syntax: bool,
        newline_after_block: bool,
    ) -> String {
        self._text_join_config()
            .get(&id)
            .map(|config| config.default_item)
            .map(|id| self._text_join_item().get(&id))
            .map(Option::unwrap)
            .map(|item| item.text_join_text)
            .map(|text| {
                crate::formatter::Formatter::new(self)
                    .media_wiki_syntax(media_wiki_syntax)
                    .newline_after_block(newline_after_block)
                    .format(self.text(text), &[])
            })
            .unwrap_or_default()
    }

    fn has_extra_effect_config(&self, name: &str) -> bool {
        self._extra_effect_config_by_name().get(name).is_some()
    }
}
