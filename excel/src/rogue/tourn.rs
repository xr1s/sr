use crate::{ExcelOutput, FromModel};

use base::{Name, Wiki};
pub use model::rogue::tourn::{FormulaCategory, MiracleCategory};
pub use model::Path;

use std::borrow::Cow;
use std::num::NonZero;

#[derive(Clone, Debug)]
pub struct RogueBonus<'a> {
    pub id: u16,
    pub title: &'a str,
    pub desc: &'a str,
    pub tag: &'a str,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for RogueBonus<'a> {
    type Model = model::rogue::tourn::RogueBonus;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            id: model.bonus_id,
            title: game.text(model.bonus_title),
            desc: game.text(model.bonus_desc),
            tag: game.text(model.bonus_tag),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RogueTournContentDisplay<'a> {
    pub id: u16,
    pub content: &'a str,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for RogueTournContentDisplay<'a> {
    type Model = model::rogue::tourn::RogueTournContentDisplay;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            id: model.display_id,
            content: game.text(model.display_content),
        }
    }
}

#[derive(educe::Educe)]
#[educe(Clone, Debug)]
/// 周期演算
pub struct RogueTournWeeklyChallenge<'a, Data: ExcelOutput + ?Sized> {
    #[educe(Debug(ignore))]
    game: &'a Data,
    pub id: u8,
    /// 标题
    pub name: &'a str,
    /// 文字介绍，一般是初始获得方程和初始获得奇物的介绍
    pub content: Vec<String>,
    /// 点进介绍后的详情，一般是多一句进入第一位面时获得本周预设构筑
    pub content_detail: Vec<String>,
    /// 左下角展示的奖励，目前为止全部都是固定的 3 遗失晶块 + 30 遗器残骸
    pub reward: crate::misc::RewardData<'a>,
    /// 从 .content 成员中提取出来的本周所有预设构筑方程
    pub formula: Vec<RogueTournFormula<'a>>,
    /// 从 .content 成员中提取出来的本周所有预设构筑奇物
    pub miracle: Vec<RogueTournMiracle<'a>>,
    /// 第一位面首领
    pub monster_group_1: Vec<(u8, crate::rogue::RogueMonsterGroup<'a>)>,
    /// 第二位面首领
    pub monster_group_2: Vec<(u8, crate::rogue::RogueMonsterGroup<'a>)>,
    /// 第三位面首领
    pub monster_group_3: Vec<(u8, crate::rogue::RogueMonsterGroup<'a>)>,

    /// 如果有方程，会在进入区域获得随机方程对应的祝福
    pub bonus: std::sync::OnceLock<Option<RogueBonus<'a>>>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for RogueTournWeeklyChallenge<'a, Data> {
    type Model = model::rogue::tourn::RogueTournWeeklyChallenge;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        let mut content_list = model
            .weekly_content_list
            .iter()
            .map(|&id| game.rogue_tourn_weekly_display(id))
            .map(Option::unwrap)
            .collect::<Vec<_>>();
        Self {
            game,
            id: model.challenge_id,
            name: game.text(model.weekly_name),
            content: content_list
                .iter_mut()
                .map(|content| std::mem::take(&mut content.content))
                .collect(),
            content_detail: model
                .weekly_content_detail_list
                .iter()
                // 2.7 版本倒数第二周（2.8 前瞻所在周）的 DisplayID 是 1302 和 1303，缺数据，注意一下
                .filter(|&&id| id != 1302 && id != 1303)
                .map(|&id| game.rogue_tourn_weekly_display(id))
                .map(Option::unwrap)
                .map(|display| display.content)
                .collect(),
            reward: game.reward_data(model.reward_id).unwrap(),
            formula: content_list
                .iter_mut()
                .flat_map(|content| std::mem::take(&mut content.formula))
                .collect(),
            miracle: content_list
                .iter_mut()
                .flat_map(|content| std::mem::take(&mut content.miracle))
                .collect(),
            monster_group_1: model
                .display_monster_groups_1
                .iter()
                .map(|&(lv, id)| (lv, game.rogue_monster_group(id).unwrap()))
                .collect(),
            monster_group_2: model
                .display_monster_groups_2
                .iter()
                .map(|&(lv, id)| (lv, game.rogue_monster_group(id).unwrap()))
                .collect(),
            monster_group_3: model
                .display_monster_groups_3
                .iter()
                .map(|&(lv, id)| (lv, game.rogue_monster_group(id).unwrap()))
                .collect(),
            bonus: std::sync::OnceLock::new(),
        }
    }
}

impl<Data: ExcelOutput> RogueTournWeeklyChallenge<'_, Data> {
    const FIRST_CHALLENGE_MONDAY: chrono::DateTime<chrono::FixedOffset> =
        chrono::DateTime::from_naive_utc_and_offset(
            chrono::NaiveDateTime::new(
                chrono::NaiveDate::from_ymd_opt(2024, 6, 16).unwrap(),
                chrono::NaiveTime::from_hms_opt(20, 0, 0).unwrap(),
            ),
            chrono::FixedOffset::east_opt(8 * 60 * 60).unwrap(),
        );

    pub fn begin_time(&self) -> chrono::DateTime<chrono::FixedOffset> {
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

    pub fn issue(&self) -> u16 {
        self.id as u16
    }

    pub fn bonus(&self) -> &Option<RogueBonus<'_>> {
        self.bonus.get_or_init(|| {
            if self.formula.is_empty() {
                return None;
            }
            let mut bonus = self.game.rogue_bonus(410).unwrap();
            for id in 2..=self.id {
                let challenge = self.game.rogue_tourn_weekly_challenge(id).unwrap();
                if challenge.formula.is_empty() {
                    continue;
                }
                bonus = self.game.rogue_bonus(bonus.id + 1).unwrap();
                while !bonus.desc.starts_with("获得一些") {
                    bonus = self.game.rogue_bonus(bonus.id + 1).unwrap();
                }
            }
            Some(bonus)
        })
    }
}

impl<Data: ExcelOutput> Wiki for RogueTournWeeklyChallenge<'_, Data> {
    fn wiki(&self) -> Cow<'static, str> {
        let mut wiki = String::from("{{#subobject:");
        wiki.push_str(self.name);
        wiki.push_str("\n|名称=");
        wiki.push_str(self.name);
        wiki.push_str("\n|开始时间=");
        wiki.push_str(&self.begin_time().format("%Y/%m/%d").to_string());
        wiki.push_str("\n|结束时间=");
        let end_date = self.end_time().date_naive().pred_opt().unwrap();
        wiki.push_str(&end_date.format("%Y/%m/%d").to_string());
        if !self.miracle.is_empty() {
            let miracles = self
                .miracle
                .iter()
                .map(RogueTournMiracle::wiki_name)
                .intersperse(Cow::Borrowed(", "))
                .collect::<String>();
            wiki.push_str("\n|起始奇物=");
            wiki.push_str(&miracles);
        }
        if !self.formula.is_empty() {
            let formulas = self
                .formula
                .iter()
                .map(RogueTournFormula::wiki_name)
                .intersperse(Cow::Borrowed(", "))
                .collect::<String>();
            wiki.push_str("\n|起始方程=");
            wiki.push_str(&formulas);
        }
        if let Some(bonus) = self.bonus() {
            wiki.push_str("\n|开拓祝福=");
            wiki.push_str(bonus.title);
            wiki.push_str("\n|开拓祝福内容=");
            wiki.push_str(bonus.desc);
        }
        fn boss(wiki: &mut String, plane: char, groups: &[(u8, crate::rogue::RogueMonsterGroup)]) {
            for (level, group) in groups {
                wiki.push_str("\n|第");
                wiki.push(plane);
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
        }
        boss(&mut wiki, '一', &self.monster_group_1);
        boss(&mut wiki, '二', &self.monster_group_2);
        boss(&mut wiki, '三', &self.monster_group_3);
        let contents = self
            .content_detail
            .iter()
            .map(String::as_str)
            .map(|s| s.strip_prefix("●").unwrap_or(s))
            .intersperse(", ")
            .map(format::format_wiki)
            .collect::<String>();
        wiki.push_str("\n|规则=");
        wiki.push_str(&contents);
        wiki.push_str("\n}}");
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

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for RogueTournWeeklyDisplay<'a> {
    type Model = model::rogue::tourn::RogueTournWeeklyDisplay;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        use model::rogue::tourn::DescParamType::{Formula, Miracle};
        let formula = model
            .desc_params
            .iter()
            .filter(|param| param.r#type == Formula)
            .map(|param| game.rogue_tourn_formula(param.value))
            .map(Option::unwrap)
            .collect();
        let miracle = model
            .desc_params
            .iter()
            .filter(|param| param.r#type == Miracle)
            .map(|param| game.rogue_tourn_miracle(param.value as _))
            .map(Option::unwrap)
            .collect();
        use either::Either;
        let params = model
            .desc_params
            .iter()
            .map(|param| match param.r#type {
                Formula => Either::Left(game.rogue_tourn_formula(param.value).unwrap()),
                Miracle => Either::Right(game.rogue_tourn_miracle(param.value as _).unwrap()),
            })
            .collect::<Vec<_>>();
        let names = params
            .iter()
            .map(Name::name)
            .map(format::Argument::from)
            .collect::<Vec<_>>();
        let content = format::format(game.text(model.weekly_display_content), &names);

        Self {
            id: model.weekly_display_id,
            content,
            formula,
            miracle,
        }
    }
}

#[derive(Clone, Debug)]
/// 差分宇宙奇物
pub struct RogueTournMiracle<'a> {
    pub id: u16,
    /// 奇物星级：加权、三星、二星、一星、负面
    pub category: MiracleCategory,
    /// 奇物文案和图标
    pub display: crate::rogue::RogueMiracleDisplay<'a>,
    /// 图鉴中的奇物展示
    pub handbook: Option<RogueTournHandbookMiracle<'a>>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for RogueTournMiracle<'a> {
    type Model = model::rogue::tourn::RogueTournMiracle;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            id: model.miracle_id,
            category: model.miracle_category,
            display: game
                .rogue_tourn_miracle_display(model.miracle_display_id)
                .unwrap(),
            handbook: model
                .handbook_miracle_id
                .map(NonZero::get)
                .map(|id| game.rogue_tourn_handbook_miracle(id))
                .map(Option::unwrap),
        }
    }
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
    pub display: crate::rogue::RogueMiracleDisplay<'a>,
    /// 奇物稀有度
    pub category: MiracleCategory,
    /// 图鉴中未解锁时的提示文案，目前只有一种
    pub unlock_desc: RogueTournContentDisplay<'a>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for RogueTournHandbookMiracle<'a> {
    type Model = model::rogue::tourn::RogueTournHandbookMiracle;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            id: model.handbook_miracle_id,
            display: game
                .rogue_tourn_miracle_display(model.miracle_display_id)
                .unwrap(),
            category: model.miracle_category,
            unlock_desc: game.rogue_tourn_content_display(model.unlock_desc).unwrap(),
        }
    }
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
    pub maze_buff: crate::misc::MazeBuff<'a>,
    /// 方程的背景故事文案和特殊效果说明
    pub display: crate::rogue::tourn::RogueTournFormulaDisplay<'a>,
    /// 是否在图鉴中（临界方程均为 false）
    pub is_in_handbook: bool,
    /// 临界方程和三星方程首次展开的推演故事
    pub story: &'a std::path::Path,
    /// 图鉴中未解锁时的提示文案，目前只有一种
    pub unlock_display: Option<RogueTournContentDisplay<'a>>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for RogueTournFormula<'a> {
    type Model = model::rogue::tourn::RogueTournFormula;
    fn from_model(game: &'a Data, model: &'a Self::Model) -> Self {
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
        Self {
            id: model.formula_id,
            main_buff_type: buff_type_id_to_path(model.main_buff_type_id),
            main_buff_num: model.main_buff_num,
            sub_buff_type: model.sub_buff_type_id.map(buff_type_id_to_path),
            sub_buff_num: model.sub_buff_num.map(NonZero::get).unwrap_or_default(),
            category: model.formula_category,
            maze_buff: game
                .rogue_maze_buff(model.maze_buff_id)
                .into_iter()
                .next()
                .unwrap(),
            display: game
                .rogue_tourn_formula_display(model.formula_display_id)
                .unwrap(),
            is_in_handbook: model.is_in_handbook,
            story: &model.formula_story_json,
            unlock_display: model
                .unlock_display_id
                .map(NonZero::get)
                .map(|id| game.rogue_tourn_content_display(id))
                .map(Option::unwrap),
        }
    }
}

impl Name for RogueTournFormula<'_> {
    fn name(&self) -> &str {
        self.maze_buff.name
    }
    fn wiki_name(&self) -> Cow<'_, str> {
        if self.category == FormulaCategory::PathEcho
            || ["赏金猎人", "筑城者", "混沌医师"].contains(&self.name())
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
    pub extra_effect: Vec<crate::misc::ExtraEffectConfig<'a>>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for RogueTournFormulaDisplay<'a> {
    type Model = model::rogue::tourn::RogueTournFormulaDisplay;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            id: model.formula_display_id,
            story: game.text(model.formula_story),
            extra_effect: model
                .extra_effect
                .iter()
                .map(|&id| game.extra_effect_config(id))
                .map(Option::unwrap)
                .collect(),
        }
    }
}
