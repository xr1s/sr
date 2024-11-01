#[derive(Clone, Debug)]
/// 模拟宇宙祝福（方程、回响也在此）
pub struct RogueMazeBuff<'a> {
    pub id: u32,
    /// 初始等级
    pub lv: u8,
    /// 只有 1 或者 2
    /// 1 表示祝福无法强化（如回响交错、差分宇宙方程等）
    /// 2 表示祝福可以被强化
    pub max_lv: u8,
    /// 祝福名称
    pub name: &'a str,
    /// 祝福详细文案
    pub desc: String,
    /// 祝福简单文案
    pub simple_desc: String,
    pub desc_battle: &'a str,
}

#[derive(Clone, Debug)]
/// 模拟宇宙奇物
pub struct RogueMiracle<'a> {
    pub id: u16,
    pub display: RogueMiracleDisplay<'a>,
    pub desc: &'a str,
    pub handbook: Option<RogueHandbookMiracle<'a>>,
}

#[derive(Clone, Debug)]
/// 模拟宇宙奇物展示数据（效果、背景故事等）
pub struct RogueMiracleDisplay<'a> {
    pub id: u16,
    pub name: &'a str,
    pub desc: String,
    pub bg_desc: &'a str,
    pub tag: &'a str,
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
