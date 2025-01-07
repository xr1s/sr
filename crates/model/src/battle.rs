use std::{num::NonZero, path::PathBuf};

use base::ID;

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
pub struct BattleEventOverrideProperty {
    pub property_type: BattleEventOverridePropertyType,
    pub value: Value<f32>,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct BattleEventConfig {
    #[serde(rename = "BattleEventID")]
    pub battle_event_id: u32,
    pub team: BattleEventTeam,
    pub event_sub_type: BattleEventSubType,
    pub battle_event_button_type: Option<NonZero<u8>>,
    #[serde(rename = "BEActionBarType")]
    pub be_action_bar_type: Option<NonZero<u8>>,
    pub head_icon: PathBuf,
    pub battle_event_name: String,
    pub ability_list: Vec<String>,
    pub override_property: Vec<BattleEventOverrideProperty>,
    pub speed: Value<u16>,
    #[serde(default)]
    pub hard_level: bool,
    #[serde(default)]
    pub elite_group: bool,
    pub descrption_text: String,
    pub param_list: Vec<Value<f32>>,
    pub asset_pack_name: String,
}

impl ID for BattleEventConfig {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.battle_event_id
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
pub struct StageConfigData {
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
    pub r#type: StageConfigType,
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
    pub value: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct StageConfig {
    #[serde(rename = "StageID")]
    pub stage_id: u32,
    pub stage_type: StageType,
    pub stage_name: Text,
    pub hard_level_group: u16,
    pub level: u8,
    pub elite_group: Option<NonZero<u16>>,
    pub level_graph_path: PathBuf,
    pub stage_ability_config: Vec<String>,
    pub battle_scoring_group: Option<NonZero<u16>>,
    // 各种配置文件, Key 也没解密，
    pub sub_level_graphs: Vec<fnv::FnvHashMap<String, String>>,
    pub stage_config_data: Vec<StageConfigData>,
    pub monster_list: Vec<fnv::FnvHashMap<String, u32>>,
    pub level_lose_condition: Vec<String>,
    pub level_win_condition: Vec<String>,
    #[serde(default)]
    pub forbid_auto_battle: bool,
    #[serde(default)]
    pub forbid_view_mode: bool,
    #[serde(default)]
    pub release: bool,
    #[serde(default)]
    pub forbid_exit_battle: bool,
    pub monster_warning_ratio: Option<f32>,
    #[serde(default)]
    pub reset_battle_speed: bool,
    pub trial_avatar_list: Vec<u32>,
}

impl ID for StageConfig {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.stage_id
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct StageInfiniteGroup {
    #[serde(rename = "WaveGroupID")]
    pub wave_group_id: u32,
    #[serde(rename = "WaveIDList")]
    pub wave_id_list: Vec<u32>,
}

impl ID for StageInfiniteGroup {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.wave_group_id
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct StageInfiniteMonsterGroup {
    #[serde(rename = "InfiniteMonsterGroupID")]
    pub infinite_monster_group_id: u32,
    pub monster_list: Vec<u32>,
    pub elite_group: Option<NonZero<u16>>,
}

impl ID for StageInfiniteMonsterGroup {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.infinite_monster_group_id
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct StageInfiniteWaveConfig {
    #[serde(rename = "InfiniteWaveID")]
    pub infinite_wave_id: u32,
    #[serde(rename = "MonsterGroupIDList")]
    pub monster_group_id_list: Vec<u32>,
    pub max_monster_count: u16,
    pub max_teammate_count: u8,
    pub ability: String,
    pub param_list: Vec<Value<f32>>,
    pub clear_previous_ability: bool,
}

impl ID for StageInfiniteWaveConfig {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.infinite_wave_id
    }
}
