use std::num::NonZero;

use crate::Text;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct RogueSimpleTalk {
    #[serde(rename = "TalkBgID")]
    pub talk_bg_id: Option<NonZero<u8>>,
    #[serde(rename = "TalkSentenceID")]
    pub talk_sentence_id: u32,
    pub text_speed: Option<u8>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum OptionIconType {
    AbyssIcon,
    ChallengeStoryIcon,
    ChatBackIcon,
    ChatContinueIcon,
    ChatIcon,
    ChatLoopIcon,
    ChatMissionIcon,
    ChatOutIcon,
    CheckIcon,
    GeneralActivityIcon,
    OrigamiBirdIcon,
    PickUpIcon,
    RogueHeita,
    SecretMissionIcon,
    ShopIcon,
    SpecialChatIcon,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct RogueOptionTalk {
    #[serde(rename = "TalkSentenceID")]
    pub talk_sentence_id: Option<NonZero<u32>>,
    #[serde(rename = "OptionTextmapID")]
    pub option_textmap_id: Option<crate::Text>,
    pub option_icon_type: Option<OptionIconType>,
    #[serde(rename = "RogueOptionID")]
    pub rogue_option_id: Option<NonZero<u32>>,
    pub trigger_custom_string: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct SimpleTalk {
    pub text_speed: Option<NonZero<u8>>,
    #[serde(rename = "TalkSentenceID")]
    pub talk_sentence_id: u32,
    pub protect_time: f32,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[serde(tag = "$type")]
pub enum OptionTalk {
    #[serde(rename = "RPG.GameCore.OptionTalkInfo", rename_all = "PascalCase")]
    OptionTalkInfo {
        #[serde(default)]
        delete_after_selection: bool,
        finish_key: Option<u8>,
        #[serde(default)]
        has_triggered: bool,
        option_icon_type: OptionIconType,
        #[serde(rename = "OptionTextmapID")]
        option_textmap_id: Option<Text>,
        submission_id: Option<NonZero<u32>>,
        #[serde(rename = "TalkEventID")]
        talk_event_id: Option<NonZero<u32>>,
        #[serde(rename = "TalkSentenceID")]
        talk_sentence_id: u32,
        trigger_custom_string: String,
    },
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum SimpleTalkBackground {
    None,
    CG {
        #[serde(rename = "CgID")]
        cg_id: u16,
    },
    #[serde(rename_all = "PascalCase")]
    Image {
        image_path: String,
    },
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct TargetBehavior {
    unique_name: String,
    #[serde(default)]
    use_mouth_talk: bool,
}
