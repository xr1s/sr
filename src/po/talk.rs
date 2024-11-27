use std::num::NonZero;

use crate::{vo, GameData, ID, PO};

use super::Text;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct TalkSentenceConfig {
    #[serde(rename = "TalkSentenceID")]
    #[serde(default)]
    talk_sentence_id: u32,
    textmap_talk_sentence_name: Text,
    talk_sentence_text: Text,
    #[serde(rename = "VoiceID")]
    voice_id: Option<NonZero<u32>>,
}

impl ID for TalkSentenceConfig {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.talk_sentence_id
    }
}

impl<'a> PO<'a> for TalkSentenceConfig {
    type VO = vo::talk::TalkSentenceConfig<'a>;
    fn vo(&self, game: &'a GameData) -> Self::VO {
        const MISSING_VOICE: &[u32] = &[
            100001305, 102120714, 103159006, 103250417, 103250419, 103250421, 103250624, 103250626,
            103250628, 103250709, 103250710, 103250730, 103250732, 103251006, 103301205, 103301207,
            103301209, 103301211, 103329901, 103329902, 200031436, 201071601, 201071602, 201071603,
            201331904, 500180916,
        ];
        Self::VO {
            id: self.talk_sentence_id,
            name: game.text(self.textmap_talk_sentence_name),
            text: game.text(self.talk_sentence_text),
            voice: self
                .voice_id
                .map(NonZero::get)
                .filter(|id| !MISSING_VOICE.contains(id)) // 疑似缺数据
                .map(|id| game.voice_config(id))
                .map(Option::unwrap),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum VoiceType {
    Archive,
    BroadcastFar,
    BroadcastNear,
    BroadcastNormal,
    BroadcastUltraFar3,
    Cutscene,
    #[serde(rename = "MissionTalk_3d")]
    MissionTalk3d,
    #[serde(rename = "NPC_Far")]
    NPCFar,
    #[serde(rename = "NPC_Far_NoDuck")]
    NPCFarNoDuck,
    #[serde(rename = "NPC_Near")]
    NPCNear,
    #[serde(rename = "NPC_Normal")]
    NPCNormal,
    #[serde(rename = "NPC_Normal_NoDuck")]
    NPCNormalNoDuck,
    StoryNew,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct VoiceConfig {
    #[serde(rename = "VoiceID")]
    voice_id: u32,
    #[serde(default)]
    is_player_involved: bool,
    voice_path: String,
    voice_type: Option<VoiceType>,
}

impl ID for VoiceConfig {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.voice_id
    }
}

impl PO<'_> for VoiceConfig {
    type VO = vo::talk::VoiceConfig;
    fn vo(&self, _game: &GameData) -> Self::VO {
        Self::VO {
            id: self.voice_id,
            is_player_involved: self.is_player_involved,
            r#type: self.voice_type,
        }
    }
}
