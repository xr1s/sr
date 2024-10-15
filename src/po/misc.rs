use super::{Text, Value};
use crate::vo;
use crate::{GameData, ID, PO};

use std::path::PathBuf;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct ExtraEffect {
    #[serde(rename = "ExtraEffectID")]
    extra_effect_id: u32,
    extra_effect_name: Text,
    extra_effect_desc: Text,
    desc_param_list: Vec<Value<f32>>,
    extra_effect_icon_path: PathBuf,
    extra_effect_type: u8,
}

impl ID for ExtraEffect {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.extra_effect_id
    }
}

impl<'a> PO<'a> for ExtraEffect {
    type VO = vo::misc::ExtraEffect<'a>;

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
            r#type: self.extra_effect_type,
        }
    }
}
