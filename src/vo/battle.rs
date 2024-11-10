use crate::{
    po::battle::{
        BattleEventOverridePropertyType, BattleEventSubType, BattleEventTeam, StageConfigType,
        StageType,
    },
    vo, FnvIndexMap, GameData,
};

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

#[derive(Clone, Debug)]
pub struct EliteGroup {
    pub id: u16,
    pub attack_ratio: f32,
    pub defence_ratio: f32,
    pub hp_ratio: f32,
    pub speed_ratio: f32,
    pub stance_ratio: f32,
}

#[derive(derivative::Derivative)]
#[derivative(Clone, Debug)]
pub struct StageConfig<'a> {
    #[derivative(Debug = "ignore")]
    pub(crate) game: &'a GameData,
    pub id: u32,
    pub r#type: StageType,
    pub name: &'a str,
    pub hard_level_group: u16,
    pub level: u8,
    pub elite_group: Option<EliteGroup>,
    pub stage_config_data: fnv::FnvHashMap<StageConfigType, &'a str>,
    pub monster_list: Vec<Vec<vo::monster::MonsterConfig<'a>>>,
    pub forbid_auto_battle: bool,
    pub release: bool,
    pub forbid_exit_battle: bool,
}

impl StageConfig<'_> {
    pub fn infinite_group(&self) -> Option<StageInfiniteGroup<'_>> {
        self.stage_config_data[&StageConfigType::_StageInfiniteGroup]
            .parse::<u32>()
            .ok()
            .map(|id| self.game.stage_infinite_group(id))
            .map(Option::unwrap)
    }
}

#[derive(Clone, Debug)]
pub struct StageInfiniteGroup<'a> {
    pub id: u32,
    pub wave_list: Vec<StageInfiniteWaveConfig<'a>>,
}

#[derive(Clone, Debug)]
pub struct StageInfiniteMonsterGroup<'a> {
    pub id: u32,
    pub monster_list: Vec<vo::monster::MonsterConfig<'a>>,
    pub elite_group: Option<EliteGroup>,
}

#[derive(Clone, Debug)]
pub struct StageInfiniteWaveConfig<'a> {
    pub id: u32,
    pub monster_group_list: Vec<StageInfiniteMonsterGroup<'a>>,
    pub max_monster_count: u16,
    pub max_teammate_count: u8,
    pub ability: &'a str,
    pub param_list: Vec<f32>,
    pub clear_previous_ability: bool,
}
