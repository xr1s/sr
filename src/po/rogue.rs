use std::{num::NonZero, path::PathBuf};

use crate::{vo, GameData, ID, PO};

use super::{Text, Value};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum InBattleBindingType {
    StageAbilityBeforeCharacterBorn,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum BuffEffect {
    #[serde(rename = "")]
    None,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum MazeBuffType {
    Level,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
// 模拟宇宙祝福
pub(crate) struct RogueMazeBuff {
    #[serde(rename = "ID")]
    id: u32,
    buff_series: u8, // 目前值只有 1
    buff_rarity: u8, // 目前值只有 1
    lv: u8,
    lv_max: u8,
    modifier_name: String,
    in_battle_binding_type: InBattleBindingType,
    in_battle_binding_key: String,
    param_list: Vec<Value<f32>>,
    buff_icon: PathBuf,
    buff_name: Text,
    buff_desc: Text,
    buff_simple_desc: Text,
    buff_desc_battle: Text,
    buff_effect: BuffEffect,
    maze_buff_type: MazeBuffType,
}

impl ID for RogueMazeBuff {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.id
    }
}

impl<'a> PO<'a> for RogueMazeBuff {
    type VO = vo::rogue::RogueMazeBuff<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        let params = self
            .param_list
            .iter()
            .map(|v| crate::format::Argument::from(&v.value))
            .collect::<Vec<_>>();
        Self::VO {
            id: self.id,
            lv: self.lv,
            max_lv: self.lv_max,
            name: game.text(&self.buff_name),
            desc: crate::format::format(game.text(&self.buff_desc), &params),
            simple_desc: crate::format::format(game.text(&self.buff_simple_desc), &params),
            desc_battle: game.text(&self.buff_desc_battle),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
// 模拟宇宙奇物
pub(crate) struct RogueMiracle {
    #[serde(rename = "MiracleID")]
    miracle_id: u16,
    #[serde(rename = "MiracleDisplayID")]
    miracle_display_id: u16,
    #[serde(rename = "UnlockHandbookMiracleID")]
    unlock_handbook_miracle_id: Option<NonZero<u16>>,
    miracle_desc: Text,
    desc_param_list: Vec<()>, // 只有空 []
    extra_effect: Vec<u32>,   // 只有空 []
}

impl ID for RogueMiracle {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.miracle_id
    }
}

impl<'a> PO<'a> for RogueMiracle {
    type VO = vo::rogue::RogueMiracle<'a>;
    fn vo(&self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.miracle_id,
            display: game.rogue_miracle_display(self.miracle_display_id).unwrap(),
            desc: game.text(&self.miracle_desc),
            handbook: self
                .unlock_handbook_miracle_id
                .and_then(|id| game.rogue_handbook_miracle(id.get())),
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
    extra_effect: Vec<u32>, // 只有空 []
    #[serde(rename = "MiracleBGDesc")]
    miracle_bg_desc: Text,
    miracle_tag: Text,
    miracle_icon_path: PathBuf,
    miracle_figure_icon_path: PathBuf,
}

impl ID for RogueMiracleDisplay {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.miracle_display_id
    }
}

impl<'a> PO<'a> for RogueMiracleDisplay {
    type VO = vo::rogue::RogueMiracleDisplay<'a>;
    fn vo(&self, game: &'a GameData) -> Self::VO {
        let arguments = self
            .desc_param_list
            .iter()
            .map(|param| &param.value)
            .map(crate::format::Argument::from)
            .collect::<Vec<_>>();
        Self::VO {
            id: self.miracle_display_id,
            name: game.text(&self.miracle_name),
            desc: crate::format::format(game.text(&self.miracle_desc), &arguments),
            bg_desc: game.text(&self.miracle_bg_desc),
            tag: game.text(&self.miracle_tag),
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
            display: game.rogue_miracle_display(self.miracle_dispaly_id).unwrap(),
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
            title: game.text(&self.rogue_miracle_type_title),
        }
    }
}
