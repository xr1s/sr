use crate::{po, vo, FnvIndexMap, FnvMultiMap, GroupID, ID, PO};

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::OnceLock;

#[derive(Default)]
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
    _challenge_group_in_maze: OnceLock<FnvMultiMap<u16, u16>>,
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
    _mapping_info: OnceLock<FnvMultiMap<u32, po::map::MappingInfo>>,
    _maze_floor: OnceLock<FnvIndexMap<u32, po::map::MazeFloor>>,
    _maze_plane: OnceLock<FnvIndexMap<u32, po::map::MazePlane>>,
    _maze_prop: OnceLock<FnvIndexMap<u32, po::map::MazeProp>>,
    _world_data_config: OnceLock<FnvIndexMap<u16, po::map::WorldDataConfig>>,

    // message
    _emoji_config: OnceLock<FnvIndexMap<u32, po::message::EmojiConfig>>,
    _emoji_group: OnceLock<FnvIndexMap<u8, po::message::EmojiGroup>>,
    _message_contacts_camp: OnceLock<FnvIndexMap<u8, po::message::MessageContactsCamp>>,
    _message_contacts_config: OnceLock<FnvIndexMap<u16, po::message::MessageContactsConfig>>,
    _message_contacts_type: OnceLock<FnvIndexMap<u8, po::message::MessageContactsType>>,
    _message_group_config: OnceLock<FnvIndexMap<u16, po::message::MessageGroupConfig>>,
    _message_item_config: OnceLock<FnvIndexMap<u32, po::message::MessageItemConfig>>,
    _message_item_image: OnceLock<FnvIndexMap<u32, po::message::MessageItemImage>>,
    _message_section_config: OnceLock<FnvIndexMap<u32, po::message::MessageSectionConfig>>,
    _message_section_in_contacts: OnceLock<FnvMultiMap<u16, u32>>,

    // misc
    /// 效果说明，比如模拟宇宙中
    _extra_effect_config: OnceLock<FnvIndexMap<u32, po::misc::ExtraEffectConfig>>,
    _maze_buff: OnceLock<FnvMultiMap<u32, po::misc::MazeBuff>>,
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
    _monster_camp: OnceLock<FnvIndexMap<u8, po::monster::Camp>>,
    _monster_config: OnceLock<FnvIndexMap<u32, po::monster::Config>>,
    _monster_difficulty_guide: OnceLock<FnvIndexMap<u16, po::monster::guide::Difficulty>>,
    _monster_guide_config: OnceLock<FnvIndexMap<u32, po::monster::guide::Config>>,
    _monster_guide_phase: OnceLock<FnvIndexMap<u16, po::monster::guide::Phase>>,
    _monster_guide_skill: OnceLock<FnvIndexMap<u32, po::monster::guide::Skill>>,
    _monster_guide_skill_text: OnceLock<FnvIndexMap<u32, po::monster::guide::SkillText>>,
    _monster_guide_tag: OnceLock<FnvIndexMap<u32, po::monster::guide::Tag>>,
    _monster_skill_config: OnceLock<FnvIndexMap<u32, po::monster::SkillConfig>>,
    _monster_skill_unique_config: OnceLock<FnvIndexMap<u32, po::monster::SkillConfig>>,
    _monster_template_config: OnceLock<FnvIndexMap<u32, po::monster::TemplateConfig>>,
    _monster_template_unique_config: OnceLock<FnvIndexMap<u32, po::monster::TemplateConfig>>,
    /// 因为存在自引用, 所以只好储存 group_id 到 id 的映射;
    _monster_template_config_group: OnceLock<FnvMultiMap<u32, u32>>,
    _monster_text_guide: OnceLock<FnvIndexMap<u16, po::monster::guide::Text>>,
    _monster_unique_config: OnceLock<FnvIndexMap<u32, po::monster::Config>>,
    _npc_monster_data: OnceLock<FnvIndexMap<u32, po::monster::NPCMonsterData>>,

    // rogue
    // 模拟宇宙
    _rogue_handbook_miracle: OnceLock<FnvIndexMap<u16, po::rogue::RogueHandbookMiracle>>,
    _rogue_handbook_miracle_type: OnceLock<FnvIndexMap<u16, po::rogue::RogueHandbookMiracleType>>,
    /// 模拟宇宙祝福
    _rogue_maze_buff: OnceLock<FnvMultiMap<u32, po::misc::MazeBuff>>,
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

    // talk
    _talk_sentence_config: OnceLock<FnvIndexMap<u32, po::talk::TalkSentenceConfig>>,
    _voice_config: OnceLock<FnvIndexMap<u32, po::talk::VoiceConfig>>,
}

impl GameData {
    pub fn new(base: impl Into<PathBuf>) -> Self {
        let base = base.into();
        let file = File::open(base.join("TextMap/TextMapCHS.json"))
            .or_else(|_| File::open(base.join("TextMap/TextMapCN.json")))
            .unwrap();
        let text_map_reader = BufReader::new(file);
        let text_map = serde_json::from_reader(text_map_reader).unwrap();
        GameData {
            base,
            text_map,
            ..GameData::default()
        }
    }

    pub(crate) fn text(&self, text: po::Text) -> &str {
        self.text_map
            .get(&text.hash)
            .map(String::as_str)
            .unwrap_or_default()
    }

    fn load_to_map<K, V>(
        &self,
        dir: impl Into<std::path::PathBuf>,
    ) -> std::io::Result<FnvIndexMap<K, V>>
    where
        K: std::cmp::Eq + std::hash::Hash,
        V: ID<ID = K>,
        for<'a> K: serde::Deserialize<'a>,
        for<'a> V: serde::Deserialize<'a>,
    {
        let path = self.base.join(dir.into());
        let file = File::open(&path)?;
        let file_size = file.metadata().unwrap().len();
        let mut reader = BufReader::new(file);
        let mut bytes = Vec::with_capacity(file_size as _);
        std::io::Read::read_to_end(&mut reader, &mut bytes).unwrap();
        Ok(serde_json::from_slice(&bytes)
            // 仅应在处理 2.3 版本及以下的数据集时输出错误
            // 每个版本更新后也存在某些特殊字段未解密导致一直在变 serde 失败的情况
            // 具体搜 "serde(alias" 字符串。每个版本更新后初始化一下日志看看是什么报错
            .inspect_err(|e| log::warn!("疑似 2.3 之前的老数据格式: {:?}", e))
            .map_or_else(
                // 2.3 版本及以下, 采用的数据结构是 {"123": {"ID": 123, ...} } 形式
                |_| serde_json::from_slice::<FnvIndexMap<K, V>>(&bytes).unwrap(),
                // 2.4 版本及以上, 采用的数据结构是 [ {"ID": 123, ...} ] 形式
                |po: Vec<V>| po.into_iter().map(|po| (po.id(), po)).collect(), // >= 2.4
            ))
    }

    fn load_to_group<G, I, V>(&self, dir: impl Into<std::path::PathBuf>) -> FnvMultiMap<G, V>
    where
        G: std::cmp::Eq + std::hash::Hash,
        I: std::cmp::Eq + std::hash::Hash,
        V: GroupID<GroupID = G, InnerID = I>,
        for<'a> G: serde::Deserialize<'a>,
        for<'a> I: serde::Deserialize<'a>,
        for<'a> V: serde::Deserialize<'a>,
    {
        let path = self.base.join(dir.into());
        let file = File::open(path).unwrap();
        let file_size = file.metadata().unwrap().len();
        let mut reader = BufReader::new(file);
        let mut bytes = Vec::with_capacity(file_size as _);
        std::io::Read::read_to_end(&mut reader, &mut bytes).unwrap();
        serde_json::from_slice(&bytes)
            // 仅应在处理 2.3 版本及以下的数据集时输出错误
            .inspect_err(|e| log::warn!("疑似 2.3 之前的老数据格式: {:?}", e))
            .map_or_else(
                // 2.3 版本及以下, 采用的数据结构是 {"123": { "4": { "GroupID": 123, "InnerID": 4, ... } } } 形式
                |_| {
                    serde_json::from_slice::<FnvIndexMap<G, FnvIndexMap<I, V>>>(&bytes)
                        .unwrap()
                        .into_values()
                        .flat_map(FnvIndexMap::into_values)
                        .map(|po| (po.group_id(), po))
                        .collect()
                },
                // 2.4 版本及以上, 采用的数据结构是 [{"GroupID": 123, "InnerID": 4, ...} ] 形式
                |po: Vec<V>| po.into_iter().map(|po| (po.group_id(), po)).collect(),
            )
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
    ) -> impl Iterator<Item = vo::monster::TemplateConfig> {
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

    fn _challenge_maze_in_group(&self) -> &FnvMultiMap<u16, u16> {
        self._challenge_group_in_maze.get_or_init(|| {
            std::iter::empty()
                .chain(self.list_challenge_maze_config())
                .chain(self.list_challenge_story_maze_config())
                .chain(self.list_challenge_boss_maze_config())
                .map(|maze| (maze.group.id, maze.id))
                .collect()
        })
    }

    pub fn challenge_maze_in_group(&self, id: u16) -> Vec<vo::challenge::MazeConfig> {
        use po::challenge::GroupType;
        let is_memory = self._challenge_group_config().contains_key(&id) as u8;
        let is_story = self._challenge_story_group_config().contains_key(&id) as u8;
        let is_boss = self._challenge_boss_group_config().contains_key(&id) as u8;
        let group_type = match (is_memory, is_story, is_boss) {
            (1, 0, 0) => GroupType::Memory,
            (0, 1, 0) => GroupType::Story,
            (0, 0, 1) => GroupType::Boss,
            (0, 0, 0) => return Vec::new(),
            _ => unreachable!(),
        };
        self._challenge_maze_in_group()
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

    fn _current_challenge_group_config<'a, F, I>(
        &'a self,
        lister: F,
    ) -> Option<vo::challenge::GroupConfig<'a>>
    where
        F: Fn(&'a GameData) -> I,
        I: Iterator<Item = vo::challenge::GroupConfig<'a>>,
    {
        let now = chrono::Local::now();
        lister(self).find(|challenge| {
            challenge
                .schedule_data
                .map(|sched| sched.begin_time <= now && now <= sched.end_time)
                .unwrap_or_default()
        })
    }

    pub fn current_challenge_group_config(&self) -> Option<vo::challenge::GroupConfig> {
        self._current_challenge_group_config(Self::list_challenge_group_config)
    }

    pub fn current_challenge_story_group_config(&self) -> Option<vo::challenge::GroupConfig> {
        self._current_challenge_group_config(Self::list_challenge_story_group_config)
    }

    pub fn current_challenge_boss_group_config(&self) -> Option<vo::challenge::GroupConfig> {
        self._current_challenge_group_config(Self::list_challenge_boss_group_config)
    }

    fn _message_section_in_contacts(&self) -> &FnvMultiMap<u16, u32> {
        self._message_section_in_contacts.get_or_init(|| {
            let mut sections_in_contacts = FnvMultiMap::default();
            self._message_group_config()
                .values()
                .map(|group| (group.message_contacts_id, &group.message_section_id_list))
                .for_each(|(contacts_id, section_id)| {
                    sections_in_contacts.insert_many_from_slice(contacts_id, section_id)
                });
            sections_in_contacts
        })
    }

    pub fn message_section_in_contacts(
        &self,
        contacts_id: u16,
    ) -> Vec<vo::message::MessageSectionConfig> {
        self._message_section_in_contacts()
            .get_vec(&contacts_id)
            .map(Vec::as_slice)
            .unwrap_or_default()
            .iter()
            .map(|&section_id| self.message_section_config(section_id))
            .map(Option::unwrap)
            .collect()
    }
}

macro_rules! group_field {
    ($field:ident, $group_id:ty => $typ:ty) => {
        group_field!($field, $group_id => $typ, paste::paste!(stringify!([<$field:camel>])));
    };
    ($field:ident, $group_id:ty => $typ:ty, $json:expr) => {
        paste::paste! {
            fn [<_$field>](&self) -> &FnvMultiMap<$group_id, po::$typ> {
                self.[<_$field>].get_or_init(|| {
                    self.load_to_group(concat!("ExcelOutput/", $json, ".json"))
                })
            }
            pub fn $field(&self, group_id: $group_id) -> Vec<vo::$typ> {
                self.[<_$field>]()
                    .get_vec(&group_id)
                    .map(Vec::as_slice)
                    .unwrap_or_default()
                    .iter()
                    .map(|po| po.vo(self))
                    .collect()
            }
        }
    };
}

impl GameData {
    group_field!(challenge_boss_reward_line, u16 => challenge::RewardLine);
    group_field!(challenge_maze_reward_line, u16 => challenge::RewardLine);
    group_field!(challenge_story_reward_line, u16 => challenge::RewardLine);
    group_field!(maze_buff, u32 => misc::MazeBuff);
    group_field!(rogue_maze_buff, u32 => misc::MazeBuff);
    group_field!(mapping_info, u32 => map::MappingInfo);
}

macro_rules! field {
    ($field:ident, $id:ty => $typ:ty) => {
        field!($field, $id => $typ, paste::paste!(stringify!([<$field:camel>])));
    };

    ($field:ident, $id:ty => $typ:ty, $json:expr $(, $candidates:expr)* ) => {
        paste::paste! {
            fn [<_$field>](&self) -> &FnvIndexMap<$id, po::$typ> {
                self.[<_$field>].get_or_init(|| {
                    let map = self.load_to_map(concat!("ExcelOutput/", $json, ".json"))
                    $(
                        .or_else(|_| self.load_to_map(concat!("ExcelOutput/", $candidates, ".json")))
                    )*;
                    if let Err(err) = &map {
                        if err.kind() == std::io::ErrorKind::NotFound {
                            return FnvIndexMap::default();
                        }
                    }
                    map.unwrap()
                })
            }
            pub fn $field(&self, id: $id) -> Option<vo::$typ> {
                self.[<_$field>]().get(&id).map(|po| po.vo(self))
            }
            pub fn [<list_$field>](&self) -> impl Iterator<Item = vo::$typ> + use<'_> {
                self.[<_$field>]().values().map(|po| po.vo(self))
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
    field!(maze_floor, u32 => map::MazeFloor);
    field!(maze_plane, u32 => map::MazePlane);
    field!(maze_prop, u32 => map::MazeProp);
    field!(world_data_config, u16 => map::WorldDataConfig, "WorldDataConfig", "WorldConfig");
    // message
    field!(emoji_config, u32 => message::EmojiConfig);
    field!(emoji_group, u8 => message::EmojiGroup);
    field!(message_contacts_camp, u8 => message::MessageContactsCamp);
    field!(message_contacts_config, u16 => message::MessageContactsConfig);
    field!(message_contacts_type, u8 => message::MessageContactsType);
    field!(message_group_config, u16 => message::MessageGroupConfig);
    field!(message_item_config, u32 => message::MessageItemConfig);
    field!(message_item_image, u32 => message::MessageItemImage);
    field!(message_section_config, u32 => message::MessageSectionConfig);
    // misc
    field!(extra_effect_config, u32 => misc::ExtraEffectConfig);
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
    field!(monster_camp, u8 => monster::Camp);
    field!(monster_config, u32 => monster::Config);
    field!(monster_difficulty_guide, u16 => monster::guide::Difficulty);
    field!(monster_guide_config, u32 => monster::guide::Config);
    field!(monster_guide_phase, u16 => monster::guide::Phase);
    field!(monster_guide_skill, u32 => monster::guide::Skill);
    field!(monster_guide_skill_text, u32 => monster::guide::SkillText);
    field!(monster_guide_tag, u32 => monster::guide::Tag);
    field!(monster_unique_config, u32 => monster::Config);
    field!(monster_skill_config, u32 => monster::SkillConfig);
    field!(monster_skill_unique_config, u32 => monster::SkillConfig);
    field!(monster_template_config, u32 => monster::TemplateConfig);
    field!(monster_template_unique_config, u32 => monster::TemplateConfig);
    field!(monster_text_guide, u16 => monster::guide::Text);
    field!(npc_monster_data, u32 => monster::NPCMonsterData, "NPCMonsterData");
    // rogue
    field!(rogue_handbook_miracle, u16 => rogue::RogueHandbookMiracle);
    field!(rogue_handbook_miracle_type, u16 => rogue::RogueHandbookMiracleType);
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
    // talk
    field!(talk_sentence_config, u32 => talk::TalkSentenceConfig);
    field!(voice_config, u32 => talk::VoiceConfig);
}
