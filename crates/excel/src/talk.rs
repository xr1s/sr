use std::num::NonZero;

pub use model::talk::VoiceType;

use crate::{ExcelOutput, FromModel};

#[derive(Clone, Debug)]
pub struct TalkSentenceConfig<'a> {
    pub id: u32,
    pub name: &'a str,
    pub text: &'a str,
    pub voice: Option<VoiceConfig>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for TalkSentenceConfig<'a> {
    type Model = model::talk::TalkSentenceConfig;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        const MISSING_VOICE: &[u32] = &[
            100001305, 102120714, 103159006, 103250417, 103250419, 103250421, 103250624, 103250626,
            103250628, 103250709, 103250710, 103250730, 103250732, 103251006, 103301205, 103301207,
            103301209, 103301211, 103329901, 103329902, 200031436, 201071601, 201071602, 201071603,
            201331904, 500180916,
        ];
        Self {
            id: model.talk_sentence_id,
            name: game.text(model.textmap_talk_sentence_name),
            text: game.text(model.talk_sentence_text),
            voice: model
                .voice_id
                .map(NonZero::get)
                .filter(|id| !MISSING_VOICE.contains(id)) // 疑似缺数据
                .map(|id| game.voice_config(id))
                .map(Option::unwrap),
        }
    }
}

#[derive(Clone, Debug)]
pub struct VoiceConfig {
    pub id: u32,
    pub is_player_involved: bool,
    pub r#type: Option<VoiceType>,
}

impl<Data: ExcelOutput> FromModel<'_, Data> for VoiceConfig {
    type Model = model::talk::VoiceConfig;
    fn from_model(_game: &Data, model: &Self::Model) -> Self {
        Self {
            id: model.voice_id,
            is_player_involved: model.is_player_involved,
            r#type: model.voice_type,
        }
    }
}
