#[derive(Clone, Debug)]
/// 游戏中的备注文案，一般来说是在一大段文案中的下划线，点一下会有介绍遮罩
/// 如 【反震】 的介绍是：由特定「存护」命途祝福造成的附加伤害。
/// 这里 `name = "反震"`，`desc = "由特定「存护」命途祝福造成的附加伤害。"`
pub struct ExtraEffectConfig<'a> {
    pub id: u32,
    pub name: &'a str,
    pub desc: String,
}
