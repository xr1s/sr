pub mod tourn;

use std::{collections::HashMap, num::NonZero, path::PathBuf};

use base::{MainSubID, Wiki, ID};

use super::{Text, Value};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum RogueBuffCategory {
    /// 一星祝福
    Common,
    /// 三星祝福
    Legendary,
    /// 二星祝福
    Rare,
}

impl Wiki for RogueBuffCategory {
    fn wiki(&self) -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(match self {
            Self::Common => "1星",
            Self::Legendary => "3星",
            Self::Rare => "2星",
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum BattleEventBuffType {
    BattleEventBuff,
    BattleEventBuffCross,
    BattleEventBuffEnhance,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
/// 模拟宇宙祝福
pub struct RogueBuff {
    #[serde(rename = "MazeBuffID")]
    pub maze_buff_id: u32,
    pub maze_buff_level: u8,
    pub rogue_buff_type: u8,
    pub rogue_buff_rarity: Option<NonZero<u8>>,
    pub rogue_buff_category: Option<RogueBuffCategory>,
    pub rogue_buff_tag: u32,
    #[serde(rename = "ExtraEffectIDList")]
    pub extra_effect_id_list: Vec<u32>,
    #[serde(rename = "AeonID")]
    pub aeon_id: Option<NonZero<u8>>,
    pub rogue_version: u8, // 目前仅有 1
    #[serde(rename = "UnlockIDList")]
    pub unlock_id_list: Vec<u32>,
    #[serde(default)]
    pub is_show: bool,
    pub battle_event_buff_type: Option<BattleEventBuffType>,
    #[serde(rename = "ActivityModuleID")]
    pub activity_module_id: Option<NonZero<u32>>,
    pub handbook_unlock_desc: Text,
    pub aeon_cross_icon: String,
}

impl MainSubID for RogueBuff {
    type ID = u32;
    type SubID = u8;
    fn id(&self) -> Self::ID {
        self.maze_buff_id
    }
    fn sub_id(&self) -> Self::SubID {
        self.maze_buff_level
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct RogueBuffType {
    pub rogue_buff_type: u8,
    #[serde(rename = "RogueBuffTypeTextmapID")]
    pub rogue_buff_type_textmap_id: Text,
    pub rogue_buff_type_icon: String,
    pub rogue_buff_type_title: Text,
    pub rugue_buff_type_reward_quest_list: Vec<u32>,
    pub rogue_buff_type_sub_title: Text,
    pub hint_desc: Text,
}

impl ID for RogueBuffType {
    type ID = u8;
    fn id(&self) -> Self::ID {
        self.rogue_buff_type
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum MonsterDropType {
    AreaDrop,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
// 模拟宇宙奇物
pub struct RogueMiracle {
    #[serde(rename = "MiracleID")]
    pub miracle_id: u16,
    #[serde(rename = "MiracleDisplayID")]
    pub miracle_display_id: Option<NonZero<u16>>, // 1.2 及之前无此字段
    #[serde(rename = "UnlockIDList")]
    /// 不太清楚是什么作用, 只有少数几个可能值, 大部分为空: [], [5], [6], [27], [1000021], [1000022]
    pub unlock_id_list: Option<Vec<u32>>, // 2.3 及之后无此字段
    // 似乎是无尽活动的字段，新版本都没了
    pub use_effect: Option<Text>, // 2.3 及之后无此字段
    #[serde(default)]
    /// 应该是展示在图鉴里的奇物
    pub is_show: bool, // 1.6 及之后无此字段
    /// 只有 106011 一个值
    pub miracle_reward: Option<NonZero<u32>>, // 1.6 及之后无此字段
    /// 被废弃的字段, 只有 1 一个值
    pub rogue_version: Option<NonZero<u8>>, // 1.2 及之后无此字段
    #[serde(rename = "UnlockHandbookMiracleID")]
    pub unlock_handbook_miracle_id: Option<NonZero<u16>>,
    // 后面几个 1.3 ~ 2.5 无字段的, 1.0 ~ 1.2 时候无 RogueMiracleDisplay.json, 全部塞在这个结构体里
    // 1.3 ~ 2.5 之后拆出 RogueMiracleDisplay.json 后便没有这个字段了, 2.6 之后不清楚什么情况
    pub miracle_name: Option<Text>,               // 1.3 及之后无此字段
    pub miracle_desc: Option<Text>,               // 1.3 ~ 2.5 无此字段, 2.6 及之后该字段始终为空
    pub desc_param_list: Option<Vec<Value<f32>>>, // 1.3 ~ 2.5 版本无此字段, 2.6 及之后该字段始终为空
    #[serde(rename = "MiracleBGDesc")]
    pub miracle_bg_desc: Option<Text>, // 1.3 ~ 2.5 无此字段, 2.6 及之后该字段始终为空
    pub miracle_tag: Option<Text>,                // 1.3 及之后无此字段
    pub miracle_icon_path: Option<String>,        // 1.3 及之后无此字段
    pub miracle_figure_icon_path: Option<String>, // 1.3 及之后无此字段
    pub extra_effect: Option<Vec<u32>>,           // 2.5 及之前无此字段, 2.6 及之后该字段始终为空
    #[serde(rename = "BrokenChangeMiracleID")]
    /// 损坏后会变成什么样, 目前看都是「乱七八糟的代码」系列奇物
    pub broken_change_miracle_id: Option<NonZero<u16>>, // 2.3 及之后无此字段
}

impl ID for RogueMiracle {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.miracle_id
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
// 模拟宇宙奇物展示数据（效果、背景故事等）
pub struct RogueMiracleDisplay {
    #[serde(rename = "MiracleDisplayID")]
    pub miracle_display_id: u16,
    pub miracle_name: Text,
    pub miracle_desc: Text,
    pub desc_param_list: Vec<Value<f32>>,
    pub extra_effect: Option<Vec<u32>>,
    #[serde(rename = "MiracleBGDesc")]
    pub miracle_bg_desc: Text,
    pub miracle_tag: Text,
    pub miracle_icon_path: String,
    pub miracle_figure_icon_path: String,
}

impl ID for RogueMiracleDisplay {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.miracle_display_id
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
// 模拟宇宙奇物图鉴信息（解锁奖励、在哪些 DLC 中出现等）
pub struct RogueHandbookMiracle {
    #[serde(rename = "MiracleHandbookID")]
    pub miracle_handbook_id: u16,
    pub miracle_reward: u32,
    pub miracle_type_list: Vec<u16>,
    #[serde(rename = "MiracleDisplayID")]
    pub miracle_dispaly_id: u16,
    pub order: u8,
    #[serde(rename = "MiracleIDForEffectDisplay")]
    pub miracle_id_for_effect_display: Option<NonZero<u16>>,
}

impl ID for RogueHandbookMiracle {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.miracle_handbook_id
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
// 模拟宇宙奇物图鉴所属 DLC
pub struct RogueHandbookMiracleType {
    pub rogue_handbook_miracle_type: u16,
    pub rogue_miracle_type_title: Text,
    pub type_icon: PathBuf,
    #[serde(rename = "ActivityModuleID")]
    pub activity_module_id: Option<NonZero<u32>>, // 作用不明
}

impl ID for RogueHandbookMiracleType {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.rogue_handbook_miracle_type
    }
}

#[serde_with::serde_as]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct RogueMonsterGroup {
    #[serde(rename = "RogueMonsterGroupID")]
    pub rogue_monster_group_id: u32,
    #[serde_as(as = "HashMap<_, _>")]
    pub rogue_monster_list_and_weight: Vec<(u32, u8)>,
}

impl ID for RogueMonsterGroup {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.rogue_monster_group_id
    }
}

#[serde_with::serde_as]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct RogueMonster {
    #[serde(rename = "RogueMonsterID")]
    pub rogue_monster_id: u32,
    #[serde(rename = "NpcMonsterID")]
    pub npc_monster_id: u32,
    #[serde(rename = "EventID")]
    pub event_id: u32, // 不明，不是 StageConfig.json
    pub monster_drop_type: Option<MonsterDropType>,
}

impl ID for RogueMonster {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.rogue_monster_id
    }
}
