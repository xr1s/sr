use crate::{po, vo, FnvIndexMap, FnvMultiMap, ID, PO};

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::OnceLock;

pub struct GameData {
    base: PathBuf,
    text_map: std::collections::HashMap<i32, String, fnv::FnvBuildHasher>,

    // battle
    // 战斗配置
    _battle_event_config: OnceLock<FnvIndexMap<u32, po::battle::BattleEventConfig>>,
    _elite_group: OnceLock<FnvIndexMap<u16, po::battle::EliteGroup>>,
    _stage_config: OnceLock<FnvIndexMap<u32, po::battle::StageConfig>>,
    _stage_infinite_group: OnceLock<FnvIndexMap<u32, po::battle::StageInfiniteGroup>>,
    _stage_infinite_monster_group:
        OnceLock<FnvIndexMap<u32, po::battle::StageInfiniteMonsterGroup>>,
    _stage_infinite_wave_config: OnceLock<FnvIndexMap<u32, po::battle::StageInfiniteWaveConfig>>,
    // challenge
    // 逐光捡金
    _challenge_boss_group_config: OnceLock<FnvIndexMap<u16, po::challenge::GroupConfig>>,
    _challenge_boss_group_extra: OnceLock<FnvIndexMap<u16, po::challenge::GroupExtra>>,
    _challenge_boss_maze_config: OnceLock<FnvIndexMap<u16, po::challenge::MazeConfig>>,
    _challenge_boss_reward_line: OnceLock<FnvMultiMap<u16, po::challenge::RewardLine>>,
    _challenge_boss_target_config: OnceLock<FnvIndexMap<u16, po::challenge::TargetConfig>>,
    _challenge_group_config: OnceLock<FnvIndexMap<u16, po::challenge::GroupConfig>>,
    _challenge_group_maze: OnceLock<FnvMultiMap<u16, u16>>,
    _challenge_maze_reward_line: OnceLock<FnvMultiMap<u16, po::challenge::RewardLine>>,
    _challenge_maze_group_extra: OnceLock<FnvIndexMap<u16, po::challenge::GroupExtra>>,
    _challenge_maze_config: OnceLock<FnvIndexMap<u16, po::challenge::MazeConfig>>,
    _challenge_story_group_config: OnceLock<FnvIndexMap<u16, po::challenge::GroupConfig>>,
    _challenge_story_group_extra: OnceLock<FnvIndexMap<u16, po::challenge::GroupExtra>>,
    _challenge_story_maze_config: OnceLock<FnvIndexMap<u16, po::challenge::MazeConfig>>,
    _challenge_story_maze_extra: OnceLock<FnvIndexMap<u16, po::challenge::MazeExtra>>,
    _challenge_story_reward_line: OnceLock<FnvMultiMap<u16, po::challenge::RewardLine>>,
    _challenge_story_target_config: OnceLock<FnvIndexMap<u16, po::challenge::TargetConfig>>,
    _challenge_target_config: OnceLock<FnvIndexMap<u16, po::challenge::TargetConfig>>,

    // item
    /// 道具
    _item_config: OnceLock<FnvIndexMap<u32, po::item::ItemConfig>>,
    _item_config_avatar_rank: OnceLock<FnvIndexMap<u32, po::item::ItemConfig>>,
    _item_config_equipment: OnceLock<FnvIndexMap<u32, po::item::ItemConfig>>,
    /// 道具使用效果
    _item_use_data: OnceLock<FnvIndexMap<u32, po::item::ItemUseData>>,

    // map
    _map_entrance: OnceLock<FnvIndexMap<u32, po::map::MapEntrance>>,
    _mapping_info: OnceLock<FnvIndexMap<u32, po::map::MappingInfo>>,
    _maze_floor: OnceLock<FnvIndexMap<u32, po::map::MazeFloor>>,
    _maze_plane: OnceLock<FnvIndexMap<u32, po::map::MazePlane>>,
    _maze_prop: OnceLock<FnvIndexMap<u32, po::map::MazeProp>>,
    _world_data_config: OnceLock<FnvIndexMap<u16, po::map::WorldDataConfig>>,

    // misc
    /// 效果说明，比如模拟宇宙中
    _extra_effect_config: OnceLock<FnvIndexMap<u32, po::misc::ExtraEffectConfig>>,
    _maze_buff: OnceLock<FnvIndexMap<u32, po::misc::MazeBuff>>,
    _reward_data: OnceLock<FnvIndexMap<u32, po::misc::RewardData>>,
    _schedule_data_challenge_boss: OnceLock<FnvIndexMap<u32, po::misc::ScheduleData>>,
    _schedule_data_challenge_maze: OnceLock<FnvIndexMap<u32, po::misc::ScheduleData>>,
    _schedule_data_challenge_story: OnceLock<FnvIndexMap<u32, po::misc::ScheduleData>>,
    _schedule_data_global: OnceLock<FnvIndexMap<u32, po::misc::ScheduleDataGlobal>>,

    // mission
    _main_mission: OnceLock<FnvIndexMap<u32, po::mission::MainMission>>,
    _mission_chapter_config: OnceLock<FnvIndexMap<u32, po::mission::MissionChapterConfig>>,
    _sub_mission: OnceLock<FnvIndexMap<u32, po::mission::SubMission>>,

    // monster
    _monster_camp: OnceLock<FnvIndexMap<u8, po::monster::MonsterCamp>>,
    _monster_config: OnceLock<FnvIndexMap<u32, po::monster::MonsterConfig>>,
    _monster_unique_config: OnceLock<FnvIndexMap<u32, po::monster::MonsterConfig>>,
    _monster_skill_config: OnceLock<FnvIndexMap<u32, po::monster::MonsterSkillConfig>>,
    _monster_skill_unique_config: OnceLock<FnvIndexMap<u32, po::monster::MonsterSkillConfig>>,
    _monster_template_config: OnceLock<FnvIndexMap<u32, po::monster::MonsterTemplateConfig>>,
    _monster_template_unique_config: OnceLock<FnvIndexMap<u32, po::monster::MonsterTemplateConfig>>,
    /// 因为存在自引用, 所以只好储存 group_id 到 id 的映射;
    _monster_template_config_group: OnceLock<FnvMultiMap<u32, u32>>,
    _npc_monster_data: OnceLock<FnvIndexMap<u32, po::monster::NPCMonsterData>>,

    // rogue
    // 模拟宇宙
    _rogue_handbook_miracle: OnceLock<FnvIndexMap<u16, po::rogue::RogueHandbookMiracle>>,
    _rogue_handbook_miracle_type: OnceLock<FnvIndexMap<u16, po::rogue::RogueHandbookMiracleType>>,
    /// 模拟宇宙祝福
    _rogue_maze_buff: OnceLock<FnvIndexMap<u32, po::misc::MazeBuff>>,
    /// 模拟宇宙奇物
    _rogue_miracle: OnceLock<FnvIndexMap<u16, po::rogue::RogueMiracle>>,
    _rogue_miracle_display: OnceLock<FnvIndexMap<u16, po::rogue::RogueMiracleDisplay>>,
    _rogue_monster: OnceLock<FnvIndexMap<u32, po::rogue::RogueMonster>>,
    _rogue_monster_group: OnceLock<FnvIndexMap<u32, po::rogue::RogueMonsterGroup>>,

    // rogue magic
    // 模拟宇宙：不可知域
    /// 不可知域奇物
    _rogue_magic_miracle: OnceLock<FnvIndexMap<u16, po::rogue::RogueMiracle>>,

    // rogue tourn 差分宇宙
    /// 差分宇宙文案
    _rogue_tourn_content_display:
        OnceLock<FnvIndexMap<u16, po::rogue::tourn::RogueTournContentDisplay>>,
    /// 差分宇宙方程
    _rogue_tourn_formula: OnceLock<FnvIndexMap<u32, po::rogue::tourn::RogueTournFormula>>,
    _rogue_tourn_formula_display:
        OnceLock<FnvIndexMap<u32, po::rogue::tourn::RogueTournFormulaDisplay>>,
    _rogue_tourn_handbook_miracle:
        OnceLock<FnvIndexMap<u16, po::rogue::tourn::RogueTournHandbookMiracle>>,
    /// 差分宇宙奇物
    _rogue_tourn_miracle: OnceLock<FnvIndexMap<u16, po::rogue::tourn::RogueTournMiracle>>,
    _rogue_tourn_miracle_display: OnceLock<FnvIndexMap<u16, po::rogue::RogueMiracleDisplay>>,
    /// 差分宇宙周期演算
    _rogue_tourn_weekly_challenge:
        OnceLock<FnvIndexMap<u8, po::rogue::tourn::RogueTournWeeklyChallenge>>,
    _rogue_tourn_weekly_display:
        OnceLock<FnvIndexMap<u16, po::rogue::tourn::RogueTournWeeklyDisplay>>,
}

impl GameData {
    pub fn new(base: impl Into<PathBuf>) -> Self {
        let base = base.into();
        let text_map_reader =
            BufReader::new(File::open(base.join("TextMap/TextMapCHS.json")).unwrap());
        GameData {
            base,
            text_map: serde_json::from_reader(text_map_reader).unwrap(),
            // challenge
            _challenge_boss_group_config: OnceLock::new(),
            _challenge_boss_group_extra: OnceLock::new(),
            _challenge_boss_maze_config: OnceLock::new(),
            _challenge_boss_reward_line: OnceLock::new(),
            _challenge_boss_target_config: OnceLock::new(),
            _challenge_group_config: OnceLock::new(),
            _challenge_group_maze: OnceLock::new(),
            _challenge_maze_config: OnceLock::new(),
            _challenge_maze_group_extra: OnceLock::new(),
            _challenge_maze_reward_line: OnceLock::new(),
            _challenge_story_group_config: OnceLock::new(),
            _challenge_story_group_extra: OnceLock::new(),
            _challenge_story_maze_config: OnceLock::new(),
            _challenge_story_maze_extra: OnceLock::new(),
            _challenge_story_reward_line: OnceLock::new(),
            _challenge_story_target_config: OnceLock::new(),
            _challenge_target_config: OnceLock::new(),
            _stage_infinite_group: OnceLock::new(),
            _stage_infinite_monster_group: OnceLock::new(),
            _stage_infinite_wave_config: OnceLock::new(),
            // map
            _map_entrance: OnceLock::new(),
            _mapping_info: OnceLock::new(),
            _maze_floor: OnceLock::new(),
            _maze_plane: OnceLock::new(),
            _maze_prop: OnceLock::new(),
            _world_data_config: OnceLock::new(),
            // misc
            _battle_event_config: OnceLock::new(),
            _elite_group: OnceLock::new(),
            _extra_effect_config: OnceLock::new(),
            _maze_buff: OnceLock::new(),
            _reward_data: OnceLock::new(),
            _schedule_data_challenge_boss: OnceLock::new(),
            _schedule_data_challenge_maze: OnceLock::new(),
            _schedule_data_challenge_story: OnceLock::new(),
            _schedule_data_global: OnceLock::new(),
            _stage_config: OnceLock::new(),
            // mission
            _main_mission: OnceLock::new(),
            _mission_chapter_config: OnceLock::new(),
            _sub_mission: OnceLock::new(),
            // item
            _item_config: OnceLock::new(),
            _item_config_avatar_rank: OnceLock::new(),
            _item_config_equipment: OnceLock::new(),
            _item_use_data: OnceLock::new(),
            // monster
            _monster_template_config: OnceLock::new(),
            _monster_template_unique_config: OnceLock::new(),
            _monster_template_config_group: OnceLock::new(),
            _monster_config: OnceLock::new(),
            _monster_unique_config: OnceLock::new(),
            _npc_monster_data: OnceLock::new(),
            _monster_skill_config: OnceLock::new(),
            _monster_skill_unique_config: OnceLock::new(),
            _monster_camp: OnceLock::new(),
            // rogue
            _rogue_maze_buff: OnceLock::new(),
            _rogue_miracle: OnceLock::new(),
            _rogue_miracle_display: OnceLock::new(),
            _rogue_handbook_miracle: OnceLock::new(),
            _rogue_handbook_miracle_type: OnceLock::new(),
            _rogue_monster_group: OnceLock::new(),
            _rogue_monster: OnceLock::new(),
            // rogue_magic_miracle
            _rogue_magic_miracle: OnceLock::new(),
            // rogue_tourn
            _rogue_tourn_content_display: OnceLock::new(),
            _rogue_tourn_weekly_challenge: OnceLock::new(),
            _rogue_tourn_weekly_display: OnceLock::new(),
            _rogue_tourn_miracle: OnceLock::new(),
            _rogue_tourn_miracle_display: OnceLock::new(),
            _rogue_tourn_handbook_miracle: OnceLock::new(),
            _rogue_tourn_formula: OnceLock::new(),
            _rogue_tourn_formula_display: OnceLock::new(),
        }
    }

    pub(crate) fn text(&self, text: po::Text) -> &str {
        self.text_map
            .get(&text.hash)
            .map(String::as_str)
            .unwrap_or_default()
    }

    fn load_to_map<K, V>(&self, dir: impl Into<std::path::PathBuf>) -> FnvIndexMap<K, V>
    where
        K: std::cmp::Eq + std::hash::Hash,
        for<'a> V: serde::Deserialize<'a> + crate::ID<ID = K>,
    {
        let path = self.base.join(dir.into());
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let po: Vec<V> = serde_json::from_reader(reader).unwrap();
        po.into_iter().map(|po| (po.id(), po)).collect()
    }
}

impl GameData {
    fn _monster_template_config_group(&self) -> &FnvMultiMap<u32, u32> {
        self._monster_template_config_group.get_or_init(|| {
            self._monster_template_config()
                .values()
                .filter(|monster| monster.template_group_id.is_some())
                .map(|monster| (monster.template_group_id.unwrap().get(), monster.id()))
                .collect()
        })
    }

    pub fn monster_template_config_group(
        &self,
        id: u32,
    ) -> impl Iterator<Item = vo::monster::MonsterTemplateConfig> {
        if id == 0 {
            return either::Either::Left(std::iter::empty());
        }
        either::Either::Right(
            self._monster_template_config_group()
                .get_vec(&id)
                .map(Vec::as_slice)
                .unwrap_or_default()
                .iter()
                .map(|&id| self.monster_template_config(id))
                .map(Option::unwrap), // map 的值就是从 monster_template_config 生成的
                                      // 所以这里不会 panic
        )
    }

    fn _challenge_group_maze(&self) -> &FnvMultiMap<u16, u16> {
        self._challenge_group_maze.get_or_init(|| {
            std::iter::empty()
                .chain(self.list_challenge_maze_config())
                .chain(self.list_challenge_story_maze_config())
                .chain(self.list_challenge_boss_maze_config())
                .map(|maze| (maze.group.id, maze.id))
                .collect()
        })
    }

    pub fn challenge_group_maze(&self, id: u16) -> Vec<vo::challenge::MazeConfig> {
        use po::challenge::GroupType;
        let is_memory = self._challenge_group_config().contains_key(&id) as u8;
        let is_story = self._challenge_story_group_config().contains_key(&id) as u8;
        let is_boss = self._challenge_boss_group_config().contains_key(&id) as u8;
        let group_type = match (is_memory, is_story, is_boss) {
            (1, 0, 0) => GroupType::Memory,
            (0, 1, 0) => GroupType::Story,
            (0, 0, 1) => GroupType::Boss,
            _ => return Vec::new(),
        };
        self._challenge_group_maze()
            .get_vec(&id)
            .map(Vec::as_slice)
            .unwrap_or_default()
            .iter()
            .map(move |&id| match group_type {
                GroupType::Memory => self.challenge_maze_config(id),
                GroupType::Story => self.challenge_story_maze_config(id),
                GroupType::Boss => self.challenge_boss_maze_config(id),
            })
            .map(Option::unwrap)
            .collect()
    }
}

macro_rules! reward_line {
    ($field:ident) => {
        paste::paste! {
            fn [<_$field>](&self) -> &FnvMultiMap<u16, po::challenge::RewardLine> {
                self.[<_$field>].get_or_init(|| {
                    let path = self.base.join(concat!("ExcelOutput/", stringify!([<$field:camel>]), ".json"));
                    let file = std::fs::File::open(path).unwrap();
                    let reader = std::io::BufReader::new(file);
                    let po: Vec<po::challenge::RewardLine> = serde_json::from_reader(reader).unwrap();
                    po.into_iter().map(|po| (po.group_id, po)).collect()
                })
            }
            pub fn $field(&self, group_id: u16) -> Vec<vo::challenge::RewardLine<'_>> {
                self.[<_$field>]().get_vec(&group_id).unwrap().into_iter().map(|po| po.vo(self)).collect()
            }
        }
    };
}

impl GameData {
    reward_line!(challenge_boss_reward_line);
    reward_line!(challenge_maze_reward_line);
    reward_line!(challenge_story_reward_line);
}

macro_rules! field {
    ($field:ident, $id:ty => $typ:ty) => {
        paste::paste! {
            field!($field, $id => $typ, stringify!([<$field:camel>]));
        }
    };

    ($field:ident, $id:ty => $typ:ty, $json:expr) => {
        paste::paste! {
            fn [<_$field>](&self) -> &FnvIndexMap<<po::$typ as ID>::ID, po::$typ> {
                self.[<_ $field>].get_or_init(|| {
                    self.load_to_map(concat!("ExcelOutput/", $json, ".json"))
                })
            }
            pub fn [<$field>](&self, id: $id) -> Option<vo::$typ> {
                self.[<_$field>]().get(&id).map(|po| po.vo(self))
            }
            pub fn [<list_$field>](&self) -> Vec<vo::$typ> {
                self.[<_$field>]().values().map(|po| po.vo(self)).collect()
            }
        }
    };
}

impl GameData {
    // 宏效果示例
    /* fn _item_config(&self) -> &FnvIndexMap<u32, po::item::ItemConfig> {
     *      self._item_config.get_or_init(|| {
     *          self.load_to_map(concat!("ExcelOutput/ItemConfig.json"))
     *      })
     *  }
     *
     *  pub fn item_config(&self, id: u32) -> Option<vo::item::ItemConfig> {
     *      self._item_config().get(&id).map(|po| po.vo(self))
     *  }
     *
     *  pub fn list_item_config(&self) -> Vec<vo::item::ItemConfig> {
     *      self._item_config().values().map(|po| po.vo(self)).collect()
     *  }
     */

    // battle
    field!(battle_event_config, u32 => battle::BattleEventConfig);
    field!(elite_group, u16 => battle::EliteGroup);
    field!(stage_infinite_group, u32 => battle::StageInfiniteGroup);
    field!(stage_infinite_monster_group, u32 => battle::StageInfiniteMonsterGroup);
    field!(stage_infinite_wave_config, u32 => battle::StageInfiniteWaveConfig);
    field!(stage_config, u32 => battle::StageConfig);
    // challenge
    field!(challenge_boss_group_config, u16 => challenge::GroupConfig);
    field!(challenge_boss_group_extra, u16 => challenge::GroupExtra);
    field!(challenge_boss_maze_config, u16 => challenge::MazeConfig);
    field!(challenge_boss_target_config, u16 => challenge::TargetConfig);
    field!(challenge_group_config, u16 => challenge::GroupConfig);
    field!(challenge_maze_config, u16 => challenge::MazeConfig);
    field!(challenge_maze_group_extra, u16 => challenge::GroupExtra);
    field!(challenge_story_group_config, u16 => challenge::GroupConfig);
    field!(challenge_story_group_extra, u16 => challenge::GroupExtra);
    field!(challenge_story_maze_config, u16 => challenge::MazeConfig);
    field!(challenge_story_maze_extra, u16 => challenge::MazeExtra);
    field!(challenge_story_target_config, u16 => challenge::TargetConfig);
    field!(challenge_target_config, u16 => challenge::TargetConfig);
    // item
    field!(item_config, u32 => item::ItemConfig);
    field!(item_config_avatar_rank, u32 => item::ItemConfig);
    field!(item_config_equipment, u32 => item::ItemConfig);
    field!(item_use_data, u32 => item::ItemUseData);
    // map
    field!(map_entrance, u32 => map::MapEntrance);
    field!(mapping_info, u32 => map::MappingInfo);
    field!(maze_floor, u32 => map::MazeFloor);
    field!(maze_plane, u32 => map::MazePlane);
    field!(maze_prop, u32 => map::MazeProp);
    field!(world_data_config, u16 => map::WorldDataConfig);
    // misc
    field!(extra_effect_config, u32 => misc::ExtraEffectConfig);
    field!(maze_buff, u32 => misc::MazeBuff);
    field!(reward_data, u32 => misc::RewardData);
    field!(schedule_data_challenge_boss, u32 => misc::ScheduleData);
    field!(schedule_data_challenge_maze, u32 => misc::ScheduleData);
    field!(schedule_data_challenge_story, u32 => misc::ScheduleData);
    field!(schedule_data_global, u32 => misc::ScheduleDataGlobal);
    // mission
    field!(main_mission, u32 => mission::MainMission);
    field!(mission_chapter_config, u32 => mission::MissionChapterConfig);
    field!(sub_mission, u32 => mission::SubMission);
    // monster
    field!(monster_camp, u8 => monster::MonsterCamp);
    field!(monster_config, u32 => monster::MonsterConfig);
    field!(monster_unique_config, u32 => monster::MonsterConfig);
    field!(monster_skill_config, u32 => monster::MonsterSkillConfig);
    field!(monster_skill_unique_config, u32 => monster::MonsterSkillConfig);
    field!(monster_template_config, u32 => monster::MonsterTemplateConfig);
    field!(monster_template_unique_config, u32 => monster::MonsterTemplateConfig);
    field!(npc_monster_data, u32 => monster::NPCMonsterData, "NPCMonsterData");
    // rogue
    field!(rogue_handbook_miracle, u16 => rogue::RogueHandbookMiracle);
    field!(rogue_handbook_miracle_type, u16 => rogue::RogueHandbookMiracleType);
    field!(rogue_maze_buff, u32 => misc::MazeBuff);
    field!(rogue_miracle, u16 => rogue::RogueMiracle);
    field!(rogue_miracle_display, u16 => rogue::RogueMiracleDisplay);
    field!(rogue_monster, u32 => rogue::RogueMonster);
    field!(rogue_monster_group, u32 => rogue::RogueMonsterGroup);
    // rogue magic
    field!(rogue_magic_miracle, u16 => rogue::RogueMiracle);
    // rogue tourn
    field!(rogue_tourn_content_display, u16 => rogue::tourn::RogueTournContentDisplay);
    field!(rogue_tourn_formula, u32 => rogue::tourn::RogueTournFormula);
    field!(rogue_tourn_formula_display, u32 => rogue::tourn::RogueTournFormulaDisplay);
    field!(rogue_tourn_handbook_miracle, u16 => rogue::tourn::RogueTournHandbookMiracle);
    field!(rogue_tourn_miracle, u16 => rogue::tourn::RogueTournMiracle);
    field!(rogue_tourn_miracle_display, u16 => rogue::RogueMiracleDisplay);
    field!(rogue_tourn_weekly_challenge, u8 => rogue::tourn::RogueTournWeeklyChallenge);
    field!(rogue_tourn_weekly_display, u16 => rogue::tourn::RogueTournWeeklyDisplay);
}
