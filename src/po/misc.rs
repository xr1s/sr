use super::{Text, Value};
use crate::vo;
use crate::{GameData, ID, PO};

use std::path::PathBuf;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct ExtraEffectConfig {
    #[serde(rename = "ExtraEffectID")]
    extra_effect_id: u32,
    extra_effect_name: Text,
    extra_effect_desc: Text,
    desc_param_list: Vec<Value<f32>>,
    extra_effect_icon_path: PathBuf,
    /// 目前只有 1, 2, 3，从对应的描述上看 1 3 都是开发用数据，不露出
    extra_effect_type: u8,
}

impl ID for ExtraEffectConfig {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.extra_effect_id
    }
}

impl<'a> PO<'a> for ExtraEffectConfig {
    type VO = vo::misc::ExtraEffectConfig<'a>;

    fn vo(&self, game: &'a GameData) -> Self::VO {
        let params = self
            .desc_param_list
            .iter()
            .map(|v| v.value)
            .collect::<Vec<_>>();
        Self::VO {
            id: self.extra_effect_id,
            name: game.text(&self.extra_effect_name),
            desc: crate::format::format(game.text(&self.extra_effect_desc), &params),
        }
    }
}
