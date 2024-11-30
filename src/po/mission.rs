use std::{borrow::Cow, num::NonZero, path::PathBuf};

use crate::{vo, GameData, Wiki, ID, PO};

use super::Text;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum ChapterType {
    Activity,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct MissionChapterConfig {
    #[serde(rename = "ID")]
    id: u32,
    chapter_name: String,
    stage_name: Option<String>, // 1.1 及之后
    chapter_desc: String,
    chapter_type: Option<ChapterType>,
    link_chapter_list: Option<Vec<u32>>, // 1.5 及之后
    chapter_display_priority: u32,
    origin_main_mission: Option<NonZero<u32>>,
    final_main_mission: Option<NonZero<u32>>,
    chapter_icon_path: PathBuf,
    chapter_figure_icon_path: PathBuf,
}

impl ID for MissionChapterConfig {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.id
    }
}
impl PO<'_> for MissionChapterConfig {
    type VO = vo::mission::MissionChapterConfig;
    fn vo(&self, _game: &GameData) -> Self::VO {
        Self::VO {
            id: self.id,
            display_priority: self.chapter_display_priority,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum Type {
    Branch,
    Companion,
    Daily,
    Gap,
    Main,
}

impl Wiki for Type {
    fn wiki(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Type::Branch => "冒险任务",
            Type::Companion => "同行任务",
            Type::Daily => "日常任务",
            Type::Gap => "间章任务",
            Type::Main => "主线任务",
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum Operation {
    And,
    Or,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum ParamType {
    Auto,
    HeliobusPhaseReach,
    Manual,
    MultiSequence,
    MuseumPhaseRenewPointReach,
    PlayerLevel,
    Sequence,
    SequenceNextDay,
    WorldLevel,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct Param {
    r#type: ParamType,
    value: Option<NonZero<u32>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum AudioEmotionState {
    #[serde(rename = "")]
    None,
    #[serde(rename = "State_Tense")]
    Tense,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct MainMission {
    #[serde(rename = "MainMissionID")]
    main_mission_id: u32,
    r#type: Type, // TODO: 有个 MainMissionType.json
    display_priority: u32,
    #[serde(default)]
    is_display_activity_icon: bool,
    #[serde(default)]
    is_in_raid: bool,
    next_main_mission_list: Vec<u32>, // 只有空 []
    name: Text,
    take_type_a: Option<ParamType>,          // 1.0 及之前
    take_param_a_int_1: Option<u32>,         // 1.0 及之前
    take_param_a_int_list: Option<Vec<u32>>, // 1.0 及之前
    take_type_b: Option<ParamType>,          // 1.0 及之前
    take_param_b_int_1: Option<u32>,         // 1.0 及之前
    take_param_b_int_list: Option<Vec<u32>>, // 1.0 及之前
    take_operation: Option<Operation>,
    begin_operation: Operation,
    take_param: Option<Vec<Param>>,
    begin_param: Vec<Param>,
    next_track_main_mission: Option<NonZero<u32>>,
    #[serde(default)]
    is_show_red_dot: bool, // 1.2 及之前
    track_weight: Option<NonZero<u8>>,
    mission_suspend: Option<NonZero<u8>>, // 1.6 及之前，只有 1
    mission_advance: Option<NonZero<u8>>,
    #[serde(rename = "RewardID")]
    reward_id: Option<NonZero<u32>>,
    #[serde(rename = "DisplayRewardID")]
    display_reward_id: Option<NonZero<u32>>,
    audio_emotion_state: Option<AudioEmotionState>, // 仅出现于 1.4 以前
    mission_pack: Option<NonZero<u32>>,
    #[serde(rename = "ChapterID")]
    chapter_id: Option<NonZero<u32>>,
    sub_reward_list: Vec<u32>,
    // 仅在 2.0 存在
    #[serde(default, rename = "StoryLineIDList")]
    story_line_id_list: [(); 0],
}

impl ID for MainMission {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.main_mission_id
    }
}

impl<'a> PO<'a> for MainMission {
    type VO = vo::mission::MainMission<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.main_mission_id,
            r#type: self.r#type,
            display_priority: self.display_priority,
            name: game.text(self.name),
            next_track_main_mission: self.next_track_main_mission,
            track_weight: self.track_weight,
            reward: self
                .reward_id
                .map(NonZero::get)
                .map(|id| game.reward_data(id))
                .map(Option::unwrap),
            display_reward: self
                .display_reward_id
                .map(NonZero::get)
                .map(|id| game.reward_data(id))
                .map(Option::unwrap),
            chapter: self
                .chapter_id
                .map(NonZero::get)
                .map(|id| game.mission_chapter_config(id))
                .map(Option::unwrap),
            sub_reward_list: self
                .sub_reward_list
                .iter()
                .map(|&id| game.reward_data(id))
                .map(Option::unwrap)
                .collect(),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct SubMission {
    #[serde(rename = "SubMissionID")]
    sub_missoin_id: u32,
    target_text: Text,
    descrption_text: Text,
}

impl ID for SubMission {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.sub_missoin_id
    }
}

impl<'a> PO<'a> for SubMission {
    type VO = vo::mission::SubMission<'a>;
    fn vo(&self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.sub_missoin_id,
            target: game.text(self.target_text),
            description: game.text(self.descrption_text),
        }
    }
}
