use std::borrow::Cow;

use crate::po::rogue::tourn::{FormulaCategory, MiracleCategory};
use crate::po::Path;
use crate::{vo, Name, Wiki};

#[derive(Clone, Debug)]
/// 周期演算
pub struct RogueTournWeeklyChallenge<'a> {
    pub id: u8,
    /// 标题
    pub name: &'a str,
    /// 文字介绍，一般是初始获得方程和初始获得奇物的介绍
    pub content: Vec<String>,
    /// 点进介绍后的详情，一般是多一句进入第一位面时获得本周预设构筑
    pub content_detail: Vec<String>,
    /// 左下角展示的奖励，目前为止全部都是固定的 3 遗失晶块 + 30 遗器残骸
    pub reward: vo::misc::RewardData<'a>,
    /// 从 .content 成员中提取出来的本周所有预设构筑方程
    pub formula: Vec<RogueTournFormula<'a>>,
    /// 从 .content 成员中提取出来的本周所有预设构筑奇物
    pub miracle: Vec<RogueTournMiracle<'a>>,
    /// 第一位面首领
    pub monster_group_1: Vec<(u8, vo::rogue::RogueMonsterGroup<'a>)>,
    /// 第二位面首领
    pub monster_group_2: Vec<(u8, vo::rogue::RogueMonsterGroup<'a>)>,
    /// 第三位面首领
    pub monster_group_3: Vec<(u8, vo::rogue::RogueMonsterGroup<'a>)>,
}

impl RogueTournWeeklyChallenge<'_> {
    const FIRST_CHALLENGE_MONDAY: chrono::DateTime<chrono::FixedOffset> =
        chrono::DateTime::from_naive_utc_and_offset(
            chrono::NaiveDateTime::new(
                chrono::NaiveDate::from_ymd_opt(2024, 6, 17).unwrap(),
                chrono::NaiveTime::from_hms_opt(4, 0, 0).unwrap(),
            ),
            chrono::FixedOffset::east_opt(8 * 60 * 60).unwrap(),
        );

    pub fn start_time(&self) -> chrono::DateTime<chrono::FixedOffset> {
        if self.id == 1 {
            // 第一期跟随版本开放日期，周三十点开始
            return Self::FIRST_CHALLENGE_MONDAY
                + chrono::TimeDelta::days(2)
                + chrono::TimeDelta::hours(6);
        }
        Self::FIRST_CHALLENGE_MONDAY + chrono::TimeDelta::weeks(self.id as i64 - 1)
    }

    pub fn end_time(&self) -> chrono::DateTime<chrono::FixedOffset> {
        Self::FIRST_CHALLENGE_MONDAY + chrono::TimeDelta::weeks(self.id as _)
            - chrono::TimeDelta::nanoseconds(1)
    }
}

impl Wiki for RogueTournWeeklyChallenge<'_> {
    fn wiki(&self) -> Cow<'static, str> {
        let mut wiki = String::from("{{周期演算|");
        wiki.push_str(self.name);
        wiki.push_str("|开始时间=");
        wiki.push_str(&self.start_time().format("%Y/%m/%d").to_string());
        wiki.push_str("|结束时间=");
        let end_date = self.end_time().date_naive().pred_opt().unwrap();
        wiki.push_str(&end_date.format("%Y/%m/%d").to_string());
        if !self.miracle.is_empty() {
            let miracles = self
                .miracle
                .iter()
                .map(RogueTournMiracle::wiki_name)
                .intersperse(Cow::Borrowed(", "))
                .collect::<String>();
            wiki.push_str("|起始奇物=");
            wiki.push_str(&miracles);
        }
        if !self.formula.is_empty() {
            let formulas = self
                .formula
                .iter()
                .map(RogueTournFormula::wiki_name)
                .intersperse(Cow::Borrowed(", "))
                .collect::<String>();
            wiki.push_str("|起始方程=");
            wiki.push_str(&formulas);
        }
        macro_rules! boss {
            ($plane:ident, $number:literal) => {
                for (level, group) in &self.$plane {
                    wiki.push_str("|第");
                    wiki.push($number);
                    wiki.push_str("位面首领");
                    if *level != 0 {
                        wiki.push('V');
                        wiki.push_str(&level.to_string())
                    }
                    wiki.push('=');
                    let monsters = group
                        .list_and_weight
                        .iter()
                        .map(|(monster, _)| monster.wiki_name())
                        .intersperse(Cow::Borrowed(", "))
                        .collect::<String>();
                    wiki.push_str(&monsters);
                }
            };
        }
        boss!(monster_group_1, '一');
        boss!(monster_group_2, '二');
        boss!(monster_group_3, '三');
        let contents = self
            .content_detail
            .iter()
            .map(String::as_str)
            .map(|s| s.strip_prefix("●").unwrap_or(s))
            .intersperse(", ")
            .collect::<String>();
        wiki.push_str("|规则=");
        wiki.push_str(&contents);
        wiki.push_str("}}");
        Cow::Owned(wiki)
    }
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
    fn wiki_name(&self) -> Cow<'_, str> {
        match self.id {
            6122 => Cow::Borrowed("邪恶机械卫星＃900"),
            6505 => Cow::Borrowed("醒觉-310"),
            _ => Cow::Borrowed(self.name()),
        }
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
    pub maze_buff: vo::misc::MazeBuff<'a>,
    /// 方程的背景故事文案和特殊效果说明
    pub display: vo::rogue::tourn::RogueTournFormulaDisplay<'a>,
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
    fn wiki_name(&self) -> Cow<'_, str> {
        if self.category == FormulaCategory::PathEcho
            || ["赏金猎人", "筑城者"].contains(&self.name())
        {
            Cow::Owned(self.name().to_string() + "（方程）")
        } else {
            Cow::Borrowed(self.name())
        }
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
