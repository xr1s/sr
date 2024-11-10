// 末日幻影 Boss 注释

use crate::vo;

#[derive(Clone, Debug)]
pub struct Difficulty<'a> {
    pub id: u16,
    pub description: &'a str,
    pub skill: Option<vo::monster::SkillConfig<'a>>,
    pub parameter_list: &'a [f32],
}

#[derive(Clone, Debug)]
pub struct Config<'a> {
    pub id: u32,
    pub difficulty: u8,
    pub difficulty_list: &'a [u8],
    pub tag_list: Vec<Tag<'a>>,
    pub phase_list: Vec<Phase<'a>>,
    pub brief_guide: &'a str,
    pub difficulty_guide_list: Vec<Difficulty<'a>>,
    pub text_guide_list: Vec<Text<'a>>,
}

#[derive(Clone, Debug)]
pub struct Phase<'a> {
    pub id: u16,
    pub difficulty: u8,
    pub name: &'a str,
    pub answer: &'a str,
    pub description: &'a str,
    pub skill_list: Vec<Skill<'a>>,
}

#[derive(Clone, Debug)]
pub struct Skill<'a> {
    pub id: u32,
    pub difficulty: u8,
    pub name: &'a str,
    pub text_list: Vec<SkillText<'a>>,
    pub answer: &'a str,
}

#[derive(Clone, Debug)]
pub struct SkillText<'a> {
    pub id: u32,
    pub difficulty: u8,
    pub description: &'a str,
    pub effect_list: Vec<vo::misc::ExtraEffectConfig<'a>>,
}

#[derive(Clone, Debug)]
pub struct Tag<'a> {
    pub id: u32,
    pub name: &'a str,
    pub brief_description: String,
    pub detail_description: String,
    pub skill: Option<vo::monster::SkillConfig<'a>>,
    pub effect: Vec<vo::misc::ExtraEffectConfig<'a>>,
}

#[derive(Clone, Debug)]
pub struct Text<'a> {
    pub id: u16,
    pub description: &'a str,
    pub parameter_list: &'a [f32],
}
