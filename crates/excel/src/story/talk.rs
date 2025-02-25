use model::story::talk::OptionIconType;

use crate::ExcelOutput;
use std::num::NonZero;

#[derive(Clone, Debug)]
pub struct RogueSimpleTalk<'a> {
    pub bg_id: u8,
    pub sentence: crate::talk::TalkSentenceConfig<'a>,
    pub text_speed: Option<u8>,
}

impl<'a> RogueSimpleTalk<'a> {
    pub fn from_model<Data: ExcelOutput>(
        game: &'a Data,
        model: model::story::RogueSimpleTalk,
    ) -> Self {
        Self {
            bg_id: model.talk_bg_id.map(NonZero::get).unwrap_or_default(),
            sentence: game.talk_sentence_config(model.talk_sentence_id).unwrap(),
            text_speed: model.text_speed,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RogueOptionTalk<'a> {
    pub sentence: Option<crate::talk::TalkSentenceConfig<'a>>,
    pub option: &'a str,
    pub option_icon_type: Option<OptionIconType>,
    pub rogue_option_id: u32,
    pub trigger_custom_string: String,
    // pub trigger_custom_string_talk: Option<crate::talk::TalkSentenceConfig<'a>>,
}

impl<'a> RogueOptionTalk<'a> {
    pub fn from_model<Data: ExcelOutput>(
        game: &'a Data,
        model: model::story::RogueOptionTalk,
    ) -> Self {
        Self {
            sentence: model
                .talk_sentence_id
                .map(NonZero::get)
                .map(|id| game.talk_sentence_config(id))
                .map(Option::unwrap),
            option: model
                .option_textmap_id
                .map(|text| game.text(text))
                .unwrap_or_default(),
            option_icon_type: model.option_icon_type,
            rogue_option_id: model.rogue_option_id.map(NonZero::get).unwrap_or_default(),
            // trigger_custom_string_talk: if model.trigger_custom_string.starts_with("TalkSentence_")
            // {
            //     let id = model
            //         .trigger_custom_string
            //         .split('_')
            //         .nth(1)
            //         .unwrap()
            //         .parse::<u32>()
            //         .unwrap();
            //     Some(game.talk_sentence_config(id).unwrap())
            // } else {
            //     None
            // },
            trigger_custom_string: model.trigger_custom_string,
        }
    }
}

#[derive(Clone, Debug)]
pub struct OptionTalk<'a> {
    pub delete_after_selection: bool,
    pub finish_key: Option<u8>,
    pub has_triggered: bool,
    pub option_icon_type: OptionIconType,
    pub option_textmap_id: &'a str,
    pub submission_id: u32,
    pub talk_event_id: u32,
    pub sentence: crate::talk::TalkSentenceConfig<'a>,
    pub trigger_custom_string: String,
}

impl<'a> OptionTalk<'a> {
    pub fn from_model<Data: ExcelOutput>(game: &'a Data, model: model::story::OptionTalk) -> Self {
        match model {
            model::story::OptionTalk::OptionTalkInfo {
                delete_after_selection,
                finish_key,
                has_triggered,
                option_icon_type,
                option_textmap_id,
                submission_id,
                talk_event_id,
                talk_sentence_id,
                trigger_custom_string,
            } => Self {
                delete_after_selection,
                finish_key,
                has_triggered,
                option_icon_type,
                option_textmap_id: option_textmap_id
                    .map(|text| game.text(text))
                    .unwrap_or_default(),
                submission_id: submission_id.map(NonZero::get).unwrap_or_default(),
                talk_event_id: talk_event_id.map(NonZero::get).unwrap_or_default(),
                sentence: game.talk_sentence_config(talk_sentence_id).unwrap(),
                trigger_custom_string,
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct SimpleTalk<'a> {
    pub text_speed: u8,
    pub sentence: crate::talk::TalkSentenceConfig<'a>,
    pub protect_time: f32,
}

impl<'a> SimpleTalk<'a> {
    pub fn from_model<Data: ExcelOutput>(game: &'a Data, model: model::story::SimpleTalk) -> Self {
        Self {
            text_speed: model.text_speed.map(NonZero::get).unwrap_or_default(),
            sentence: game.talk_sentence_config(model.talk_sentence_id).unwrap(),
            protect_time: model.protect_time,
        }
    }
}
