pub mod guide;

use crate::po::{Element, Text, Value};
use crate::vo;
use crate::{GameData, ID, PO};
use std::num::NonZero;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum CampType {
    Monster,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct Camp {
    #[serde(rename = "ID")]
    id: u8,
    #[serde(rename = "SortID")]
    sort_id: u8,
    name: Text,
    icon_path: PathBuf,
    camp_type: Option<CampType>, // 1.5 及之后
}

impl ID for Camp {
    type ID = u8;
    fn id(&self) -> Self::ID {
        self.id
    }
}

impl<'a> PO<'a> for Camp {
    type VO = vo::monster::Camp<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.id,
            sort_id: self.sort_id,
            name: game.text(self.name),
            r#type: self.camp_type,
        }
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

impl crate::Wiki for DebuffResistKey {
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
struct DebuffResist {
    key: DebuffResistKey,
    value: Value<f32>,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
struct DamageTypeResistance {
    damage_type: Element,
    value: Value<f32>,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
struct AISkillSequence {
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
    id: u32,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct Config {
    #[serde(rename = "MonsterID")]
    monster_id: u32,
    #[serde(rename = "MonsterTemplateID")]
    monster_template_id: u32,
    monster_name: Text,
    monster_introduction: Text,
    monster_battle_introduction: Option<Text>, // 1.0 及之前
    hard_level_group: u8,                      // 目前只有 1
    elite_group: u16,
    attack_modify_ratio: Value<f32>,
    defence_modify_ratio: Value<f32>,
    #[serde(rename = "HPModifyRatio")]
    hp_modify_ratio: Value<f32>,
    speed_modify_ratio: Value<f32>,  // 目前只有 1
    stance_modify_ratio: Value<f32>, // 目前只有 1
    speed_modify_value: Option<Value<i16>>,
    stance_modify_value: Option<Value<i16>>,
    skill_list: Vec<u32>,
    custom_values: Vec<CustomValue>,
    dynamic_values: [(); 0], // 目前只有空 []
    debuff_resist: Vec<DebuffResist>,
    custom_value_tags: Vec<String>,
    stance_weak_list: Vec<Element>,
    damage_type_resistance: Vec<DamageTypeResistance>,
    ability_name_list: Vec<String>,
    #[serde(rename = "OverrideAIPath")]
    override_ai_path: PathBuf,
    #[serde(rename = "OverrideAISkillSequence")]
    override_ai_skill_sequence: Vec<AISkillSequence>,
}

impl ID for Config {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.monster_id
    }
}

impl<'a> PO<'a> for Config {
    type VO = vo::monster::Config<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        Self::VO {
            game,
            id: self.monster_id,
            // 1.0~1.3, 2.0 存在几个数据，会导致 panic
            template: None
                .or_else(|| game.monster_template_config(self.monster_template_id))
                .or_else(|| game.monster_template_unique_config(self.monster_template_id)),
            name: game.text(self.monster_name),
            introduction: game.text(self.monster_introduction),
            battle_introduction: self
                .monster_battle_introduction
                .map(|hash| game.text(hash))
                .unwrap_or_default(),
            attack_modify_ratio: self.attack_modify_ratio.value,
            defence_modify_ratio: self.defence_modify_ratio.value,
            hp_modify_ratio: self.hp_modify_ratio.value,
            speed_modify_ratio: self.speed_modify_ratio.value,
            stance_modify_ratio: self.stance_modify_ratio.value,
            speed_modify_value: self.speed_modify_value.unwrap_or_default().value,
            stance_modify_value: self.stance_modify_value.unwrap_or_default().value,
            skill_list: self
                .skill_list
                .iter()
                .map(|&id| {
                    None.or_else(|| game.monster_skill_config(id))
                        .or_else(|| game.monster_skill_unique_config(id))
                })
                .map(Option::unwrap)
                .collect(),
            custom_values: self
                .custom_values
                .iter()
                .map(|o| (o.key.as_str(), o.value))
                .collect(),
            debuff_resist: self
                .debuff_resist
                .iter()
                .map(|o| (o.key, o.value.value))
                .collect(),
            custom_value_tags: self.custom_value_tags.iter().map(String::as_str).collect(),
            stance_weak_list: &self.stance_weak_list,
            damage_type_resistance: self
                .damage_type_resistance
                .iter()
                .map(|o| (o.damage_type, o.value.value))
                .collect(),
            ability_name_list: self.ability_name_list.iter().map(String::as_str).collect(),
            override_ai_skill_sequence: self
                .override_ai_skill_sequence
                .iter()
                .map(|seq| game.monster_skill_config(seq.id))
                .map(Option::unwrap)
                .collect(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum CharacterType {
    NPCMonster,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum SubType {
    Monster,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct NPCMonsterData {
    #[serde(rename = "ID")]
    id: u32,
    #[serde(rename = "NPCName")]
    npc_name: Text,
    prefab_path: Option<String>, // 1.1 及之前
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
            name: game.text(self.npc_name),
            title: game.text(self.npc_title),
            character_type: self.character_type,
            sub_type: self.sub_type,
            rank: self.rank,
        }
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
pub(crate) struct SkillConfig {
    #[serde(rename = "SkillID")]
    skill_id: u32,
    skill_name: Text,
    icon_path: PathBuf,
    skill_desc: Text,
    skill_type_desc: Text,
    skill_tag: Option<Text>,
    phase_list: Vec<u8>,
    #[serde(default)]
    is_threat: bool,
    #[serde(rename = "ExtraEffectIDList")]
    extra_effect_id_list: Vec<u32>,
    damage_type: Option<Element>,
    skill_trigger_key: String,
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
    modifier_list: Option<Vec<SkillModifier>>, // 2.0 无该字段
}

impl ID for SkillConfig {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.skill_id
    }
}

impl<'a> PO<'a> for SkillConfig {
    type VO = vo::monster::SkillConfig<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        let params = self
            .param_list
            .iter()
            .map(|v| crate::format::Argument::from(&v.value))
            .collect::<Vec<_>>();
        Self::VO {
            id: self.skill_id,
            name: game.text(self.skill_name),
            desc: crate::format::format(game.text(self.skill_desc), &params),
            type_desc: game.text(self.skill_type_desc),
            tag: self
                .skill_tag
                .map(|hash| game.text(hash))
                .unwrap_or_default(),
            phase_list: &self.phase_list,
            is_threat: self.is_threat,
            extra_effect_list: self
                .extra_effect_id_list
                .iter()
                .map(|&id| game.extra_effect_config(id))
                .map(Option::unwrap)
                .collect(),
            damage_type: self.damage_type,
            skill_trigger_key: self.skill_trigger_key.as_str(),
            sp_hit_base: self.sp_hit_base.unwrap_or_default().value,
        }
    }
}

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

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
struct CustomValue {
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
pub(crate) struct TemplateConfig {
    #[serde(rename = "MonsterTemplateID")]
    monster_template_id: u32,
    #[serde(rename = "TemplateGroupID")]
    pub(crate) template_group_id: Option<NonZero<u32>>,
    #[serde(default)]
    release_in_atlas: bool, // 1.0 及之前
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
    manikin_image_path: Option<String>, // 1.2 及之后
    #[serde(rename = "NatureID")]
    nature_id: u8, // 目前只有 1
    attack_base: Value<u16>,
    defence_base: Option<Value<NonZero<u16>>>,
    #[serde(rename = "HPBase")]
    hp_base: Value<f32>,
    speed_base: Option<Value<NonZero<u16>>>,
    stance_base: Option<Value<NonZero<u16>>>,
    stance_type: Option<Element>,
    critical_damage_base: Option<Value<f32>>,
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

impl ID for TemplateConfig {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.monster_template_id
    }
}

impl<'a> PO<'a> for TemplateConfig {
    type VO = vo::monster::TemplateConfig<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        #[rustfmt::skip]
        const MISSING_NPC_MONSTER: &[u32] = &[
            1005010, 1012010, 3024012, 8022020,
            // 1.2 缺漏数据
            2013011,
        ];
        let camp = self
            .monster_camp_id
            .map(NonZero::get)
            .map(|id| game.monster_camp(id))
            .map(Option::unwrap);
        Self::VO {
            game,
            id: self.monster_template_id,
            group_id: self.template_group_id.map(NonZero::get).unwrap_or_default(),
            name: game.text(self.monster_name),
            camp_name: camp.map(|camp| camp.name).unwrap_or_default(),
            rank: self.rank,
            attack_base: self.attack_base.value,
            defence_base: self.defence_base.map(|v| v.value.get()).unwrap_or_default(),
            hp_base: self.hp_base.value,
            speed_base: self.speed_base.map(|v| v.value.get()).unwrap_or_default(),
            stance_base: self.stance_base.map(|v| v.value.get()).unwrap_or_default(),
            critical_damage_base: self
                .critical_damage_base
                .map(|v| v.value)
                .unwrap_or_default(),
            status_resistance_base: self.status_resistance_base.unwrap_or_default().value,
            minimum_fatigue_ratio: self.minimum_fatigue_ratio.value,
            stance_count: self.stance_count.map(NonZero::get).unwrap_or_default(),
            initial_delay_ratio: self.initial_delay_ratio.unwrap_or_default().value,
            npc_monster_list: self
                .npc_monster_list
                .iter()
                .filter(|&id| !MISSING_NPC_MONSTER.contains(id)) // TODO: 疑似缺数据
                .map(|&id| game.npc_monster_data(id))
                .map(Option::unwrap)
                .collect(),
            stance_type: self.stance_type,
        }
    }
}
