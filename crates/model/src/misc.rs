use std::{num::NonZero, path::PathBuf};

use base::{MainSubID, ID};

use super::{Text, Value};

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct ExtraEffectConfig {
    #[serde(rename = "ExtraEffectID")]
    pub extra_effect_id: u32,
    pub extra_effect_name: Text,
    pub extra_effect_desc: Text,
    pub desc_param_list: Vec<Value<f32>>,
    pub extra_effect_icon_path: PathBuf,
    /// 目前只有 1, 2, 3，从对应的描述上看 1 3 都是开发用数据，不露出
    pub extra_effect_type: u8,
}

impl ID for ExtraEffectConfig {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.extra_effect_id
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum MazeBuffType {
    Assistant,
    Character,
    CharacterKeepScene,
    Level,
    LevelKeepScene,
    Team,
    TeamKeepScene,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum InBattleBindingType {
    CharacterAbility,
    CharacterSkill,
    StageAbilityAfterCharacterBorn,
    StageAbilityBeforeCharacterBorn,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum MazeBuffUseType {
    AddBattleBuff,
    Special,
    SummonUnit,
    TriggerBattle,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum MazeBuffIconType {
    Buff,
    Debuff,
    Other,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
// 模拟宇宙祝福
pub struct MazeBuff {
    #[serde(rename = "ID")]
    pub id: u32,
    pub buff_series: u8, // 目前值只有 1
    pub buff_rarity: u8, // 目前值只有 1
    pub lv: u8,
    pub lv_max: u8,
    pub modifier_name: String,
    pub in_battle_binding_type: Option<InBattleBindingType>,
    pub in_battle_binding_key: String,
    pub param_list: Vec<Value<f32>>,
    #[serde(rename = "BuffDescParamByAvatarSkillID")]
    pub buff_desc_param_by_avatar_skill_id: Option<NonZero<u32>>,
    pub buff_icon: PathBuf,
    pub buff_name: Text,
    pub buff_desc: Text,
    pub buff_simple_desc: Text,
    pub buff_desc_battle: Text,
    pub buff_effect: String,
    pub maze_buff_type: MazeBuffType,
    pub use_type: Option<MazeBuffUseType>, // 只在 1.6 及之前出现
    pub maze_buff_icon_type: Option<MazeBuffIconType>,
    pub maze_buff_pool: Option<NonZero<u8>>,
    #[serde(default)]
    pub is_display: bool,
    #[serde(default)]
    pub is_display_env_in_level: bool,
}

impl MainSubID for MazeBuff {
    type ID = u32;
    type SubID = u8;
    fn id(&self) -> Self::ID {
        self.id
    }
    fn sub_id(&self) -> Self::SubID {
        self.lv
    }
}

#[derive(Default)]
pub struct RewardData {
    pub reward_id: u32,
    pub item_ids: [u32; 6],
    pub counts: [u32; 6],
    pub levels: [u8; 6], // 只有 null 和 1
    pub ranks: [u8; 6],  // 只有 null 和 1
    /// 星琼
    pub hcoin: u16,
    pub is_special: bool,
}

impl ID for RewardData {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.reward_id
    }
}

impl RewardData {
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
}

impl<'de> serde::Deserialize<'de> for RewardData {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
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
                                let index = index.parse::<usize>().map_err(|_| {
                                    Error::unknown_field(key.as_ref(), RewardData::FIELDS)
                                })? - 1;
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
                    return Err(Error::unknown_field(key.as_ref(), RewardData::FIELDS));
                }
                if data.reward_id == 0 {
                    return Err(Error::missing_field("RewardID"));
                }
                Ok(data)
            }
        }
        deserializer.deserialize_struct("RewardData", RewardData::FIELDS, Visitor)
    }
}

impl serde::Serialize for RewardData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        for index in 0..6 {
            len += (self.item_ids[index] != 0) as usize;
            len += (self.counts[index] != 0) as usize;
            len += (self.levels[index] != 0) as usize;
            len += (self.ranks[index] != 0) as usize;
        }
        len += 1 + self.is_special as usize + (self.hcoin != 0) as usize;
        let mut state = serializer.serialize_struct("RewardData", len)?;
        state.serialize_field("RewardID", &self.reward_id)?;
        if self.is_special {
            state.serialize_field("IsSpecial", &self.is_special)?;
        }
        if self.hcoin != 0 {
            state.serialize_field("Hcoin", &self.hcoin)?;
        }
        for index in 0..6 {
            if self.item_ids[index] != 0 {
                state.serialize_field(Self::FIELDS[3 + index], &self.item_ids[index])?;
            }
            if self.counts[index] != 0 {
                state.serialize_field(Self::FIELDS[9 + index], &self.counts[index])?;
            }
            if self.levels[index] != 0 {
                state.serialize_field(Self::FIELDS[15 + index], &self.levels[index])?;
            }
            if self.ranks[index] != 0 {
                state.serialize_field(Self::FIELDS[21 + index], &self.ranks[index])?;
            }
        }
        state.end()
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct ScheduleData {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(with = "base::serde::datetime")]
    pub begin_time: chrono::DateTime<chrono::FixedOffset>,
    #[serde(with = "base::serde::datetime")]
    pub end_time: chrono::DateTime<chrono::FixedOffset>,
}

impl ID for ScheduleData {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.id
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct ScheduleDataGlobal {
    #[serde(flatten)]
    pub schedule: ScheduleData,
    #[serde(with = "base::serde::datetime")]
    pub global_end_time: chrono::DateTime<chrono::FixedOffset>,
}

impl ID for ScheduleDataGlobal {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.schedule.id
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum TextJoinType {
    AvatarID,
    CustomText,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct TextJoinConfig {
    #[serde(rename = "TextJoinID")]
    pub text_join_id: u8,
    pub default_item: u16,
    pub text_join_item_list: Vec<u16>,
    #[serde(default)]
    pub is_override: bool,
    pub r#type: Option<TextJoinType>,
}

impl ID for TextJoinConfig {
    type ID = u8;
    fn id(&self) -> Self::ID {
        self.text_join_id
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct TextJoinItem {
    #[serde(rename = "TextJoinItemID")]
    pub text_join_item_id: u16,
    pub text_join_text: Text,
}

impl ID for TextJoinItem {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.text_join_item_id
    }
}
