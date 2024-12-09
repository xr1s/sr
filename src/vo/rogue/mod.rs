pub mod tourn;

use std::borrow::Cow;

use crate::{vo, Name};

#[derive(Clone, Debug)]
/// 模拟宇宙奇物
pub struct RogueMiracle<'a> {
    pub id: u16,
    /// 1.2 及之前版本的 display 为空
    pub display: RogueMiracleDisplay<'a>,
    pub desc: String,
    /// 没有 unlock_handbook 的一般是可以同时携带多个、效果不同的奇物
    /// 如分裂咕咕钟、绝对失败处方
    pub unlock_handbook: Option<RogueHandbookMiracle<'a>>,
}

impl Name for RogueMiracle<'_> {
    fn name(&self) -> &str {
        self.display.name
    }
    fn wiki_name(&self) -> Cow<'_, str> {
        Cow::Borrowed(self.name())
    }
}

#[derive(Clone, Debug)]
/// 模拟宇宙奇物展示数据（效果、背景故事等）
pub struct RogueMiracleDisplay<'a> {
    pub id: u16,
    /// 名称
    pub name: &'a str,
    /// 奇物效果
    pub desc: String,
    /// 奇物效果中，带有下划线的特殊效果的详细介绍
    pub extra_effect: Vec<vo::misc::ExtraEffectConfig<'a>>,
    /// 背景故事
    pub bg_desc: &'a str,
    /// 无意义，目前只有空字符串
    pub tag: &'a str,
    // 图标
    pub icon_path: &'a str,
    pub figure_icon_path: &'a str,
}

#[derive(Clone, Debug)]
// 模拟宇宙奇物图鉴信息（解锁奖励、在哪些 DLC 中出现等）
pub struct RogueHandbookMiracle<'a> {
    pub id: u16,
    pub reward: crate::vo::misc::RewardData<'a>,
    pub type_list: Vec<RogueHandbookMiracleType<'a>>,
    pub display: RogueMiracleDisplay<'a>,
    pub order: u8,
}

#[derive(Clone, Debug)]
// 模拟宇宙奇物图鉴所属 DLC
pub struct RogueHandbookMiracleType<'a> {
    pub id: u16,
    pub title: &'a str,
}

#[derive(Clone, Debug)]
// 模拟宇宙一轮战斗的敌人，目前只用于差分宇宙周期演算 Boss
pub struct RogueMonsterGroup<'a> {
    pub id: u32,
    pub list_and_weight: Vec<(vo::rogue::RogueMonster<'a>, u8)>,
}

#[derive(Clone, Debug)]
pub struct RogueMonster<'a> {
    pub id: u32,
    pub npc_monster: vo::monster::NPCMonsterData<'a>,
}

impl Name for RogueMonster<'_> {
    fn name(&self) -> &str {
        self.npc_monster.name
    }
    fn wiki_name(&self) -> Cow<'_, str> {
        vo::monster::wiki_name(self.name())
    }
}
