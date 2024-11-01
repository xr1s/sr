use crate::po::rogue_tourn::{FormulaCategory, MiracleCategory};
use crate::po::Path;
use crate::{vo, Name};

#[derive(Clone, Debug)]
/// 周期演算
pub struct RogueTournWeeklyChallenge<'a> {
    pub id: u16,
    /// 标题
    pub name: &'a str,
    /// 文字介绍，一般是初始获得方程和初始获得奇物的介绍
    pub content: Vec<String>,
    /// 点进介绍后的详情，一般是多一句进入第一位面时获得本周预设构筑
    pub content_detail: Vec<String>,
    /// 左下角展示的奖励，目前为止全部都是固定的 3 遗失晶块 + 30 遗器残骸
    pub reward: (), // TODO,
    /// 从 .content 成员中提取出来的本周所有预设构筑方程
    pub formula: Vec<RogueTournFormula<'a>>,
    /// 从 .content 成员中提取出来的本周所有预设构筑奇物
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
    /// 奇物星级：加权、三星、二星、一星、负面
    pub category: MiracleCategory,
    /// 奇物文案和图标
    pub display: vo::rogue::RogueMiracleDisplay<'a>,
    /// 图鉴中的奇物展示
    pub handbook: Option<RogueTournHandbookMiracle<'a>>,
}

impl Name for RogueTournMiracle<'_> {
    fn name(&self) -> &str {
        self.display.name
    }
}

#[derive(Clone, Debug)]
pub struct RogueTournHandbookMiracle<'a> {
    pub id: u16,
    pub display: vo::rogue::RogueMiracleDisplay<'a>,
    /// 奇物稀有度
    pub category: MiracleCategory,
    /// 图鉴中未解锁时的提示文案，目前只有一种
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
    /// 主要命途
    pub main_buff_type: Path,
    /// 主要命途需求数量
    pub main_buff_num: u8,
    /// 次要命途（临界方程时为 None）
    pub sub_buff_type: Option<Path>,
    /// 次要命途需求数量
    pub sub_buff_num: u8,
    /// 方程稀有度
    pub category: FormulaCategory,
    /// 对应模拟宇宙祝福（方程名称、效果文案都在此）
    pub maze_buff: vo::rogue::RogueMazeBuff<'a>,
    /// 方程的背景故事文案和特殊效果说明
    pub display: vo::rogue_tourn::RogueTournFormulaDisplay<'a>,
    /// 是否在图鉴中（临界方程均为 false）
    pub is_in_handbook: bool,
    /// 临界方程和三星方程首次展开的推演故事
    pub story: (), // TODO
    /// 图鉴中未解锁时的提示文案，目前只有一种
    pub unlock_display: Option<RogueTournContentDisplay<'a>>,
}

impl Name for RogueTournFormula<'_> {
    fn name(&self) -> &str {
        self.maze_buff.name
    }
}

#[derive(Clone, Debug)]
pub struct RogueTournFormulaDisplay<'a> {
    pub id: u32,
    /// 方程的背景故事文案
    pub story: &'a str,
    /// 方程特殊效果（如存护的【反震】、巡猎的【会心】等）的详细说明文案
    pub extra_effect: Vec<vo::misc::ExtraEffectConfig<'a>>,
}
