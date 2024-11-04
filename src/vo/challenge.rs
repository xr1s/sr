use crate::{
    po::{
        challenge::{GroupType, TargetType},
        Element,
    },
    vo,
};

use super::map::WorldDataConfig;

#[derive(Clone, Debug)]
pub struct GroupConfig<'a> {
    pub id: u16,
    pub name: &'a str,
    pub reward_line_group: Vec<RewardLine<'a>>,
    pub pre_mission: vo::mission::MainMission<'a>,
    pub global_schedule: Option<vo::misc::ScheduleDataGlobal>,
    pub schedule_data: Option<vo::misc::ScheduleData>,
    pub maze_buff: Option<vo::misc::MazeBuff<'a>>,
    pub map_entrance: Option<vo::map::MapEntrance<'a>>,
    pub mapping_info: Option<vo::map::MappingInfo<'a>>,
    pub world: Option<WorldDataConfig<'a>>,
    pub r#type: GroupType,
}

#[derive(Clone, Debug)]
pub struct MazeConfig<'a> {
    pub id: u16,
    pub name: &'a str,
    pub group: GroupConfig<'a>,
    pub map_entrance: vo::map::MapEntrance<'a>,
    pub map_entrance_2: vo::map::MapEntrance<'a>,
    pub pre_level: u8,
    pub pre_challenge_maze_id: u16,
    pub floor: u8,
    pub reward: vo::misc::RewardData<'a>,
    pub damage_type_1: &'a [Element],
    pub damage_type_2: &'a [Element],
    pub target: [TargetConfig; 3],
    pub stage_num: u8,
    pub monster_id_1: Vec<vo::monster::MonsterConfig<'a>>,
    pub monster_id_2: Vec<vo::monster::MonsterConfig<'a>>,
    /// 回合数内打倒敌人，仅出现在混沌回忆中
    pub challenge_count_down: u8,
    pub npc_monster_id_list_1: Vec<vo::monster::NPCMonsterData<'a>>,
    pub event_id_list_1: Vec<vo::misc::StageConfig<'a>>,
    pub npc_monster_id_list_2: Vec<vo::monster::NPCMonsterData<'a>>,
    pub event_id_list_2: Vec<vo::misc::StageConfig<'a>>,
    pub maze_buff: vo::misc::MazeBuff<'a>,
}

#[derive(Clone, Debug)]
pub struct RewardLine<'a> {
    pub group_id: u16,
    pub star_count: u8,
    pub reward: vo::misc::RewardData<'a>,
}

#[derive(Clone, Debug)]
pub struct StageInfiniteGroup<'a> {
    pub id: u32,
    pub wave_list: Vec<StageInfiniteWaveConfig<'a>>,
}

#[derive(Clone, Debug)]
pub struct StageInfiniteMonsterGroup<'a> {
    pub id: u32,
    pub monster_list: Vec<vo::monster::MonsterConfig<'a>>,
    pub elite_group: u16,
}

#[derive(Clone, Debug)]
pub struct StageInfiniteWaveConfig<'a> {
    pub id: u32,
    pub monster_group_list: Vec<StageInfiniteMonsterGroup<'a>>,
    pub max_monster_count: u16,
    pub max_teammate_count: u8,
    pub clear_previous_ability: bool,
}

#[derive(Clone, Debug)]
pub struct TargetConfig {
    pub id: u16,
    pub r#type: TargetType,
    pub name: String,
    /// 不明，不是 RewardData
    /// 只有 ChallengeBossTargetConfig.json 没有 RewardID
    pub reward_id: u32,
}
