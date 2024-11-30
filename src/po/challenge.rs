/// WIP
/// 逐光捡金，也就是俗称的深渊
/// 看文件前缀
/// Challenge:      混沌回忆
/// ChallengeStory: 虚构叙事
/// ChallengeBoss:  末日幻影
use std::num::NonZero;

use crate::{vo, GameData, GroupID, ID, PO};

use super::{Element, Text};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
/// 逐光捡金类型
pub enum GroupType {
    /// 混沌回忆
    Memory,
    /// 虚构叙事
    Story,
    /// 末日幻影
    Boss,
}

impl Default for GroupType {
    fn default() -> Self {
        Self::Memory
    }
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
    back_ground_path: Option<String>,    // 1.2 及之后
    tab_pic_path: Option<String>,        // 1.2 及之后
    tab_pic_select_path: Option<String>, // 1.2 及之后
    #[serde(default)]
    challenge_group_type: GroupType, // 1.5 及之前虚构叙事出现前，无此字段
    theme_pic_path: Option<String>,      // 1.5 及之前没有该字段
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
            game,
            id: self.group_id,
            name: game.text(self.group_name),
            reward_line_group: match self.challenge_group_type {
                GroupType::Memory => game.challenge_maze_reward_line(self.reward_line_group_id),
                GroupType::Story => game.challenge_story_reward_line(self.reward_line_group_id),
                GroupType::Boss => game.challenge_boss_reward_line(self.reward_line_group_id),
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
                    GroupType::Memory => game.schedule_data_challenge_maze(id),
                    GroupType::Story => game.schedule_data_challenge_story(id),
                    GroupType::Boss => game.schedule_data_challenge_boss(id),
                })
                .map(Option::unwrap),
            maze_buff: self
                .maze_buff_id
                .map(NonZero::get)
                .map(|id| game.maze_buff(id))
                .map(Vec::into_iter)
                .map(|mut iter| iter.next())
                .map(Option::unwrap),
            map_entrance: self
                .map_entrance_id
                .map(NonZero::get)
                .map(|id| game.map_entrance(id))
                .map(Option::unwrap),
            mapping_info: self
                .mapping_info_id
                .map(NonZero::get)
                .filter(|&id| id != 1220) // TODO: 疑似缺数据
                .map(|id| game.mapping_info(id))
                .unwrap_or_default(),
            world: self
                .world_id
                .map(NonZero::get)
                .map(|id| game.world_data_config(id))
                .map(Option::unwrap),
            r#type: self.challenge_group_type,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum BossPattern {
    SmallAndLarge,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
/// 单期配置
pub(crate) struct GroupExtra {
    #[serde(rename = "GroupID")]
    group_id: u16,
    theme_poster_bg_pic_path: String,
    // 以下 4 个只在虚构叙事和末日幻影中出现
    theme_toast_pic_path: Option<String>,
    theme_icon_pic_path: Option<String>,
    theme_poster_effect_prefab_path: Option<String>,
    theme_poster_tab_pic_path: Option<String>,
    // 以下 2 个只在虚构叙事中出现
    buff_list: Option<[u32; 3]>,
    #[serde(rename = "ThemeID")]
    theme_id: Option<NonZero<u8>>,
    // 以下 6 个只在末日幻影中出现
    buff_list_1: Option<[u32; 3]>,
    buff_list_2: Option<[u32; 3]>,
    boss_pattern: Option<BossPattern>, // 2.4 之后该字段消失
    boss_pattern_prefab_path: Option<String>,
    boss_position_prefab_path_1: Option<String>,
    boss_position_prefab_path_2: Option<String>,
}

impl ID for GroupExtra {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.group_id
    }
}

impl<'a> PO<'a> for GroupExtra {
    type VO = vo::challenge::GroupExtra<'a>;
    fn vo(&self, game: &'a GameData) -> Self::VO {
        let assemble = |buffs: [u32; 3]| buffs.iter().flat_map(|&id| game.maze_buff(id)).collect();
        Self::VO {
            id: self.group_id,
            buff_list: self.buff_list.map(assemble).unwrap_or_default(),
            buff_list_1: self.buff_list_1.map(assemble).unwrap_or_default(),
            buff_list_2: self.buff_list_2.map(assemble).unwrap_or_default(),
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
    map_entrance_id_2: Option<NonZero<u32>>,
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
            map_entrance_2: self
                .map_entrance_id_2
                .map(NonZero::get)
                .map(|id| game.map_entrance(id))
                .map(Option::unwrap),
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
                    GroupType::Memory => GameData::challenge_target_config,
                    GroupType::Story => GameData::challenge_story_target_config,
                    GroupType::Boss => GameData::challenge_boss_target_config,
                })(game, self.challenge_target_id[index])
                .unwrap()
            }),
            stage_num: self.stage_num,
            monster_1: self
                .monster_id_1
                .iter()
                .map(|&id| game.monster_config(id))
                .map(Option::unwrap)
                .collect(),
            monster_2: self
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
            event_list_1: self
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
            event_list_2: self
                .event_id_list_2
                .iter()
                .map(|&id| game.stage_config(id))
                .map(Option::unwrap)
                .collect(),
            maze_buff: game
                .maze_buff(self.maze_buff_id)
                .into_iter()
                .next()
                .unwrap(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct MazeExtra {
    #[serde(rename = "ID")]
    id: u16,
    // 虚构叙事
    turn_limit: Option<NonZero<u8>>,
    #[serde(rename = "BattleTargetID")]
    battle_target_id: Option<Vec<u16>>, // TODO: BattleTargetConfig.json
    clear_score: Option<NonZero<u16>>,
    // 末日幻影
    #[serde(rename = "MonsterID1")]
    monster_id_1: Option<NonZero<u32>>,
    #[serde(rename = "MonsterID2")]
    monster_id_2: Option<NonZero<u32>>,
}

impl ID for MazeExtra {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.id
    }
}

impl<'a> PO<'a> for MazeExtra {
    type VO = vo::challenge::MazeExtra<'a>;
    fn vo(&self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.id,
            turn_limit: self.turn_limit.map(NonZero::get).unwrap_or_default(),
            monster_1: self
                .monster_id_1
                .map(NonZero::get)
                .map(|id| game.monster_config(id))
                .map(Option::unwrap),
            monster_2: self
                .monster_id_2
                .map(NonZero::get)
                .map(|id| game.monster_config(id))
                .map(Option::unwrap),
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

impl GroupID for RewardLine {
    type GroupID = u16;
    type InnerID = u8;
    fn group_id(&self) -> Self::GroupID {
        self.group_id
    }
    fn inner_id(&self) -> Self::InnerID {
        self.star_count
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
