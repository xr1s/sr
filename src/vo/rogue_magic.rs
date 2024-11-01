use crate::vo;

#[derive(Clone, Debug)]
/// 不可知域奇物
pub struct RogueMagicMiracle<'a> {
    pub id: u16,
    pub display: vo::rogue::RogueMiracleDisplay<'a>,
    pub unlock_handbook: vo::rogue::RogueHandbookMiracle<'a>,
    pub desc: String,
}
