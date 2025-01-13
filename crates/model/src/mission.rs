use std::{borrow::Cow, num::NonZero, path::PathBuf};

use base::{Wiki, ID};

use super::Text;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum ChapterType {
    Activity,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct MissionChapterConfig {
    #[serde(rename = "ID")]
    pub id: u32,
    pub chapter_name: String,
    pub stage_name: Option<String>, // 1.1 及之后
    pub chapter_desc: String,
    pub chapter_type: Option<ChapterType>,
    pub link_chapter_list: Option<Vec<u32>>, // 1.5 及之后
    pub chapter_display_priority: u32,
    pub origin_main_mission: Option<NonZero<u32>>,
    pub final_main_mission: Option<NonZero<u32>>,
    pub chapter_icon_path: PathBuf,
    pub chapter_figure_icon_path: PathBuf,
}

impl ID for MissionChapterConfig {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.id
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum MainMissionType {
    Branch,
    Companion,
    Daily,
    Gap,
    Main,
}

impl Wiki for MainMissionType {
    fn wiki(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            MainMissionType::Branch => "冒险任务",
            MainMissionType::Companion => "同行任务",
            MainMissionType::Daily => "日常任务",
            MainMissionType::Gap => "间章任务",
            MainMissionType::Main => "主线任务",
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
pub struct Param {
    r#type: ParamType,
    value: Option<NonZero<u32>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum AudioEmotionState {
    #[serde(rename = "")]
    None,
    #[serde(rename = "State_Bgm_E1")]
    BgmE1,
    #[serde(rename = "State_Bgm_E2")]
    BgmE2,
    #[serde(rename = "State_Bgm_E3")]
    BgmE3,
    #[serde(rename = "State_Bgm_E4")]
    BgmE4,
    #[serde(rename = "State_Bgm_E5")]
    BgmE5,
    #[serde(rename = "State_Bgm_E6")]
    BgmE6,
    #[serde(rename = "State_Bgm_E7")]
    BgmE7,
    #[serde(rename = "State_Bgm_Ending")]
    BgmEnding,
    #[serde(rename = "State_Eslience")]
    Eslience,
    #[serde(rename = "State_Hollowing")]
    Hollowing,
    #[serde(rename = "State_Hollowing_D")]
    HollowingD,
    #[serde(rename = "State_Joyful")]
    Joyful,
    #[serde(rename = "State_Nervous")]
    Nervous,
    #[serde(rename = "State_Relaxing")]
    Relaxing,
    #[serde(rename = "State_Severe")]
    Severe,
    #[serde(rename = "State_Sorrow")]
    Sorrow,
    #[serde(rename = "State_Tense")]
    Tense,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum SubType {
    Activity,
    Game,
    System,
    World,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct MainMission {
    #[serde(rename = "MainMissionID")]
    pub main_mission_id: u32,
    pub r#type: MainMissionType,
    pub sub_type: Option<SubType>, // 3.0 新增字段
    #[serde(rename = "WorldID")]
    pub world_id: Option<NonZero<u16>>, // 3.0 新增字段
    pub display_priority: u32,
    #[serde(default)]
    pub is_display_activity_icon: bool,
    #[serde(default)]
    pub is_in_raid: bool,
    pub next_main_mission_list: Vec<u32>, // 只有空 []
    pub name: Text,
    pub take_type_a: Option<ParamType>,          // 1.0 及之前
    pub take_param_a_int_1: Option<u32>,         // 1.0 及之前
    pub take_param_a_int_list: Option<Vec<u32>>, // 1.0 及之前
    pub take_type_b: Option<ParamType>,          // 1.0 及之前
    pub take_param_b_int_1: Option<u32>,         // 1.0 及之前
    pub take_param_b_int_list: Option<Vec<u32>>, // 1.0 及之前
    pub take_operation: Option<Operation>,
    pub begin_operation: Operation,
    pub take_param: Option<Vec<Param>>,
    pub begin_param: Vec<Param>,
    pub next_track_main_mission: Option<NonZero<u32>>,
    #[serde(default)]
    pub is_show_red_dot: bool, // 1.2 及之前
    pub track_weight: Option<NonZero<u8>>,
    pub mission_suspend: Option<NonZero<u8>>, // 1.6 及之前，只有 1
    pub mission_advance: Option<NonZero<u8>>,
    #[serde(rename = "RewardID")]
    pub reward_id: Option<NonZero<u32>>,
    #[serde(rename = "DisplayRewardID")]
    pub display_reward_id: Option<NonZero<u32>>,
    pub audio_emotion_state: Option<AudioEmotionState>, // 仅出现于 1.4 以前
    pub mission_pack: Option<NonZero<u32>>,
    #[serde(rename = "ChapterID")]
    pub chapter_id: Option<NonZero<u32>>,
    pub sub_reward_list: Vec<u32>,
    #[serde(default, rename = "StoryLineIDList")]
    pub story_line_id_list: [(); 0], // 仅在 2.0 存在
}

impl ID for MainMission {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.main_mission_id
    }
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct MapNPC {
    HEJINHDPIED: u8,
    DHBLMALKKHI: u32,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct MapProp {
    HEJINHDPIED: u8,
    FDAKPBACCBE: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum IsShowStartHint {
    New,
    Update,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FinishActorType {
    AddMissionItem,
    AddRecoverMissionItem,
    #[serde(rename = "ChangeLineup")]
    ChangeLineup,
    DelMission,
    DelMissionItem,
    #[serde(rename = "Recover")]
    Recover,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum WayPointType {
    Anchor,
    Monster,
    NPC,
    Prop,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct SubMission {
    #[serde(rename = "SubMissionID")]
    pub sub_missoin_id: u32,
    #[serde(rename = "NextSubMissionID")]
    pub next_sub_missoin_id: Option<u32>, // 1.0 即以前
    pub next_sub_mission_list: Option<Vec<u32>>, // 1.0 及之前
    #[serde(rename = "MainMissionID")]
    pub main_mission_id: Option<NonZero<u32>>, // 1.0 及之前
    pub target_text: Text,
    pub descrption_text: Text,
    #[serde(rename = "MazePlaneID")]
    pub maze_plane_id: Option<NonZero<u32>>, // 1.0 及之前
    #[serde(rename = "MazeFloorID")]
    pub maze_floor_id: Option<NonZero<u32>>, // 1.0 及之前
    #[serde(rename = "MapNPCList")]
    pub map_npc_list: Option<Vec<MapNPC>>, // 1.0 及之前
    pub map_prop_list: Option<Vec<MapProp>>, // 1.0 及之前
    #[serde(default)] // 为了避免 Option 占用空间, 强制 default 了
    pub exclusive_group_list: [(); 0], // 1.0 及之前
    #[serde(default)]
    pub is_show: bool,  // 1.0 及之前
    #[serde(default)]
    pub mute_nav: bool, // 1.0 及之前
    pub progress_group: Option<NonZero<u32>>, // 1.0 及之前
    #[serde(default)]
    pub is_show_progress: bool, // 1.0 及之前
    pub is_show_finish_effect: Option<NonZero<u8>>, // 1.0 及之前
    pub is_show_start_hint: Option<IsShowStartHint>, // 1.0 及之前
    pub way_point_type: Option<WayPointType>, // 1.0 及之前
    #[serde(rename = "WayPointFloorID")]
    pub way_point_floor_id: Option<NonZero<u32>>, // 1.0 及之前
    #[serde(rename = "WayPointGroupID")]
    pub way_point_group_id: Option<NonZero<u8>>, // 1.0 及之前
    #[serde(rename = "WayPointEntityID")]
    pub way_point_entity_id: Option<NonZero<u32>>, // 1.0 及之前
    pub way_point_show_range_min: Option<NonZero<u16>>, // 1.0 及之前
    pub map_waypoint_icon_type: Option<NonZero<u8>>, // 1.0 及之前
    pub map_waypoint_range: Option<NonZero<u8>>, // 1.0 及之前
    pub finish_actor_type: Option<FinishActorType>,
    pub finish_actor_para: Option<String>,
    #[serde(default)]
    pub froce_map_hint: bool, // 1.0 及之前
    pub audio_emotion_state: Option<AudioEmotionState>, // 1.0 及之前
    pub process_group: Option<NonZero<u32>>,            // 1.0 及之前
    #[serde(rename = "SortID")]
    pub sort_id: Option<NonZero<u8>>, // 1.0 及之前
    pub sub_custom_value_list: Option<Vec<u8>>,         // 1.0 及之前
    #[serde(rename = "SubRewardID")]
    pub sub_reward_id: Option<NonZero<u32>>, // 1.0 及之前
    pub custom_value_reward: Option<Vec<u32>>,          // 1.0 及之前
}

impl ID for SubMission {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.sub_missoin_id
    }
}
