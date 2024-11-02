#[derive(Clone, Debug)]
/// 游戏中的备注文案，一般来说是在一大段文案中的下划线，点一下会有介绍遮罩
/// 如 【反震】 的介绍是：由特定「存护」命途祝福造成的附加伤害。
/// 这里 `name = "反震"`，`desc = "由特定「存护」命途祝福造成的附加伤害。"`
pub struct ExtraEffectConfig<'a> {
    pub id: u32,
    pub name: &'a str,
    pub desc: String,
}

#[derive(Clone, Debug)]
pub struct RewardData<'a> {
    pub id: u32,
    /// 奖励可能是物品、光锥、角色，后两者常出现于活动奖励
    pub item_ids: &'a [u32; 6],
    /// 数量
    pub counts: &'a [u32; 6],
    /// 不明，目前全部都是 1
    pub levels: &'a [u8; 6],
    /// 不明，目前全部都是 1
    pub ranks: &'a [u8; 6],
    /// 星琼
    pub hcoin: u16,
    pub is_special: bool,
}
