use crate::{vo, GameData, ID, PO};

use super::{Text, Value};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
// 不可知域奇物
pub(crate) struct RogueMagicMiracle {
    #[serde(rename = "MiracleID")]
    miracle_id: u16,
    #[serde(rename = "MiracleDisplayID")]
    miracle_display_id: u16,
    #[serde(rename = "UnlockHandbookMiracleID")]
    unlock_handbook_miracle_id: u16,
    miracle_desc: Text,
    desc_param_list: Vec<Value<f32>>,
    extra_effect: Vec<u32>, // 只有空 []
}

impl ID for RogueMagicMiracle {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.miracle_id
    }
}

impl<'a> PO<'a> for RogueMagicMiracle {
    type VO = vo::rogue_magic::RogueMagicMiracle<'a>;
    fn vo(&self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.miracle_id,
            display: game.rogue_miracle_display(self.miracle_display_id).unwrap(),
            desc: crate::format::format(game.text(&self.miracle_desc), &arguments),
            unlock_handbook: game
                .rogue_handbook_miracle(self.unlock_handbook_miracle_id)
                .unwrap(),
        }
    }
}
