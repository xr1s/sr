pub mod tourn;

use std::{collections::HashMap, num::NonZero, path::PathBuf};

use crate::{vo, GameData, ID, PO};

use super::{Text, Value};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum MonsterDropType {
    AreaDrop,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
// 模拟宇宙奇物
pub(crate) struct RogueMiracle {
    #[serde(rename = "MiracleID")]
    miracle_id: u16,
    #[serde(rename = "MiracleDisplayID")]
    miracle_display_id: Option<NonZero<u16>>, // 1.2 及之前无此字段
    #[serde(rename = "UnlockIDList")]
    /// 不太清楚是什么作用, 只有少数几个可能值, 大部分为空: [], [5], [6], [27], [1000021], [1000022]
    unlock_id_list: Option<Vec<u32>>, // 2.3 及之后无此字段
    // 似乎是无尽活动的字段，新版本都没了
    use_effect: Option<Text>, // 2.3 及之后无此字段
    #[serde(default)]
    /// 应该是展示在图鉴里的奇物
    is_show: bool, // 1.6 及之后无此字段
    /// 只有 106011 一个值
    miracle_reward: Option<NonZero<u32>>, // 1.6 及之后无此字段
    /// 被废弃的字段, 只有 1 一个值
    rogue_version: Option<NonZero<u8>>, // 1.2 及之后无此字段
    #[serde(rename = "UnlockHandbookMiracleID")]
    unlock_handbook_miracle_id: Option<NonZero<u16>>,
    // 后面几个 1.3 ~ 2.5 无字段的, 1.0 ~ 1.2 时候无 RogueMiracleDisplay.json, 全部塞在这个结构体里
    // 1.3 ~ 2.5 之后拆出 RogueMiracleDisplay.json 后便没有这个字段了, 2.6 之后不清楚什么情况
    miracle_name: Option<Text>,               // 1.3 及之后无此字段
    miracle_desc: Option<Text>,               // 1.3 ~ 2.5 无此字段, 2.6 及之后该字段始终为空
    desc_param_list: Option<Vec<Value<f32>>>, // 1.3 ~ 2.5 版本无此字段, 2.6 及之后该字段始终为空
    #[serde(rename = "MiracleBGDesc")]
    miracle_bg_desc: Option<Text>, // 1.3 ~ 2.5 无此字段, 2.6 及之后该字段始终为空
    miracle_tag: Option<Text>,                // 1.3 及之后无此字段
    miracle_icon_path: Option<String>,        // 1.3 及之后无此字段
    miracle_figure_icon_path: Option<String>, // 1.3 及之后无此字段
    extra_effect: Option<Vec<u32>>,           // 2.5 及之前无此字段, 2.6 及之后该字段始终为空
    #[serde(rename = "BrokenChangeMiracleID")]
    /// 损坏后会变成什么样, 目前看都是「乱七八糟的代码」系列奇物
    broken_change_miracle_id: Option<NonZero<u16>>, // 2.3 及之后无此字段
}

impl ID for RogueMiracle {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.miracle_id
    }
}

impl<'a> PO<'a> for RogueMiracle {
    type VO = vo::rogue::RogueMiracle<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        fn format_desc(
            game: &GameData,
            desc: &Option<Text>,
            desc_param_list: &Option<Vec<Value<f32>>>,
        ) -> String {
            let arguments = desc_param_list
                .as_deref()
                .unwrap_or_default()
                .iter()
                .map(|param| &param.value)
                .map(crate::format::Argument::from)
                .collect::<Vec<_>>();
            crate::format::format(
                desc.map(|text| game.text(text)).unwrap_or_default(),
                &arguments,
            )
        }
        Self::VO {
            id: self.miracle_id,
            // 存在一些奇物, 图鉴中展示的是模拟宇宙的效果, 游戏过程中展示的是差分宇宙的效果
            // 这一类奇物主要是差分宇宙新增的奇物和商店相关奇物 (邪恶机械卫星#900和「中等念头」群体机)
            display: if let Some(id) = self.miracle_display_id {
                None.or_else(|| game.rogue_miracle_display(id.get()))
                    .or_else(|| game.rogue_tourn_miracle_display(id.get()))
                    .unwrap()
            } else {
                vo::rogue::RogueMiracleDisplay {
                    id: 0,
                    name: self
                        .miracle_name
                        .map(|text| game.text(text))
                        .unwrap_or_default(),
                    desc: format_desc(game, &self.miracle_desc, &self.desc_param_list),
                    extra_effect: Vec::new(),
                    bg_desc: self
                        .miracle_bg_desc
                        .map(|text| game.text(text))
                        .unwrap_or_default(),
                    tag: self
                        .miracle_tag
                        .map(|text| game.text(text))
                        .unwrap_or_default(),
                    icon_path: self.miracle_icon_path.as_deref().unwrap_or_default(),
                    figure_icon_path: self.miracle_figure_icon_path.as_deref().unwrap_or_default(),
                }
            },
            desc: format_desc(game, &self.miracle_desc, &self.desc_param_list),
            unlock_handbook: self
                .unlock_handbook_miracle_id
                .map(NonZero::get)
                .map(|id| game.rogue_handbook_miracle(id))
                .map(Option::unwrap),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
// 模拟宇宙奇物展示数据（效果、背景故事等）
pub(crate) struct RogueMiracleDisplay {
    #[serde(rename = "MiracleDisplayID")]
    miracle_display_id: u16,
    miracle_name: Text,
    miracle_desc: Text,
    desc_param_list: Vec<Value<f32>>,
    extra_effect: Option<Vec<u32>>,
    #[serde(rename = "MiracleBGDesc")]
    miracle_bg_desc: Text,
    miracle_tag: Text,
    miracle_icon_path: String,
    miracle_figure_icon_path: String,
}

impl ID for RogueMiracleDisplay {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.miracle_display_id
    }
}

impl<'a> PO<'a> for RogueMiracleDisplay {
    type VO = vo::rogue::RogueMiracleDisplay<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        let arguments = self
            .desc_param_list
            .iter()
            .map(|param| &param.value)
            .map(crate::format::Argument::from)
            .collect::<Vec<_>>();
        Self::VO {
            id: self.miracle_display_id,
            name: game.text(self.miracle_name),
            desc: crate::format::format(game.text(self.miracle_desc), &arguments),
            extra_effect: self
                .extra_effect
                .as_deref()
                .unwrap_or_default()
                .iter()
                .map(|&id| game.extra_effect_config(id))
                .map(Option::unwrap)
                .collect(),
            bg_desc: game.text(self.miracle_bg_desc),
            tag: game.text(self.miracle_tag),
            icon_path: &self.miracle_icon_path,
            figure_icon_path: &self.miracle_figure_icon_path,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
// 模拟宇宙奇物图鉴信息（解锁奖励、在哪些 DLC 中出现等）
pub(crate) struct RogueHandbookMiracle {
    #[serde(rename = "MiracleHandbookID")]
    miracle_handbook_id: u16,
    miracle_reward: u32,
    miracle_type_list: Vec<u16>,
    #[serde(rename = "MiracleDisplayID")]
    miracle_dispaly_id: u16,
    order: u8,
    #[serde(rename = "MiracleIDForEffectDisplay")]
    miracle_id_for_effect_display: Option<NonZero<u16>>,
}

impl ID for RogueHandbookMiracle {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.miracle_handbook_id
    }
}

impl<'a> PO<'a> for RogueHandbookMiracle {
    type VO = vo::rogue::RogueHandbookMiracle<'a>;
    fn vo(&self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.miracle_handbook_id,
            reward: game.reward_data(self.miracle_reward).unwrap(),
            type_list: self
                .miracle_type_list
                .iter()
                .map(|&typ| game.rogue_handbook_miracle_type(typ))
                .map(Option::unwrap)
                .collect(),
            // 存在一些奇物, 图鉴中展示的是模拟宇宙的效果, 游戏过程中展示的是差分宇宙的效果
            // 这一类奇物主要是差分宇宙新增的奇物和商店相关奇物 (邪恶机械卫星#900和「中等念头」群体机)
            display: None
                .or_else(|| game.rogue_miracle_display(self.miracle_dispaly_id))
                .or_else(|| game.rogue_tourn_miracle_display(self.miracle_dispaly_id))
                .unwrap(),
            order: self.order,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
// 模拟宇宙奇物图鉴所属 DLC
pub(crate) struct RogueHandbookMiracleType {
    rogue_handbook_miracle_type: u16,
    rogue_miracle_type_title: Text,
    type_icon: PathBuf,
    #[serde(rename = "ActivityModuleID")]
    activity_module_id: Option<NonZero<u32>>, // 作用不明
}

impl ID for RogueHandbookMiracleType {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.rogue_handbook_miracle_type
    }
}

impl<'a> PO<'a> for RogueHandbookMiracleType {
    type VO = vo::rogue::RogueHandbookMiracleType<'a>;
    fn vo(&self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.rogue_handbook_miracle_type,
            title: game.text(self.rogue_miracle_type_title),
        }
    }
}

#[serde_with::serde_as]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct RogueMonsterGroup {
    #[serde(rename = "RogueMonsterGroupID")]
    rogue_monster_group_id: u32,
    #[serde_as(as = "HashMap<_, _>")]
    rogue_monster_list_and_weight: Vec<(u32, u8)>,
}

impl ID for RogueMonsterGroup {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.rogue_monster_group_id
    }
}

impl<'a> PO<'a> for RogueMonsterGroup {
    type VO = vo::rogue::RogueMonsterGroup<'a>;
    fn vo(&self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.rogue_monster_group_id,
            list_and_weight: self
                .rogue_monster_list_and_weight
                .iter()
                .map(|&(id, weight)| (game.rogue_monster(id).unwrap(), weight))
                .collect(),
        }
    }
}

#[serde_with::serde_as]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct RogueMonster {
    #[serde(rename = "RogueMonsterID")]
    rogue_monster_id: u32,
    #[serde(rename = "NpcMonsterID")]
    npc_monster_id: u32,
    #[serde(rename = "EventID")]
    event_id: u32, // 不明，不是 StageConfig.json
    monster_drop_type: Option<MonsterDropType>,
}

impl ID for RogueMonster {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.rogue_monster_id
    }
}

impl<'a> PO<'a> for RogueMonster {
    type VO = vo::rogue::RogueMonster<'a>;
    fn vo(&self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.rogue_monster_id,
            npc_monster: game.npc_monster_data(self.npc_monster_id).unwrap(),
        }
    }
}
