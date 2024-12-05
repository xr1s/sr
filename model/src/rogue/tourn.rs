use std::{borrow::Cow, collections::HashMap, num::NonZero, path::PathBuf};

use base::{Wiki, ID};

use super::Text;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct RogueBonus {
    #[serde(rename = "BonusID")]
    pub bonus_id: u16,
    pub bonus_event: u32,
    pub bonus_title: Text,
    pub bonus_desc: Text,
    pub bonus_tag: Text,
    pub bonus_icon: String,
}

impl ID for RogueBonus {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.bonus_id
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum DescParamType {
    Formula,
    Miracle,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum TournMode {
    Tourn1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum MiracleCategory {
    /// 一星奇物
    Common,
    /// 加权奇物
    Hex,
    /// 三星奇物
    Legendary,
    /// 负面奇物
    Negative,
    /// 二星奇物
    Rare,
}

impl Wiki for MiracleCategory {
    fn wiki(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            MiracleCategory::Common => "一星",
            MiracleCategory::Hex => "加权",
            MiracleCategory::Legendary => "三星",
            MiracleCategory::Negative => "负面",
            MiracleCategory::Rare => "二星",
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum FormulaCategory {
    /// 二星方程
    Epic,
    /// 三星方程
    Legendary,
    /// 临界方程
    PathEcho,
    /// 一星方程
    Rare,
}

#[serde_with::serde_as]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct DescParam {
    #[serde(alias = "IAGLGKPDLOE")] // 2.3
    #[serde(alias = "EEOLCCFMJFF")] // 2.4
    #[serde(alias = "FGMDOEKGPEE")] // 2.5
    #[serde(alias = "EOMLKKGEAEF")] // 2.6
    #[serde(alias = "MPNJPFDCBDG")] // 2.7
    pub r#type: DescParamType,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[serde(alias = "EPBOOFFCKPJ")] // 2.3
    #[serde(alias = "DIBKEHHCPAP")] // 2.4
    #[serde(alias = "NLABNDMDIKM")] // 2.5
    #[serde(alias = "HPPEILAONGE")] // 2.6
    #[serde(alias = "ODPKJEJKOIH")] // 2.7
    pub value: u32,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct RogueTournContentDisplay {
    #[serde(rename = "DisplayID")]
    pub display_id: u16,
    pub display_content: Text,
}

impl ID for RogueTournContentDisplay {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.display_id
    }
}

#[serde_with::serde_as]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct RogueTournWeeklyChallenge {
    #[serde(rename = "ChallengeID")]
    pub challenge_id: u8,
    pub weekly_name: Text,
    pub weekly_content_list: Vec<u16>,
    pub weekly_content_detail_list: Vec<u16>,
    #[serde(rename = "RewardID")]
    pub reward_id: u32,
    #[serde_as(as = "HashMap<_, _>")]
    pub display_final_monster_groups: Vec<(u8, u32)>, // 理论是 map，目前 key 只有 0
    #[serde_as(as = "HashMap<_, _>")]
    // 理论是 map，目前 key 只有 0, 3，分别是难度 V3 之前和之后的敌人列表
    pub display_monster_groups_1: Vec<(u8, u32)>,
    #[serde_as(as = "HashMap<_, _>")]
    // 理论是 map，目前 key 只有 0, 3，分别是难度 V3 之前和之后的敌人列表
    pub display_monster_groups_2: Vec<(u8, u32)>,
    #[serde_as(as = "HashMap<_, _>")]
    pub display_monster_groups_3: Vec<(u8, u32)>, // 理论是 map，目前 key 只有 0
}

impl ID for RogueTournWeeklyChallenge {
    type ID = u8;
    fn id(&self) -> Self::ID {
        self.challenge_id
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct RogueTournWeeklyDisplay {
    #[serde(rename = "WeeklyDisplayID")]
    pub weekly_display_id: u16,
    pub weekly_display_content: Text,
    pub desc_params: Vec<DescParam>,
}

impl ID for RogueTournWeeklyDisplay {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.weekly_display_id
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct RogueTournMiracle {
    #[serde(rename = "MiracleID")]
    pub miracle_id: u16,
    pub tourn_mode: TournMode,
    pub miracle_category: MiracleCategory,
    #[serde(rename = "MiracleDisplayID")]
    pub miracle_display_id: u16,
    #[serde(rename = "HandbookMiracleID")]
    pub handbook_miracle_id: Option<NonZero<u16>>,
}

impl ID for RogueTournMiracle {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.miracle_id
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct RogueTournHandbookMiracle {
    #[serde(rename = "HandbookMiracleID")]
    pub handbook_miracle_id: u16,
    #[serde(rename = "MiracleDisplayID")]
    pub miracle_display_id: u16,
    pub miracle_category: MiracleCategory,
    pub unlock_desc: u16,
}

impl ID for RogueTournHandbookMiracle {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.handbook_miracle_id
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
/// 差分宇宙方程
pub struct RogueTournFormula {
    #[serde(rename = "FormulaID")]
    pub formula_id: u32,
    pub tourn_mode: Option<TournMode>, // 不明白 undefined 和 "Tourn1" 有什么区别
    #[serde(rename = "MainBuffTypeID")]
    pub main_buff_type_id: u8,
    pub main_buff_num: u8,
    #[serde(rename = "SubBuffTypeID")]
    pub sub_buff_type_id: Option<NonZero<u8>>,
    pub sub_buff_num: Option<NonZero<u8>>,
    pub formula_category: FormulaCategory,
    #[serde(rename = "MazeBuffID")]
    pub maze_buff_id: u32,
    #[serde(rename = "FormulaDisplayID")]
    pub formula_display_id: u32,
    pub formula_icon: PathBuf,
    pub formula_sub_icon: PathBuf,
    #[serde(default)]
    pub is_in_handbook: bool,
    pub ultra_formula_icon: PathBuf,
    pub formula_story_json: PathBuf,
    #[serde(rename = "UnlockDisplayID")]
    pub unlock_display_id: Option<NonZero<u16>>,
}

impl ID for RogueTournFormula {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.formula_id
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
/// 差分宇宙方程
pub struct RogueTournFormulaDisplay {
    #[serde(rename = "FormulaDisplayID")]
    pub formula_display_id: u32,
    pub formula_type_display: Option<NonZero<u16>>, // 不知道是什么
    pub formula_story: Text,
    pub extra_effect: Vec<u32>,
}

impl ID for RogueTournFormulaDisplay {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.formula_display_id
    }
}
