use std::{num::NonZero, path::PathBuf, str::FromStr};

use crate::{
    po,
    vo::{self},
    GameData, GroupID, ID, PO,
};

use super::{Element, Text};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Serialize)]
pub enum EntranceType {
    Explore,
    Mission,
    Town,
    Four, // 1.0 及之前，怎么还有数字的我真是服了
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct MapEntrance {
    #[serde(rename = "ID")]
    id: u32,
    #[serde(default)]
    is_show_in_map_menu: bool, // 1.2 及之前
    #[serde(rename = "MapMenuSortID")]
    map_menu_sort_id: Option<NonZero<u16>>, // 1.2 及之前
    entrance_type: serde_json::Value,         // TODO: 数字字符串混杂
    name: Option<Text>,                       // 1.2 及之前
    desc: Option<Text>,                       // 1.2 及之前
    entrance_list_icon: Option<String>,       // 1.2 及之前
    image_path: Option<String>,               // 1.2 及之前
    mini_map_icon_hint_list: Option<Vec<u8>>, // 1.2 及之前
    #[serde(rename = "PlaneID")]
    plane_id: u32,
    #[serde(rename = "FloorID")]
    floor_id: u32,
    #[serde(rename = "StartGroupID")]
    start_group_id: Option<NonZero<u16>>,
    #[serde(rename = "StartAnchorID")]
    start_anchor_id: Option<NonZero<u8>>,
    target_main_mission_list: Option<Vec<u32>>, // 1.2 及之前
    begin_main_mission_list: Vec<u32>,          // 只有空 []
    finish_main_mission_list: Vec<u32>,
    finish_sub_mission_list: Vec<u32>,
    finish_quest_list: Option<Vec<u32>>,  // 1.2 及之前
    condition_expression: Option<String>, // 2.4 及之后。稍微复杂的逻辑表达式，这里就不编译了
}

impl ID for MapEntrance {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.id
    }
}

impl<'a> PO<'a> for MapEntrance {
    type VO = vo::map::MapEntrance<'a>;
    fn vo(&self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.id,
            r#type: match self.entrance_type.to_string().as_str() {
                "\"Explore\"" => EntranceType::Explore,
                "\"Mission\"" => EntranceType::Mission,
                "\"Town\"" => EntranceType::Town,
                "4" => EntranceType::Four,
                _ => unreachable!(),
            },
            plane: game.maze_plane(self.plane_id).unwrap(),
            floor: game.maze_floor(self.floor_id).unwrap(),
            begin_main_mission_list: self
                .begin_main_mission_list
                .iter()
                .map(|&id| game.main_mission(id))
                .map(Option::unwrap)
                .collect(),
            finish_main_mission_list: self
                .finish_main_mission_list
                .iter()
                .map(|&id| game.main_mission(id))
                .map(Option::unwrap)
                .collect(),
            finish_sub_mission_list: self
                .finish_sub_mission_list
                .iter()
                .map(|&id| game.sub_mission(id))
                .map(Option::unwrap)
                .collect(),
        }
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
pub(crate) struct MappingInfo {
    #[serde(rename = "ID")]
    id: u32,
    #[serde(default)]
    /// 2.3 及以前是副字段，空为 0
    world_level: u8,
    r#type: Option<MappingInfoType>,
    farm_type: Option<MappingInfoFarmType>,
    #[serde(default)]
    is_teleport: bool,
    #[serde(default)]
    is_show_in_fog: bool,
    #[serde(rename = "PlaneID")]
    plane_id: Option<NonZero<u32>>,
    #[serde(rename = "FloorID")]
    floor_id: Option<NonZero<u32>>,
    #[serde(rename = "GroupID")]
    group_id: Option<NonZero<u16>>,
    #[serde(rename = "ConfigID")]
    config_id: Option<NonZero<u32>>,
    #[serde(default)]
    initial_enable: bool,
    name: Text,
    desc: Text,
    show_monster_list: Vec<u32>,
    display_item_list: Vec<po::item::ItemList>,
    #[serde(rename = "EntranceID")]
    entrance_id: Option<NonZero<u32>>,
}

impl GroupID for MappingInfo {
    type GroupID = u32;
    type InnerID = u8;
    fn group_id(&self) -> Self::GroupID {
        self.id
    }
    fn inner_id(&self) -> Self::InnerID {
        self.world_level
    }
}

impl<'a> PO<'a> for MappingInfo {
    type VO = vo::map::MappingInfo<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.id,
            world_level: self.world_level,
            r#type: self.r#type,
            farm_type: self.farm_type,
            is_teleport: self.is_teleport,
            is_show_in_fog: self.is_show_in_fog,
            plane: self
                .plane_id
                .map(NonZero::get)
                .map(|id| game.maze_plane(id))
                .map(Option::unwrap),
            floor: self
                .floor_id
                .map(NonZero::get)
                .map(|id| game.maze_floor(id))
                .map(Option::unwrap),
            group_id: self.group_id.map(NonZero::get).unwrap_or_default(),
            config_id: self.config_id.map(NonZero::get).unwrap_or_default(),
            initial_enable: self.initial_enable,
            name: game.text(self.name),
            desc: game.text(self.desc),
            show_monster_list: self
                .show_monster_list
                .iter()
                .map(|&id| game.monster_config(id))
                .map(Option::unwrap)
                .collect(),
            display_item_list: self
                .display_item_list
                .iter()
                .map(|item| item.vo(game))
                .collect(),
            entrance: self
                .entrance_id
                .map(NonZero::get)
                .map(|id| game.map_entrance(id))
                .map(Option::unwrap),
        }
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
pub(crate) struct MazeFloor {
    #[serde(rename = "FloorID")]
    floor_id: u32,
    floor_name: String,
    #[serde(rename = "BaseFloorID")]
    base_floor_id: u32,
    floor_tag: Option<Vec<FloorTag>>,
    #[serde(rename = "BGMWorldState")]
    bgm_world_state: BGM,
    #[serde(rename = "FloorBGMGroupName")]
    floor_bgm_group_name: String,
    #[serde(rename = "FloorBGMNormalStateName")]
    floor_bgm_normal_state_name: String,
    floor_default_emotion: String,
    #[serde(rename = "FloorBGMBusyStateName")]
    floor_bgm_busy_state_name: String, // 只有 "State_Maze_Busy" 和 ""
    enter_audio_event: Vec<String>,
    exit_audio_event: Vec<String>,
    floor_type: FloorType,
    walking_effect_additive_scale: Option<f32>,
    optional_load_blocks_config: PathBuf,
    municipal_config_path: PathBuf,
    map_layer_name_list: Vec<Text>,
    #[serde(rename = "CombatBGMLow")]
    combat_bgm_low: String,
    #[serde(rename = "CombatBGMHigh")]
    combat_bgm_high: String,
}

impl ID for MazeFloor {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.floor_id
    }
}

impl<'a> PO<'a> for MazeFloor {
    type VO = vo::map::MazeFloor<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.floor_id,
            base_floor_id: self.base_floor_id,
            floor_tag: self.floor_tag.as_deref().unwrap_or_default(),
            floor_type: self.floor_type,
            map_layer_name_list: self
                .map_layer_name_list
                .iter()
                .map(|&text| game.text(text))
                .collect(),
        }
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
pub(crate) struct MazePlane {
    #[serde(rename = "PlaneID")]
    plane_id: u32,
    plane_type: PlaneType,
    sub_type: u8, // 只有 1 ~ 6
    maze_pool_type: u32,
    #[serde(rename = "WorldID")]
    world_id: u16,
    plane_name: Text,
    #[serde(rename = "StartFloorID")]
    start_floor_id: u32,
    #[serde(rename = "FloorIDList")]
    floor_id_list: Vec<u32>,
}

impl ID for MazePlane {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.plane_id
    }
}

impl<'a> PO<'a> for MazePlane {
    type VO = vo::map::MazePlane<'a>;
    fn vo(&self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.plane_id,
            r#type: self.plane_type,
            sub_type: self.sub_type,
            maze_pool_type: self.maze_pool_type,
            world: game.world_data_config(self.world_id).unwrap(),
            name: game.text(self.plane_name),
            start_floor: game.maze_floor(self.start_floor_id),
            floor_list: self
                .floor_id_list
                .iter()
                .map(|&id| game.maze_floor(id))
                .map(Option::unwrap)
                .collect(),
        }
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
pub enum PerformanceType {
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

#[derive(Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl std::str::FromStr for Color {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use serde::{de::Error, de::Unexpected};
        const EXPECTED: &&str = &"#RRGGBBAA";
        let s = s
            .strip_prefix('#')
            .ok_or_else(|| Error::invalid_value(Unexpected::Str(s), EXPECTED))?;
        if s.len() != 6 && s.len() != 8 {
            return Err(Error::invalid_value(Unexpected::Str(s), EXPECTED));
        }
        let mut color = Color(
            u8::from_str_radix(&s[0..2], 16).map_err(Error::custom)?,
            u8::from_str_radix(&s[2..4], 16).map_err(Error::custom)?,
            u8::from_str_radix(&s[4..6], 16).map_err(Error::custom)?,
            0,
        );
        if s.len() == 8 {
            color.3 = u8::from_str_radix(&s[6..8], 16).map_err(Error::custom)?;
        }
        Ok(color)
    }
}

impl<'de> serde::Deserialize<'de> for Color {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Color::from_str(&String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

impl serde::Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!("#{:02x}{:02x}{:02x}", self.0, self.1, self.2))?;
        if self.3 != 0 {
            f.write_fmt(format_args!("{:02x}", self.3))?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!("#{:02x}{:02x}{:02x}", self.0, self.1, self.2))?;
        if self.3 != 0 {
            f.write_fmt(format_args!("{:02x}", self.3))?;
        }
        Ok(())
    }
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
    color: Option<Color>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct MazeProp {
    #[serde(rename = "ID")]
    id: u32,
    prop_type: PropType,
    #[serde(default)]
    is_map_content: bool,
    prop_name: Text,
    prop_title: Text,
    prop_icon_path: PathBuf,
    board_show_list: Vec<u8>,
    config_entity_path: PathBuf,
    damage_type_list: Vec<Element>,
    mini_map_icon_type: Option<NonZero<u8>>,
    mini_map_state_icons: Vec<MiniMapStateIcon>,
    json_path: PathBuf,
    prop_state_list: Vec<PropState>,
    performance_type: PerformanceType,
    #[serde(default)]
    has_renderer_component: bool,
    lod_priority: Option<NonZero<u8>>,
}

impl ID for MazeProp {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.id
    }
}

impl<'a> PO<'a> for MazeProp {
    type VO = vo::map::MazeProp<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.id,
            r#type: self.prop_type,
            is_map_content: self.is_map_content,
            name: game.text(self.prop_name),
            title: game.text(self.prop_title),
            damage_type_list: &self.damage_type_list,
            mini_map_icon_type: self
                .mini_map_icon_type
                .map(NonZero::get)
                .unwrap_or_default(),
            mini_map_state_icons: &self.mini_map_state_icons,
            prop_state_list: &self.prop_state_list,
            performance_type: self.performance_type,
            has_renderer_component: self.has_renderer_component,
            lod_priority: self.lod_priority.map(NonZero::get).unwrap_or_default(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum SpaceType {
    Dream,
    Reality,
    Unknow,
}

#[serde_with::serde_as]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct WorldDataConfig {
    #[serde(rename = "ID")]
    id: u16,
    #[serde(default)]
    is_real_world: bool,
    #[serde(default)]
    is_show: bool,
    world_name: Text,
    world_desc: Option<Text>, // 仅在 1.6 及之前出现
    // 后面的字段都仅在 2.0 及之后出现
    world_language_name: Option<Text>,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    dynamic_optional_block: u16,
    train_space_type: Option<SpaceType>,
    map_space_type_list: Option<Vec<SpaceType>>,
    chapter_icon_big_path: Option<String>,
    chronicle_world_bg_path: Option<String>,
    chronicle_world_sub_bg_path: Option<String>,
    chronicle_world_predict_path: Option<String>,
    chronicle_world_processing_path: Option<String>,
    camera_width: Option<NonZero<u8>>,
    camera_height: Option<NonZero<u8>>,
    small_world_icon_path: Option<String>,
}

impl ID for WorldDataConfig {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.id
    }
}

impl<'a> PO<'a> for WorldDataConfig {
    type VO = vo::map::WorldDataConfig<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.id,
            is_real_world: self.is_real_world,
            is_show: self.is_show,
            name: game.text(self.world_name),
            language_name: self
                .world_language_name
                .map(|hash| game.text(hash))
                .unwrap_or_default(),
            dynamic_optional_block: self.dynamic_optional_block,
            map_space_type_list: self.map_space_type_list.as_deref().unwrap_or_default(),
            camera_width: self.camera_width.map(NonZero::get).unwrap_or_default(),
            camera_height: self.camera_height.map(NonZero::get).unwrap_or_default(),
        }
    }
}
