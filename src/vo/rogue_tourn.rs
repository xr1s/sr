use crate::po::rogue_tourn::{FormulaCategory, MiracleCategory};
use crate::po::Path;
use crate::{vo, Name};

#[derive(Clone, Debug)]
/// 周期演算
pub struct RogueTournWeeklyChallenge<'a> {
    pub id: u16,
    pub name: &'a str,
    pub content: Vec<String>,
    pub content_detail: Vec<String>,
    pub reward: (), // TODO,
    pub formula: Vec<RogueTournFormula<'a>>,
    pub miracle: Vec<RogueTournMiracle<'a>>,
}

#[derive(Clone, Debug)]
pub struct RogueTournWeeklyDisplay<'a> {
    pub id: u16,
    pub content: String,
    pub formula: Vec<RogueTournFormula<'a>>,
    pub miracle: Vec<RogueTournMiracle<'a>>,
}

#[derive(Clone, Debug)]
/// 差分宇宙奇物
pub struct RogueTournMiracle<'a> {
    pub id: u16,
    pub category: MiracleCategory,
    pub display: RogueTournMiracleDisplay<'a>,
    pub handbook: Option<RogueTournHandbookMiracle<'a>>,
}

impl<'a> Name for RogueTournMiracle<'a> {
    fn name(&self) -> &str {
        self.display.name
    }
}

#[derive(Clone, Debug)]
pub struct RogueTournMiracleDisplay<'a> {
    pub id: u16,
    pub name: &'a str,
    pub desc: String,
    pub extra_effect: Vec<vo::misc::ExtraEffect<'a>>,
    pub bg_desc: &'a str,
    pub tag: &'a str,
}

#[derive(Clone, Debug)]
pub struct RogueTournHandbookMiracle<'a> {
    pub id: u16,
    pub display: RogueTournMiracleDisplay<'a>,
    pub category: MiracleCategory,
    pub unlock_desc: RogueTournContentDisplay<'a>,
}

#[derive(Clone, Debug)]
pub struct RogueTournContentDisplay<'a> {
    pub id: u16,
    pub content: &'a str,
}

#[derive(Clone, Debug)]
/// 差分宇宙方程
pub struct RogueTournFormula<'a> {
    pub id: u32,
    pub main_buff_type: Path,
    pub main_buff_num: u8,
    pub sub_buff_type: Option<Path>,
    pub sub_buff_num: u8,
    pub category: FormulaCategory,
    pub maze_buff: vo::rogue::RogueMazeBuff<'a>,
    pub display: vo::rogue_tourn::RogueTournFormulaDisplay<'a>,
    pub is_in_handbook: bool,
    pub story: (), // TODO
    pub unlock_display: Option<RogueTournContentDisplay<'a>>,
}

impl<'a> Name for RogueTournFormula<'a> {
    fn name(&self) -> &str {
        self.maze_buff.name
    }
}

#[derive(Clone, Debug)]
pub struct RogueTournFormulaDisplay<'a> {
    pub id: u32,
    pub story: &'a str,
    pub extra_effect: Vec<vo::misc::ExtraEffect<'a>>,
}
