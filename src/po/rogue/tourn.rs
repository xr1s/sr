use std::{borrow::Cow, collections::HashMap, num::NonZero, path::PathBuf};

use super::Text;
use crate::{po::Path, vo, GameData, Name, Wiki, ID, PO};

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
pub(crate) struct DescParam {
    #[serde(alias = "IAGLGKPDLOE")] // 2.3
    #[serde(alias = "EEOLCCFMJFF")] // 2.4
    #[serde(alias = "FGMDOEKGPEE")] // 2.5
    #[serde(alias = "EOMLKKGEAEF")] // 2.6
    #[serde(alias = "MPNJPFDCBDG")] // 2.7
    r#type: DescParamType,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    #[serde(alias = "EPBOOFFCKPJ")] // 2.3
    #[serde(alias = "DIBKEHHCPAP")] // 2.4
    #[serde(alias = "NLABNDMDIKM")] // 2.5
    #[serde(alias = "HPPEILAONGE")] // 2.6
    #[serde(alias = "ODPKJEJKOIH")] // 2.7
    value: u32,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct RogueTournContentDisplay {
    #[serde(rename = "DisplayID")]
    display_id: u16,
    display_content: Text,
}

impl ID for RogueTournContentDisplay {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.display_id
    }
}

impl<'a> PO<'a> for RogueTournContentDisplay {
    type VO = vo::rogue::tourn::RogueTournContentDisplay<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.display_id,
            content: game.text(self.display_content),
        }
    }
}

#[serde_with::serde_as]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct RogueTournWeeklyChallenge {
    #[serde(rename = "ChallengeID")]
    challenge_id: u8,
    weekly_name: Text,
    weekly_content_list: Vec<u16>,
    weekly_content_detail_list: Vec<u16>,
    #[serde(rename = "RewardID")]
    reward_id: u32,
    #[serde_as(as = "HashMap<_, _>")]
    display_final_monster_groups: Vec<(u8, u32)>, // 理论是 map，目前 key 只有 0
    #[serde_as(as = "HashMap<_, _>")]
    // 理论是 map，目前 key 只有 0, 3，分别是难度 V3 之前和之后的敌人列表
    display_monster_groups_1: Vec<(u8, u32)>,
    #[serde_as(as = "HashMap<_, _>")]
    // 理论是 map，目前 key 只有 0, 3，分别是难度 V3 之前和之后的敌人列表
    display_monster_groups_2: Vec<(u8, u32)>,
    #[serde_as(as = "HashMap<_, _>")]
    display_monster_groups_3: Vec<(u8, u32)>, // 理论是 map，目前 key 只有 0
}

impl ID for RogueTournWeeklyChallenge {
    type ID = u8;
    fn id(&self) -> Self::ID {
        self.challenge_id
    }
}

impl<'a> PO<'a> for RogueTournWeeklyChallenge {
    type VO = vo::rogue::tourn::RogueTournWeeklyChallenge<'a>;
    fn vo(&self, game: &'a GameData) -> Self::VO {
        let mut content_list = self
            .weekly_content_list
            .iter()
            .map(|&id| game.rogue_tourn_weekly_display(id))
            .map(Option::unwrap)
            .collect::<Vec<_>>();
        Self::VO {
            id: self.challenge_id,
            name: game.text(self.weekly_name),
            content: content_list
                .iter_mut()
                .map(|content| std::mem::take(&mut content.content))
                .collect(),
            content_detail: self
                .weekly_content_detail_list
                .iter()
                // 2.7 版本倒数第二周（2.8 前瞻所在周）的 DisplayID 是 1302 和 1303，缺数据，注意一下
                .filter(|&&id| id != 1302 && id != 1303)
                .map(|&id| game.rogue_tourn_weekly_display(id))
                .map(Option::unwrap)
                .map(|display| display.content)
                .collect(),
            reward: game.reward_data(self.reward_id).unwrap(),
            formula: content_list
                .iter_mut()
                .flat_map(|content| std::mem::take(&mut content.formula))
                .collect(),
            miracle: content_list
                .iter_mut()
                .flat_map(|content| std::mem::take(&mut content.miracle))
                .collect(),
            monster_group_1: self
                .display_monster_groups_1
                .iter()
                .map(|&(lv, id)| (lv, game.rogue_monster_group(id).unwrap()))
                .collect(),
            monster_group_2: self
                .display_monster_groups_2
                .iter()
                .map(|&(lv, id)| (lv, game.rogue_monster_group(id).unwrap()))
                .collect(),
            monster_group_3: self
                .display_monster_groups_3
                .iter()
                .map(|&(lv, id)| (lv, game.rogue_monster_group(id).unwrap()))
                .collect(),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct RogueTournWeeklyDisplay {
    #[serde(rename = "WeeklyDisplayID")]
    weekly_display_id: u16,
    weekly_display_content: Text,
    desc_params: Vec<DescParam>,
}

impl ID for RogueTournWeeklyDisplay {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.weekly_display_id
    }
}

impl<'a> PO<'a> for RogueTournWeeklyDisplay {
    type VO = vo::rogue::tourn::RogueTournWeeklyDisplay<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        use DescParamType::{Formula, Miracle};
        let formula = self
            .desc_params
            .iter()
            .filter(|param| param.r#type == Formula)
            .map(|param| game.rogue_tourn_formula(param.value))
            .map(Option::unwrap)
            .collect();
        let miracle = self
            .desc_params
            .iter()
            .filter(|param| param.r#type == Miracle)
            .map(|param| game.rogue_tourn_miracle(param.value as _))
            .map(Option::unwrap)
            .collect();
        use either::Either;
        let params = self
            .desc_params
            .iter()
            .map(|param| match param.r#type {
                Formula => Either::Left(game.rogue_tourn_formula(param.value).unwrap()),
                Miracle => Either::Right(game.rogue_tourn_miracle(param.value as _).unwrap()),
            })
            .collect::<Vec<_>>();
        let names = params.iter().map(Name::name).collect::<Vec<_>>();
        let names = names
            .iter()
            .map(crate::format::Argument::from)
            .collect::<Vec<_>>();
        let content = crate::format::format(game.text(self.weekly_display_content), &names);

        Self::VO {
            id: self.weekly_display_id,
            content,
            formula,
            miracle,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct RogueTournMiracle {
    #[serde(rename = "MiracleID")]
    miracle_id: u16,
    tourn_mode: TournMode,
    miracle_category: MiracleCategory,
    #[serde(rename = "MiracleDisplayID")]
    miracle_display_id: u16,
    #[serde(rename = "HandbookMiracleID")]
    handbook_miracle_id: Option<NonZero<u16>>,
}

impl ID for RogueTournMiracle {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.miracle_id
    }
}

impl<'a> PO<'a> for RogueTournMiracle {
    type VO = vo::rogue::tourn::RogueTournMiracle<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.miracle_id,
            category: self.miracle_category,
            display: game
                .rogue_tourn_miracle_display(self.miracle_display_id)
                .unwrap(),
            handbook: self
                .handbook_miracle_id
                .map(NonZero::get)
                .map(|id| game.rogue_tourn_handbook_miracle(id))
                .map(Option::unwrap),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct RogueTournHandbookMiracle {
    #[serde(rename = "HandbookMiracleID")]
    handbook_miracle_id: u16,
    #[serde(rename = "MiracleDisplayID")]
    miracle_display_id: u16,
    miracle_category: MiracleCategory,
    unlock_desc: u16,
}

impl ID for RogueTournHandbookMiracle {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.handbook_miracle_id
    }
}

impl<'a> PO<'a> for RogueTournHandbookMiracle {
    type VO = vo::rogue::tourn::RogueTournHandbookMiracle<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.handbook_miracle_id,
            display: game
                .rogue_tourn_miracle_display(self.miracle_display_id)
                .unwrap(),
            category: self.miracle_category,
            unlock_desc: game.rogue_tourn_content_display(self.unlock_desc).unwrap(),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
/// 差分宇宙方程
pub(crate) struct RogueTournFormula {
    #[serde(rename = "FormulaID")]
    formula_id: u32,
    tourn_mode: Option<TournMode>, // 不明白 undefined 和 "Tourn1" 有什么区别
    #[serde(rename = "MainBuffTypeID")]
    main_buff_type_id: u8,
    main_buff_num: u8,
    #[serde(rename = "SubBuffTypeID")]
    sub_buff_type_id: Option<NonZero<u8>>,
    sub_buff_num: Option<NonZero<u8>>,
    formula_category: FormulaCategory,
    #[serde(rename = "MazeBuffID")]
    maze_buff_id: u32,
    #[serde(rename = "FormulaDisplayID")]
    formula_display_id: u32,
    formula_icon: PathBuf,
    formula_sub_icon: PathBuf,
    #[serde(default)]
    is_in_handbook: bool,
    ultra_formula_icon: PathBuf,
    formula_story_json: PathBuf,
    #[serde(rename = "UnlockDisplayID")]
    unlock_display_id: Option<NonZero<u16>>,
}

impl ID for RogueTournFormula {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.formula_id
    }
}

impl<'a> PO<'a> for RogueTournFormula {
    type VO = vo::rogue::tourn::RogueTournFormula<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        fn buff_type_id_to_path(buff_type_id: impl Into<u8>) -> Path {
            match buff_type_id.into() {
                120 => Path::Preservation,
                121 => Path::Remembrance,
                122 => Path::Nihility,
                123 => Path::Abundance,
                124 => Path::Hunt,
                125 => Path::Destruction,
                126 => Path::Elation,
                127 => Path::Propagation,
                128 => Path::Erudition,
                _ => unreachable!(),
            }
        }
        Self::VO {
            id: self.formula_id,
            main_buff_type: buff_type_id_to_path(self.main_buff_type_id),
            main_buff_num: self.main_buff_num,
            sub_buff_type: self.sub_buff_type_id.map(buff_type_id_to_path),
            sub_buff_num: self.sub_buff_num.map(NonZero::get).unwrap_or_default(),
            category: self.formula_category,
            maze_buff: game
                .rogue_maze_buff(self.maze_buff_id)
                .into_iter()
                .next()
                .unwrap(),
            display: game
                .rogue_tourn_formula_display(self.formula_display_id)
                .unwrap(),
            is_in_handbook: self.is_in_handbook,
            story: &self.formula_story_json,
            unlock_display: self
                .unlock_display_id
                .map(NonZero::get)
                .map(|id| game.rogue_tourn_content_display(id))
                .map(Option::unwrap),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
/// 差分宇宙方程
pub(crate) struct RogueTournFormulaDisplay {
    #[serde(rename = "FormulaDisplayID")]
    formula_display_id: u32,
    formula_type_display: Option<NonZero<u16>>, // 不知道是什么
    formula_story: Text,
    extra_effect: Vec<u32>,
}

impl ID for RogueTournFormulaDisplay {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.formula_display_id
    }
}

impl<'a> PO<'a> for RogueTournFormulaDisplay {
    type VO = vo::rogue::tourn::RogueTournFormulaDisplay<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.formula_display_id,
            story: game.text(self.formula_story),
            extra_effect: self
                .extra_effect
                .iter()
                .map(|&id| game.extra_effect_config(id))
                .map(Option::unwrap)
                .collect(),
        }
    }
}
