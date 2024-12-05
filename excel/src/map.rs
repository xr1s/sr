use std::num::NonZero;

pub use model::map::{
    FloorTag, FloorType, MapEntranceType, MapSpaceType, MappingInfoFarmType, MappingInfoType,
    MiniMapStateIcon, PlaneType, PropPerformanceType, PropState, PropType,
};
use model::Element;

use crate::{ExcelOutput, FromModel};

#[derive(Clone, Debug)]
pub struct MapEntrance<'a> {
    pub id: u32,
    pub r#type: MapEntranceType,
    pub plane: MazePlane<'a>,
    pub floor: MazeFloor<'a>,
    pub begin_main_mission_list: Vec<crate::mission::MainMission<'a>>,
    pub finish_main_mission_list: Vec<crate::mission::MainMission<'a>>,
    pub finish_sub_mission_list: Vec<crate::mission::SubMission<'a>>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for MapEntrance<'a> {
    type Model = model::map::MapEntrance;
    fn from_model(game: &'a Data, model: &'a Self::Model) -> Self {
        Self {
            id: model.id,
            r#type: model.entrance_type,
            plane: game.maze_plane(model.plane_id).unwrap(),
            floor: game.maze_floor(model.floor_id).unwrap(),
            begin_main_mission_list: model
                .begin_main_mission_list
                .iter()
                .map(|&id| game.main_mission(id))
                .map(Option::unwrap)
                .collect(),
            finish_main_mission_list: model
                .finish_main_mission_list
                .iter()
                .map(|&id| game.main_mission(id))
                .map(Option::unwrap)
                .collect(),
            finish_sub_mission_list: model
                .finish_sub_mission_list
                .iter()
                .map(|&id| game.sub_mission(id))
                .map(Option::unwrap)
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MappingInfo<'a, Data: ExcelOutput + ?Sized> {
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
    pub show_monster_list: Vec<crate::monster::MonsterConfig<'a, Data>>,
    pub display_item_list: Vec<crate::item::ItemList<'a>>,
    pub entrance: Option<MapEntrance<'a>>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for MappingInfo<'a, Data> {
    type Model = model::map::MappingInfo;
    fn from_model(game: &'a Data, model: &'a Self::Model) -> Self {
        Self {
            id: model.id,
            world_level: model.world_level,
            r#type: model.r#type,
            farm_type: model.farm_type,
            is_teleport: model.is_teleport,
            is_show_in_fog: model.is_show_in_fog,
            plane: model
                .plane_id
                .map(NonZero::get)
                .map(|id| game.maze_plane(id))
                .map(Option::unwrap),
            floor: model
                .floor_id
                .map(NonZero::get)
                .map(|id| game.maze_floor(id))
                .map(Option::unwrap),
            group_id: model.group_id.map(NonZero::get).unwrap_or_default(),
            config_id: model.config_id.map(NonZero::get).unwrap_or_default(),
            initial_enable: model.initial_enable,
            name: game.text(model.name),
            desc: game.text(model.desc),
            show_monster_list: model
                .show_monster_list
                .iter()
                .map(|&id| game.monster_config(id))
                .map(Option::unwrap)
                .collect(),
            display_item_list: model
                .display_item_list
                .iter()
                .map(|item| crate::item::ItemList::from_model(game, item))
                .collect(),
            entrance: model
                .entrance_id
                .map(NonZero::get)
                .map(|id| game.map_entrance(id))
                .map(Option::unwrap),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MazeFloor<'a> {
    pub id: u32,
    pub base_floor_id: u32, // 不能直接存 MazeFloor，不然会递归
    pub floor_tag: &'a [FloorTag],
    pub floor_type: FloorType,
    pub map_layer_name_list: Vec<&'a str>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for MazeFloor<'a> {
    type Model = model::map::MazeFloor;
    fn from_model(game: &'a Data, model: &'a Self::Model) -> Self {
        Self {
            id: model.floor_id,
            base_floor_id: model.base_floor_id,
            floor_tag: model.floor_tag.as_deref().unwrap_or_default(),
            floor_type: model.floor_type,
            map_layer_name_list: model
                .map_layer_name_list
                .iter()
                .map(|&text| game.text(text))
                .collect(),
        }
    }
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

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for MazePlane<'a> {
    type Model = model::map::MazePlane;
    fn from_model(game: &'a Data, model: &'a Self::Model) -> Self {
        Self {
            id: model.plane_id,
            r#type: model.plane_type,
            sub_type: model.sub_type,
            maze_pool_type: model.maze_pool_type,
            world: game.world_data_config(model.world_id).unwrap(),
            name: game.text(model.plane_name),
            start_floor: game.maze_floor(model.start_floor_id),
            floor_list: model
                .floor_id_list
                .iter()
                .map(|&id| game.maze_floor(id))
                .map(Option::unwrap)
                .collect(),
        }
    }
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
    pub performance_type: PropPerformanceType,
    pub has_renderer_component: bool,
    pub lod_priority: u8,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for MazeProp<'a> {
    type Model = model::map::MazeProp;
    fn from_model(game: &'a Data, model: &'a Self::Model) -> Self {
        Self {
            id: model.id,
            r#type: model.prop_type,
            is_map_content: model.is_map_content,
            name: game.text(model.prop_name),
            title: game.text(model.prop_title),
            damage_type_list: &model.damage_type_list,
            mini_map_icon_type: model
                .mini_map_icon_type
                .map(NonZero::get)
                .unwrap_or_default(),
            mini_map_state_icons: &model.mini_map_state_icons,
            prop_state_list: &model.prop_state_list,
            performance_type: model.performance_type,
            has_renderer_component: model.has_renderer_component,
            lod_priority: model.lod_priority.map(NonZero::get).unwrap_or_default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct WorldDataConfig<'a> {
    pub id: u16,
    pub is_real_world: bool,
    pub is_show: bool,
    pub name: &'a str,
    pub language_name: &'a str,
    pub dynamic_optional_block: u16,
    pub map_space_type_list: &'a [MapSpaceType],
    pub camera_width: u8,
    pub camera_height: u8,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for WorldDataConfig<'a> {
    type Model = model::map::WorldDataConfig;
    fn from_model(game: &'a Data, model: &'a Self::Model) -> Self {
        Self {
            id: model.id,
            is_real_world: model.is_real_world,
            is_show: model.is_show,
            name: game.text(model.world_name),
            language_name: model
                .world_language_name
                .map(|hash| game.text(hash))
                .unwrap_or_default(),
            dynamic_optional_block: model.dynamic_optional_block,
            map_space_type_list: model.map_space_type_list.as_deref().unwrap_or_default(),
            camera_width: model.camera_width.map(NonZero::get).unwrap_or_default(),
            camera_height: model.camera_height.map(NonZero::get).unwrap_or_default(),
        }
    }
}
