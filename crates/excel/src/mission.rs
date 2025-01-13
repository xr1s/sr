use std::num::NonZero;

pub use model::mission::MainMissionType;

use crate::{misc::RewardData, ExcelOutput, FromModel};

#[derive(Clone, Debug)]
pub struct MissionChapterConfig {
    pub id: u32,
    pub display_priority: u32,
}

impl<Data: ExcelOutput> FromModel<'_, Data> for MissionChapterConfig {
    type Model = model::mission::MissionChapterConfig;
    fn from_model(_game: &Data, model: &Self::Model) -> Self {
        Self {
            id: model.id,
            display_priority: model.chapter_display_priority,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MainMission<'a> {
    pub id: u32,
    pub r#type: MainMissionType,
    pub world: Option<crate::map::WorldDataConfig<'a>>,
    pub display_priority: u32,
    pub name: &'a str,
    pub next_track_main_mission: Option<NonZero<u32>>, // 不能直接存自己，不然会递归
    pub track_weight: Option<NonZero<u8>>,
    pub reward: Option<RewardData<'a>>,
    pub display_reward: Option<RewardData<'a>>,
    pub chapter: Option<MissionChapterConfig>,
    pub sub_reward_list: Vec<RewardData<'a>>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for MainMission<'a> {
    type Model = model::mission::MainMission;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            id: model.main_mission_id,
            r#type: model.r#type,
            world: model
                .world_id
                .map(NonZero::get)
                .map(|id| game.world_data_config(id))
                .map(Option::unwrap),
            display_priority: model.display_priority,
            name: game.text(model.name),
            next_track_main_mission: model.next_track_main_mission,
            track_weight: model.track_weight,
            reward: model
                .reward_id
                .map(NonZero::get)
                .map(|id| game.reward_data(id))
                .map(Option::unwrap),
            display_reward: model
                .display_reward_id
                .map(NonZero::get)
                .map(|id| game.reward_data(id))
                .map(Option::unwrap),
            chapter: model
                .chapter_id
                .map(NonZero::get)
                .map(|id| game.mission_chapter_config(id))
                .map(Option::unwrap),
            sub_reward_list: model
                .sub_reward_list
                .iter()
                .map(|&id| game.reward_data(id))
                .map(Option::unwrap)
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubMission<'a> {
    pub id: u32,
    pub target: &'a str,
    pub description: &'a str,
}
impl<'a, Data: ExcelOutput> FromModel<'a, Data> for SubMission<'a> {
    type Model = model::mission::SubMission;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            id: model.sub_missoin_id,
            target: game.text(model.target_text),
            description: game.text(model.descrption_text),
        }
    }
}
