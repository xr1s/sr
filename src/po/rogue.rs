use std::path::PathBuf;

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
        let params = self.param_list.iter().map(|v| v.value).collect::<Vec<_>>();
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
