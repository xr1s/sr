use std::num::NonZero;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum PerformanceType {
    A,
    C,
    D,
    E,
    PlayVideo,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct CreateCharacter {
    pub character_unique_name: String,
    #[serde(rename = "AvatarID")]
    pub avatar_id: String,
    pub area_name: String,
    pub anchor_name: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct CaptureNPC {
    pub character_unique_name: Option<String>,
    #[serde(rename = "GroupID")]
    pub group_id: Option<NonZero<u8>>,
    #[serde(rename = "NpcID")]
    pub npc_id: Option<NonZero<u32>>,
}
