use std::num::NonZero;

use base::ID;

use super::Text;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct TalkSentenceConfig {
    #[serde(rename = "TalkSentenceID")]
    #[serde(default)]
    pub talk_sentence_id: u32,
    pub textmap_talk_sentence_name: Text,
    pub talk_sentence_text: Text,
    #[serde(rename = "VoiceID")]
    pub voice_id: Option<NonZero<u32>>,
}

impl ID for TalkSentenceConfig {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.talk_sentence_id
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
pub struct VoiceConfig {
    #[serde(rename = "VoiceID")]
    pub voice_id: u32,
    #[serde(default)]
    pub is_player_involved: bool,
    pub voice_path: String,
    pub voice_type: Option<VoiceType>,
}

impl ID for VoiceConfig {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.voice_id
    }
}
