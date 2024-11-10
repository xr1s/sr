use serde::ser::SerializeStruct;

use super::{Text, Value};
use crate::{vo, GameData, ID, PO};

use std::{num::NonZero, path::PathBuf};

#[derive(Clone, serde::Deserialize, serde::Serialize)]
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
            name: game.text(self.extra_effect_name),
            desc: crate::format::format(game.text(self.extra_effect_desc), &arguments),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum MazeBuffType {
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
pub enum MazeBuffIconType {
    Buff,
    Debuff,
    Other,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
// 模拟宇宙祝福
pub(crate) struct MazeBuff {
    #[serde(rename = "ID")]
    id: u32,
    buff_series: u8, // 目前值只有 1
    buff_rarity: u8, // 目前值只有 1
    lv: u8,
    lv_max: u8,
    modifier_name: String,
    in_battle_binding_type: Option<InBattleBindingType>,
    in_battle_binding_key: String,
    param_list: Vec<Value<f32>>,
    buff_icon: PathBuf,
    buff_name: Text,
    buff_desc: Text,
    buff_simple_desc: Text,
    buff_desc_battle: Text,
    buff_effect: String,
    maze_buff_type: MazeBuffType,
    maze_buff_icon_type: Option<MazeBuffIconType>,
    maze_buff_pool: Option<NonZero<u8>>,
    #[serde(default)]
    is_display: bool,
    #[serde(default)]
    is_display_env_in_level: bool,
}

impl ID for MazeBuff {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.id
    }
}

impl<'a> PO<'a> for MazeBuff {
    type VO = vo::misc::MazeBuff<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        let params = self
            .param_list
            .iter()
            .map(|v| crate::format::Argument::from(&v.value))
            .collect::<Vec<_>>();
        Self::VO {
            id: self.id,
            lv: self.lv,
            lv_max: self.lv_max,
            name: game.text(self.buff_name),
            desc: crate::format::format(game.text(self.buff_desc), &params),
            simple_desc: crate::format::format(game.text(self.buff_simple_desc), &params),
            desc_battle: crate::format::format(game.text(self.buff_desc_battle), &params),
        }
    }
}

#[derive(Default)]
pub(crate) struct RewardData {
    reward_id: u32,
    item_ids: [u32; 6],
    counts: [u32; 6],
    levels: [u8; 6], // 只有 null 和 1
    ranks: [u8; 6],  // 只有 null 和 1
    /// 星琼
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
            hcoin: self.hcoin,
            is_special: self.is_special,
        }
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

mod sched {
    use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeDelta};
    use serde::{self, de::Error, Deserialize, Deserializer, Serializer};

    const ASIA_SHANGHAI: FixedOffset = FixedOffset::east_opt(8 * 60 * 60).unwrap();
    const ASIA_SHANGHAI_OFFSET: TimeDelta = TimeDelta::hours(8);
    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&date.format(FORMAT).to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)
            .and_then(|s| NaiveDateTime::parse_from_str(&s, FORMAT).map_err(Error::custom))
            .map(|datetime| DateTime::from_naive_utc_and_offset(datetime, ASIA_SHANGHAI))
            .map(|datetime| datetime - ASIA_SHANGHAI_OFFSET)
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct ScheduleData {
    #[serde(rename = "ID")]
    id: u32,
    #[serde(with = "sched")]
    begin_time: chrono::DateTime<chrono::FixedOffset>,
    #[serde(with = "sched")]
    end_time: chrono::DateTime<chrono::FixedOffset>,
}

impl ID for ScheduleData {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.id
    }
}

impl PO<'_> for ScheduleData {
    type VO = vo::misc::ScheduleData;
    fn vo(&self, _game: &'_ GameData) -> Self::VO {
        Self::VO {
            id: self.id,
            begin_time: self.begin_time,
            end_time: self.end_time,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct ScheduleDataGlobal {
    #[serde(flatten)]
    schedule: ScheduleData,
    #[serde(with = "sched")]
    global_end_time: chrono::DateTime<chrono::FixedOffset>,
}

impl ID for ScheduleDataGlobal {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.schedule.id
    }
}

impl PO<'_> for ScheduleDataGlobal {
    type VO = vo::misc::ScheduleDataGlobal;
    fn vo(&self, _game: &'_ GameData) -> Self::VO {
        Self::VO {
            id: self.schedule.id,
            begin_time: self.schedule.begin_time,
            end_time: self.schedule.end_time,
            global_end_time: self.global_end_time,
        }
    }
}
