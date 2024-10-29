use super::{Text, Value};
use crate::{vo, GameData, ID, PO};

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
        let arguments = self
            .desc_param_list
            .iter()
            .map(|v| crate::format::Argument::from(&v.value))
            .collect::<Vec<_>>();
        Self::VO {
            id: self.extra_effect_id,
            name: game.text(&self.extra_effect_name),
            desc: crate::format::format(game.text(&self.extra_effect_desc), &arguments),
        }
    }
}

#[derive(Clone, Default, Debug)]
pub(crate) struct RewardData {
    reward_id: u32,
    item_ids: [u32; 6],
    counts: [u32; 6],
    levels: [u8; 6], // 只有 null 和 1
    ranks: [u8; 6],  // 只有 null 和 1
    hcoin: u16,
    is_special: bool,
}

impl ID for RewardData {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.reward_id
    }
}

impl<'a> PO<'a> for RewardData {
    type VO = vo::misc::RewardData<'a>;
    fn vo(&'a self, _game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.reward_id,
            item_ids: &self.item_ids,
            counts: &self.counts,
            levels: &self.levels,
            ranks: &self.ranks,
        }
    }
}

impl<'de> serde::Deserialize<'de> for RewardData {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[rustfmt::skip]
        const FIELDS: &[&str] = &[
            "RewardID",
            "Hcoin",
            "IsSpecial",
            "ItemID_1", "ItemID_2", "ItemID_3", "ItemID_4", "ItemID_5", "ItemID_6",
            "Count_1",  "Count_2",  "Count_3",  "Count_4",  "Count_5",  "Count_6",
            "Level_1",  "Level_2",  "Level_3",  "Level_4",  "Level_5",  "Level_6",
            "Rank_1",   "Rank_2",   "Rank_3",   "Rank_4",   "Rank_5",   "Rank_6",
        ];
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RewardData;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct RewardData")
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                use serde::de::Error;
                let mut data = RewardData::default();
                while let Some(key) = map.next_key::<std::borrow::Cow<'_, str>>()? {
                    macro_rules! set_value_from {
                        (@prim, $key:literal, $field:ident) => {
                            if key == $key {
                                if data.$field as u32 != 0 {
                                    return Err(Error::duplicate_field($key));
                                }
                                data.$field = map.next_value()?;
                                continue;
                            }
                        };
                        (@arr, $prefix:literal, $field:ident) => {
                            if let Some(index) = key.strip_prefix($prefix) {
                                let index = index
                                    .parse::<usize>()
                                    .map_err(|_| Error::unknown_field(key.as_ref(), FIELDS))?
                                    - 1;
                                if data.$field[index] != 0 {
                                    return Err(Error::duplicate_field($prefix));
                                }
                                data.$field[index] = map.next_value()?;
                                continue;
                            }
                        };
                    }
                    set_value_from!(@prim, "RewardID", reward_id);
                    set_value_from!(@prim, "Hcoin", hcoin);
                    set_value_from!(@prim, "IsSpecial", is_special);
                    set_value_from!(@arr, "ItemID_", item_ids);
                    set_value_from!(@arr, "Count_", counts);
                    set_value_from!(@arr, "Level_", levels);
                    set_value_from!(@arr, "Rank_", ranks);
                    return Err(Error::unknown_field(key.as_ref(), FIELDS));
                }
                if data.reward_id == 0 {
                    return Err(Error::missing_field("RewardID"));
                }
                Ok(data)
            }
        }
        deserializer.deserialize_struct("RewardData", FIELDS, Visitor)
    }
}
