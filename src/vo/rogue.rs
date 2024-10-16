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
