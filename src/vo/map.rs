use crate::po::map::{
    EntranceType, FloorTag, FloorType, MappingInfoFarmType, MappingInfoType, MiniMapStateIcon,
    PerformanceType, PlaneType, PropState, PropType, SpaceType,
};
use crate::po::Element;
use crate::vo;

#[derive(Clone, Debug)]
pub struct MapEntrance<'a> {
    pub id: u32,
    pub r#type: EntranceType,
    pub plane: MazePlane<'a>,
    pub floor: MazeFloor<'a>,
    pub begin_main_mission_list: Vec<vo::mission::MainMission<'a>>,
    pub finish_main_mission_list: Vec<vo::mission::MainMission<'a>>,
    pub finish_sub_mission_list: Vec<vo::mission::SubMission<'a>>,
}

#[derive(Clone, Debug)]
pub struct MappingInfo<'a> {
    pub id: u32,
    pub world_level: u8,
    pub r#type: Option<MappingInfoType>,
    pub farm_type: Option<MappingInfoFarmType>,
    pub is_teleport: bool,
    pub is_show_in_fog: bool,
    pub plane: Option<MazePlane<'a>>,
    pub floor: Option<MazeFloor<'a>>,
    pub group_id: u16,
    pub config_id: u32,
    pub initial_enable: bool,
    pub name: &'a str,
    pub desc: &'a str,
    pub show_monster_list: Vec<vo::monster::MonsterConfig<'a>>,
    pub display_item_list: Vec<vo::item::ItemList<'a>>,
    pub entrance: Option<MapEntrance<'a>>,
}

#[derive(Clone, Debug)]
pub struct MazeFloor<'a> {
    pub id: u32,
    pub base_floor_id: u32, // 不能直接存 MazeFloor，不然会递归
    pub floor_tag: &'a [FloorTag],
    pub floor_type: FloorType,
    pub map_layer_name_list: Vec<&'a str>,
}

#[derive(Clone, Debug)]
pub struct MazePlane<'a> {
    pub id: u32,
    pub r#type: PlaneType,
    pub sub_type: u8,
    pub maze_pool_type: u32,
    pub world: WorldDataConfig<'a>,
    pub name: &'a str,
    // 奇怪的是 start_floor_id 可能在 MazeFloor 中不存在
    pub start_floor: Option<MazeFloor<'a>>,
    pub floor_list: Vec<MazeFloor<'a>>,
}

#[derive(Clone, Debug)]
pub struct MazeProp<'a> {
    pub id: u32,
    pub r#type: PropType,
    pub is_map_content: bool,
    pub name: &'a str,
    pub title: &'a str,
    pub damage_type_list: &'a [Element],
    pub mini_map_icon_type: u8,
    pub mini_map_state_icons: &'a [MiniMapStateIcon],
    pub prop_state_list: &'a [PropState],
    pub performance_type: PerformanceType,
    pub has_renderer_component: bool,
    pub lod_priority: u8,
}

#[derive(Clone, Debug)]
pub struct WorldDataConfig<'a> {
    pub id: u16,
    pub is_real_world: bool,
    pub is_show: bool,
    pub name: &'a str,
    pub language_name: &'a str,
    pub dynamic_optional_block: u16,
    pub map_space_type_list: &'a [SpaceType],
    pub camera_width: u8,
    pub camera_height: u8,
}
