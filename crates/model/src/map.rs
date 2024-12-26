use std::{num::NonZero, path::PathBuf};

use base::{MainSubID, ID};

use super::{Element, Text};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MapEntranceType {
    Explore,
    Mission,
    Town,
    Four, // 1.0 及之前，怎么还有数字的我真是服了
}

impl<'de> serde::Deserialize<'de> for MapEntranceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl serde::de::Visitor<'_> for Visitor {
            type Value = MapEntranceType;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str(r#"4, "Explore", "Mission" or "Town""#)
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v != 4 {
                    unreachable!(r#"expected 4, "Explore", "Mission", "Town", got {v}"#);
                }
                Ok(MapEntranceType::Four)
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(match v {
                    "Explore" => MapEntranceType::Explore,
                    "Mission" => MapEntranceType::Mission,
                    "Town" => MapEntranceType::Town,
                    _ => unreachable!(r#"expected 4, "Explore", "Mission", "Town", got {v:?}"#),
                })
            }
        }
        deserializer.deserialize_any(Visitor)
    }
}

impl serde::Serialize for MapEntranceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            MapEntranceType::Explore => "Explore".serialize(serializer),
            MapEntranceType::Mission => "Mission".serialize(serializer),
            MapEntranceType::Town => "Town".serialize(serializer),
            MapEntranceType::Four => 4.serialize(serializer),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct MapEntrance {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(default)]
    pub is_show_in_map_menu: bool, // 1.2 及之前
    #[serde(rename = "MapMenuSortID")]
    pub map_menu_sort_id: Option<NonZero<u16>>, // 1.2 及之前
    pub entrance_type: MapEntranceType,
    pub name: Option<Text>,                       // 1.2 及之前
    pub desc: Option<Text>,                       // 1.2 及之前
    pub entrance_list_icon: Option<String>,       // 1.2 及之前
    pub image_path: Option<String>,               // 1.2 及之前
    pub mini_map_icon_hint_list: Option<Vec<u8>>, // 1.2 及之前
    #[serde(rename = "PlaneID")]
    pub plane_id: u32,
    #[serde(rename = "FloorID")]
    pub floor_id: u32,
    #[serde(rename = "StartGroupID")]
    pub start_group_id: Option<NonZero<u16>>,
    #[serde(rename = "StartAnchorID")]
    pub start_anchor_id: Option<NonZero<u8>>,
    pub target_main_mission_list: Option<Vec<u32>>, // 1.2 及之前
    pub begin_main_mission_list: Vec<u32>,          // 只有空 []
    pub finish_main_mission_list: Vec<u32>,
    pub finish_sub_mission_list: Vec<u32>,
    pub finish_quest_list: Option<Vec<u32>>,  // 1.2 及之前
    pub condition_expression: Option<String>, // 2.4 及之后。稍微复杂的逻辑表达式，这里就不编译了
}

impl ID for MapEntrance {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.id
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MappingInfoType {
    ActivityEntrance,
    ActivityMusicRhythm,
    ActivitySummon,
    ActivityTelevision,
    #[serde(rename = "ChallengeBossEntrance")]
    ChallengeBossEntrance,
    ChallengeEntrance,
    ChallengeStory,
    DroneEntrance,
    FarmEntrance,
    FightFest,
    HeliobusChallenge,
    HeliobusRaid,
    MazzPuzzleMovie,
    OfferingReward,
    RaidEntrance,
    RewardCollection,
    RogueEntrance,
    SubMapEntrance,
    WorldShopEntrance,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MappingInfoFarmType {
    Cocoon,
    Cocoon2,
    Element,
    Relic,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct MappingInfo {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(default)]
    /// 2.3 及以前是副字段，空为 0
    pub world_level: u8,
    pub r#type: Option<MappingInfoType>,
    pub farm_type: Option<MappingInfoFarmType>,
    #[serde(default)]
    pub is_teleport: bool,
    #[serde(default)]
    pub is_show_in_fog: bool,
    #[serde(rename = "PlaneID")]
    pub plane_id: Option<NonZero<u32>>,
    #[serde(rename = "FloorID")]
    pub floor_id: Option<NonZero<u32>>,
    #[serde(rename = "GroupID")]
    pub group_id: Option<NonZero<u16>>,
    #[serde(rename = "ConfigID")]
    pub config_id: Option<NonZero<u32>>,
    #[serde(default)]
    pub initial_enable: bool,
    pub name: Text,
    pub desc: Text,
    pub show_monster_list: Vec<u32>,
    pub display_item_list: Vec<crate::item::ItemList>,
    #[serde(rename = "EntranceID")]
    pub entrance_id: Option<NonZero<u32>>,
}

impl MainSubID for MappingInfo {
    type ID = u32;
    type SubID = u8;
    fn id(&self) -> Self::ID {
        self.id
    }
    fn sub_id(&self) -> Self::SubID {
        self.world_level
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum FloorTag {
    DeepDream,
    Dream,
    PenaconyReality,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum BGM {
    #[serde(
        alias = "State_Herta_Space_Station",
        alias = "StateGroup_Herta_Space_Station"
    )]
    HertaSpaceStation,
    #[serde(alias = "State_Spaceship", alias = "StateGroup_Spaceship")]
    Spaceship,
    #[serde(
        alias = "State_City_MBelobog",
        alias = "State_City_Mbelobog",
        alias = "StateGroup_MCity_Belobog"
    )]
    Belobog,
    #[serde(
        alias = "State_City_MXianzhou_Alliance",
        alias = "StateGroup_MCity_Xianzhou_Alliance"
    )]
    XianzhouAlliance,
    #[serde(alias = "State_Penacony", alias = "StateGroup_Penocony")]
    Penacony,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum FloorType {
    Default,
    Indoor,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct MazeFloor {
    #[serde(rename = "FloorID")]
    pub floor_id: u32,
    pub floor_name: String,
    #[serde(rename = "BaseFloorID")]
    pub base_floor_id: u32,
    pub floor_tag: Option<Vec<FloorTag>>,
    #[serde(rename = "BGMWorldState")]
    pub bgm_world_state: BGM,
    #[serde(rename = "FloorBGMGroupName")]
    pub floor_bgm_group_name: String,
    #[serde(rename = "FloorBGMNormalStateName")]
    pub floor_bgm_normal_state_name: String,
    pub floor_default_emotion: String,
    #[serde(rename = "FloorBGMBusyStateName")]
    pub floor_bgm_busy_state_name: String, // 只有 "State_Maze_Busy" 和 ""
    pub enter_audio_event: Vec<String>,
    pub exit_audio_event: Vec<String>,
    pub floor_type: FloorType,
    pub walking_effect_additive_scale: Option<f32>,
    pub optional_load_blocks_config: PathBuf,
    pub municipal_config_path: PathBuf,
    pub map_layer_name_list: Vec<Text>,
    #[serde(rename = "CombatBGMLow")]
    pub combat_bgm_low: String,
    #[serde(rename = "CombatBGMHigh")]
    pub combat_bgm_high: String,
}

impl ID for MazeFloor {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.floor_id
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum PlaneType {
    AetherDivide,
    Challenge,
    Maze,
    Raid,
    Rogue,
    Town,
    Train,
    TrialActivity,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct MazePlane {
    #[serde(rename = "PlaneID")]
    pub plane_id: u32,
    pub plane_type: PlaneType,
    pub sub_type: u8, // 只有 1 ~ 6
    pub maze_pool_type: u32,
    #[serde(rename = "WorldID")]
    pub world_id: u16,
    pub plane_name: Text,
    #[serde(rename = "StartFloorID")]
    pub start_floor_id: u32,
    #[serde(rename = "FloorIDList")]
    pub floor_id_list: Vec<u32>,
}

impl ID for MazePlane {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.plane_id
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PropType {
    PropBoxmanBinded,
    PropCocoon,
    PropDestruct,
    PropElement,
    PropElevator,
    PropLight,
    PropMapRotationCharger,
    PropMapRotationSwitcher,
    PropMapRotationVolume,
    PropMazeDecal,
    PropMazeJigsaw,
    PropMazePuzzle,
    PropNoRewardDestruct,
    PropOrdinary,
    PropPerspectiveWall,
    PropPlatform,
    PropRelic,
    PropRogueChest,
    PropRogueDoor,
    PropRogueObject,
    PropRogueRewardObject,
    PropSpring,
    PropTreasureChest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum PropState {
    BridgeState1,
    BridgeState2,
    BridgeState3,
    BridgeState4,
    CheckPointDisable,
    CheckPointEnable,
    ChestClosed,
    ChestLocked,
    ChestUsed,
    Closed,
    CustomState01,
    CustomState02,
    CustomState03,
    CustomState04,
    CustomState05,
    CustomState06,
    CustomState07,
    CustomState08,
    CustomState09,
    Elevator1,
    Elevator2,
    Elevator3,
    EventClose,
    EventOpen,
    Hidden,
    Locked,
    Open,
    TeleportGate0,
    TeleportGate1,
    TeleportGate2,
    TeleportGate3,
    TriggerDisable,
    TriggerEnable,
    WaitActive,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum PropPerformanceType {
    #[serde(rename = "")]
    None,
    A,
    B,
    C,
    D,
    S,
    #[serde(rename = "S_Avatar")]
    SAvatar,
}

#[serde_with::serde_as]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct MiniMapStateIcon {
    state: Option<PropState>,
    #[serde(rename = "IconID")]
    icon_id: Option<NonZero<u8>>,
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    color: Option<base::serde::Color>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub struct MazeProp {
    #[serde(rename = "ID")]
    pub id: u32,
    pub prop_type: PropType,
    #[serde(default)]
    pub is_map_content: bool,
    pub prop_name: Text,
    pub prop_title: Text,
    pub prop_icon_path: PathBuf,
    pub board_show_list: Vec<u8>,
    pub config_entity_path: PathBuf,
    pub damage_type_list: Vec<Element>,
    pub mini_map_icon_type: Option<NonZero<u8>>,
    pub mini_map_state_icons: Vec<MiniMapStateIcon>,
    pub json_path: PathBuf,
    pub prop_state_list: Vec<PropState>,
    pub performance_type: PropPerformanceType,
    #[serde(default)]
    pub has_renderer_component: bool,
    pub lod_priority: Option<NonZero<u8>>,
}

impl ID for MazeProp {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.id
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum MapSpaceType {
    Dream,
    Reality,
    Unknow,
}

#[serde_with::serde_as]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct WorldDataConfig {
    #[serde(rename = "ID")]
    pub id: u16,
    #[serde(default)]
    pub is_real_world: bool,
    #[serde(default)]
    pub is_show: bool,
    pub world_name: Text,
    pub world_desc: Option<Text>, // 仅在 1.6 及之前出现
    // 后面的字段都仅在 2.0 及之后出现
    pub world_language_name: Option<Text>,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub dynamic_optional_block: u16,
    pub train_space_type: Option<MapSpaceType>,
    pub map_space_type_list: Option<Vec<MapSpaceType>>,
    pub chapter_icon_big_path: Option<String>,
    pub chronicle_world_bg_path: Option<String>,
    pub chronicle_world_sub_bg_path: Option<String>,
    pub chronicle_world_predict_path: Option<String>,
    pub chronicle_world_processing_path: Option<String>,
    pub camera_width: Option<NonZero<u8>>,
    pub camera_height: Option<NonZero<u8>>,
    pub small_world_icon_path: Option<String>,
}

impl ID for WorldDataConfig {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.id
    }
}
