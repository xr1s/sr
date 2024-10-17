use crate::po::{Element, Text, Value};
use crate::vo;
use crate::{GameData, ID, PO};
use std::num::NonZero;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
/// 怪物稀有度
/// - BigBoss 周本首领以及逐光捡金变种
/// - LittleBoss 剧情敌人首领，如杰帕德、银枝等
/// - Elite 精英怪，凝滞虚影（角色突破材料）
/// - Minion 小怪
/// - MinionV2 小怪
pub enum Rank {
    /// 周本 Boss
    BigBoss,
    /// 精英怪物
    Elite,
    /// 剧情 Boss
    LittleBoss,
    /// 普通怪物，目前总共就 21 种，不清楚和 MinionLv2 的区别
    Minion,
    /// 普通怪物，不清楚和 Minion 的区别
    MinionLv2,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum BaseType {
    #[serde(rename = "")]
    None,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum StanceType {
    Fire,
    Ice,
    Imaginary,
    Quantum,
    Wind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum CampType {
    Monster,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum CharacterType {
    NPCMonster,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum SubType {
    Monster,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum AttackType {
    Normal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum DebuffResistKey {
    #[serde(rename = "STAT_Confine")]
    Confine,
    #[serde(rename = "STAT_CTRL")]
    Ctrl,
    #[serde(rename = "STAT_CTRL_Frozen")]
    Frozen,
    #[serde(rename = "STAT_DOT_Burn")]
    Burn,
    #[serde(rename = "STAT_DOT_Electric")]
    Electric,
    #[serde(rename = "STAT_DOT_Poison")]
    Poison,
    #[serde(rename = "STAT_Entangle")]
    Entangle,
}

impl crate::Wiki for DebuffResistKey {
    fn wiki(&self) -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(match self {
            Self::Confine => "Confine",
            Self::Ctrl => "控制",
            Self::Frozen => "冻结",
            Self::Burn => "灼烧",
            Self::Electric => "触电",
            Self::Poison => "Poison",
            Self::Entangle => "纠缠",
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum CustomValueTag {
    #[serde(rename = "")]
    None,
    #[serde(rename = "Argenti_Totem")]
    ArgentiTotem,
    Company,
    #[serde(rename = "Huanlong_Flower")]
    HuanlongFlower,
    #[serde(rename = "IF_Boss")]
    IfBoss,
    #[serde(rename = "IF_W1_Bronya")]
    IfW1Bronya,
    #[serde(rename = "IF_W1_CocoliaP1")]
    IfW1CocoliaP1,
    #[serde(rename = "IF_W1_CocoliaP2")]
    IfW1CocoliaP2,
    #[serde(rename = "IF_W1_Gepard")]
    IfW1Gepard,
    #[serde(rename = "IF_W1_Ice")]
    IfW1Ice,
    #[serde(rename = "IF_W2_Kafka")]
    IfW2Kafka,
    #[serde(rename = "IF_W2_Sword")]
    IfW2Sword,
    #[serde(rename = "IF_W2_Xuanlu")]
    IfW2Xuanlu,
    #[serde(rename = "IF_W2_Yanqing")]
    IfW2Yanqing,
    LockTarge801201002,
    LockTarge801201003,
    LockTarge801201004,
    LockTarget,
    LockTarget01,
    LockTarget02,
    LockTarget801203001,
    LockTarget801203002,
    #[serde(rename = "Monster_Argenti")]
    MonsterArgenti,
    #[serde(rename = "Monster_DeathPart")]
    MonsterDeathPart,
    #[serde(rename = "Monster_Minion04")]
    MonsterMinion04,
    #[serde(rename = "Monster_Sam_RLBoss")]
    MonsterSamRLBoss,
    #[serde(rename = "MonsterType_W1_Mecha2")]
    MonsterTypeW1Mecha2,
    #[serde(rename = "MonsterType_W2_Lycan")]
    MonsterTypeW2Lycan,
    #[serde(rename = "MonsterType_W2_Lycan_00")]
    MonsterTypeW2Lycan00,
    #[serde(rename = "MonsterType_W2_Lycan_01")]
    MonsterTypeW2Lycan01,
    #[serde(rename = "MonsterType_W2_LycanMecha_00")]
    MonsterTypeW2LycanMecha00,
    #[serde(rename = "MonsterType_W3_Junk_00")]
    MonsterTypeW3Junk00,
    #[serde(rename = "MonsterType_W3_TV_00")]
    MonsterTypeW3TV00,
    #[serde(rename = "MonsterType_Xuanlu")]
    MonsterTypeXuanlu,
    RL,
    #[allow(clippy::upper_case_acronyms)]
    SPRL,
    Summon,
    #[serde(rename = "SuperArmor_Behit_Big")]
    SuperArmorBehitBig,
    #[serde(rename = "SuperArmor_Behit_Small")]
    SuperArmorBehitSmall,
    #[serde(rename = "SuperArmor_Behit_VerySmall")]
    SuperArmorBehitVerySmall,
    #[serde(rename = "SW_Boss")]
    SWBoss,
    #[serde(rename = "SW_Minion01")]
    SWMinion01,
    #[serde(rename = "W1_Bronya")]
    W1Bronya,
    #[serde(rename = "W1_Gepard_2Phase")]
    W1Gepard2Phase,
    #[serde(rename = "W1_Ice")]
    W1Ice,
    #[serde(rename = "W1_Mecha01")]
    W1Mecha01,
    #[serde(rename = "W1_Soldier")]
    W1Soldier,
    #[serde(rename = "W1_SvarogPart")]
    W1SvarogPart,
    #[serde(rename = "W2_Huanlong")]
    W2Huanlong,
    #[serde(rename = "W2_Kafka")]
    W2Kafka,
    #[serde(rename = "W2_Mecha02")]
    W2Mecha02,
    #[serde(rename = "W2_Yanqing_2Phase")]
    W2Yanqing2Phase,
    #[serde(rename = "W3_Figure_00")]
    W3Figure00,
    #[serde(rename = "W3_Figure_01")]
    W3Figure01,
    #[serde(rename = "W3_Figure_02")]
    W3Figure02,
    #[serde(rename = "Week")]
    Week,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum AbilityName {
    #[serde(rename = "Monster_AML_Elite01_00_MainLineN2_SpecialWin")]
    MonsterAMLElite0100MainLineN2SpecialWin,
    #[serde(rename = "Monster_Common_BossInfoBar")]
    MonsterCommonBossInfoBar,
    #[serde(rename = "Monster_Common_PassiveSkill_RemoveOneMorePerturn")]
    MonsterCommonPassiveSkillRemoveOneMorePerturn,
    #[serde(rename = "Monster_Common_SpecialDieEffect")]
    MonsterCommonSpecialDieEffect,
    #[serde(rename = "Monster_Common_SpecialDieEffect2")]
    MonsterCommonSpecialDieEffect2,
    #[serde(rename = "Monster_Junk_Special_ChangeWidth")]
    MonsterJunkSpecialChangeWidth,
    #[serde(rename = "Monster_SW_Boss_01_PassiveSkillMainBattle")]
    MonsterSwBoss01PassiveSkillMainBattle,
    #[serde(rename = "Monster_W1_Bronya_00_PassiveSkill_M1LockHP")]
    MonsterW1Bronya00PassiveSkillM1lockHp,
    #[serde(rename = "Monster_W1_Bronya_00_PassiveSkill_NoLockStance")]
    MonsterW1Bronya00PassiveSkillNoLockStance,
    #[serde(rename = "Monster_W1_Bronya_RL_ShowHPBar")]
    MonsterW1BronyaRlShowHpbar,
    #[serde(rename = "Monster_W1_CocoliaP2_00_SpecialAbility_M3_3_2")]
    MonsterW1CocoliaP2_00SpecialAbilityM3_3_2,
    #[serde(rename = "Monster_W1_Gepard_00_PassiveSkill_M0SpecialVictory")]
    MonsterW1Gepard00PassiveSkillM0specialVictory,
    #[serde(rename = "Monster_W1_GSMecha_01_PassiveSkill_KlaraCamera")]
    MonsterW1Gsmecha01PassiveSkillKlaraCamera,
    #[serde(rename = "Monster_W1_GSMecha_01_PassiveSkill_KlaraEnterBattleCamera")]
    MonsterW1Gsmecha01PassiveSkillKlaraEnterBattleCamera,
    #[serde(rename = "Monster_W1_GSMecha_01_PassiveSkill_KlaraSpecialVictory")]
    MonsterW1Gsmecha01PassiveSkillKlaraSpecialVictory,
    #[serde(rename = "Monster_W1_Mecha01_01_PassiveSkill_KlaraSpecialVictory")]
    MonsterW1Mecha01_01PassiveSkillKlaraSpecialVictory,
    #[serde(rename = "Monster_W1_Mecha04_00_PassiveSkill_KlaraSpecialVictory")]
    MonsterW1Mecha04_00PassiveSkillKlaraSpecialVictory,
    #[serde(rename = "Monster_W1_Mecha04_00_VanishTestAbility")]
    MonsterW1Mecha04_00VanishTestAbility,
    #[serde(rename = "Monster_W2_Abomi04_00_PassiveSkill_DanHeng")]
    MonsterW2Abomi04_00PassiveSkillDanHeng,
    #[serde(rename = "Monster_W2_Argenti_00_PassiveSkill202")]
    MonsterW2Argenti00PassiveSkill202,
    #[serde(rename = "Monster_W2_Lycan_00_MainStoryInitiate")]
    MonsterW2Lycan00MainStoryInitiate,
    #[serde(rename = "Monster_W2_Lycan_00_MainStoryInitiate2")]
    MonsterW2Lycan00MainStoryInitiate2,
    #[serde(rename = "Monster_W2_Lycan_01_MainStoryInitiate")]
    MonsterW2Lycan01MainStoryInitiate,
    #[serde(rename = "Monster_W2_Lycan_01_MainStoryInitiate240")]
    MonsterW2Lycan01MainStoryInitiate240,
    #[serde(rename = "Monster_W2_Lycan_01_MainStoryInitiateModu")]
    MonsterW2Lycan01MainStoryInitiateModu,
    #[serde(rename = "Monster_W2_LycanMecha_00_MainStoryInitiate")]
    MonsterW2LycanMecha00MainStoryInitiate,
    #[serde(rename = "Monster_W2_Xuanlu_00_Mainline_Final")]
    MonsterW2Xuanlu00MainlineFinal,
    #[serde(rename = "Monster_W2_Xuanlu_00_Mainline_Heal")]
    MonsterW2Xuanlu00MainlineHeal,
    #[serde(rename = "Monster_W2_Yanqing_00_PassiveSkillMainBattle")]
    MonsterW2Yanqing00PassiveSkillMainBattle,
    #[serde(rename = "Monster_W3_Death_00_PuzzleGame")]
    MonsterW3Death00PuzzleGame,
    #[serde(rename = "Monster_W3_Figure_Solo_PassiveSkill_Initiate")]
    MonsterW3FigureSoloPassiveSkillInitiate,
    #[serde(rename = "Monster_W3_Figure_Solo_RLElite_PassiveSkill_Initiate")]
    MonsterW3FigureSoloRlelitePassiveSkillInitiate,
    #[serde(rename = "Monster_W3_TV_00_PuzzleGamePassive")]
    MonsterW3Tv00PuzzleGamePassive,
    #[serde(rename = "Monster_XP_Elite02_02_BattlePerformAbility")]
    MonsterXpElite02_02BattlePerformAbility,
    #[serde(rename = "Monster_XP_Elite02_02_InstantDirtyHPAbility")]
    MonsterXpElite02_02InstantDirtyHpability,
    #[serde(rename = "TutorialAbility_Danheng_MonsterLockAlan")]
    TutorialAbilityDanhengMonsterLockAlan,
    #[serde(rename = "TutorialAbility_Danheng_MonsterLockDanheng")]
    TutorialAbilityDanhengMonsterLockDanheng,
    #[serde(rename = "TutorialAbility_Danheng_MonsterLockKlara")]
    TutorialAbilityDanhengMonsterLockKlara,
    #[serde(rename = "TutorialAbility_Frozen_MonsterLockPlayer")]
    TutorialAbilityFrozenMonsterLockPlayer,
    #[serde(rename = "TutorialAbility_Monster_XP_Minion02_00_MonsterLockPlayer")]
    TutorialAbilityMonsterXpMinion02_00MonsterLockPlayer,
    #[serde(rename = "TutorialAbility_W1_Mecha04_01_MonsterLockSeele")]
    TutorialAbilityW1Mecha04_01MonsterLockSeele,
    #[serde(rename = "TutorialAbility_W1_Soldier01_03_BuffCheck")]
    TutorialAbilityW1Soldier01_03BuffCheck,
    #[serde(rename = "TutorialAbility_W1_Soldier04_00_MonsterLockHerta")]
    TutorialAbilityW1Soldier04_00MonsterLockHerta,
    #[serde(rename = "WMonster_Common_HitAddStun")]
    WmonsterCommonHitAddStun,
    #[serde(rename = "W_SpecialBP_MonsterPassiveAbility")]
    WSpecialBpMonsterPassiveAbility,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum SkillTriggerKey {
    PassiveSkill01,
    PassiveSkill02,
    PassiveSkill03,
    PassiveSkill04,
    PassiveSkill05,
    PassiveSkillInitiate,
    Skill01,
    Skill02,
    #[serde(rename = "Skill02_Extra")]
    Skill02Extra,
    Skill03,
    Skill04,
    Skill042,
    Skill04Insert,
    Skill05,
    Skill052,
    Skill06,
    Skill07,
    Skill08,
    Skill09,
    Skill10,
    Skill11,
    Skill12,
    Skill13,
    Skill14,
    Skill15,
    Skill16,
    Skill17,
    SkillEX01,
    SkillEX02,
    SkillEX03,
    SkillEX04,
    SkillEX05,
    SkillIF01,
    SkillP01,
    SkillP02,
    SkillP03,
    SkillP04,
    SkillPerform01,
    SkillRage,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum SkillModifier {
    #[serde(rename = "Monster_APShow_Infinite_NotCancel")]
    InfiniteNotCancel,
    #[serde(rename = "Monster_APShow_OneTurn")]
    OneTurn,
    #[serde(rename = "Monster_APShow_OneTurn_NotCancel")]
    OneTurnNotCancel,
    #[serde(rename = "Monster_APShow_SevenTurn")]
    ShowSevenTurn,
    #[serde(rename = "Monster_APShow_TwoTurn")]
    TwoTurn,
    #[serde(rename = "Monster_APShow_TwoTurn_NotCancel")]
    TwoTurnNotCancel,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum MonsterConfigCustomValueKey {
    #[serde(rename = "Cocolia_ChangePhase_InsertController")]
    CocoliaChangePhaseInsertController,
    FlopSide,
    HardLevel,
    #[serde(rename = "Ice_Lance_ID")]
    IceLanceID,
    #[serde(rename = "Ice_Lance_ID_2")]
    IceLanceID2,
    #[serde(rename = "Ice_Lance_ID_3")]
    IceLanceID3,
    #[serde(rename = "Ice_Lance_ID_4")]
    IceLanceID4,
    #[serde(rename = "_IsWeeklyBoss")]
    IsWeeklyBoss,
    #[serde(rename = "Monster_AML_Elite01_00_AICounter_01")]
    MonsterAMLElite0100AICounter01,
    MonsterCount,
    #[serde(rename = "Monster_RO_015_SummonID")]
    MonsterRO015SummonID,
    #[serde(rename = "Monster_SummonID")]
    MonsterSummonID,
    #[serde(rename = "Monster_SummonID0")]
    MonsterSummonID0,
    #[serde(rename = "Monster_SummonID1")]
    MonsterSummonID1,
    #[serde(rename = "Monster_SummonID2")]
    MonsterSummonID2,
    #[serde(rename = "Monster_SummonID3")]
    MonsterSummonID3,
    #[serde(rename = "Monster_W1_CocoliaP2_00_SummonMonsterID01")]
    MonsterW1CocoliaP200SummonMonsterID01,
    #[serde(rename = "Monster_W1_CocoliaP2_00_SummonMonsterID02")]
    MonsterW1CocoliaP200SummonMonsterID02,
    #[serde(rename = "Monster_XP_Elite02_01_AIFlag")]
    MonsterXPElite0201AIFlag,
    #[serde(rename = "PartEntity1_MonsterID")]
    PartEntity1MonsterID,
    #[serde(rename = "PartEntity2_MonsterID")]
    PartEntity2MonsterID,
    #[serde(rename = "PartEntity3_MonsterID")]
    PartEntity3MonsterID,
    SummonEliteMonster,
    SummonerID,
    SummonID,
    SummonID0,
    SummonID01,
    SummonID02,
    SummonID03,
    SummonID1,
    #[serde(alias = "SummonID_2")]
    SummonID2,
    SummonID3,
    SummonID4,
    #[serde(rename = "TV_01_EliteChance")]
    TV01EliteChance,
    #[serde(rename = "TV_01_RandomPoolID")]
    TV01RandomPoolID,
    #[serde(rename = "W2_Abomi04_00_SummonID")]
    W2Abomi0400SummonID,
    #[serde(rename = "W2_Abomi04_00_SummonID2")]
    W2Abomi0400SummonID2,
    #[serde(rename = "W2_Mecha03_00_SummonID")]
    W2Mecha0300SummonID,
    #[serde(rename = "W2_Yanqing_00_Skill02_SummonID01")]
    W2Yanqing00Skill02SummonID01,
    #[serde(rename = "W2_Yanqing_00_Skill02_SummonID02")]
    W2Yanqing00Skill02SummonID02,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
struct CustomValue {
    #[serde(rename = "PFMLCKGCKOB")]
    pub key: MonsterConfigCustomValueKey,
    #[serde(rename = "NLABNDMDIKM")]
    pub value: i32,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct MonsterTemplateConfig {
    #[serde(rename = "MonsterTemplateID")]
    monster_template_id: u32,
    #[serde(rename = "TemplateGroupID")]
    pub(crate) template_group_id: Option<NonZero<u32>>,
    #[serde(rename = "AtlasSortID")]
    atlas_sort_id: Option<NonZero<u8>>,
    monster_name: Text,
    #[serde(rename = "MonsterCampID")]
    monster_camp_id: Option<NonZero<u8>>,
    monster_base_type: BaseType,
    rank: Rank,
    json_config: PathBuf,
    icon_path: PathBuf,
    round_icon_path: PathBuf,
    image_path: PathBuf,
    prefab_path: PathBuf,
    manikin_prefab_path: PathBuf,
    manikin_config_path: PathBuf,
    manikin_image_path: PathBuf,
    #[serde(rename = "NatureID")]
    nature_id: u8, // 目前只有 1
    attack_base: Value<u16>,
    defence_base: Value<u16>,
    #[serde(rename = "HPBase")]
    hp_base: Value<f32>,
    speed_base: Option<Value<NonZero<u16>>>,
    stance_base: Option<Value<NonZero<u16>>>,
    stance_type: Option<StanceType>,
    critical_damage_base: Value<f32>,
    status_resistance_base: Option<Value<f32>>,
    minimum_fatigue_ratio: Value<f32>,
    #[serde(rename = "AIPath")]
    ai_path: PathBuf,
    stance_count: Option<NonZero<u8>>,
    initial_delay_ratio: Option<Value<f32>>,
    #[serde(rename = "AISkillSequence")]
    ai_skill_sequence: Vec<AISkillSequence>,
    #[serde(rename = "NPCMonsterList")]
    npc_monster_list: Vec<u32>,
}

impl ID for MonsterTemplateConfig {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.monster_template_id
    }
}

impl<'a> PO<'a> for MonsterTemplateConfig {
    type VO = vo::monster::MonsterTemplateConfig<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        let camp = self
            .monster_camp_id
            .map(|id| id.get())
            .and_then(|id| game.monster_camp(id));
        Self::VO {
            game,
            id: self.monster_template_id,
            group_id: self
                .template_group_id
                .map(|id| id.get())
                .unwrap_or_default(),
            name: game.text(&self.monster_name),
            camp_name: camp.map(|camp| camp.name).unwrap_or_default(),
            rank: self.rank,
            attack_base: self.attack_base.value,
            defence_base: self.defence_base.value,
            hp_base: self.hp_base.value,
            speed_base: self.speed_base.map(|v| v.value.get()).unwrap_or_default(),
            stance_base: self.stance_base.map(|v| v.value.get()).unwrap_or_default(),
            critical_damage_base: self.critical_damage_base.value,
            status_resistance_base: self.status_resistance_base.unwrap_or_default().value,
            minimum_fatigue_ratio: self.minimum_fatigue_ratio.value,
            stance_count: self.stance_count.map(|v| v.get()).unwrap_or_default(),
            initial_delay_ratio: self.initial_delay_ratio.unwrap_or_default().value,
            npc_monster_list: self
                .npc_monster_list
                .iter()
                .filter_map(|&id| {
                    let npc = game.npc_monster_data(id);
                    if npc.is_none() {
                        log::warn!(
                            "monster_template_config {} npc_monster_list not found: {id}",
                            self.monster_template_id
                        );
                    }
                    npc
                })
                .collect(),
            stance_type: self.stance_type,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct MonsterConfig {
    #[serde(rename = "MonsterID")]
    monster_id: u32,
    #[serde(rename = "MonsterTemplateID")]
    monster_template_id: u32,
    monster_name: Text,
    monster_introduction: Text,
    monster_battle_introduction: Text,
    hard_level_group: u8, // 目前只有 1
    elite_group: u16,
    attack_modify_ratio: Value<f32>,
    defence_modify_ratio: Value<f32>,
    #[serde(rename = "HPModifyRatio")]
    hp_modify_ratio: Value<f32>,
    speed_modify_ratio: Value<u8>,  // 目前只有 1
    stance_modify_ratio: Value<u8>, // 目前只有 1
    speed_modify_value: Option<Value<i16>>,
    stance_modify_value: Option<Value<i16>>,
    skill_list: Vec<u32>,
    custom_values: Vec<CustomValue>,
    dynamic_values: [(); 0], // 目前只有空 []
    debuff_resist: Vec<DebuffResist>,
    custom_value_tags: Vec<CustomValueTag>,
    stance_weak_list: Vec<Element>,
    damage_type_resistance: Vec<DamageTypeResistance>,
    ability_name_list: Vec<AbilityName>,
    #[serde(rename = "OverrideAIPath")]
    override_ai_path: PathBuf,
    #[serde(rename = "OverrideAISkillSequence")]
    override_ai_skill_sequence: Vec<AISkillSequence>,
}

impl ID for MonsterConfig {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.monster_id
    }
}

impl<'a> PO<'a> for MonsterConfig {
    type VO = vo::monster::MonsterConfig<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        Self::VO {
            game,
            id: self.monster_id,
            template: game
                .monster_template_config(self.monster_template_id)
                .unwrap(),
            name: game.text(&self.monster_name),
            introduction: game.text(&self.monster_introduction),
            battle_introduction: game.text(&self.monster_battle_introduction),
            attack_modify_ratio: self.attack_modify_ratio.value,
            defence_modify_ratio: self.defence_modify_ratio.value,
            hp_modify_ratio: self.hp_modify_ratio.value,
            speed_modify_value: self.speed_modify_value.unwrap_or_default().value,
            stance_modify_value: self.stance_modify_value.unwrap_or_default().value,
            skill_list: self
                .skill_list
                .iter()
                .map(|&id| game.monster_skill_config(id))
                .map(Option::unwrap)
                .collect(),
            custom_values: self
                .custom_values
                .iter()
                .map(|o| (o.key, o.value))
                .collect(),
            debuff_resist: self
                .debuff_resist
                .iter()
                .map(|o| (o.key, o.value.value))
                .collect(),
            custom_value_tags: &self.custom_value_tags,
            stance_weak_list: &self.stance_weak_list,
            damage_type_resistance: self
                .damage_type_resistance
                .iter()
                .map(|o| (o.damage_type, o.value.value))
                .collect(),
            ability_name_list: &self.ability_name_list,
            override_ai_skill_sequence: self
                .override_ai_skill_sequence
                .iter()
                .map(|seq| game.monster_skill_config(seq.id))
                .map(Option::unwrap)
                .collect(),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct NPCMonsterData {
    #[serde(rename = "ID")]
    id: u32,
    #[serde(rename = "NPCName")]
    npc_name: Text,
    config_entity_path: PathBuf,
    #[serde(rename = "NPCIconPath")]
    npc_icon_path: PathBuf,
    #[serde(rename = "NPCTitle")]
    npc_title: Text,
    board_show_list: [u8; 1], // 目前只有 [2]
    json_path: PathBuf,
    #[serde(rename = "DefaultAIPath")]
    default_ai_path: PathBuf,
    character_type: CharacterType,
    sub_type: SubType,
    mini_map_icon_type: Option<NonZero<u8>>, // 目前只有 5
    rank: Rank,
    #[serde(default)]
    is_maze_link: bool,
    #[serde(rename = "PrototypeID")]
    prototype_id: u32,
    #[serde(rename = "MappingInfoID")]
    mapping_info_id: Option<NonZero<u32>>,
}

impl ID for NPCMonsterData {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.id
    }
}

impl<'a> PO<'a> for NPCMonsterData {
    type VO = vo::monster::NPCMonsterData<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.id,
            name: game.text(&self.npc_name),
            title: game.text(&self.npc_title),
            character_type: self.character_type,
            sub_type: self.sub_type,
            rank: self.rank,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct MonsterSkillConfig {
    #[serde(rename = "SkillID")]
    skill_id: u32,
    skill_name: Text,
    icon_path: PathBuf,
    skill_desc: Text,
    skill_type_desc: Text,
    skill_tag: Text,
    phase_list: Vec<u8>,
    #[serde(default)]
    is_threat: bool,
    #[serde(rename = "ExtraEffectIDList")]
    extra_effect_id_list: Vec<u32>,
    damage_type: Option<Element>,
    skill_trigger_key: SkillTriggerKey,
    #[serde(rename = "SPHitBase")]
    sp_hit_base: Option<Value<u16>>,
    #[serde(rename = "DelayRatio")]
    delay_ratio: Option<Value<f32>>,
    param_list: Vec<Value<f32>>,
    attack_type: AttackType,
    #[serde(rename = "AI_CD")]
    ai_cd: u8, // 只有 1
    #[serde(rename = "AI_ICD")]
    ai_icd: u8, // 只有 1
    modifier_list: Vec<SkillModifier>,
}

impl ID for MonsterSkillConfig {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.skill_id
    }
}
impl<'a> PO<'a> for MonsterSkillConfig {
    type VO = vo::monster::MonsterSkillConfig<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        let params = self
            .param_list
            .iter()
            .map(|v| crate::format::Formattable::from(&v.value))
            .collect::<Vec<_>>();
        Self::VO {
            id: self.skill_id,
            name: game.text(&self.skill_name),
            desc: crate::format::format(game.text(&self.skill_desc), &params),
            type_desc: game.text(&self.skill_type_desc),
            tag: game.text(&self.skill_tag),
            phase_list: &self.phase_list,
            is_threat: self.is_threat,
            extra_effect_list: self
                .extra_effect_id_list
                .iter()
                .filter_map(|&id| game.extra_effect_config(id))
                .collect(),
            damage_type: self.damage_type,
            skill_trigger_key: self.skill_trigger_key,
            sp_hit_base: self.sp_hit_base.unwrap_or_default().value,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct MonsterCamp {
    #[serde(rename = "ID")]
    id: u8,
    #[serde(rename = "SortID")]
    sort_id: u8,
    name: Text,
    icon_path: PathBuf,
    camp_type: CampType,
}

impl ID for MonsterCamp {
    type ID = u8;
    fn id(&self) -> Self::ID {
        self.id
    }
}

impl<'a> PO<'a> for MonsterCamp {
    type VO = vo::monster::MonsterCamp<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.id,
            sort_id: self.sort_id,
            name: game.text(&self.name),
            r#type: self.camp_type,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
struct AISkillSequence {
    #[serde(rename = "GKBBPHMLLNG")]
    id: u32,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
struct DebuffResist {
    key: DebuffResistKey,
    value: Value<f32>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
struct DamageTypeResistance {
    damage_type: Element,
    value: Value<f32>,
}
