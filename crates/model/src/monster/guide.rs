use std::num::NonZero;

use base::ID;

use crate::Text;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct MonsterDifficultyGuide {
    #[serde(rename = "DifficultyGuideID")]
    pub difficulty_guide_id: u16,
    pub difficulty_guide_description: Text,
    #[serde(rename = "SkillID")]
    pub skill_id: Option<NonZero<u32>>,
    pub parameter_list: Vec<f32>,
}

impl ID for MonsterDifficultyGuide {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.difficulty_guide_id
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct MonsterGuideConfig {
    #[serde(rename = "MonsterID")]
    pub monster_id: u32,
    pub difficulty: u8, // 1, 2, 3, 4
    pub difficulty_list: Vec<u8>,
    pub tag_list: Vec<u32>,
    pub phase_list: Vec<u16>,
    pub brief_guide: Option<Text>,
    pub difficulty_guide_list: Vec<u16>,
    pub text_guide_list: Vec<u16>,
}

impl ID for MonsterGuideConfig {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.monster_id
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct MonsterGuidePhase {
    #[serde(rename = "PhaseID")]
    pub phase_id: u16,
    pub difficulty: u8,
    pub phase_pic: String,
    pub phase_name: Text,
    pub phase_answer: Text,
    pub phase_description: Text,
    pub skill_list: Vec<u32>,
}

impl ID for MonsterGuidePhase {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.phase_id
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum SkillType {
    Normal,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct MonsterGuideSkill {
    #[serde(rename = "SkillID")]
    pub skill_id: u32,
    pub difficulty: u8,
    pub r#type: SkillType,
    pub skill_name: Text,
    #[serde(rename = "SkillTextIDList")]
    pub skill_text_id_list: Vec<u32>,
    pub skill_answer: Option<Text>,
}

impl ID for MonsterGuideSkill {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.skill_id
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct MonsterGuideSkillText {
    #[serde(rename = "SkillTextID")]
    pub skill_text_id: u32,
    pub difficulty: u8,
    pub skill_description: Text,
    pub parameter_list: Vec<f32>,
    #[serde(rename = "EffectIDList")]
    pub effect_id_list: Vec<u32>,
}

impl ID for MonsterGuideSkillText {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.skill_text_id
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct MonsterGuideTag {
    #[serde(rename = "TagID")]
    pub tag_id: u32,
    pub tag_name: Text,
    pub tag_brief_description: Text,
    pub tag_detail_description: Option<Text>,
    pub parameter_list: Vec<f32>,
    #[serde(rename = "SkillID")]
    pub skill_id: Option<NonZero<u32>>,
    #[serde(rename = "EffectID")]
    pub effect_id: Vec<u32>,
}

impl ID for MonsterGuideTag {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.tag_id
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct MonsterTextGuide {
    #[serde(rename = "TextGuideID")]
    pub text_guide_id: u16,
    pub text_guide_description: Text,
    pub parameter_list: Vec<f32>,
}

impl ID for MonsterTextGuide {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.text_guide_id
    }
}
