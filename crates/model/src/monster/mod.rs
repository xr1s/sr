pub mod guide;

use std::num::NonZero;
use std::path::PathBuf;

use base::{MainSubID, ID};

use crate::{Element, Text, Value};

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct EliteGroup {
    pub elite_group: u16,
    pub attack_ratio: Value<f32>,
    pub defence_ratio: Value<f32>,
    #[serde(rename = "HPRatio")]
    pub hp_ratio: Value<f32>,
    pub speed_ratio: Value<f32>,
    pub stance_ratio: Value<f32>,
}

impl ID for EliteGroup {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.elite_group
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct HardLevelGroup {
    pub hard_level_group: u16,
    pub level: u8,
    pub attack_ratio: Value<f32>,
    pub defence_ratio: Option<Value<f32>>,
    #[serde(rename = "HPRatio")]
    pub hp_ratio: Value<f32>,
    pub speed_ratio: Value<f32>,
    pub stance_ratio: Value<f32>,
    pub combat_power_list: Vec<Value<u16>>,
    pub status_probability: Option<Value<f32>>,
    pub status_resistance: Option<Value<f32>>,
}

impl MainSubID for HardLevelGroup {
    type ID = u16;
    type SubID = u8;
    fn id(&self) -> Self::ID {
        self.hard_level_group
    }
    fn sub_id(&self) -> Self::SubID {
        self.level
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum MonsterCampType {
    Monster,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct MonsterCamp {
    #[serde(rename = "ID")]
    pub id: u8,
    #[serde(rename = "SortID")]
    pub sort_id: u8,
    pub name: Text,
    pub icon_path: PathBuf,
    pub camp_type: Option<MonsterCampType>, // 1.5 及之后
}

impl base::ID for MonsterCamp {
    type ID = u8;
    fn id(&self) -> Self::ID {
        self.id
    }
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

impl base::Wiki for DebuffResistKey {
    fn wiki(&self) -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(match self {
            Self::Confine => "禁锢抵抗",
            Self::Ctrl => "控制抵抗",
            Self::Frozen => "冻结抵抗",
            Self::Burn => "灼烧抵抗",
            Self::Electric => "触电抵抗",
            Self::Poison => "风化抵抗",
            Self::Entangle => "纠缠抵抗",
        })
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct DebuffResist {
    pub key: DebuffResistKey,
    pub value: Value<f32>,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct DamageTypeResistance {
    pub damage_type: Element,
    pub value: Value<f32>,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct AISkillSequence {
    #[serde(alias = "KIFGIAMDGPI")] // 1.0
    #[serde(alias = "JEKCKJBHBKN")] // 1.1
    #[serde(alias = "MNMACAIHJCE")] // 1.2
    #[serde(alias = "CKFOCMJDLGG")] // 1.3
    #[serde(alias = "HMBDFGFHFAI")] // 1.4
    #[serde(alias = "OBBNCDOAKEF")] // 1.5
    #[serde(alias = "CMFIFDAHNOG")] // 1.6
    #[serde(alias = "DOHIPPHAGLG")] // 2.0
    #[serde(alias = "DGBJNJFOGHN")] // 2.1
    #[serde(alias = "ILOCKGFGCIF")] // 2.2
    #[serde(alias = "IPIGPCKIEMA")] // 2.3
    #[serde(alias = "IDNGFMLCGHB")] // 2.4
    #[serde(alias = "GKBBPHMLLNG")] // 2.5
    #[serde(alias = "PGKKLADJKGK")] // 2.6
    #[serde(alias = "CAMGCAFNKPK")] // 2.7
    pub id: u32,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct MonsterConfig {
    #[serde(rename = "MonsterID")]
    pub monster_id: u32,
    #[serde(rename = "MonsterTemplateID")]
    pub monster_template_id: u32,
    pub monster_name: Text,
    pub monster_introduction: Text,
    pub monster_battle_introduction: Option<Text>, // 1.0 及之前
    pub hard_level_group: u16,                     // 目前只有 1
    pub elite_group: u16,
    pub attack_modify_ratio: Value<f32>,
    pub defence_modify_ratio: Value<f32>,
    #[serde(rename = "HPModifyRatio")]
    pub hp_modify_ratio: Value<f32>,
    pub speed_modify_ratio: Value<f32>,  // 目前只有 1
    pub stance_modify_ratio: Value<f32>, // 目前只有 1
    pub speed_modify_value: Option<Value<i16>>,
    pub stance_modify_value: Option<Value<i16>>,
    pub skill_list: Vec<u32>,
    pub custom_values: Vec<CustomValue>,
    pub dynamic_values: [(); 0], // 目前只有空 []
    pub debuff_resist: Vec<DebuffResist>,
    pub custom_value_tags: Vec<String>,
    pub stance_weak_list: Vec<Element>,
    pub damage_type_resistance: Vec<DamageTypeResistance>,
    pub ability_name_list: Vec<String>,
    #[serde(rename = "OverrideAIPath")]
    pub override_ai_path: PathBuf,
    #[serde(rename = "OverrideAISkillSequence")]
    pub override_ai_skill_sequence: Vec<AISkillSequence>,
}

impl ID for MonsterConfig {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.monster_id
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum MonsterCharacterType {
    NPCMonster,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum MonsterSubType {
    Monster,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct NPCMonsterData {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "NPCName")]
    pub npc_name: Text,
    pub prefab_path: Option<String>, // 1.1 及之前
    pub config_entity_path: PathBuf,
    #[serde(rename = "NPCIconPath")]
    pub npc_icon_path: PathBuf,
    #[serde(rename = "NPCTitle")]
    pub npc_title: Text,
    pub board_show_list: [u8; 1], // 目前只有 [2]
    pub json_path: PathBuf,
    #[serde(rename = "DefaultAIPath")]
    pub default_ai_path: PathBuf,
    pub character_type: MonsterCharacterType,
    pub sub_type: MonsterSubType,
    pub mini_map_icon_type: Option<NonZero<u8>>, // 目前只有 5
    pub rank: MonsterRank,
    #[serde(default)]
    pub is_maze_link: bool,
    #[serde(rename = "PrototypeID")]
    pub prototype_id: u32,
    #[serde(rename = "MappingInfoID")]
    pub mapping_info_id: Option<NonZero<u32>>,
}

impl ID for NPCMonsterData {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.id
    }
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
pub enum AttackType {
    Normal,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct SkillConfig {
    #[serde(rename = "SkillID")]
    pub skill_id: u32,
    pub skill_name: Text,
    pub icon_path: PathBuf,
    pub skill_desc: Text,
    pub skill_type_desc: Text,
    pub skill_tag: Option<Text>,
    pub phase_list: Vec<u8>,
    #[serde(default)]
    pub is_threat: bool,
    #[serde(rename = "ExtraEffectIDList")]
    pub extra_effect_id_list: Vec<u32>,
    pub damage_type: Option<Element>,
    pub skill_trigger_key: String,
    #[serde(rename = "SPHitBase")]
    pub sp_hit_base: Option<Value<u16>>,
    #[serde(rename = "DelayRatio")]
    pub delay_ratio: Option<Value<f32>>,
    pub param_list: Vec<Value<f32>>,
    pub attack_type: AttackType,
    #[serde(rename = "AI_CD")]
    pub ai_cd: u8, // 只有 1
    #[serde(rename = "AI_ICD")]
    pub ai_icd: u8, // 只有 1
    pub modifier_list: Option<Vec<SkillModifier>>, // 2.0 无该字段
}

impl ID for SkillConfig {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.skill_id
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
/// 怪物稀有度
/// - BigBoss 周本首领以及逐光捡金变种
/// - LittleBoss 剧情敌人首领，如杰帕德、银枝等
/// - Elite 精英怪，凝滞虚影（角色突破材料）
/// - Minion 小怪
/// - MinionV2 小怪
pub enum MonsterRank {
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

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct CustomValue {
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
    #[serde(alias = "MFKLINKCPPA")] // 2.6
    #[serde(alias = "BNCHHJCHKON")] // 2.7
    pub key: String,
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
    pub value: i32,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct MonsterTemplateConfig {
    #[serde(rename = "MonsterTemplateID")]
    pub monster_template_id: u32,
    #[serde(rename = "TemplateGroupID")]
    pub template_group_id: Option<NonZero<u32>>,
    #[serde(default)]
    pub release_in_atlas: bool, // 1.0 及之前
    #[serde(rename = "AtlasSortID")]
    pub atlas_sort_id: Option<NonZero<u8>>,
    pub monster_name: Text,
    #[serde(rename = "MonsterCampID")]
    pub monster_camp_id: Option<NonZero<u8>>,
    pub monster_base_type: BaseType,
    pub rank: MonsterRank,
    pub json_config: PathBuf,
    pub icon_path: String,
    pub round_icon_path: String,
    pub image_path: String,
    pub prefab_path: PathBuf,
    pub manikin_prefab_path: PathBuf,
    pub manikin_config_path: PathBuf,
    pub manikin_image_path: Option<String>, // 1.2 及之后
    #[serde(rename = "NatureID")]
    pub nature_id: u8, // 目前只有 1
    pub attack_base: Value<u16>,
    pub defence_base: Option<Value<NonZero<u16>>>,
    #[serde(rename = "HPBase")]
    pub hp_base: Value<f32>,
    pub speed_base: Option<Value<NonZero<u16>>>,
    pub stance_base: Option<Value<NonZero<u16>>>,
    pub stance_type: Option<Element>,
    pub critical_damage_base: Option<Value<f32>>,
    pub status_resistance_base: Option<Value<f32>>,
    pub minimum_fatigue_ratio: Value<f32>,
    #[serde(rename = "AIPath")]
    pub ai_path: PathBuf,
    pub stance_count: Option<NonZero<u8>>,
    pub initial_delay_ratio: Option<Value<f32>>,
    #[serde(rename = "AISkillSequence")]
    pub ai_skill_sequence: Vec<AISkillSequence>,
    #[serde(rename = "NPCMonsterList")]
    pub npc_monster_list: Vec<u32>,
}

impl ID for MonsterTemplateConfig {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.monster_template_id
    }
}
