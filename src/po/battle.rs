use std::{num::NonZero, path::PathBuf};

use crate::{vo, GameData, ID, PO};

use super::{Text, Value};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum BattleEventTeam {
    TeamDark,
    TeamLight,
    TeamNeutral,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum BattleEventSubType {
    AbyssTurnCountDownEvent,
    AssisEvent,
    ChallengerEvent,
    DummyCharacter,
    EnterStage,
    EvolveBuildCoundDownWarningEvent,
    Item,
    RogueMagicCoundDownWarningEvent,
    SummonUnit,
    TurnCountDownWarningEvent,
    TurnPrompt,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum BattleEventOverridePropertyType {
    AllDamangeTypeAddedRatio,
    AttackAddedRatio,
    AttackDelta,
    BaseAttack,
    BaseDefence,
    BaseHP,
    BaseSpeed,
    BreakDamageAddedRatioBase,
    CriticalChance,
    CriticalDamage,
    CriticalDamageBase,
    FireAddedRatio,
    FirePenetrate,
    HealTakenRatio,
    Level,
    MaxSP,
    StatusProbability,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct BattleEventOverrideProperty {
    property_type: BattleEventOverridePropertyType,
    value: Value<f32>,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct BattleEventConfig {
    #[serde(rename = "BattleEventID")]
    battle_event_id: u32,
    team: BattleEventTeam,
    event_sub_type: BattleEventSubType,
    battle_event_button_type: Option<NonZero<u8>>,
    #[serde(rename = "BEActionBarType")]
    be_action_bar_type: Option<NonZero<u8>>,
    head_icon: PathBuf,
    battle_event_name: String,
    ability_list: Vec<String>,
    override_property: Vec<BattleEventOverrideProperty>,
    speed: Value<u16>,
    #[serde(default)]
    hard_level: bool,
    #[serde(default)]
    elite_group: bool,
    descrption_text: String,
    param_list: Vec<Value<f32>>,
    asset_pack_name: String,
}

impl ID for BattleEventConfig {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.battle_event_id
    }
}

impl PO<'_> for BattleEventConfig {
    type VO = vo::battle::BattleEventConfig;
    fn vo(&self, _game: &GameData) -> Self::VO {
        Self::VO {
            id: self.battle_event_id,
            team: self.team,
            event_sub_type: self.event_sub_type,
            override_property: self
                .override_property
                .iter()
                .map(|prop| (prop.property_type, prop.value.value))
                .collect(),
            speed: self.speed.value,
            hard_level: self.hard_level,
            elite_group: self.elite_group,
        }
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct EliteGroup {
    elite_group: u16,
    attack_ratio: Value<f32>,
    defence_ratio: Value<f32>,
    #[serde(rename = "HPRatio")]
    hp_ratio: Value<f32>,
    speed_ratio: Value<f32>,
    stance_ratio: Value<f32>,
}

impl ID for EliteGroup {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.elite_group
    }
}

impl PO<'_> for EliteGroup {
    type VO = vo::battle::EliteGroup;
    fn vo(&self, _game: &GameData) -> Self::VO {
        Self::VO {
            id: self.elite_group,
            attack_ratio: self.attack_ratio.value,
            defence_ratio: self.defence_ratio.value,
            hp_ratio: self.hp_ratio.value,
            speed_ratio: self.speed_ratio.value,
            stance_ratio: self.stance_ratio.value,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum StageType {
    AetherDivide,
    BattleCollege,
    BoxingClub,
    Challenge,
    ClockParkActivity,
    Cocoon,
    EvolveBuildActivity,
    FantasticStory,
    FarmElement,
    FeverTimeActivity,
    FightActivity,
    FightFest,
    Heliobus,
    Mainline,
    PunkLord,
    RogueChallengeActivity,
    RogueEndlessActivity,
    RogueRelic,
    StarFightActivity,
    StrongChallengeActivity,
    SummonActivity,
    SwordTraining,
    TelevisionActivity,
    TreasureDungeon,
    Trial,
    VerseSimulation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum StageConfigType {
    _BattleCondition,
    _BattleTarget,
    _BGM,
    _BindingMazeBuff,
    _ChallengeStoryType,
    _CloseBattleStartDialog,
    _CreateBattleActionEvent,
    _CreateBattleEvent,
    _DeferCreateTrialPlayer,
    _EnsureTeamAliveKey,
    _IsEliteBattle,
    _MainMonster,
    _SpecialBattleStartCamera,
    _StageBannedAvatarID,
    _StageInfiniteGroup,
    _Wave,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub(crate) struct StageConfigData {
    #[serde(alias = "JOAHDHLLMDK")] // 1.0
    #[serde(alias = "OEOPENFDEML")] // 1.1
    #[serde(alias = "LFCIILHABDO")] // 1.2
    #[serde(alias = "COJNNIIOEAK")] // 1.3
    #[serde(alias = "JDKAMOANICM")] // 1.4
    #[serde(alias = "CFNMGGCLFHN")] // 1.5
    #[serde(alias = "JJNBOIODCCF")] // 1.6
    #[serde(alias = "DJBGPLLGOEF")] // 2.0
    #[serde(alias = "CEDKLKIHFEK")] // 2.1
    #[serde(alias = "MLMLDHKBPLM")] // 2.2
    #[serde(alias = "LFKFFCJNFKN")] // 2.3
    #[serde(alias = "MBBNDDLBEPE")] // 2.4
    #[serde(alias = "PFMLCKGCKOB")] // 2.5
    #[serde(alias = "MFKLINKCPPA")] // 2.6
    #[serde(alias = "BNCHHJCHKON")] // 2.7
    r#type: StageConfigType,
    #[serde(alias = "LKJLPJMIGNJ")] // 1.0
    #[serde(alias = "BHLILFMLNEE")] // 1.1
    #[serde(alias = "LGKGOMNMBAH")] // 1.2
    #[serde(alias = "MBOHKHKHFPD")] // 1.3
    #[serde(alias = "MOJJBFBKBNC")] // 1.4
    #[serde(alias = "JCFBPDLNMLH")] // 1.5
    #[serde(alias = "AMMAAKPAKAA")] // 1.6
    #[serde(alias = "BOANKOCFAIM")] // 2.0
    #[serde(alias = "IEDALJJJBCE")] // 2.1
    #[serde(alias = "PKPGBCJMDEK")] // 2.2
    #[serde(alias = "EPBOOFFCKPJ")] // 2.3
    #[serde(alias = "DIBKEHHCPAP")] // 2.4
    #[serde(alias = "NLABNDMDIKM")] // 2.5
    #[serde(alias = "HPPEILAONGE")] // 2.6
    #[serde(alias = "ODPKJEJKOIH")] // 2.7
    value: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct StageConfig {
    #[serde(rename = "StageID")]
    stage_id: u32,
    stage_type: StageType,
    stage_name: Text,
    hard_level_group: u16,
    level: u8,
    elite_group: Option<NonZero<u16>>,
    level_graph_path: PathBuf,
    stage_ability_config: Vec<String>,
    battle_scoring_group: Option<NonZero<u16>>,
    // 各种配置文件, Key 也没解密，
    sub_level_graphs: Vec<fnv::FnvHashMap<String, String>>,
    stage_config_data: Vec<StageConfigData>,
    monster_list: Vec<fnv::FnvHashMap<String, u32>>,
    level_lose_condition: Vec<String>,
    level_win_condition: Vec<String>,
    #[serde(default)]
    forbid_auto_battle: bool,
    #[serde(default)]
    forbid_view_mode: bool,
    #[serde(default)]
    release: bool,
    #[serde(default)]
    forbid_exit_battle: bool,
    monster_warning_ratio: Option<f32>,
    #[serde(default)]
    reset_battle_speed: bool,
    trial_avatar_list: Vec<u32>,
}

impl ID for StageConfig {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.stage_id
    }
}

impl<'a> PO<'a> for StageConfig {
    type VO = vo::battle::StageConfig<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
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
        Self::VO {
            game,
            id: self.stage_id,
            r#type: self.stage_type,
            name: game.text(self.stage_name),
            hard_level_group: self.hard_level_group,
            level: self.level,
            elite_group: self
                .elite_group
                .map(NonZero::get)
                .map(|id| game.elite_group(id))
                .map(Option::unwrap),
            stage_config_data: self
                .stage_config_data
                .iter()
                .map(|data| (data.r#type, data.value.as_str()))
                .collect(),
            monster_list: self
                .monster_list
                .iter()
                .map(monster_hm_to_vec)
                .collect::<Vec<_>>(),
            forbid_auto_battle: self.forbid_auto_battle,
            release: self.release,
            forbid_exit_battle: self.forbid_exit_battle,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct StageInfiniteGroup {
    #[serde(rename = "WaveGroupID")]
    wave_group_id: u32,
    #[serde(rename = "WaveIDList")]
    wave_id_list: Vec<u32>,
}

impl ID for StageInfiniteGroup {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.wave_group_id
    }
}

impl<'a> PO<'a> for StageInfiniteGroup {
    type VO = vo::battle::StageInfiniteGroup<'a>;
    fn vo(&self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.wave_group_id,
            wave_list: self
                .wave_id_list
                .iter()
                .map(|&id| game.stage_infinite_wave_config(id))
                .map(Option::unwrap)
                .collect(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct StageInfiniteMonsterGroup {
    #[serde(rename = "InfiniteMonsterGroupID")]
    infinite_monster_group_id: u32,
    monster_list: Vec<u32>,
    elite_group: Option<NonZero<u16>>,
}

impl ID for StageInfiniteMonsterGroup {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.infinite_monster_group_id
    }
}

impl<'a> PO<'a> for StageInfiniteMonsterGroup {
    type VO = vo::battle::StageInfiniteMonsterGroup<'a>;
    fn vo(&self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.infinite_monster_group_id,
            monster_list: self
                .monster_list
                .iter()
                .filter(|&&id| id != 0 && id != 300205001) // TODO: 疑似缺数据
                // 应该是王下一桶
                .map(|&id| game.monster_config(id))
                .map(Option::unwrap)
                .collect(),
            elite_group: self
                .elite_group
                .map(NonZero::get)
                .map(|id| game.elite_group(id))
                .map(Option::unwrap),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct StageInfiniteWaveConfig {
    #[serde(rename = "InfiniteWaveID")]
    infinite_wave_id: u32,
    #[serde(rename = "MonsterGroupIDList")]
    monster_group_id_list: Vec<u32>,
    max_monster_count: u16,
    max_teammate_count: u8,
    ability: String,
    param_list: Vec<Value<f32>>,
    clear_previous_ability: bool,
}

impl ID for StageInfiniteWaveConfig {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.infinite_wave_id
    }
}

impl<'a> PO<'a> for StageInfiniteWaveConfig {
    type VO = vo::battle::StageInfiniteWaveConfig<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.infinite_wave_id,
            monster_group_list: self
                .monster_group_id_list
                .iter()
                .map(|&id| game.stage_infinite_monster_group(id))
                .map(Option::unwrap)
                .collect(),
            max_monster_count: self.max_monster_count,
            max_teammate_count: self.max_teammate_count,
            ability: &self.ability,
            param_list: self.param_list.iter().map(|value| value.value).collect(),
            clear_previous_ability: self.clear_previous_ability,
        }
    }
}
