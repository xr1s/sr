use base::{FnvHashMap, FnvIndexMap, FnvMultiMap, MainSubID, ID};

use std::fs::File;
use std::io::BufReader;
use std::num::NonZero;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::OnceLock;

#[derive(Default)]
pub struct GameData {
    base: PathBuf,
    text_map: std::collections::HashMap<i32, Arc<str>, fnv::FnvBuildHasher>,

    // battle
    // 战斗配置
    _battle_event_config: OnceLock<FnvIndexMap<u32, Arc<model::battle::BattleEventConfig>>>,
    _stage_config: OnceLock<FnvIndexMap<u32, Arc<model::battle::StageConfig>>>,
    _stage_infinite_group: OnceLock<FnvIndexMap<u32, Arc<model::battle::StageInfiniteGroup>>>,
    _stage_infinite_monster_group:
        OnceLock<FnvIndexMap<u32, Arc<model::battle::StageInfiniteMonsterGroup>>>,
    _stage_infinite_wave_config:
        OnceLock<FnvIndexMap<u32, Arc<model::battle::StageInfiniteWaveConfig>>>,
    // book
    // 游戏内阅读物
    _book_display_type: OnceLock<FnvIndexMap<u8, Arc<model::book::BookDisplayType>>>,
    _book_series_config: OnceLock<FnvIndexMap<u16, Arc<model::book::BookSeriesConfig>>>,
    _book_series_world: OnceLock<FnvIndexMap<u8, Arc<model::book::BookSeriesWorld>>>,
    _localbook_config: OnceLock<FnvIndexMap<u32, Arc<model::book::LocalbookConfig>>>,
    // challenge
    // 逐光捡金
    _challenge_boss_group_config:
        OnceLock<FnvIndexMap<u16, Arc<model::challenge::ChallengeGroupConfig>>>,
    _challenge_boss_group_extra:
        OnceLock<FnvIndexMap<u16, Arc<model::challenge::ChallengeGroupExtra>>>,
    _challenge_boss_maze_config:
        OnceLock<FnvIndexMap<u16, Arc<model::challenge::ChallengeMazeConfig>>>,
    _challenge_boss_maze_extra:
        OnceLock<FnvIndexMap<u16, Arc<model::challenge::ChallengeMazeExtra>>>,
    _challenge_boss_reward_line: OnceLock<FnvMultiMap<u16, Arc<model::challenge::RewardLine>>>,
    _challenge_boss_target_config:
        OnceLock<FnvIndexMap<u16, Arc<model::challenge::ChallengeTargetConfig>>>,
    _challenge_group_config:
        OnceLock<FnvIndexMap<u16, Arc<model::challenge::ChallengeGroupConfig>>>,
    _challenge_maze_reward_line: OnceLock<FnvMultiMap<u16, Arc<model::challenge::RewardLine>>>,
    _challenge_maze_group_extra:
        OnceLock<FnvIndexMap<u16, Arc<model::challenge::ChallengeGroupExtra>>>,
    _challenge_maze_config: OnceLock<FnvIndexMap<u16, Arc<model::challenge::ChallengeMazeConfig>>>,
    _challenge_story_group_config:
        OnceLock<FnvIndexMap<u16, Arc<model::challenge::ChallengeGroupConfig>>>,
    _challenge_story_group_extra:
        OnceLock<FnvIndexMap<u16, Arc<model::challenge::ChallengeGroupExtra>>>,
    _challenge_story_maze_config:
        OnceLock<FnvIndexMap<u16, Arc<model::challenge::ChallengeMazeConfig>>>,
    _challenge_story_maze_extra:
        OnceLock<FnvIndexMap<u16, Arc<model::challenge::ChallengeMazeExtra>>>,
    _challenge_story_reward_line: OnceLock<FnvMultiMap<u16, Arc<model::challenge::RewardLine>>>,
    _challenge_story_target_config:
        OnceLock<FnvIndexMap<u16, Arc<model::challenge::ChallengeTargetConfig>>>,
    _challenge_target_config:
        OnceLock<FnvIndexMap<u16, Arc<model::challenge::ChallengeTargetConfig>>>,
    // item
    /// 道具
    _item_config: OnceLock<FnvIndexMap<u32, Arc<model::item::ItemConfig>>>,
    _item_config_avatar_rank: OnceLock<FnvIndexMap<u32, Arc<model::item::ItemConfig>>>,
    _item_config_book: OnceLock<FnvIndexMap<u32, Arc<model::item::ItemConfig>>>,
    _item_config_equipment: OnceLock<FnvIndexMap<u32, Arc<model::item::ItemConfig>>>,
    /// 道具使用效果
    _item_use_data: OnceLock<FnvIndexMap<u32, Arc<model::item::ItemUseData>>>,
    // map
    _map_entrance: OnceLock<FnvIndexMap<u32, Arc<model::map::MapEntrance>>>,
    _mapping_info: OnceLock<FnvMultiMap<u32, Arc<model::map::MappingInfo>>>,
    _maze_floor: OnceLock<FnvIndexMap<u32, Arc<model::map::MazeFloor>>>,
    _maze_plane: OnceLock<FnvIndexMap<u32, Arc<model::map::MazePlane>>>,
    _maze_prop: OnceLock<FnvIndexMap<u32, Arc<model::map::MazeProp>>>,
    _world_data_config: OnceLock<FnvIndexMap<u16, Arc<model::map::WorldDataConfig>>>,
    // message
    _emoji_config: OnceLock<FnvIndexMap<u32, Arc<model::message::EmojiConfig>>>,
    _emoji_group: OnceLock<FnvIndexMap<u8, Arc<model::message::EmojiGroup>>>,
    _message_contacts_camp: OnceLock<FnvIndexMap<u8, Arc<model::message::MessageContactsCamp>>>,
    _message_contacts_config:
        OnceLock<FnvIndexMap<u16, Arc<model::message::MessageContactsConfig>>>,
    _message_contacts_type: OnceLock<FnvIndexMap<u8, Arc<model::message::MessageContactsType>>>,
    _message_group_config: OnceLock<FnvIndexMap<u16, Arc<model::message::MessageGroupConfig>>>,
    _message_item_config: OnceLock<FnvIndexMap<u32, Arc<model::message::MessageItemConfig>>>,
    _message_item_image: OnceLock<FnvIndexMap<u32, Arc<model::message::MessageItemImage>>>,
    _message_section_config: OnceLock<FnvIndexMap<u32, Arc<model::message::MessageSectionConfig>>>,
    // misc
    /// 效果说明
    _extra_effect_config: OnceLock<FnvIndexMap<u32, Arc<model::misc::ExtraEffectConfig>>>,
    _maze_buff: OnceLock<FnvMultiMap<u32, Arc<model::misc::MazeBuff>>>,
    _reward_data: OnceLock<FnvIndexMap<u32, Arc<model::misc::RewardData>>>,
    _schedule_data_challenge_boss: OnceLock<FnvIndexMap<u32, Arc<model::misc::ScheduleData>>>,
    _schedule_data_challenge_maze: OnceLock<FnvIndexMap<u32, Arc<model::misc::ScheduleData>>>,
    _schedule_data_challenge_story: OnceLock<FnvIndexMap<u32, Arc<model::misc::ScheduleData>>>,
    _schedule_data_global: OnceLock<FnvIndexMap<u32, Arc<model::misc::ScheduleDataGlobal>>>,
    _text_join_config: OnceLock<FnvIndexMap<u8, Arc<model::misc::TextJoinConfig>>>,
    _text_join_item: OnceLock<FnvIndexMap<u16, Arc<model::misc::TextJoinItem>>>,
    // mission
    _main_mission: OnceLock<FnvIndexMap<u32, Arc<model::mission::MainMission>>>,
    _mission_chapter_config: OnceLock<FnvIndexMap<u32, Arc<model::mission::MissionChapterConfig>>>,
    _sub_mission: OnceLock<FnvIndexMap<u32, Arc<model::mission::SubMission>>>,
    // monster
    _elite_group: OnceLock<FnvIndexMap<u16, Arc<model::monster::EliteGroup>>>,
    _hard_level_group: OnceLock<FnvMultiMap<u16, Arc<model::monster::HardLevelGroup>>>,
    _monster_camp: OnceLock<FnvIndexMap<u8, Arc<model::monster::MonsterCamp>>>,
    _monster_config: OnceLock<FnvIndexMap<u32, Arc<model::monster::MonsterConfig>>>,
    _monster_skill_config: OnceLock<FnvIndexMap<u32, Arc<model::monster::SkillConfig>>>,
    _monster_skill_unique_config: OnceLock<FnvIndexMap<u32, Arc<model::monster::SkillConfig>>>,
    _monster_template_config:
        OnceLock<FnvIndexMap<u32, Arc<model::monster::MonsterTemplateConfig>>>,
    _monster_template_unique_config:
        OnceLock<FnvIndexMap<u32, Arc<model::monster::MonsterTemplateConfig>>>,
    _monster_unique_config: OnceLock<FnvIndexMap<u32, Arc<model::monster::MonsterConfig>>>,
    _npc_monster_data: OnceLock<FnvIndexMap<u32, Arc<model::monster::NPCMonsterData>>>,
    // monster guide
    _monster_difficulty_guide:
        OnceLock<FnvIndexMap<u16, Arc<model::monster::guide::MonsterDifficultyGuide>>>,
    _monster_guide_config:
        OnceLock<FnvIndexMap<u32, Arc<model::monster::guide::MonsterGuideConfig>>>,
    _monster_guide_phase: OnceLock<FnvIndexMap<u16, Arc<model::monster::guide::MonsterGuidePhase>>>,
    _monster_guide_skill: OnceLock<FnvIndexMap<u32, Arc<model::monster::guide::MonsterGuideSkill>>>,
    _monster_guide_skill_text:
        OnceLock<FnvIndexMap<u32, Arc<model::monster::guide::MonsterGuideSkillText>>>,
    _monster_guide_tag: OnceLock<FnvIndexMap<u32, Arc<model::monster::guide::MonsterGuideTag>>>,
    _monster_text_guide: OnceLock<FnvIndexMap<u16, Arc<model::monster::guide::MonsterTextGuide>>>,
    // rogue
    // 模拟宇宙
    _rogue_buff: OnceLock<FnvMultiMap<u32, Arc<model::rogue::RogueBuff>>>,
    _rogue_buff_type: OnceLock<FnvIndexMap<u8, Arc<model::rogue::RogueBuffType>>>,
    _rogue_extra_config: OnceLock<FnvIndexMap<u32, Arc<model::misc::ExtraEffectConfig>>>,
    _rogue_handbook_miracle: OnceLock<FnvIndexMap<u16, Arc<model::rogue::RogueHandbookMiracle>>>,
    _rogue_handbook_miracle_type:
        OnceLock<FnvIndexMap<u16, Arc<model::rogue::RogueHandbookMiracleType>>>,
    /// 模拟宇宙祝福
    _rogue_maze_buff: OnceLock<FnvMultiMap<u32, Arc<model::misc::MazeBuff>>>,
    /// 模拟宇宙奇物
    _rogue_miracle: OnceLock<FnvIndexMap<u16, Arc<model::rogue::RogueMiracle>>>,
    _rogue_miracle_display: OnceLock<FnvIndexMap<u16, Arc<model::rogue::RogueMiracleDisplay>>>,
    _rogue_monster: OnceLock<FnvIndexMap<u32, Arc<model::rogue::RogueMonster>>>,
    _rogue_monster_group: OnceLock<FnvIndexMap<u32, Arc<model::rogue::RogueMonsterGroup>>>,
    // rogue magic
    // 模拟宇宙：不可知域
    /// 不可知域奇物
    _rogue_magic_miracle: OnceLock<FnvIndexMap<u16, Arc<model::rogue::RogueMiracle>>>,
    // rogue tourn 差分宇宙
    _rogue_bonus: OnceLock<FnvIndexMap<u16, Arc<model::rogue::tourn::RogueBonus>>>,
    _rogue_tourn_buff: OnceLock<FnvMultiMap<u32, Arc<model::rogue::tourn::RogueTournBuff>>>,
    _rogue_tourn_buff_type: OnceLock<FnvIndexMap<u8, Arc<model::rogue::tourn::RogueTournBuffType>>>,
    /// 差分宇宙文案
    _rogue_tourn_content_display:
        OnceLock<FnvIndexMap<u16, Arc<model::rogue::tourn::RogueTournContentDisplay>>>,
    /// 差分宇宙方程
    _rogue_tourn_formula: OnceLock<FnvIndexMap<u32, Arc<model::rogue::tourn::RogueTournFormula>>>,
    _rogue_tourn_formula_display:
        OnceLock<FnvIndexMap<u32, Arc<model::rogue::tourn::RogueTournFormulaDisplay>>>,
    _rogue_tourn_handbook_miracle:
        OnceLock<FnvIndexMap<u16, Arc<model::rogue::tourn::RogueTournHandbookMiracle>>>,
    /// 差分宇宙奇物
    _rogue_tourn_miracle: OnceLock<FnvIndexMap<u16, Arc<model::rogue::tourn::RogueTournMiracle>>>,
    _rogue_tourn_miracle_display:
        OnceLock<FnvIndexMap<u16, Arc<model::rogue::RogueMiracleDisplay>>>,
    /// 差分宇宙周期演算
    _rogue_tourn_weekly_challenge:
        OnceLock<FnvIndexMap<u8, Arc<model::rogue::tourn::RogueTournWeeklyChallenge>>>,
    _rogue_tourn_weekly_display:
        OnceLock<FnvIndexMap<u16, Arc<model::rogue::tourn::RogueTournWeeklyDisplay>>>,
    // talk
    _talk_sentence_config: OnceLock<FnvIndexMap<u32, Arc<model::talk::TalkSentenceConfig>>>,
    _voice_config: OnceLock<FnvIndexMap<u32, Arc<model::talk::VoiceConfig>>>,

    // 大多是二次聚合过，方便数据处理用的缓存
    /// 同一期逐光捡金的每一层数据
    /// 按照 ChallengeMazeConfig 中的 GroupID 重新聚合 ChallengeMazeConfig
    /// 其中 GroupID 字段就是 ChallengeGroupConfig 中的 ID 外键
    _challenge_maze_in_group:
        OnceLock<FnvMultiMap<u16, Arc<model::challenge::ChallengeMazeConfig>>>,
    /// 按照 LocalbookConfig 中 SeriesID 聚合的 LocalbookConfig
    _localbook_in_book_series: OnceLock<FnvMultiMap<u16, Arc<model::book::LocalbookConfig>>>,
    /// 按照 MonsterConfig 中的 GroupID 字段重新聚合 MonsterConfig
    _monster_template_config_group:
        OnceLock<FnvMultiMap<u32, Arc<model::monster::MonsterTemplateConfig>>>,
    /// 按照 MessageGroupConfig 中的 MessageContactsID 和 MessageSectionIDList 将联系人和消息关联起来
    /// 形成从联系人到消息的映射
    _message_section_in_contacts:
        OnceLock<FnvMultiMap<u16, Arc<model::message::MessageSectionConfig>>>,
    _message_contacts_of_section:
        OnceLock<FnvHashMap<u32, Arc<model::message::MessageContactsConfig>>>,

    // 名称反向索引
    /// 按名称索引的 ExtraEffectConfig
    _extra_effect_config_by_name:
        OnceLock<FnvHashMap<Arc<str>, Arc<model::misc::ExtraEffectConfig>>>,
    _rogue_buff_by_name: OnceLock<FnvHashMap<Arc<str>, Arc<model::rogue::RogueBuff>>>,
    _rogue_tourn_buff_by_name:
        OnceLock<FnvHashMap<Arc<str>, Arc<model::rogue::tourn::RogueTournBuff>>>,
}

impl std::fmt::Debug for GameData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GameData")
            .field("base", &self.base)
            .finish_non_exhaustive()
    }
}

pub trait Text {
    fn text(&self, text: model::Text) -> &str;
}

impl Text for GameData {
    fn text(&self, text: model::Text) -> &str {
        self.text_map
            .get(&text.hash)
            .map(Arc::as_ref)
            .unwrap_or_default()
    }
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

    fn load<K, V>(&self, dir: &str) -> std::io::Result<FnvIndexMap<K, Arc<V>>>
    where
        K: std::cmp::Eq + std::hash::Hash,
        V: ID<ID = K>,
        for<'a> K: serde::Deserialize<'a>,
        for<'a> V: serde::Deserialize<'a>,
    {
        let path = self.base.join(dir);
        let bytes = std::fs::read(path)?;
        Ok(serde_json::from_slice(&bytes)
            // 仅应在处理 2.3 版本及以下的数据集时输出错误
            // 每个版本更新后也存在某些特殊字段未解密导致一直在变 serde 失败的情况
            // 具体搜 "serde(alias" 字符串。每个版本更新后初始化一下日志看看是什么报错
            .inspect_err(|e| log::warn!("疑似 2.3 之前的老数据格式: {:?}", e))
            .map_or_else(
                // 2.3 及以下, 采用的数据结构是 {"123": {"ID": 123, ...} } 形式
                |_| serde_json::from_slice::<FnvIndexMap<K, Arc<V>>>(&bytes).unwrap(),
                // 2.4 及以上, 采用的数据结构是 [ {"ID": 123, ...} ] 形式
                |model: Vec<Arc<V>>| model.into_iter().map(|model| (model.id(), model)).collect(),
            ))
    }

    fn load_main_sub<I, S, V>(&self, dir: &str) -> std::io::Result<FnvMultiMap<I, Arc<V>>>
    where
        I: std::cmp::Eq + std::hash::Hash,
        S: std::cmp::Eq + std::hash::Hash,
        V: MainSubID<ID = I, SubID = S>,
        for<'a> I: serde::Deserialize<'a>,
        for<'a> S: serde::Deserialize<'a>,
        for<'a> V: serde::Deserialize<'a>,
    {
        let path = self.base.join(dir);
        let bytes = std::fs::read(path)?;
        Ok(serde_json::from_slice(&bytes)
            // 仅应在处理 2.3 版本及以下的数据集时输出错误
            .inspect_err(|e| log::warn!("疑似 2.3 之前的老数据格式: {:?}", e))
            .map_or_else(
                // 2.3 版本及以下, 采用的数据结构是 {"123": { "4": { "MainID": 123, "SubID": 4, ... } } } 形式
                |_| {
                    serde_json::from_slice::<FnvIndexMap<I, FnvIndexMap<S, Arc<V>>>>(&bytes)
                        .unwrap()
                        .into_values()
                        .flat_map(FnvIndexMap::into_values)
                        .map(|model| (model.id(), model))
                        .collect()
                },
                // 2.4 版本及以上, 采用的数据结构是 [{"MainID": 123, "SubID": 4, ...} ] 摊平的形式
                |model: Vec<Arc<V>>| model.into_iter().map(|model| (model.id(), model)).collect(),
            ))
    }
}

macro_rules! declare {
    ($field:ident, $id:ty => $typ:path) => {
        fn $field(&self) -> &FnvIndexMap<$id, Arc<paste::paste!(model::$typ)>>;
    };
}

macro_rules! main_sub_declare {
    ($field:ident, $id:ty => $typ:ty) => {
        fn $field(&self) -> &FnvMultiMap<$id, Arc<paste::paste!(model::$typ)>>;
    };
}

pub trait SealedGameData {
    // battle
    declare!(_battle_event_config, u32 => battle::BattleEventConfig);
    declare!(_stage_infinite_group, u32 => battle::StageInfiniteGroup);
    declare!(_stage_infinite_monster_group, u32 => battle::StageInfiniteMonsterGroup);
    declare!(_stage_infinite_wave_config, u32 => battle::StageInfiniteWaveConfig);
    declare!(_stage_config, u32 => battle::StageConfig);
    // book
    declare!(_book_display_type, u8 => book::BookDisplayType);
    declare!(_book_series_config, u16 => book::BookSeriesConfig);
    declare!(_book_series_world, u8 => book::BookSeriesWorld);
    declare!(_localbook_config, u32 => book::LocalbookConfig);
    // challenge
    declare!(_challenge_boss_group_config, u16 => challenge::ChallengeGroupConfig);
    declare!(_challenge_boss_group_extra, u16 => challenge::ChallengeGroupExtra);
    declare!(_challenge_boss_maze_config, u16 => challenge::ChallengeMazeConfig);
    declare!(_challenge_boss_maze_extra, u16 => challenge::ChallengeMazeExtra);
    main_sub_declare!(_challenge_boss_reward_line, u16 => challenge::RewardLine);
    declare!(_challenge_boss_target_config, u16 => challenge::ChallengeTargetConfig);
    declare!(_challenge_group_config, u16 => challenge::ChallengeGroupConfig);
    declare!(_challenge_maze_config, u16 => challenge::ChallengeMazeConfig);
    declare!(_challenge_maze_group_extra, u16 => challenge::ChallengeGroupExtra);
    main_sub_declare!(_challenge_maze_reward_line, u16 => challenge::RewardLine);
    declare!(_challenge_story_group_config, u16 => challenge::ChallengeGroupConfig);
    declare!(_challenge_story_group_extra, u16 => challenge::ChallengeGroupExtra);
    declare!(_challenge_story_maze_config, u16 => challenge::ChallengeMazeConfig);
    declare!(_challenge_story_maze_extra, u16 => challenge::ChallengeMazeExtra);
    main_sub_declare!(_challenge_story_reward_line, u16 => challenge::RewardLine);
    declare!(_challenge_story_target_config, u16 => challenge::ChallengeTargetConfig);
    declare!(_challenge_target_config, u16 => challenge::ChallengeTargetConfig);
    // item
    declare!(_item_config, u32 => item::ItemConfig);
    declare!(_item_config_avatar_rank, u32 => item::ItemConfig);
    declare!(_item_config_book, u32 => item::ItemConfig);
    declare!(_item_config_equipment, u32 => item::ItemConfig);
    declare!(_item_use_data, u32 => item::ItemUseData);
    // map
    declare!(_map_entrance, u32 => map::MapEntrance);
    main_sub_declare!(_mapping_info, u32 => map::MappingInfo);
    declare!(_maze_floor, u32 => map::MazeFloor);
    declare!(_maze_plane, u32 => map::MazePlane);
    declare!(_maze_prop, u32 => map::MazeProp);
    declare!(_world_data_config, u16 => map::WorldDataConfig);
    // message
    declare!(_emoji_config, u32 => message::EmojiConfig);
    declare!(_emoji_group, u8 => message::EmojiGroup);
    declare!(_message_contacts_camp, u8 => message::MessageContactsCamp);
    declare!(_message_contacts_config, u16 => message::MessageContactsConfig);
    declare!(_message_contacts_type, u8 => message::MessageContactsType);
    declare!(_message_group_config, u16 => message::MessageGroupConfig);
    declare!(_message_item_config, u32 => message::MessageItemConfig);
    declare!(_message_item_image, u32 => message::MessageItemImage);
    declare!(_message_section_config, u32 => message::MessageSectionConfig);
    // misc
    declare!(_extra_effect_config, u32 => misc::ExtraEffectConfig);
    main_sub_declare!(_maze_buff, u32 => misc::MazeBuff);
    declare!(_reward_data, u32 => misc::RewardData);
    declare!(_schedule_data_challenge_boss, u32 => misc::ScheduleData);
    declare!(_schedule_data_challenge_maze, u32 => misc::ScheduleData);
    declare!(_schedule_data_challenge_story, u32 => misc::ScheduleData);
    declare!(_schedule_data_global, u32 => misc::ScheduleDataGlobal);
    declare!(_text_join_config, u8 => misc::TextJoinConfig);
    declare!(_text_join_item, u16 => misc::TextJoinItem);
    // mission
    declare!(_main_mission, u32 => mission::MainMission);
    declare!(_mission_chapter_config, u32 => mission::MissionChapterConfig);
    declare!(_sub_mission, u32 => mission::SubMission);
    // monster
    declare!(_elite_group, u16 => monster::EliteGroup);
    main_sub_declare!(_hard_level_group, u16 => monster::HardLevelGroup);
    declare!(_monster_camp, u8 => monster::MonsterCamp);
    declare!(_monster_config, u32 => monster::MonsterConfig);
    declare!(_monster_unique_config, u32 => monster::MonsterConfig);
    declare!(_monster_skill_config, u32 => monster::SkillConfig);
    declare!(_monster_skill_unique_config, u32 => monster::SkillConfig);
    declare!(_monster_template_config, u32 => monster::MonsterTemplateConfig);
    declare!(_monster_template_unique_config, u32 => monster::MonsterTemplateConfig);
    declare!(_npc_monster_data, u32 => monster::NPCMonsterData);
    // monster guide
    declare!(_monster_difficulty_guide, u16 => monster::guide::MonsterDifficultyGuide);
    declare!(_monster_guide_config, u32 => monster::guide::MonsterGuideConfig);
    declare!(_monster_guide_phase, u16 => monster::guide::MonsterGuidePhase);
    declare!(_monster_guide_skill, u32 => monster::guide::MonsterGuideSkill);
    declare!(_monster_guide_skill_text, u32 => monster::guide::MonsterGuideSkillText);
    declare!(_monster_guide_tag, u32 => monster::guide::MonsterGuideTag);
    declare!(_monster_text_guide, u16 => monster::guide::MonsterTextGuide);
    // rogue
    main_sub_declare!(_rogue_buff, u32 => rogue::RogueBuff);
    declare!(_rogue_buff_type, u8 => rogue::RogueBuffType);
    declare!(_rogue_extra_config, u32 => misc::ExtraEffectConfig);
    declare!(_rogue_handbook_miracle, u16 => rogue::RogueHandbookMiracle);
    declare!(_rogue_handbook_miracle_type, u16 => rogue::RogueHandbookMiracleType);
    main_sub_declare!(_rogue_maze_buff, u32 => misc::MazeBuff);
    declare!(_rogue_miracle, u16 => rogue::RogueMiracle);
    declare!(_rogue_miracle_display, u16 => rogue::RogueMiracleDisplay);
    declare!(_rogue_monster, u32 => rogue::RogueMonster);
    declare!(_rogue_monster_group, u32 => rogue::RogueMonsterGroup);
    // rogue magic
    declare!(_rogue_magic_miracle, u16 => rogue::RogueMiracle);
    // rogue tourn
    declare!(_rogue_bonus, u16 => rogue::tourn::RogueBonus);
    main_sub_declare!(_rogue_tourn_buff, u32 => rogue::tourn::RogueTournBuff);
    declare!(_rogue_tourn_buff_type, u8 => rogue::tourn::RogueTournBuffType);
    declare!(_rogue_tourn_content_display, u16 => rogue::tourn::RogueTournContentDisplay);
    declare!(_rogue_tourn_formula, u32 => rogue::tourn::RogueTournFormula);
    declare!(_rogue_tourn_formula_display, u32 => rogue::tourn::RogueTournFormulaDisplay);
    declare!(_rogue_tourn_handbook_miracle, u16 => rogue::tourn::RogueTournHandbookMiracle);
    declare!(_rogue_tourn_miracle, u16 => rogue::tourn::RogueTournMiracle);
    declare!(_rogue_tourn_miracle_display, u16 => rogue::RogueMiracleDisplay);
    declare!(_rogue_tourn_weekly_challenge, u8 => rogue::tourn::RogueTournWeeklyChallenge);
    declare!(_rogue_tourn_weekly_display, u16 => rogue::tourn::RogueTournWeeklyDisplay);
    // talk
    declare!(_talk_sentence_config, u32 => talk::TalkSentenceConfig);
    declare!(_voice_config, u32 => talk::VoiceConfig);

    #[rustfmt::skip]
    fn _monster_template_config_group(&self) -> &FnvMultiMap<u32, Arc<model::monster::MonsterTemplateConfig>>;
    #[rustfmt::skip]
    fn _challenge_maze_in_group(&self) -> &FnvMultiMap<u16, Arc<model::challenge::ChallengeMazeConfig>>;
    fn _localbook_in_book_series(&self) -> &FnvMultiMap<u16, Arc<model::book::LocalbookConfig>>;
    #[rustfmt::skip]
    fn _message_section_in_contacts(&self) -> &FnvMultiMap<u16, Arc<model::message::MessageSectionConfig>>;
    #[rustfmt::skip]
    fn _message_contacts_of_section(&self) -> &FnvHashMap<u32, Arc<model::message::MessageContactsConfig>>;

    #[rustfmt::skip]
    fn _current_challenge_group_config<F>(&self, iter: F)
        -> Option<&model::challenge::ChallengeGroupConfig>
    where
        F: Fn(&crate::GameData) -> &FnvIndexMap<u16, Arc<model::challenge::ChallengeGroupConfig>>;

    // 名称反向索引
    /// 按名称索引的 ExtraEffectConfig
    #[rustfmt::skip]
    fn _extra_effect_config_by_name(&self) -> &FnvHashMap<Arc<str>, Arc<model::misc::ExtraEffectConfig>>;
    fn _rogue_buff_by_name(&self) -> &FnvHashMap<Arc<str>, Arc<model::rogue::RogueBuff>>;
    #[rustfmt::skip]
    fn _rogue_tourn_buff_by_name(&self) -> &FnvHashMap<Arc<str>, Arc<model::rogue::tourn::RogueTournBuff>>;
}

macro_rules! implement {
    ($field:ident, $id:ty => $typ:path) => {
        implement!($field, $id => $typ, paste::paste!(stringify!([<$field:camel>])));
    };

    ($field:ident, $id:ty => $typ:path, $json:expr $(, $candidates:expr)* ) => {
        fn $field(&self) -> &FnvIndexMap<$id, Arc<paste::paste!(model::$typ)>> {
            self.$field.get_or_init(|| {
                self.load(concat!("ExcelOutput/", $json, ".json"))
                $(
                    .or_else(|_| self.load(concat!("ExcelOutput/", $candidates, ".json")))
                )*
                    .or_else(|err| {
                        if err.kind() == std::io::ErrorKind::NotFound {
                            // 很无奈，因为存在无此文件的情况（随着版本更新新增的文件）
                            // 这里只好默认不存在的文件均为这种数据，并返回空
                            return Ok(FnvIndexMap::default());
                        }
                        Err(err)
                    })
                    .unwrap()
            })
        }
    };
}

macro_rules! main_sub_implement {
    ($field:ident, $id:ty => $typ:ty) => {
        main_sub_implement!($field, $id => $typ, paste::paste!(stringify!([<$field:camel>])));
    };
    ($field:ident, $id:ty => $typ:ty, $json:expr) => {
        fn $field(&self) -> &FnvMultiMap<$id, Arc<paste::paste!(model::$typ)>> {
            self.$field.get_or_init(|| {
                self.load_main_sub(concat!("ExcelOutput/", $json, ".json"))
                    .or_else(|err| {
                        if err.kind() == std::io::ErrorKind::NotFound {
                            // 很无奈，因为存在无此文件的情况（随着版本更新新增的文件）
                            // 这里只好默认不存在的文件均为这种数据，并返回空
                            return Ok(FnvMultiMap::default());
                        }
                        Err(err)
                    })
                    .unwrap()
            })
        }
    };
}

impl SealedGameData for GameData {
    // battle
    implement!(_battle_event_config, u32 => battle::BattleEventConfig);
    implement!(_stage_infinite_group, u32 => battle::StageInfiniteGroup);
    implement!(_stage_infinite_monster_group, u32 => battle::StageInfiniteMonsterGroup);
    implement!(_stage_infinite_wave_config, u32 => battle::StageInfiniteWaveConfig);
    implement!(_stage_config, u32 => battle::StageConfig);
    // book
    implement!(_book_display_type, u8 => book::BookDisplayType);
    implement!(_book_series_config, u16 => book::BookSeriesConfig);
    implement!(_book_series_world, u8 => book::BookSeriesWorld);
    implement!(_localbook_config, u32 => book::LocalbookConfig);
    // challenge
    implement!(_challenge_boss_group_config, u16 => challenge::ChallengeGroupConfig);
    implement!(_challenge_boss_group_extra, u16 => challenge::ChallengeGroupExtra);
    implement!(_challenge_boss_maze_config, u16 => challenge::ChallengeMazeConfig);
    implement!(_challenge_boss_maze_extra, u16 => challenge::ChallengeMazeExtra);
    main_sub_implement!(_challenge_boss_reward_line, u16 => challenge::RewardLine);
    implement!(_challenge_boss_target_config, u16 => challenge::ChallengeTargetConfig);
    implement!(_challenge_group_config, u16 => challenge::ChallengeGroupConfig);
    implement!(_challenge_maze_config, u16 => challenge::ChallengeMazeConfig);
    implement!(_challenge_maze_group_extra, u16 => challenge::ChallengeGroupExtra);
    main_sub_implement!(_challenge_maze_reward_line, u16 => challenge::RewardLine);
    implement!(_challenge_story_group_config, u16 => challenge::ChallengeGroupConfig);
    implement!(_challenge_story_group_extra, u16 => challenge::ChallengeGroupExtra);
    implement!(_challenge_story_maze_config, u16 => challenge::ChallengeMazeConfig);
    implement!(_challenge_story_maze_extra, u16 => challenge::ChallengeMazeExtra);
    main_sub_implement!(_challenge_story_reward_line, u16 => challenge::RewardLine);
    implement!(_challenge_story_target_config, u16 => challenge::ChallengeTargetConfig);
    implement!(_challenge_target_config, u16 => challenge::ChallengeTargetConfig);
    // item
    implement!(_item_config, u32 => item::ItemConfig);
    implement!(_item_config_avatar_rank, u32 => item::ItemConfig);
    implement!(_item_config_book, u32 => item::ItemConfig);
    implement!(_item_config_equipment, u32 => item::ItemConfig);
    implement!(_item_use_data, u32 => item::ItemUseData);
    // map
    implement!(_map_entrance, u32 => map::MapEntrance);
    main_sub_implement!(_mapping_info, u32 => map::MappingInfo);
    implement!(_maze_floor, u32 => map::MazeFloor);
    implement!(_maze_plane, u32 => map::MazePlane);
    implement!(_maze_prop, u32 => map::MazeProp);
    implement!(_world_data_config, u16 => map::WorldDataConfig, "WorldDataConfig", "WorldConfig");
    // message
    implement!(_emoji_config, u32 => message::EmojiConfig);
    implement!(_emoji_group, u8 => message::EmojiGroup);
    implement!(_message_contacts_camp, u8 => message::MessageContactsCamp);
    implement!(_message_contacts_config, u16 => message::MessageContactsConfig);
    implement!(_message_contacts_type, u8 => message::MessageContactsType);
    implement!(_message_group_config, u16 => message::MessageGroupConfig);
    implement!(_message_item_config, u32 => message::MessageItemConfig);
    implement!(_message_item_image, u32 => message::MessageItemImage);
    implement!(_message_section_config, u32 => message::MessageSectionConfig);
    // misc
    implement!(_extra_effect_config, u32 => misc::ExtraEffectConfig);
    main_sub_implement!(_maze_buff, u32 => misc::MazeBuff);
    implement!(_reward_data, u32 => misc::RewardData);
    implement!(_schedule_data_challenge_boss, u32 => misc::ScheduleData);
    implement!(_schedule_data_challenge_maze, u32 => misc::ScheduleData);
    implement!(_schedule_data_challenge_story, u32 => misc::ScheduleData);
    implement!(_schedule_data_global, u32 => misc::ScheduleDataGlobal);
    implement!(_text_join_config, u8 => misc::TextJoinConfig);
    implement!(_text_join_item, u16 => misc::TextJoinItem);
    // mission
    implement!(_main_mission, u32 => mission::MainMission);
    implement!(_mission_chapter_config, u32 => mission::MissionChapterConfig);
    implement!(_sub_mission, u32 => mission::SubMission);
    // monster
    implement!(_elite_group, u16 => monster::EliteGroup);
    main_sub_implement!(_hard_level_group, u16 => monster::HardLevelGroup);
    implement!(_monster_camp, u8 => monster::MonsterCamp);
    implement!(_monster_config, u32 => monster::MonsterConfig);
    implement!(_monster_unique_config, u32 => monster::MonsterConfig);
    implement!(_monster_skill_config, u32 => monster::SkillConfig);
    implement!(_monster_skill_unique_config, u32 => monster::SkillConfig);
    implement!(_monster_template_config, u32 => monster::MonsterTemplateConfig);
    implement!(_monster_template_unique_config, u32 => monster::MonsterTemplateConfig);
    implement!(_npc_monster_data, u32 => monster::NPCMonsterData, "NPCMonsterData");
    // monster guide
    implement!(_monster_difficulty_guide, u16 => monster::guide::MonsterDifficultyGuide);
    implement!(_monster_guide_config, u32 => monster::guide::MonsterGuideConfig);
    implement!(_monster_guide_phase, u16 => monster::guide::MonsterGuidePhase);
    implement!(_monster_guide_skill, u32 => monster::guide::MonsterGuideSkill);
    implement!(_monster_guide_skill_text, u32 => monster::guide::MonsterGuideSkillText);
    implement!(_monster_guide_tag, u32 => monster::guide::MonsterGuideTag);
    implement!(_monster_text_guide, u16 => monster::guide::MonsterTextGuide);
    // rogue
    main_sub_implement!(_rogue_buff, u32 => rogue::RogueBuff);
    implement!(_rogue_buff_type, u8 => rogue::RogueBuffType);
    implement!(_rogue_extra_config, u32 => misc::ExtraEffectConfig);
    implement!(_rogue_handbook_miracle, u16 => rogue::RogueHandbookMiracle);
    implement!(_rogue_handbook_miracle_type, u16 => rogue::RogueHandbookMiracleType);
    main_sub_implement!(_rogue_maze_buff, u32 => misc::MazeBuff);
    implement!(_rogue_miracle, u16 => rogue::RogueMiracle);
    implement!(_rogue_miracle_display, u16 => rogue::RogueMiracleDisplay);
    implement!(_rogue_monster, u32 => rogue::RogueMonster);
    implement!(_rogue_monster_group, u32 => rogue::RogueMonsterGroup);
    // rogue magic
    implement!(_rogue_magic_miracle, u16 => rogue::RogueMiracle);
    // rogue tourn
    implement!(_rogue_bonus, u16 => rogue::tourn::RogueBonus);
    main_sub_implement!(_rogue_tourn_buff, u32 => rogue::tourn::RogueTournBuff);
    implement!(_rogue_tourn_buff_type, u8 => rogue::tourn::RogueTournBuffType);
    implement!(_rogue_tourn_content_display, u16 => rogue::tourn::RogueTournContentDisplay);
    implement!(_rogue_tourn_formula, u32 => rogue::tourn::RogueTournFormula);
    implement!(_rogue_tourn_formula_display, u32 => rogue::tourn::RogueTournFormulaDisplay);
    implement!(_rogue_tourn_handbook_miracle, u16 => rogue::tourn::RogueTournHandbookMiracle);
    implement!(_rogue_tourn_miracle, u16 => rogue::tourn::RogueTournMiracle);
    implement!(_rogue_tourn_miracle_display, u16 => rogue::RogueMiracleDisplay);
    implement!(_rogue_tourn_weekly_challenge, u8 => rogue::tourn::RogueTournWeeklyChallenge);
    implement!(_rogue_tourn_weekly_display, u16 => rogue::tourn::RogueTournWeeklyDisplay);
    // talk
    implement!(_talk_sentence_config, u32 => talk::TalkSentenceConfig);
    implement!(_voice_config, u32 => talk::VoiceConfig);

    fn _monster_template_config_group(
        &self,
    ) -> &FnvMultiMap<u32, Arc<model::monster::MonsterTemplateConfig>> {
        self._monster_template_config_group.get_or_init(|| {
            self._monster_template_config()
                .values()
                .filter(|monster| monster.template_group_id.is_some())
                .map(|monster| (monster.template_group_id.unwrap().get(), monster.clone()))
                .collect()
        })
    }

    fn _challenge_maze_in_group(
        &self,
    ) -> &FnvMultiMap<u16, Arc<model::challenge::ChallengeMazeConfig>> {
        self._challenge_maze_in_group.get_or_init(|| {
            std::iter::empty()
                .chain(self._challenge_maze_config().values())
                .chain(self._challenge_story_maze_config().values())
                .chain(self._challenge_boss_maze_config().values())
                .map(|maze| (maze.group_id, Arc::clone(maze)))
                .collect()
        })
    }

    fn _current_challenge_group_config<F>(
        &self,
        iter: F,
    ) -> Option<&model::challenge::ChallengeGroupConfig>
    where
        F: Fn(&GameData) -> &FnvIndexMap<u16, Arc<model::challenge::ChallengeGroupConfig>>,
    {
        let now = chrono::Local::now();
        iter(self).values().map(Arc::as_ref).find(|challenge| {
            use model::challenge::ChallengeGroupType;
            let schedule = match challenge.challenge_group_type {
                ChallengeGroupType::Memory => self._schedule_data_challenge_maze(),
                ChallengeGroupType::Story => self._schedule_data_challenge_story(),
                ChallengeGroupType::Boss => self._schedule_data_challenge_boss(),
            };
            let schedule_id = challenge
                .schedule_data_id
                .map(NonZero::get)
                .unwrap_or_default();
            schedule
                .get(&schedule_id)
                .map(|sched| sched.begin_time <= now && now <= sched.end_time)
                .unwrap_or_default()
        })
    }

    fn _localbook_in_book_series(&self) -> &FnvMultiMap<u16, Arc<model::book::LocalbookConfig>> {
        self._localbook_in_book_series.get_or_init(|| {
            self._localbook_config()
                .values()
                .map(|book| (book.book_series_id, Arc::clone(book)))
                .collect()
        })
    }

    fn _message_section_in_contacts(
        &self,
    ) -> &FnvMultiMap<u16, Arc<model::message::MessageSectionConfig>> {
        self._message_section_in_contacts.get_or_init(|| {
            self._message_group_config()
                .values()
                .map(|group| (group.message_contacts_id, &group.message_section_id_list))
                .flat_map(|(contacts_id, section_id_list)| {
                    section_id_list
                        .iter()
                        .map(|section_id| (contacts_id, section_id))
                        .collect::<Vec<_>>()
                })
                .map(|(contacts_id, section_id)| {
                    (contacts_id, &self._message_section_config()[section_id])
                })
                .map(|(contacts_id, section)| (contacts_id, Arc::clone(section)))
                .collect()
        })
    }

    fn _message_contacts_of_section(
        &self,
    ) -> &FnvHashMap<u32, Arc<model::message::MessageContactsConfig>> {
        self._message_contacts_of_section.get_or_init(|| {
            self._message_group_config()
                .values()
                .map(|group| (group.message_contacts_id, &group.message_section_id_list))
                .flat_map(|(contacts_id, section_id_list)| {
                    section_id_list
                        .iter()
                        .map(|&section_id| (contacts_id, section_id))
                        .collect::<Vec<_>>()
                })
                .map(|(contacts_id, section_id)| {
                    (
                        section_id,
                        Arc::clone(self._message_contacts_config().get(&contacts_id).unwrap()),
                    )
                })
                .collect()
        })
    }

    fn _extra_effect_config_by_name(
        &self,
    ) -> &FnvHashMap<Arc<str>, Arc<model::misc::ExtraEffectConfig>> {
        self._extra_effect_config_by_name.get_or_init(|| {
            self._extra_effect_config()
                .values()
                .chain(self._rogue_extra_config().values())
                .filter_map(|effect| {
                    self.text_map
                        .get(&effect.extra_effect_name.hash)
                        .map(|name| (Arc::clone(name), Arc::clone(effect)))
                })
                .collect()
        })
    }

    fn _rogue_buff_by_name(&self) -> &FnvHashMap<Arc<str>, Arc<model::rogue::RogueBuff>> {
        self._rogue_buff_by_name.get_or_init(|| {
            self._rogue_buff()
                .flat_iter()
                .filter(|&(_, buff)| buff.maze_buff_level == 1)
                .filter_map(|(_, buff)| {
                    self._rogue_maze_buff()
                        .get(&buff.maze_buff_id)
                        .map(|maze_buff| maze_buff.buff_name.hash)
                        .and_then(|hash| self.text_map.get(&hash))
                        .map(|name| (Arc::clone(name), Arc::clone(buff)))
                })
                .collect()
        })
    }

    fn _rogue_tourn_buff_by_name(
        &self,
    ) -> &FnvHashMap<Arc<str>, Arc<model::rogue::tourn::RogueTournBuff>> {
        self._rogue_tourn_buff_by_name.get_or_init(|| {
            self._rogue_tourn_buff()
                .flat_iter()
                .filter(|&(_, buff)| buff.maze_buff_level == 1)
                .filter_map(|(_, buff)| {
                    self._rogue_maze_buff()
                        .get(&buff.maze_buff_id)
                        .map(|maze_buff| maze_buff.buff_name.hash)
                        .and_then(|hash| self.text_map.get(&hash))
                        .map(|name| (Arc::clone(name), Arc::clone(buff)))
                })
                .collect()
        })
    }
}
