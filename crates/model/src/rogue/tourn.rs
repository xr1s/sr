use std::{borrow::Cow, collections::HashMap, num::NonZero, path::PathBuf};

use base::{MainSubID, Wiki, ID};

use super::Text;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
/// 进入宇宙的时候获取祝福的名称
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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct RogueTournBuff {
    #[serde(rename = "MazeBuffID")]
    pub maze_buff_id: u32,
    pub maze_buff_level: u8,
    pub rogue_buff_type: u8,
    pub rogue_buff_category: Option<crate::rogue::RogueBuffCategory>,
    pub rogue_buff_tag: u32,
    #[serde(rename = "ExtraEffectIDList")]
    pub extra_effect_id_list: Vec<u32>,
    pub unlock_display: u16, // 只有 835 一个值
    #[serde(default)]
    pub is_in_handbook: bool,
}

impl MainSubID for RogueTournBuff {
    type ID = u32;
    type SubID = u8;
    fn id(&self) -> Self::ID {
        self.maze_buff_id
    }
    fn sub_id(&self) -> Self::SubID {
        self.maze_buff_level
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum BuffDecoName {
    Path(crate::Path),
    Test(BuffDecoNameTest),
}

impl From<BuffDecoName> for crate::Path {
    fn from(value: BuffDecoName) -> Self {
        match value {
            BuffDecoName::Path(path) => path,
            BuffDecoName::Test(BuffDecoNameTest::Preservation) => crate::Path::Preservation,
            BuffDecoName::Test(BuffDecoNameTest::Remembrance) => crate::Path::Remembrance,
            BuffDecoName::Test(BuffDecoNameTest::Elation) => crate::Path::Elation,
            BuffDecoName::Test(BuffDecoNameTest::TheHunt) => crate::Path::TheHunt,
            BuffDecoName::Test(BuffDecoNameTest::Destruction) => crate::Path::Destruction,
            BuffDecoName::Test(BuffDecoNameTest::Nihility) => crate::Path::Nihility,
            BuffDecoName::Test(BuffDecoNameTest::Abundance) => crate::Path::Abundance,
            BuffDecoName::Test(BuffDecoNameTest::Propagation) => crate::Path::Propagation,
            BuffDecoName::Test(BuffDecoNameTest::Erudition) => crate::Path::Erudition,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum BuffDecoNameTest {
    #[serde(rename = "test_Preservation")]
    Preservation,
    #[serde(rename = "test_Remembrance")]
    Remembrance,
    #[serde(rename = "test_Elation")]
    Elation,
    #[serde(rename = "test_TheHunt")]
    TheHunt,
    #[serde(rename = "test_Destruction")]
    Destruction,
    #[serde(rename = "test_Nihility")]
    Nihility,
    #[serde(rename = "test_Abundance")]
    Abundance,
    #[serde(rename = "test_Propagation")]
    Propagation,
    #[serde(rename = "test_Erudition")]
    Erudition,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct RogueTournBuffType {
    pub rogue_buff_type: u8,
    pub rogue_buff_type_name: Text,
    pub rogue_buff_type_title: Option<Text>,
    pub rogue_buff_type_sub_title: Option<Text>,
    pub rogue_buff_type_deco_name: BuffDecoName,
    pub rogue_buff_type_icon: String,
    pub rogue_buff_type_small_icon: String,
    pub rogue_buff_type_large_icon: String,
}

impl ID for RogueTournBuffType {
    type ID = u8;
    fn id(&self) -> Self::ID {
        self.rogue_buff_type
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum DescParamType {
    Formula,
    Miracle,
    TitanBless,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum TournMode {
    Tourn1,
    Tourn2,
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
    #[serde(alias = "PGCFPBGPDGG")] // 3.0
    #[serde(alias = "PICHIHHCOCB")] // 3.1
    pub r#type: DescParamType,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[serde(alias = "EPBOOFFCKPJ")] // 2.3
    #[serde(alias = "DIBKEHHCPAP")] // 2.4
    #[serde(alias = "NLABNDMDIKM")] // 2.5
    #[serde(alias = "HPPEILAONGE")] // 2.6
    #[serde(alias = "ODPKJEJKOIH")] // 2.7
    #[serde(alias = "CPPHDJHHGGN")] // 3.0
    #[serde(alias = "HMCDHMFHABF")] // 3.1
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
    #[serde(rename = "MiracleEffectDisplayID")]
    pub miracle_effect_display_id: Option<NonZero<u16>>,
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
    #[serde(rename = "MiracleEffectDisplayID")]
    pub miracle_effect_display_id: Option<NonZero<u16>>,
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
    pub formula_icon: Option<PathBuf>,
    pub formula_sub_icon: Option<PathBuf>,
    #[serde(default)]
    pub is_in_handbook: bool,
    pub ultra_formula_icon: Option<PathBuf>,
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
    #[serde(rename = "HandbookUnlockDisplayID")]
    pub handbook_unlock_display_id: Option<NonZero<u16>>,
}

impl ID for RogueTournFormulaDisplay {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.formula_display_id
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum TitanType {
    Ianos,
    Moneta,
    Nikadory,
    Phageina,
    Xenatos,
    Zagreus,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum BlessBattleDisplayCategory {
    Day,
    Night,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct RogueTournTitanBless {
    #[serde(rename = "TitanBlessID")]
    pub titan_bless_id: u16,
    pub titan_type: TitanType,
    pub titan_bless_level: u8,
    #[serde(rename = "MazeBuffID")]
    pub maze_buff_id: u32,
    #[serde(rename = "ExtraEffectIDList")]
    pub extra_effect_id_list: Vec<u32>,
    pub bless_ratio: Option<NonZero<i8>>,
    pub bless_battle_display_category_list: Vec<BlessBattleDisplayCategory>,
}

impl ID for RogueTournTitanBless {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.titan_bless_id
    }
}
