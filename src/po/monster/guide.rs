use std::num::NonZero;

use crate::{po::Text as TextHash, vo, GameData, ID, PO};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct Difficulty {
    #[serde(rename = "DifficultyGuideID")]
    difficulty_guide_id: u16,
    difficulty_guide_description: TextHash,
    #[serde(rename = "SkillID")]
    skill_id: Option<NonZero<u32>>,
    parameter_list: Vec<f32>,
}

impl ID for Difficulty {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.difficulty_guide_id
    }
}

impl<'a> PO<'a> for Difficulty {
    type VO = vo::monster::guide::Difficulty<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.difficulty_guide_id,
            description: game.text(self.difficulty_guide_description),
            skill: self
                .skill_id
                .map(NonZero::get)
                .map(|id| game.monster_skill_config(id))
                .map(Option::unwrap),
            parameter_list: &self.parameter_list,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct Config {
    #[serde(rename = "MonsterID")]
    monster_id: u32,
    difficulty: u8, // 1, 2, 3, 4
    difficulty_list: Vec<u8>,
    tag_list: Vec<u32>,
    phase_list: Vec<u16>,
    brief_guide: TextHash,
    difficulty_guide_list: Vec<u16>,
    text_guide_list: Vec<u16>,
}

impl ID for Config {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.monster_id
    }
}

impl<'a> PO<'a> for Config {
    type VO = vo::monster::guide::Config<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.monster_id,
            difficulty: self.difficulty,
            difficulty_list: &self.difficulty_list,
            tag_list: self
                .tag_list
                .iter()
                .map(|&id| game.monster_guide_tag(id))
                .map(Option::unwrap)
                .collect(),
            phase_list: self
                .phase_list
                .iter()
                .map(|&id| game.monster_guide_phase(id))
                .map(Option::unwrap)
                .collect(),
            brief_guide: game.text(self.brief_guide),
            difficulty_guide_list: self
                .difficulty_guide_list
                .iter()
                .map(|&id| game.monster_difficulty_guide(id))
                .map(Option::unwrap)
                .collect(),
            text_guide_list: self
                .text_guide_list
                .iter()
                .map(|&id| game.monster_text_guide(id))
                .map(Option::unwrap)
                .collect(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct Phase {
    #[serde(rename = "PhaseID")]
    phase_id: u16,
    difficulty: u8,
    phase_pic: String,
    phase_name: TextHash,
    phase_answer: TextHash,
    phase_description: TextHash,
    skill_list: Vec<u32>,
}

impl ID for Phase {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.phase_id
    }
}

impl<'a> PO<'a> for Phase {
    type VO = vo::monster::guide::Phase<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.phase_id,
            difficulty: self.difficulty,
            name: game.text(self.phase_name),
            answer: game.text(self.phase_answer),
            description: game.text(self.phase_description),
            skill_list: self
                .skill_list
                .iter()
                .map(|&id| game.monster_guide_skill(id))
                .map(Option::unwrap)
                .collect(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum SkillType {
    Normal,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct Skill {
    #[serde(rename = "SkillID")]
    skill_id: u32,
    difficulty: u8,
    r#type: SkillType,
    skill_name: TextHash,
    #[serde(rename = "SkillTextIDList")]
    skill_text_id_list: Vec<u32>,
    skill_answer: TextHash,
}

impl ID for Skill {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.skill_id
    }
}

impl<'a> PO<'a> for Skill {
    type VO = vo::monster::guide::Skill<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.skill_id,
            difficulty: self.difficulty,
            name: game.text(self.skill_name),
            text_list: self
                .skill_text_id_list
                .iter()
                .map(|&id| game.monster_guide_skill_text(id))
                .map(Option::unwrap)
                .collect(),
            answer: game.text(self.skill_answer),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct SkillText {
    #[serde(rename = "SkillTextID")]
    skill_text_id: u32,
    difficulty: u8,
    skill_description: TextHash,
    parameter_list: Vec<f32>,
    #[serde(rename = "EffectIDList")]
    effect_id_list: Vec<u32>,
}

impl ID for SkillText {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.skill_text_id
    }
}

impl<'a> PO<'a> for SkillText {
    type VO = vo::monster::guide::SkillText<'a>;
    fn vo(&self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.skill_text_id,
            difficulty: self.difficulty,
            description: game.text(self.skill_description),
            effect_list: self
                .effect_id_list
                .iter()
                .map(|&id| game.extra_effect_config(id))
                .map(Option::unwrap)
                .collect(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct Tag {
    #[serde(rename = "TagID")]
    tag_id: u32,
    tag_name: TextHash,
    tag_brief_description: TextHash,
    tag_detail_description: TextHash,
    parameter_list: Vec<f32>,
    #[serde(rename = "SkillID")]
    skill_id: Option<NonZero<u32>>,
    #[serde(rename = "EffectID")]
    effect_id: Vec<u32>,
}

impl ID for Tag {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.tag_id
    }
}

impl<'a> PO<'a> for Tag {
    type VO = vo::monster::guide::Tag<'a>;
    fn vo(&self, game: &'a GameData) -> Self::VO {
        use crate::format::format;
        let arguments = self
            .parameter_list
            .iter()
            .map(crate::format::Argument::from)
            .collect::<Vec<_>>();
        Self::VO {
            id: self.tag_id,
            name: game.text(self.tag_name),
            brief_description: format(game.text(self.tag_brief_description), &arguments),
            detail_description: format(game.text(self.tag_detail_description), &arguments),
            skill: self
                .skill_id
                .map(NonZero::get)
                .map(|id| game.monster_skill_config(id))
                .map(Option::unwrap),
            effect: self
                .effect_id
                .iter()
                .map(|&id| game.extra_effect_config(id))
                .map(Option::unwrap)
                .collect(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct Text {
    #[serde(rename = "TextGuideID")]
    text_guide_id: u16,
    text_guide_description: TextHash,
    parameter_list: Vec<f32>,
}

impl ID for Text {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.text_guide_id
    }
}

impl<'a> PO<'a> for Text {
    type VO = vo::monster::guide::Text<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.text_guide_id,
            description: game.text(self.text_guide_description),
            parameter_list: &self.parameter_list,
        }
    }
}
