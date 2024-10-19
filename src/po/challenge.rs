/// WIP
/// 逐光捡金，也就是俗称的深渊
/// 看文件前缀
/// Challenge:      混沌回忆
/// ChallengeStory: 虚构叙事
/// ChallengeBoss:  末日幻影
use std::{num::NonZero, path::PathBuf};

use super::{Element, Text};

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
/// 逐光捡金类型
pub enum ChallengeGroupType {
    /// 混沌回忆
    Boss,
    /// 混沌回忆
    Memory,
    /// 虚构叙事
    Story,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
/// 混沌回忆 单期配置
pub(crate) struct ChallengeGroupConfig {
    #[serde(rename = "GroupID")]
    group_id: u16,
    group_name: Text,
    #[serde(rename = "RewardLineGroupID")]
    reward_line_group_id: u8,
    #[serde(rename = "PreMissionID")]
    pre_mission_id: u32,
    #[serde(rename = "GlobalScheduleID")]
    global_schedule_id: Option<NonZero<u32>>,
    #[serde(rename = "ScheduleDataID")]
    schedule_data_id: u32,
    #[serde(rename = "MazeBuffID")]
    maze_buff_id: u32,
    #[serde(rename = "MapEntranceID")]
    map_entrance_id: u32,
    #[serde(rename = "MappingInfoID")]
    mapping_info_id: u16,
    #[serde(rename = "WorldID")]
    world_id: Option<NonZero<u16>>,
    back_ground_path: PathBuf,
    tab_pic_path: PathBuf,
    tab_pic_select_path: PathBuf,
    challenge_group_type: ChallengeGroupType,
    theme_pic_path: PathBuf,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
/// 混沌回忆 单层配置
pub(crate) struct ChallengeMazeConfig {
    #[serde(rename = "ID")]
    id: u16,
    name: Text,
    #[serde(rename = "GroupID")]
    group_id: u16,
    #[serde(rename = "MapEntranceID")]
    map_entrance_id: u32,
    #[serde(rename = "MapEntranceID2")]
    map_entrance_id_2: u32,
    pre_level: u8, // 目前只有 1
    #[serde(rename = "PreChallengeMazeID")]
    pre_challenge_maze_id: u16,
    floor: u8,
    #[serde(rename = "RewardID")]
    reward_id: u32,
    damage_type_1: Vec<Element>,
    damage_type_2: Vec<Element>,
    challenge_target_id: Vec<u16>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
/// 虚构叙事 单期配置
pub(crate) struct ChallengeStoryGroupConfig {
    #[serde(rename = "GroupID")]
    group_id: u16,
    group_name: Text,
    reward_line_group_id: u16,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
/// 虚构叙事 单层配置
pub(crate) struct ChallengeStoryMazeConfig {
    #[serde(rename = "EventIDList1")]
    /// StageConfig
    event_id_list_1: Vec<u32>,
    #[serde(rename = "EventIDList2")]
    /// StageConfig
    event_id_list_2: Vec<u32>,
}
