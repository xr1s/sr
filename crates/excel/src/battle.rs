use std::num::NonZero;

use base::FnvIndexMap;
pub use model::battle::{
    BattleEventOverridePropertyType, BattleEventSubType, BattleEventTeam, StageConfigType,
    StageType,
};

use crate::{ExcelOutput, FromModel};

#[derive(Clone, Debug)]
pub struct BattleEventConfig {
    pub id: u32,
    pub team: BattleEventTeam,
    pub event_sub_type: BattleEventSubType,
    pub override_property: FnvIndexMap<BattleEventOverridePropertyType, f32>,
    pub speed: u16,
    pub hard_level: bool,
    pub elite_group: bool,
}

impl<Data: ExcelOutput> FromModel<'_, Data> for BattleEventConfig {
    type Model = model::battle::BattleEventConfig;
    fn from_model(_game: &Data, model: &Self::Model) -> Self {
        Self {
            id: model.battle_event_id,
            team: model.team,
            event_sub_type: model.event_sub_type,
            override_property: model
                .override_property
                .iter()
                .map(|prop| (prop.property_type, prop.value.value))
                .collect(),
            speed: model.speed.value,
            hard_level: model.hard_level,
            elite_group: model.elite_group,
        }
    }
}

#[derive(educe::Educe)]
#[educe(Clone, Debug)]
pub struct StageConfig<'a, Data: crate::ExcelOutput + ?Sized> {
    #[educe(Debug(ignore))]
    game: &'a Data,
    pub id: u32,
    pub r#type: StageType,
    pub name: &'a str,
    /// 敌方属性成长曲线，这里根据 level 字段从 1~100 级所有成长曲线里取出了对应的值
    pub hard_level_group: crate::monster::HardLevelGroup,
    /// 敌方等级
    pub level: u8,
    pub elite_group: Option<crate::monster::EliteGroup>,
    /// 一些额外信息的键值对
    /// 比如 BGM，比如虚构叙事对应的 StageInfiniteGroup 信息
    pub stage_config_data: fnv::FnvHashMap<StageConfigType, &'a str>,
    /// 只有混沌回忆和虚构叙事该字段非空
    /// 混沌回忆就是敌方阵容，也是未进入秘境时预览用的敌方信息、敌人列表
    /// 虚构叙事只是未进入秘境时预览用的敌方信息、敌人列表
    /// 有两波则外层 Vec 长度为 2、有三波则外层 Vec 长度为 3，以此类推
    /// 内层 Vec 是每一波不同的敌人，即使波次内怪物会重复出现，这里不会有重复
    pub monster_list: Vec<Vec<crate::monster::MonsterConfig<'a, Data>>>,
    pub forbid_auto_battle: bool,
    pub release: bool,
    pub forbid_exit_battle: bool,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for StageConfig<'a, Data> {
    type Model = model::battle::StageConfig;
    fn from_model(game: &'a Data, model: &'a Self::Model) -> Self {
        let monster_id_to_object = |id: &u32| {
            None.or_else(|| game.monster_config(*id))
                .or_else(|| game.monster_unique_config(*id))
        };
        let monster_hm_to_vec = |monster_list: &fnv::FnvHashMap<String, u32>| {
            monster_list
                .values()
                .map(monster_id_to_object)
                .map(Option::unwrap)
                .collect::<Vec<_>>()
        };
        Self {
            game,
            id: model.stage_id,
            r#type: model.stage_type,
            name: game.text(model.stage_name),
            hard_level_group: game
                .hard_level_group(model.hard_level_group)
                .into_iter()
                .find(|group| group.level == model.level)
                .unwrap(),
            level: model.level,
            elite_group: model
                .elite_group
                .map(NonZero::get)
                .map(|id| game.elite_group(id))
                .map(Option::unwrap),
            stage_config_data: model
                .stage_config_data
                .iter()
                .map(|data| (data.r#type, data.value.as_str()))
                .collect(),
            monster_list: model
                .monster_list
                .iter()
                .map(monster_hm_to_vec)
                .collect::<Vec<_>>(),
            forbid_auto_battle: model.forbid_auto_battle,
            release: model.release,
            forbid_exit_battle: model.forbid_exit_battle,
        }
    }
}

impl<Data: ExcelOutput> StageConfig<'_, Data> {
    pub fn infinite_group(&self) -> Option<StageInfiniteGroup<'_, Data>> {
        self.stage_config_data[&StageConfigType::_StageInfiniteGroup]
            .parse::<u32>()
            .ok()
            .map(|id| self.game.stage_infinite_group(id))
            .map(Option::unwrap)
    }
}

#[derive(educe::Educe)]
#[educe(Clone, Debug)]
pub struct StageInfiniteGroup<'a, Data: ExcelOutput + ?Sized> {
    pub id: u32,
    pub wave_list: Vec<StageInfiniteWaveConfig<'a, Data>>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for StageInfiniteGroup<'a, Data> {
    type Model = model::battle::StageInfiniteGroup;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            id: model.wave_group_id,
            wave_list: model
                .wave_id_list
                .iter()
                .map(|&id| game.stage_infinite_wave_config(id))
                .map(Option::unwrap)
                .collect(),
        }
    }
}

#[derive(educe::Educe)]
#[educe(Clone, Debug)]
pub struct StageInfiniteMonsterGroup<'a, Data: ExcelOutput + ?Sized> {
    pub id: u32,
    pub monster_list: Vec<crate::monster::MonsterConfig<'a, Data>>,
    pub elite_group: Option<crate::monster::EliteGroup>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for StageInfiniteMonsterGroup<'a, Data> {
    type Model = model::battle::StageInfiniteMonsterGroup;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            id: model.infinite_monster_group_id,
            monster_list: model
                .monster_list
                .iter()
                .filter(|&&id| id != 0 && id != 300205001) // TODO: 疑似缺数据
                // 应该是王下一桶
                .map(|&id| game.monster_config(id))
                .map(Option::unwrap)
                .collect(),
            elite_group: model
                .elite_group
                .map(NonZero::get)
                .map(|id| game.elite_group(id))
                .map(Option::unwrap),
        }
    }
}

#[derive(educe::Educe)]
#[educe(Clone, Debug)]
pub struct StageInfiniteWaveConfig<'a, Data: ExcelOutput + ?Sized> {
    pub id: u32,
    pub monster_group_list: Vec<StageInfiniteMonsterGroup<'a, Data>>,
    pub max_monster_count: u16,
    pub max_teammate_count: u8,
    pub ability: &'a str,
    pub param_list: Vec<f32>,
    pub clear_previous_ability: bool,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for StageInfiniteWaveConfig<'a, Data> {
    type Model = model::battle::StageInfiniteWaveConfig;
    fn from_model(game: &'a Data, model: &'a Self::Model) -> Self {
        Self {
            id: model.infinite_wave_id,
            monster_group_list: model
                .monster_group_id_list
                .iter()
                .map(|&id| game.stage_infinite_monster_group(id))
                .map(Option::unwrap)
                .collect(),
            max_monster_count: model.max_monster_count,
            max_teammate_count: model.max_teammate_count,
            ability: &model.ability,
            param_list: model.param_list.iter().map(|value| value.value).collect(),
            clear_previous_ability: model.clear_previous_ability,
        }
    }
}
