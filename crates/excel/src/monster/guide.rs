// 末日幻影 Boss 注释

use std::num::NonZero;

use crate::{ExcelOutput, FromModel};

#[derive(Clone, Debug)]
pub struct MonsterDifficultyGuide<'a> {
    pub id: u16,
    pub description: &'a str,
    pub skill: Option<crate::monster::MonsterSkillConfig<'a>>,
    pub parameter_list: &'a [f32],
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for MonsterDifficultyGuide<'a> {
    type Model = model::monster::guide::MonsterDifficultyGuide;
    fn from_model(game: &'a Data, model: &'a Self::Model) -> Self {
        Self {
            id: model.difficulty_guide_id,
            description: game.text(model.difficulty_guide_description),
            skill: model
                .skill_id
                .map(NonZero::get)
                .map(|id| game.monster_skill_config(id))
                .map(Option::unwrap),
            parameter_list: &model.parameter_list,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MonsterGuideConfig<'a> {
    pub id: u32,
    pub difficulty: u8,
    pub difficulty_list: &'a [u8],
    pub tag_list: Vec<MonsterGuideTag<'a>>,
    pub phase_list: Vec<MonsterGuidePhase<'a>>,
    pub brief_guide: &'a str,
    pub difficulty_guide_list: Vec<MonsterDifficultyGuide<'a>>,
    pub text_guide_list: Vec<MonsterTextGuide<'a>>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for MonsterGuideConfig<'a> {
    type Model = model::monster::guide::MonsterGuideConfig;
    fn from_model(game: &'a Data, model: &'a Self::Model) -> Self {
        Self {
            id: model.monster_id,
            difficulty: model.difficulty,
            difficulty_list: &model.difficulty_list,
            tag_list: model
                .tag_list
                .iter()
                .map(|&id| game.monster_guide_tag(id))
                .map(Option::unwrap)
                .collect(),
            phase_list: model
                .phase_list
                .iter()
                .map(|&id| game.monster_guide_phase(id))
                .map(Option::unwrap)
                .collect(),
            brief_guide: model
                .brief_guide
                .map(|hash| game.text(hash))
                .unwrap_or_default(),
            difficulty_guide_list: model
                .difficulty_guide_list
                .iter()
                .map(|&id| game.monster_difficulty_guide(id))
                .map(Option::unwrap)
                .collect(),
            text_guide_list: model
                .text_guide_list
                .iter()
                .map(|&id| game.monster_text_guide(id))
                .map(Option::unwrap)
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MonsterGuidePhase<'a> {
    pub id: u16,
    pub difficulty: u8,
    pub name: &'a str,
    pub answer: &'a str,
    pub description: &'a str,
    pub skill_list: Vec<MonsterGuideSkill<'a>>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for MonsterGuidePhase<'a> {
    type Model = model::monster::guide::MonsterGuidePhase;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            id: model.phase_id,
            difficulty: model.difficulty,
            name: game.text(model.phase_name),
            answer: game.text(model.phase_answer),
            description: game.text(model.phase_description),
            skill_list: model
                .skill_list
                .iter()
                .map(|&id| game.monster_guide_skill(id))
                .map(Option::unwrap)
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MonsterGuideSkill<'a> {
    pub id: u32,
    pub difficulty: u8,
    pub name: &'a str,
    pub text_list: Vec<MonsterGuideSkillText<'a>>,
    pub answer: &'a str,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for MonsterGuideSkill<'a> {
    type Model = model::monster::guide::MonsterGuideSkill;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            id: model.skill_id,
            difficulty: model.difficulty,
            name: game.text(model.skill_name),
            text_list: model
                .skill_text_id_list
                .iter()
                .map(|&id| game.monster_guide_skill_text(id))
                .map(Option::unwrap)
                .collect(),
            answer: model
                .skill_answer
                .map(|hash| game.text(hash))
                .unwrap_or_default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MonsterGuideSkillText<'a> {
    pub id: u32,
    pub difficulty: u8,
    pub description: &'a str,
    pub effect_list: Vec<crate::misc::ExtraEffectConfig<'a>>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for MonsterGuideSkillText<'a> {
    type Model = model::monster::guide::MonsterGuideSkillText;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            id: model.skill_text_id,
            difficulty: model.difficulty,
            description: game.text(model.skill_description),
            effect_list: model
                .effect_id_list
                .iter()
                .map(|&id| game.extra_effect_config(id))
                .map(Option::unwrap)
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MonsterGuideTag<'a> {
    pub id: u32,
    pub name: &'a str,
    pub brief_description: &'a str,
    pub detail_description: &'a str,
    pub parameter_list: Vec<format::Argument<'a>>,
    pub skill: Option<crate::monster::MonsterSkillConfig<'a>>,
    pub effect: Vec<crate::misc::ExtraEffectConfig<'a>>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for MonsterGuideTag<'a> {
    type Model = model::monster::guide::MonsterGuideTag;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            id: model.tag_id,
            name: game.text(model.tag_name),
            brief_description: game.text(model.tag_brief_description),
            detail_description: model
                .tag_detail_description
                .map(|hash| game.text(hash))
                .unwrap_or_default(),
            parameter_list: format::Argument::from_array(&model.parameter_list),
            skill: model
                .skill_id
                .map(NonZero::get)
                .map(|id| game.monster_skill_config(id))
                .map(Option::unwrap),
            effect: model
                .effect_id
                .iter()
                .map(|&id| game.extra_effect_config(id))
                .map(Option::unwrap)
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MonsterTextGuide<'a> {
    pub id: u16,
    pub description: &'a str,
    pub parameter_list: &'a [f32],
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for MonsterTextGuide<'a> {
    type Model = model::monster::guide::MonsterTextGuide;
    fn from_model(game: &'a Data, model: &'a Self::Model) -> Self {
        Self {
            id: model.text_guide_id,
            description: game.text(model.text_guide_description),
            parameter_list: &model.parameter_list,
        }
    }
}
