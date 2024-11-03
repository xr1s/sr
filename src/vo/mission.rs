use std::num::NonZero;

use crate::po::mission::Type;

use super::misc::RewardData;

#[derive(Clone, Debug)]
pub struct MissionChapterConfig {
    pub id: u32,
    pub display_priority: u32,
}

#[derive(Clone, Debug)]
pub struct MainMission<'a> {
    pub id: u32,
    pub r#type: Type,
    pub display_priority: u32,
    pub name: &'a str,
    pub next_track_main_mission: Option<NonZero<u32>>, // 不能直接存自己，不然会递归
    pub track_weight: Option<NonZero<u8>>,
    pub reward: Option<RewardData<'a>>,
    pub display_reward: Option<RewardData<'a>>,
    pub chapter: Option<MissionChapterConfig>,
    pub sub_reward_list: Vec<RewardData<'a>>,
}

#[derive(Clone, Debug)]
pub struct SubMission<'a> {
    pub id: u32,
    pub target: &'a str,
    pub description: &'a str,
}
