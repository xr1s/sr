#![feature(iter_intersperse)]

pub mod battle;
pub mod book;
pub mod challenge;
pub mod item;
pub mod map;
pub mod message;
pub mod misc;
pub mod mission;
pub mod monster;
pub mod rogue;
pub mod talk;

pub mod prelude {
    pub use crate::battle::*;
    pub use crate::challenge::*;
    pub use crate::item::*;
    pub use crate::map::*;
    pub use crate::message::*;
    pub use crate::misc::*;
    pub use crate::mission::*;
    pub use crate::monster::guide::*;
    pub use crate::monster::*;
    pub use crate::rogue::tourn::*;
    pub use crate::rogue::*;
    pub use crate::talk::*;
}

pub trait FromModel<'a, Data: ExcelOutput>
where
    Self: 'a,
{
    type Model;
    fn from_model(game: &'a Data, model: &'a Self::Model) -> Self;
}

use data::SealedGameData;

macro_rules! declare {
    ($method:ident, $id:ty => $typ:ty) => {
        fn $method(&self, id: $id) -> Option<$typ>;
        paste::paste! {
            fn [<list_$method>](&self) -> impl Iterator<Item = $typ>;
        }
    };
}

macro_rules! implement {
    ($field:ident, $id:ty => $typ:ty) => {
        paste::paste! {
            fn $field(&self, id: $id) -> Option<$typ> {
                self.[<_$field>]().get(&id).map(|model| <$typ>::from_model(self, model))
            }
            fn [<list_$field>](&self) -> impl Iterator<Item = $typ> {
                self.[<_$field>]().values().map(|model| <$typ>::from_model(self, model))
            }
        }
    };
}

macro_rules! main_sub_declare {
    ($method:ident, $id:ty => $typ:ty) => {
        paste::paste! {
            fn [<list_$method>](&self) -> impl Iterator<Item = Vec<$typ>>;
            fn $method(&self, id: $id) -> Vec<$typ>;
        }
    };
}

macro_rules! main_sub_implement {
    ($field:ident, $id:ty => $typ:ty) => {
        main_sub_implement!($field, $id => $typ, paste::paste!(stringify!([<$field:camel>])));
    };
    ($field:ident, $id:ty => $typ:ty, $json:expr) => {
        paste::paste! {
            fn [<list_$field>](&self) -> impl Iterator<Item = Vec<$typ>> {
                self.[<_$field>]()
                    .iter_all()
                    .map(|(_, value)| value)
                    .map(|models| {
                        models.iter().map(|model| <$typ>::from_model(self, model)).collect()
                    })
            }

            fn $field(&self, id: $id) -> Vec<$typ> {
                self.[<_$field>]()
                    .get_vec(&id)
                    .map(Vec::as_slice)
                    .unwrap_or_default()
                    .iter()
                    .map(|model| <$typ>::from_model(self, model))
                    .collect()
            }
        }
    };
}

// 这段注释是为了方便后续继续拆 trait 加的，如果需要拆的话
//
// misc 无依赖
// monster 无依赖
// talk 无依赖
//
// item 依赖 misc
// mission 依赖 misc
// message 依赖 mission
// battle 依赖 monster
//
// monster guide 依赖 misc, monster
// map 依赖 mission, monster
//
// challenge 依赖 battle, misc, monster
//
// rogue tourn 和 rogue 互相依赖
// rogue 依赖 misc, mission 和 monster
// rogue tourn 依赖 misc 和 monster

// 为了后面不用到处 use data::Text, 这里直接作为 trait 本身的依赖了
pub trait ExcelOutput: data::Text {
    // battle
    declare!(battle_event_config, u32 => battle::BattleEventConfig);
    declare!(stage_infinite_group, u32 => battle::StageInfiniteGroup<Self>);
    declare!(stage_infinite_monster_group, u32 => battle::StageInfiniteMonsterGroup<Self>);
    declare!(stage_infinite_wave_config, u32 => battle::StageInfiniteWaveConfig<Self>);
    declare!(stage_config, u32 => battle::StageConfig<Self>);
    // book
    declare!(book_display_type, u8 => book::BookDisplayType);
    declare!(book_series_config, u16 => book::BookSeriesConfig);
    declare!(book_series_world, u8 => book::BookSeriesWorld);
    declare!(localbook_config, u32 => book::LocalbookConfig);
    // challenge
    declare!(challenge_boss_group_config, u16 => challenge::ChallengeGroupConfig<Self>);
    declare!(challenge_boss_group_extra, u16 => challenge::ChallengeGroupExtra);
    declare!(challenge_boss_maze_config, u16 => challenge::ChallengeMazeConfig<Self>);
    declare!(challenge_boss_maze_extra, u16 => challenge::ChallengeMazeExtra<Self>);
    main_sub_declare!(challenge_boss_reward_line, u16 => challenge::ChallengeRewardLine);
    declare!(challenge_boss_target_config, u16 => challenge::ChallengeTargetConfig);
    declare!(challenge_group_config, u16 => challenge::ChallengeGroupConfig<Self>);
    declare!(challenge_maze_config, u16 => challenge::ChallengeMazeConfig<Self>);
    declare!(challenge_maze_group_extra, u16 => challenge::ChallengeGroupExtra);
    main_sub_declare!(challenge_maze_reward_line, u16 => challenge::ChallengeRewardLine);
    declare!(challenge_story_group_config, u16 => challenge::ChallengeGroupConfig<Self>);
    declare!(challenge_story_group_extra, u16 => challenge::ChallengeGroupExtra);
    declare!(challenge_story_maze_config, u16 => challenge::ChallengeMazeConfig<Self>);
    declare!(challenge_story_maze_extra, u16 => challenge::ChallengeMazeExtra<Self>);
    main_sub_declare!(challenge_story_reward_line, u16 => challenge::ChallengeRewardLine);
    declare!(challenge_story_target_config, u16 => challenge::ChallengeTargetConfig);
    declare!(challenge_target_config, u16 => challenge::ChallengeTargetConfig);
    // item
    declare!(item_config, u32 => item::ItemConfig);
    declare!(item_config_avatar_rank, u32 => item::ItemConfig);
    declare!(item_config_equipment, u32 => item::ItemConfig);
    declare!(item_use_data, u32 => item::ItemUseData);
    // map
    declare!(map_entrance, u32 => map::MapEntrance);
    main_sub_declare!(mapping_info, u32 => map::MappingInfo<Self>);
    declare!(maze_floor, u32 => map::MazeFloor);
    declare!(maze_plane, u32 => map::MazePlane);
    declare!(maze_prop, u32 => map::MazeProp);
    declare!(world_data_config, u16 => map::WorldDataConfig);
    // message
    declare!(emoji_config, u32 => message::EmojiConfig);
    declare!(emoji_group, u8 => message::EmojiGroup);
    declare!(message_contacts_camp, u8 => message::MessageContactsCamp);
    declare!(message_contacts_config, u16 => message::MessageContactsConfig<Self>);
    declare!(message_contacts_type, u8 => message::MessageContactsType);
    declare!(message_group_config, u16 => message::MessageGroupConfig<Self>);
    declare!(message_item_config, u32 => message::MessageItemConfig<Self>);
    declare!(message_item_image, u32 => message::MessageItemImage);
    declare!(message_section_config, u32 => message::MessageSectionConfig<Self>);
    // misc
    declare!(extra_effect_config, u32 => misc::ExtraEffectConfig);
    main_sub_declare!(maze_buff, u32 => misc::MazeBuff);
    declare!(reward_data, u32 => misc::RewardData);
    declare!(schedule_data_challenge_boss, u32 => misc::ScheduleData);
    declare!(schedule_data_challenge_maze, u32 => misc::ScheduleData);
    declare!(schedule_data_challenge_story, u32 => misc::ScheduleData);
    declare!(schedule_data_global, u32 => misc::ScheduleDataGlobal);
    // mission
    declare!(main_mission, u32 => mission::MainMission);
    declare!(mission_chapter_config, u32 => mission::MissionChapterConfig);
    declare!(sub_mission, u32 => mission::SubMission);
    // monster
    declare!(elite_group, u16 => monster::EliteGroup);
    main_sub_declare!(hard_level_group, u16 => monster::HardLevelGroup);
    declare!(monster_camp, u8 => monster::MonsterCamp);
    declare!(monster_config, u32 => monster::MonsterConfig<Self>);
    declare!(monster_unique_config, u32 => monster::MonsterConfig<Self>);
    declare!(monster_skill_config, u32 => monster::MonsterSkillConfig);
    declare!(monster_skill_unique_config, u32 => monster::MonsterSkillConfig);
    declare!(monster_template_config, u32 => monster::MonsterTemplateConfig<Self>);
    declare!(monster_template_unique_config, u32 => monster::MonsterTemplateConfig<Self>);
    declare!(npc_monster_data, u32 => monster::NPCMonsterData);
    // monster guide
    declare!(monster_difficulty_guide, u16 => monster::guide::MonsterDifficultyGuide);
    declare!(monster_guide_config, u32 => monster::guide::MonsterGuideConfig);
    declare!(monster_guide_phase, u16 => monster::guide::MonsterGuidePhase);
    declare!(monster_guide_skill, u32 => monster::guide::MonsterGuideSkill);
    declare!(monster_guide_skill_text, u32 => monster::guide::MonsterGuideSkillText);
    declare!(monster_guide_tag, u32 => monster::guide::MonsterGuideTag);
    declare!(monster_text_guide, u16 => monster::guide::MonsterTextGuide);
    // rogue
    main_sub_declare!(rogue_buff, u32 => rogue::RogueBuff<Self>);
    declare!(rogue_buff_type, u8 => rogue::RogueBuffType);
    declare!(rogue_extra_config, u32 => misc::ExtraEffectConfig);
    declare!(rogue_handbook_miracle, u16 => rogue::RogueHandbookMiracle);
    declare!(rogue_handbook_miracle_type, u16 => rogue::RogueHandbookMiracleType);
    main_sub_declare!(rogue_maze_buff, u32 => misc::MazeBuff);
    declare!(rogue_miracle, u16 => rogue::RogueMiracle);
    declare!(rogue_miracle_display, u16 => rogue::RogueMiracleDisplay);
    declare!(rogue_monster, u32 => rogue::RogueMonster);
    declare!(rogue_monster_group, u32 => rogue::RogueMonsterGroup);
    // rogue magic
    declare!(rogue_magic_miracle, u16 => rogue::RogueMiracle);
    // rogue tourn
    declare!(rogue_bonus, u16 => rogue::tourn::RogueBonus);
    main_sub_declare!(rogue_tourn_buff, u32 => rogue::tourn::RogueTournBuff<Self>);
    declare!(rogue_tourn_buff_type, u8 => rogue::tourn::RogueTournBuffType);
    declare!(rogue_tourn_content_display, u16 => rogue::tourn::RogueTournContentDisplay);
    declare!(rogue_tourn_formula, u32 => rogue::tourn::RogueTournFormula);
    declare!(rogue_tourn_formula_display, u32 => rogue::tourn::RogueTournFormulaDisplay);
    declare!(rogue_tourn_handbook_miracle, u16 => rogue::tourn::RogueTournHandbookMiracle);
    declare!(rogue_tourn_miracle, u16 => rogue::tourn::RogueTournMiracle);
    declare!(rogue_tourn_miracle_display, u16 => rogue::RogueMiracleDisplay);
    declare!(rogue_tourn_weekly_challenge, u8 => rogue::tourn::RogueTournWeeklyChallenge<Self>);
    declare!(rogue_tourn_weekly_display, u16 => rogue::tourn::RogueTournWeeklyDisplay);
    // talk
    declare!(talk_sentence_config, u32 => talk::TalkSentenceConfig);
    declare!(voice_config, u32 => talk::VoiceConfig);

    // caches
    #[rustfmt::skip]
    fn message_section_in_contacts(&self, contacts_id: u16) -> impl Iterator<Item = message::MessageSectionConfig<Self>>;
    #[rustfmt::skip]
    fn message_contacts_of_section(&self, section_id: u32) -> Option<message::MessageContactsConfig<Self>>;
    #[rustfmt::skip]
    fn monster_template_config_group(&self, id: u32) -> impl Iterator<Item = monster::MonsterTemplateConfig<Self>>;
    fn challenge_maze_in_group(&self, id: u16) -> Vec<challenge::ChallengeMazeConfig<Self>>;
    fn current_challenge_boss_group_config(&self) -> Option<challenge::ChallengeGroupConfig<Self>>;
    fn current_challenge_group_config(&self) -> Option<challenge::ChallengeGroupConfig<Self>>;
    fn current_challenge_story_group_config(&self)
        -> Option<challenge::ChallengeGroupConfig<Self>>;

    // 按名称索引
    fn rogue_buff_by_name(&self, name: &str) -> Option<rogue::RogueBuff<Self>>;
    fn rogue_tourn_buff_by_name(&self, name: &str) -> Option<rogue::tourn::RogueTournBuff<Self>>;
}

impl ExcelOutput for data::GameData {
    // battle
    implement!(battle_event_config, u32 => battle::BattleEventConfig);
    implement!(stage_infinite_group, u32 => battle::StageInfiniteGroup<Self>);
    implement!(stage_infinite_monster_group, u32 => battle::StageInfiniteMonsterGroup<Self>);
    implement!(stage_infinite_wave_config, u32 => battle::StageInfiniteWaveConfig<Self>);
    implement!(stage_config, u32 => battle::StageConfig<Self>);
    // book
    implement!(book_display_type, u8 => book::BookDisplayType);
    implement!(book_series_config, u16 => book::BookSeriesConfig);
    implement!(book_series_world, u8 => book::BookSeriesWorld);
    implement!(localbook_config, u32 => book::LocalbookConfig);
    // challenge
    implement!(challenge_boss_group_config, u16 => challenge::ChallengeGroupConfig<Self>);
    implement!(challenge_boss_group_extra, u16 => challenge::ChallengeGroupExtra);
    implement!(challenge_boss_maze_config, u16 => challenge::ChallengeMazeConfig<Self>);
    implement!(challenge_boss_maze_extra, u16 => challenge::ChallengeMazeExtra<Self>);
    main_sub_implement!(challenge_boss_reward_line, u16 => challenge::ChallengeRewardLine);
    implement!(challenge_boss_target_config, u16 => challenge::ChallengeTargetConfig);
    implement!(challenge_group_config, u16 => challenge::ChallengeGroupConfig<Self>);
    implement!(challenge_maze_config, u16 => challenge::ChallengeMazeConfig<Self>);
    implement!(challenge_maze_group_extra, u16 => challenge::ChallengeGroupExtra);
    main_sub_implement!(challenge_maze_reward_line, u16 => challenge::ChallengeRewardLine);
    implement!(challenge_story_group_config, u16 => challenge::ChallengeGroupConfig<Self>);
    implement!(challenge_story_group_extra, u16 => challenge::ChallengeGroupExtra);
    implement!(challenge_story_maze_config, u16 => challenge::ChallengeMazeConfig<Self>);
    implement!(challenge_story_maze_extra, u16 => challenge::ChallengeMazeExtra<Self>);
    main_sub_implement!(challenge_story_reward_line, u16 => challenge::ChallengeRewardLine);
    implement!(challenge_story_target_config, u16 => challenge::ChallengeTargetConfig);
    implement!(challenge_target_config, u16 => challenge::ChallengeTargetConfig);
    // item
    implement!(item_config, u32 => item::ItemConfig);
    implement!(item_config_avatar_rank, u32 => item::ItemConfig);
    implement!(item_config_equipment, u32 => item::ItemConfig);
    implement!(item_use_data, u32 => item::ItemUseData);
    // map
    implement!(map_entrance, u32 => map::MapEntrance);
    main_sub_implement!(mapping_info, u32 => map::MappingInfo<Self>);
    implement!(maze_floor, u32 => map::MazeFloor);
    implement!(maze_plane, u32 => map::MazePlane);
    implement!(maze_prop, u32 => map::MazeProp);
    implement!(world_data_config, u16 => map::WorldDataConfig);
    // message
    implement!(emoji_config, u32 => message::EmojiConfig);
    implement!(emoji_group, u8 => message::EmojiGroup);
    implement!(message_contacts_camp, u8 => message::MessageContactsCamp);
    implement!(message_contacts_config, u16 => message::MessageContactsConfig<Self>);
    implement!(message_contacts_type, u8 => message::MessageContactsType);
    implement!(message_group_config, u16 => message::MessageGroupConfig<Self>);
    implement!(message_item_config, u32 => message::MessageItemConfig<Self>);
    implement!(message_item_image, u32 => message::MessageItemImage);
    implement!(message_section_config, u32 => message::MessageSectionConfig<Self>);
    // misc
    implement!(extra_effect_config, u32 => misc::ExtraEffectConfig);
    implement!(reward_data, u32 => misc::RewardData);
    main_sub_implement!(maze_buff, u32 => misc::MazeBuff);
    implement!(schedule_data_challenge_boss, u32 => misc::ScheduleData);
    implement!(schedule_data_challenge_maze, u32 => misc::ScheduleData);
    implement!(schedule_data_challenge_story, u32 => misc::ScheduleData);
    implement!(schedule_data_global, u32 => misc::ScheduleDataGlobal);
    // mission
    implement!(main_mission, u32 => mission::MainMission);
    implement!(mission_chapter_config, u32 => mission::MissionChapterConfig);
    implement!(sub_mission, u32 => mission::SubMission);
    // monster
    implement!(elite_group, u16 => monster::EliteGroup);
    main_sub_implement!(hard_level_group, u16 => monster::HardLevelGroup);
    implement!(monster_camp, u8 => monster::MonsterCamp);
    implement!(monster_config, u32 => monster::MonsterConfig<Self>);
    implement!(monster_unique_config, u32 => monster::MonsterConfig<Self>);
    implement!(monster_skill_config, u32 => monster::MonsterSkillConfig);
    implement!(monster_skill_unique_config, u32 => monster::MonsterSkillConfig);
    implement!(monster_template_config, u32 => monster::MonsterTemplateConfig<Self>);
    implement!(monster_template_unique_config, u32 => monster::MonsterTemplateConfig<Self>);
    implement!(npc_monster_data, u32 => monster::NPCMonsterData);
    // monster guide
    implement!(monster_difficulty_guide, u16 => monster::guide::MonsterDifficultyGuide);
    implement!(monster_guide_config, u32 => monster::guide::MonsterGuideConfig);
    implement!(monster_guide_phase, u16 => monster::guide::MonsterGuidePhase);
    implement!(monster_guide_skill, u32 => monster::guide::MonsterGuideSkill);
    implement!(monster_guide_skill_text, u32 => monster::guide::MonsterGuideSkillText);
    implement!(monster_guide_tag, u32 => monster::guide::MonsterGuideTag);
    implement!(monster_text_guide, u16 => monster::guide::MonsterTextGuide);
    // rogue
    main_sub_implement!(rogue_buff, u32 => rogue::RogueBuff<Self>);
    implement!(rogue_buff_type, u8 => rogue::RogueBuffType);
    implement!(rogue_extra_config, u32 => misc::ExtraEffectConfig);
    implement!(rogue_handbook_miracle, u16 => rogue::RogueHandbookMiracle);
    implement!(rogue_handbook_miracle_type, u16 => rogue::RogueHandbookMiracleType);
    main_sub_implement!(rogue_maze_buff, u32 => misc::MazeBuff);
    implement!(rogue_miracle, u16 => rogue::RogueMiracle);
    implement!(rogue_miracle_display, u16 => rogue::RogueMiracleDisplay);
    implement!(rogue_monster, u32 => rogue::RogueMonster);
    implement!(rogue_monster_group, u32 => rogue::RogueMonsterGroup);
    // rogue magic
    implement!(rogue_magic_miracle, u16 => rogue::RogueMiracle);
    // rogue tourn
    implement!(rogue_bonus, u16 => rogue::tourn::RogueBonus);
    main_sub_implement!(rogue_tourn_buff, u32 => rogue::tourn::RogueTournBuff<Self>);
    implement!(rogue_tourn_buff_type, u8 => rogue::tourn::RogueTournBuffType);
    implement!(rogue_tourn_content_display, u16 => rogue::tourn::RogueTournContentDisplay);
    implement!(rogue_tourn_formula, u32 => rogue::tourn::RogueTournFormula);
    implement!(rogue_tourn_formula_display, u32 => rogue::tourn::RogueTournFormulaDisplay);
    implement!(rogue_tourn_handbook_miracle, u16 => rogue::tourn::RogueTournHandbookMiracle);
    implement!(rogue_tourn_miracle, u16 => rogue::tourn::RogueTournMiracle);
    implement!(rogue_tourn_miracle_display, u16 => rogue::RogueMiracleDisplay);
    implement!(rogue_tourn_weekly_challenge, u8 => rogue::tourn::RogueTournWeeklyChallenge<Self>);
    implement!(rogue_tourn_weekly_display, u16 => rogue::tourn::RogueTournWeeklyDisplay);
    // talk
    implement!(talk_sentence_config, u32 => talk::TalkSentenceConfig);
    implement!(voice_config, u32 => talk::VoiceConfig);

    fn message_section_in_contacts(
        &self,
        contacts_id: u16,
    ) -> impl Iterator<Item = message::MessageSectionConfig<Self>> {
        self._message_section_in_contacts()
            .get_vec(&contacts_id)
            .map(Vec::as_slice)
            .unwrap_or_default()
            .iter()
            .map(|section| message::MessageSectionConfig::from_model(self, section))
    }

    fn message_contacts_of_section(
        &self,
        section_id: u32,
    ) -> Option<message::MessageContactsConfig<Self>> {
        self._message_contacts_of_section()
            .get(&section_id)
            .map(|contacts| message::MessageContactsConfig::from_model(self, contacts.as_ref()))
    }

    fn monster_template_config_group(
        &self,
        id: u32,
    ) -> impl Iterator<Item = monster::MonsterTemplateConfig<Self>> {
        if id == 0 {
            return either::Either::Left(std::iter::empty());
        }
        either::Either::Right(
            self._monster_template_config_group()
                .get_vec(&id)
                .map(Vec::as_slice)
                .unwrap_or_default()
                .iter()
                .map(|template| monster::MonsterTemplateConfig::from_model(self, template)),
        )
    }

    fn challenge_maze_in_group(&self, id: u16) -> Vec<challenge::ChallengeMazeConfig<Self>> {
        self._challenge_maze_in_group()
            .get_vec(&id)
            .map(Vec::as_slice)
            .unwrap_or_default()
            .iter()
            .map(|maze| challenge::ChallengeMazeConfig::from_model(self, maze))
            .collect()
    }

    fn current_challenge_boss_group_config(&self) -> Option<challenge::ChallengeGroupConfig<Self>> {
        self._current_challenge_group_config(Self::_challenge_boss_group_config)
            .map(|challenge| challenge::ChallengeGroupConfig::from_model(self, challenge))
    }

    fn current_challenge_group_config(&self) -> Option<challenge::ChallengeGroupConfig<Self>> {
        self._current_challenge_group_config(Self::_challenge_group_config)
            .map(|challenge| challenge::ChallengeGroupConfig::from_model(self, challenge))
    }

    fn current_challenge_story_group_config(
        &self,
    ) -> Option<challenge::ChallengeGroupConfig<Self>> {
        self._current_challenge_group_config(Self::_challenge_story_group_config)
            .map(|challenge| challenge::ChallengeGroupConfig::from_model(self, challenge))
    }

    fn rogue_tourn_buff_by_name(&self, name: &str) -> Option<rogue::tourn::RogueTournBuff<Self>> {
        self._rogue_tourn_buff_by_name()
            .get(name)
            .map(|model| rogue::tourn::RogueTournBuff::from_model(self, model))
    }

    fn rogue_buff_by_name(&self, name: &str) -> Option<rogue::RogueBuff<Self>> {
        self._rogue_buff_by_name()
            .get(name)
            .map(|model| rogue::RogueBuff::from_model(self, model))
    }
}
