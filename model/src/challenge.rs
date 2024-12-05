/// WIP
/// 逐光捡金，也就是俗称的深渊
/// 看文件前缀
/// Challenge:      混沌回忆
/// ChallengeStory: 虚构叙事
/// ChallengeBoss:  末日幻影
use std::num::NonZero;

use base::{MainSubID, ID};

use super::{Element, Text};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
/// 逐光捡金类型
pub enum ChallengeGroupType {
    /// 混沌回忆
    Memory,
    /// 虚构叙事
    Story,
    /// 末日幻影
    Boss,
}

impl Default for ChallengeGroupType {
    fn default() -> Self {
        Self::Memory
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
/// 单期配置
pub struct ChallengeGroupConfig {
    #[serde(rename = "GroupID")]
    pub group_id: u16,
    pub group_name: Text,
    #[serde(rename = "RewardLineGroupID")]
    pub reward_line_group_id: u16,
    #[serde(rename = "PreMissionID")]
    pub pre_mission_id: u32,
    #[serde(rename = "GlobalScheduleID")]
    pub global_schedule_id: Option<NonZero<u32>>,
    #[serde(rename = "ScheduleDataID")]
    pub schedule_data_id: Option<NonZero<u32>>,
    #[serde(rename = "MazeBuffID")]
    pub maze_buff_id: Option<NonZero<u32>>,
    #[serde(rename = "MapEntranceID")]
    pub map_entrance_id: Option<NonZero<u32>>,
    #[serde(rename = "MappingInfoID")]
    pub mapping_info_id: Option<NonZero<u32>>,
    #[serde(rename = "WorldID")]
    pub world_id: Option<NonZero<u16>>,
    pub back_ground_path: Option<String>,    // 1.2 及之后
    pub tab_pic_path: Option<String>,        // 1.2 及之后
    pub tab_pic_select_path: Option<String>, // 1.2 及之后
    #[serde(default)]
    pub challenge_group_type: ChallengeGroupType, // 1.5 及之前虚构叙事出现前，无此字段
    pub theme_pic_path: Option<String>,      // 1.5 及之前没有该字段
}

impl ID for ChallengeGroupConfig {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.group_id
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum BossPattern {
    SmallAndLarge,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum ChallengeStoryType {
    /// 1.6 版本首期到 2.6 版本的虚构叙事
    /// 多波次怪物和增援，利好群攻多动角色
    /// 按照之前的经验，每 3 期会轮换分别给终结技、DoT、追击类角色增益
    Normal,
    /// 2.7 新版虚构叙事
    /// 每一轮都有一个首领，击败首领直接获得所有分数
    /// 击败小怪获得分数，并且首领扣除一定生命
    /// 数据保证首领生命肯定在清空小怪前被扣除完
    Fever,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
/// 单期配置
pub struct ChallengeGroupExtra {
    #[serde(rename = "GroupID")]
    pub group_id: u16,
    pub theme_poster_bg_pic_path: String,
    // 以下 4 个只在虚构叙事和末日幻影中出现
    pub theme_toast_pic_path: Option<String>,
    pub theme_icon_pic_path: Option<String>,
    pub theme_poster_effect_prefab_path: Option<String>,
    pub theme_poster_tab_pic_path: Option<String>,
    // 以下 2 个只在虚构叙事中出现
    pub buff_list: Option<[u32; 3]>,
    #[serde(rename = "ThemeID")]
    pub theme_id: Option<NonZero<u8>>,
    pub story_type: Option<ChallengeStoryType>,
    pub sub_maze_buff_list: Option<Vec<u32>>,
    // 以下 6 个只在末日幻影中出现
    pub buff_list_1: Option<[u32; 3]>,
    pub buff_list_2: Option<[u32; 3]>,
    pub boss_pattern: Option<BossPattern>, // 2.4 之后该字段消失
    pub boss_pattern_prefab_path: Option<String>,
    pub boss_position_prefab_path_1: Option<String>,
    pub boss_position_prefab_path_2: Option<String>,
}

impl ID for ChallengeGroupExtra {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.group_id
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
/// 单层配置
pub struct ChallengeMazeConfig {
    #[serde(rename = "ID")]
    pub id: u16,
    pub name: Text,
    #[serde(rename = "GroupID")]
    pub group_id: u16,
    #[serde(rename = "MapEntranceID")]
    pub map_entrance_id: u32,
    #[serde(rename = "MapEntranceID2")]
    pub map_entrance_id_2: Option<NonZero<u32>>,
    pub pre_level: Option<NonZero<u8>>, // 目前只有 1
    #[serde(rename = "PreChallengeMazeID")]
    pub pre_challenge_maze_id: Option<NonZero<u16>>,
    pub floor: Option<NonZero<u8>>,
    #[serde(rename = "RewardID")]
    pub reward_id: u32,
    pub damage_type_1: Vec<Element>,
    pub damage_type_2: Vec<Element>,
    #[serde(rename = "ChallengeTargetID")]
    pub challenge_target_id: [u16; 3],
    pub stage_num: u8,
    #[serde(rename = "MonsterID1")]
    pub monster_id_1: Vec<u32>,
    #[serde(rename = "MonsterID2")]
    pub monster_id_2: Vec<u32>,
    pub challenge_count_down: Option<NonZero<u8>>,
    #[serde(rename = "MazeGroupID1")]
    pub maze_group_id_1: u8,
    pub config_list_1: Vec<u32>,
    #[serde(rename = "NpcMonsterIDList1")]
    pub npc_monster_id_list_1: Vec<u32>,
    #[serde(rename = "EventIDList1")]
    pub event_id_list_1: Vec<u32>,
    #[serde(rename = "MazeGroupID2")]
    /// 不明，但是 Option 可能是因为混沌回忆（入门级）没有下半场
    pub maze_group_id_2: Option<NonZero<u8>>,
    /// 不明，数量都很小
    pub config_list_2: Vec<u32>,
    #[serde(rename = "NpcMonsterIDList2")]
    pub npc_monster_id_list_2: Vec<u32>,
    #[serde(rename = "EventIDList2")]
    pub event_id_list_2: Vec<u32>,
    #[serde(rename = "MazeBuffID")]
    pub maze_buff_id: u32,
}

impl ID for ChallengeMazeConfig {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.id
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct ChallengeMazeExtra {
    #[serde(rename = "ID")]
    pub id: u16,
    // 虚构叙事
    pub turn_limit: Option<NonZero<u8>>,
    #[serde(rename = "BattleTargetID")]
    pub battle_target_id: Option<Vec<u16>>, // TODO: BattleTargetConfig.json
    pub clear_score: Option<NonZero<u16>>,
    // 末日幻影
    #[serde(rename = "MonsterID1")]
    pub monster_id_1: Option<NonZero<u32>>,
    #[serde(rename = "MonsterID2")]
    pub monster_id_2: Option<NonZero<u32>>,
}

impl ID for ChallengeMazeExtra {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.id
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
/// 每三星获得的奖励
pub struct RewardLine {
    #[serde(rename = "GroupID")]
    pub group_id: u16,
    pub star_count: u8,
    #[serde(rename = "RewardID")]
    pub reward_id: u32,
}

impl MainSubID for RewardLine {
    type ID = u16;
    type SubID = u8;
    fn id(&self) -> Self::ID {
        self.group_id
    }
    fn sub_id(&self) -> Self::SubID {
        self.star_count
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// 分别对应三种挑战胜利条件
pub enum ChallengeTargetType {
    DeadAvatar,
    RoundsLeft,
    TotalScore,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct TargetConfig {
    #[serde(rename = "ID")]
    pub id: u16,
    pub challenge_target_type: ChallengeTargetType,
    pub challenge_target_name: Text,
    pub challenge_target_param_1: Option<NonZero<u16>>,
    #[serde(rename = "RewardID")]
    pub reward_id: Option<NonZero<u32>>,
}

impl ID for TargetConfig {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.id
    }
}
