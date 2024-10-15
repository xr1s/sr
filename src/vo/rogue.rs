#[derive(Clone, Debug)]
/// 模拟宇宙祝福
pub struct RogueMazeBuff<'a> {
    pub id: u32,
    pub lv: u8,
    pub max_lv: u8,
    pub name: &'a str,
    pub desc: String,
    pub simple_desc: String,
    pub desc_battle: &'a str,
}
