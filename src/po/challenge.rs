/// WIP
/// 逐光捡金，也就是俗称的深渊
/// 看文件前缀
/// Challenge:      混沌回忆
/// ChallengeStory: 虚构叙事
/// ChallengeBoss:  末日幻影
use std::{num::NonZero, path::PathBuf};

use crate::{vo, GameData, ID, PO};

use super::{Element, Text};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
/// 逐光捡金类型
pub enum GroupType {
    /// 混沌回忆
    Boss,
    /// 混沌回忆
    Memory,
    /// 虚构叙事
    Story,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
/// 单期配置
pub(crate) struct GroupConfig {
    #[serde(rename = "GroupID")]
    group_id: u16,
    group_name: Text,
    #[serde(rename = "RewardLineGroupID")]
    reward_line_group_id: u16,
    #[serde(rename = "PreMissionID")]
    pre_mission_id: u32,
    #[serde(rename = "GlobalScheduleID")]
    global_schedule_id: Option<NonZero<u32>>,
    #[serde(rename = "ScheduleDataID")]
    schedule_data_id: Option<NonZero<u32>>,
    #[serde(rename = "MazeBuffID")]
    maze_buff_id: Option<NonZero<u32>>,
    #[serde(rename = "MapEntranceID")]
    map_entrance_id: Option<NonZero<u32>>,
    #[serde(rename = "MappingInfoID")]
    mapping_info_id: Option<NonZero<u32>>,
    #[serde(rename = "WorldID")]
    world_id: Option<NonZero<u16>>,
    back_ground_path: PathBuf,
    tab_pic_path: PathBuf,
    tab_pic_select_path: PathBuf,
    challenge_group_type: GroupType,
    theme_pic_path: PathBuf,
}

impl ID for GroupConfig {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.group_id
    }
}

impl<'a> PO<'a> for GroupConfig {
    type VO = vo::challenge::GroupConfig<'a>;
    fn vo(&self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.group_id,
            name: game.text(self.group_name),
            reward_line_group: match self.challenge_group_type {
                GroupType::Boss => game.challenge_boss_reward_line(self.reward_line_group_id),
                GroupType::Memory => game.challenge_maze_reward_line(self.reward_line_group_id),
                GroupType::Story => game.challenge_story_reward_line(self.reward_line_group_id),
            },
            pre_mission: game.main_mission(self.pre_mission_id).unwrap(),
            global_schedule: self
                .global_schedule_id
                .map(NonZero::get)
                .map(|id| game.schedule_data_global(id))
                .map(Option::unwrap),
            schedule_data: self
                .schedule_data_id
                .map(NonZero::get)
                .map(|id| match self.challenge_group_type {
                    GroupType::Boss => game.schedule_data_challenge_boss(id),
                    GroupType::Memory => game.schedule_data_challenge_maze(id),
                    GroupType::Story => game.schedule_data_challenge_story(id),
                })
                .map(Option::unwrap),
            maze_buff: self
                .maze_buff_id
                .map(NonZero::get)
                .map(|id| game.maze_buff(id))
                .map(Option::unwrap),
            map_entrance: self
                .map_entrance_id
                .map(NonZero::get)
                .map(|id| game.map_entrance(id))
                .map(Option::unwrap),
            mapping_info: self.mapping_info_id.map(NonZero::get).and_then(|id| {
                if id == 1220 {
                    None // TODO: 疑似缺数据
                } else {
                    Some(game.mapping_info(id).unwrap())
                }
            }),
            world: self
                .world_id
                .map(NonZero::get)
                .map(|id| game.world_data_config(id))
                .map(Option::unwrap),
            r#type: self.challenge_group_type,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
/// 单层配置
pub(crate) struct MazeConfig {
    #[serde(rename = "ID")]
    id: u16,
    name: Text,
    #[serde(rename = "GroupID")]
    group_id: u16,
    #[serde(rename = "MapEntranceID")]
    map_entrance_id: u32,
    #[serde(rename = "MapEntranceID2")]
    map_entrance_id_2: u32,
    pre_level: Option<NonZero<u8>>, // 目前只有 1
    #[serde(rename = "PreChallengeMazeID")]
    pre_challenge_maze_id: Option<NonZero<u16>>,
    floor: Option<NonZero<u8>>,
    #[serde(rename = "RewardID")]
    reward_id: u32,
    damage_type_1: Vec<Element>,
    damage_type_2: Vec<Element>,
    #[serde(rename = "ChallengeTargetID")]
    challenge_target_id: [u16; 3],
    stage_num: u8,
    #[serde(rename = "MonsterID1")]
    monster_id_1: Vec<u32>,
    #[serde(rename = "MonsterID2")]
    monster_id_2: Vec<u32>,
    challenge_count_down: Option<NonZero<u8>>,
    #[serde(rename = "MazeGroupID1")]
    maze_group_id_1: u8,
    config_list_1: Vec<u32>,
    #[serde(rename = "NpcMonsterIDList1")]
    npc_monster_id_list_1: Vec<u32>,
    #[serde(rename = "EventIDList1")]
    event_id_list_1: Vec<u32>,
    #[serde(rename = "MazeGroupID2")]
    /// 不明，但是 Option 可能是因为混沌回忆（入门级）没有下半场
    maze_group_id_2: Option<NonZero<u8>>,
    /// 不明，数量都很小
    config_list_2: Vec<u32>,
    #[serde(rename = "NpcMonsterIDList2")]
    npc_monster_id_list_2: Vec<u32>,
    #[serde(rename = "EventIDList2")]
    event_id_list_2: Vec<u32>,
    #[serde(rename = "MazeBuffID")]
    maze_buff_id: u32,
}

impl ID for MazeConfig {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.id
    }
}

impl<'a> PO<'a> for MazeConfig {
    type VO = vo::challenge::MazeConfig<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        let group = None
            .or_else(|| game.challenge_group_config(self.group_id))
            .or_else(|| game.challenge_story_group_config(self.group_id))
            .or_else(|| game.challenge_boss_group_config(self.group_id))
            .unwrap();
        let group_type = group.r#type;
        Self::VO {
            id: self.id,
            name: game.text(self.name),
            group,
            map_entrance: game.map_entrance(self.map_entrance_id).unwrap(),
            map_entrance_2: game.map_entrance(self.map_entrance_id_2).unwrap(),
            pre_level: self.pre_level.map(NonZero::get).unwrap_or_default(),
            pre_challenge_maze_id: self
                .pre_challenge_maze_id
                .map(NonZero::get)
                .unwrap_or_default(),
            floor: self.floor.map(NonZero::get).unwrap_or_default(),
            reward: game.reward_data(self.reward_id).unwrap(),
            damage_type_1: &self.damage_type_1,
            damage_type_2: &self.damage_type_2,
            target: std::array::from_fn(|index| {
                (match group_type {
                    GroupType::Boss => GameData::challenge_boss_target_config,
                    GroupType::Memory => GameData::challenge_target_config,
                    GroupType::Story => GameData::challenge_story_target_config,
                })(game, self.challenge_target_id[index])
                .unwrap()
            }),
            stage_num: self.stage_num,
            monster_id_1: self
                .monster_id_1
                .iter()
                .map(|&id| game.monster_config(id))
                .map(Option::unwrap)
                .collect(),
            monster_id_2: self
                .monster_id_2
                .iter()
                .map(|&id| game.monster_config(id))
                .map(Option::unwrap)
                .collect(),
            challenge_count_down: self
                .challenge_count_down
                .map(NonZero::get)
                .unwrap_or_default(),
            npc_monster_id_list_1: self
                .npc_monster_id_list_1
                .iter()
                .map(|&id| game.npc_monster_data(id))
                .map(Option::unwrap)
                .collect(),
            event_id_list_1: self
                .event_id_list_1
                .iter()
                .map(|&id| game.stage_config(id))
                .map(Option::unwrap)
                .collect(),
            npc_monster_id_list_2: self
                .npc_monster_id_list_2
                .iter()
                .map(|&id| game.npc_monster_data(id))
                .map(Option::unwrap)
                .collect(),
            event_id_list_2: self
                .event_id_list_2
                .iter()
                .map(|&id| game.stage_config(id))
                .map(Option::unwrap)
                .collect(),
            maze_buff: game.maze_buff(self.maze_buff_id).unwrap(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
/// 每三星获得的奖励
pub(crate) struct RewardLine {
    #[serde(rename = "GroupID")]
    pub(crate) group_id: u16,
    star_count: u8,
    #[serde(rename = "RewardID")]
    reward_id: u32,
}

impl ID for RewardLine {
    type ID = (u16, u8);
    fn id(&self) -> Self::ID {
        (self.group_id, self.star_count)
    }
}

impl<'a> PO<'a> for RewardLine {
    type VO = vo::challenge::RewardLine<'a>;
    fn vo(&self, game: &'a GameData) -> Self::VO {
        Self::VO {
            group_id: self.group_id,
            star_count: self.star_count,
            reward: game.reward_data(self.reward_id).unwrap(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// 分别对应三种挑战胜利条件
pub enum TargetType {
    DeadAvatar,
    RoundsLeft,
    TotalScore,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct TargetConfig {
    #[serde(rename = "ID")]
    id: u16,
    challenge_target_type: TargetType,
    challenge_target_name: Text,
    challenge_target_param_1: Option<NonZero<u16>>,
    #[serde(rename = "RewardID")]
    reward_id: Option<NonZero<u32>>,
}

impl ID for TargetConfig {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.id
    }
}

impl PO<'_> for TargetConfig {
    type VO = vo::challenge::TargetConfig;
    fn vo(&self, game: &GameData) -> Self::VO {
        Self::VO {
            id: self.id,
            r#type: self.challenge_target_type,
            name: crate::format::format(
                game.text(self.challenge_target_name),
                &[crate::format::Argument::from(
                    &self
                        .challenge_target_param_1
                        .map(NonZero::get)
                        .unwrap_or_default(),
                )],
            ),
            reward_id: self.reward_id.map(NonZero::get).unwrap_or_default(),
        }
    }
}
