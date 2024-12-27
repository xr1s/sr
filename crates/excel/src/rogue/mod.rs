pub mod tourn;

use std::{borrow::Cow, num::NonZero};

use base::Name;

use crate::{ExcelOutput, FromModel};

#[derive(Clone, Debug)]
/// 模拟宇宙奇物
pub struct RogueMiracle<'a> {
    pub id: u16,
    /// 1.2 及之前版本的 display 为空
    pub display: RogueMiracleDisplay<'a>,
    pub desc: &'a str,
    pub desc_params: Vec<format::Argument<'a>>,
    /// 没有 unlock_handbook 的一般是可以同时携带多个、效果不同的奇物
    /// 如分裂咕咕钟、绝对失败处方
    pub unlock_handbook: Option<RogueHandbookMiracle<'a>>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for RogueMiracle<'a> {
    type Model = model::rogue::RogueMiracle;
    fn from_model(game: &'a Data, model: &'a Self::Model) -> Self {
        Self {
            id: model.miracle_id,
            // 存在一些奇物, 图鉴中展示的是模拟宇宙的效果, 游戏过程中展示的是差分宇宙的效果
            // 这一类奇物主要是差分宇宙新增的奇物和商店相关奇物 (邪恶机械卫星#900和「中等念头」群体机)
            display: if let Some(id) = model.miracle_display_id {
                None.or_else(|| game.rogue_miracle_display(id.get()))
                    .or_else(|| game.rogue_tourn_miracle_display(id.get()))
                    .unwrap()
            } else {
                RogueMiracleDisplay {
                    id: 0,
                    name: model
                        .miracle_name
                        .map(|text| game.text(text))
                        .unwrap_or_default(),
                    desc: model
                        .miracle_desc
                        .map(|hash| game.text(hash))
                        .unwrap_or_default(),
                    desc_params: model
                        .desc_param_list
                        .as_deref()
                        .map(format::Argument::from_array)
                        .unwrap_or_default(),
                    extra_effect: Vec::new(),
                    bg_desc: model
                        .miracle_bg_desc
                        .map(|text| game.text(text))
                        .unwrap_or_default(),
                    tag: model
                        .miracle_tag
                        .map(|text| game.text(text))
                        .unwrap_or_default(),
                    icon_path: model.miracle_icon_path.as_deref().unwrap_or_default(),
                    figure_icon_path: model
                        .miracle_figure_icon_path
                        .as_deref()
                        .unwrap_or_default(),
                }
            },
            desc: model
                .miracle_desc
                .map(|hash| game.text(hash))
                .unwrap_or_default(),
            desc_params: model
                .desc_param_list
                .as_deref()
                .map(format::Argument::from_array)
                .unwrap_or_default(),
            unlock_handbook: model
                .unlock_handbook_miracle_id
                .map(NonZero::get)
                .map(|id| game.rogue_handbook_miracle(id))
                .map(Option::unwrap),
        }
    }
}

impl Name for RogueMiracle<'_> {
    fn name(&self) -> &str {
        self.display.name
    }
    fn wiki_name(&self) -> Cow<'_, str> {
        Cow::Borrowed(self.name())
    }
}

#[derive(Clone, Debug)]
/// 模拟宇宙奇物展示数据（效果、背景故事等）
pub struct RogueMiracleDisplay<'a> {
    pub id: u16,
    /// 名称
    pub name: &'a str,
    /// 奇物效果
    pub desc: &'a str,
    pub desc_params: Vec<format::Argument<'a>>,
    /// 奇物效果中，带有下划线的特殊效果的详细介绍
    pub extra_effect: Vec<crate::misc::ExtraEffectConfig<'a>>,
    /// 背景故事
    pub bg_desc: &'a str,
    /// 无意义，目前只有空字符串
    pub tag: &'a str,
    // 图标
    pub icon_path: &'a str,
    pub figure_icon_path: &'a str,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for RogueMiracleDisplay<'a> {
    type Model = model::rogue::RogueMiracleDisplay;
    fn from_model(game: &'a Data, model: &'a Self::Model) -> Self {
        Self {
            id: model.miracle_display_id,
            name: game.text(model.miracle_name),
            desc: game.text(model.miracle_desc),
            desc_params: format::Argument::from_array(&model.desc_param_list),
            extra_effect: model
                .extra_effect
                .as_deref()
                .unwrap_or_default()
                .iter()
                .map(|&id| game.extra_effect_config(id))
                .map(Option::unwrap)
                .collect(),
            bg_desc: game.text(model.miracle_bg_desc),
            tag: game.text(model.miracle_tag),
            icon_path: &model.miracle_icon_path,
            figure_icon_path: &model.miracle_figure_icon_path,
        }
    }
}

#[derive(Clone, Debug)]
// 模拟宇宙奇物图鉴信息（解锁奖励、在哪些 DLC 中出现等）
pub struct RogueHandbookMiracle<'a> {
    pub id: u16,
    pub reward: crate::misc::RewardData<'a>,
    pub type_list: Vec<RogueHandbookMiracleType<'a>>,
    pub display: RogueMiracleDisplay<'a>,
    pub order: u8,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for RogueHandbookMiracle<'a> {
    type Model = model::rogue::RogueHandbookMiracle;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            id: model.miracle_handbook_id,
            reward: game.reward_data(model.miracle_reward).unwrap(),
            type_list: model
                .miracle_type_list
                .iter()
                .map(|&typ| game.rogue_handbook_miracle_type(typ))
                .map(Option::unwrap)
                .collect(),
            // 存在一些奇物, 图鉴中展示的是模拟宇宙的效果, 游戏过程中展示的是差分宇宙的效果
            // 这一类奇物主要是差分宇宙新增的奇物和商店相关奇物 (邪恶机械卫星#900和「中等念头」群体机)
            display: None
                .or_else(|| game.rogue_miracle_display(model.miracle_dispaly_id))
                .or_else(|| game.rogue_tourn_miracle_display(model.miracle_dispaly_id))
                .unwrap(),
            order: model.order,
        }
    }
}

#[derive(Clone, Debug)]
// 模拟宇宙奇物图鉴所属 DLC
pub struct RogueHandbookMiracleType<'a> {
    pub id: u16,
    pub title: &'a str,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for RogueHandbookMiracleType<'a> {
    type Model = model::rogue::RogueHandbookMiracleType;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            id: model.rogue_handbook_miracle_type,
            title: game.text(model.rogue_miracle_type_title),
        }
    }
}

#[derive(Clone, Debug)]
// 模拟宇宙一轮战斗的敌人，目前只用于差分宇宙周期演算 Boss
pub struct RogueMonsterGroup<'a> {
    pub id: u32,
    pub list_and_weight: Vec<(RogueMonster<'a>, u8)>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for RogueMonsterGroup<'a> {
    type Model = model::rogue::RogueMonsterGroup;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            id: model.rogue_monster_group_id,
            list_and_weight: model
                .rogue_monster_list_and_weight
                .iter()
                .map(|&(id, weight)| (game.rogue_monster(id).unwrap(), weight))
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RogueMonster<'a> {
    pub id: u32,
    pub npc_monster: crate::monster::NPCMonsterData<'a>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for RogueMonster<'a> {
    type Model = model::rogue::RogueMonster;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            id: model.rogue_monster_id,
            npc_monster: game.npc_monster_data(model.npc_monster_id).unwrap(),
        }
    }
}

impl Name for RogueMonster<'_> {
    fn name(&self) -> &str {
        self.npc_monster.name
    }
    fn wiki_name(&self) -> Cow<'_, str> {
        crate::monster::wiki_name(self.name())
    }
}
