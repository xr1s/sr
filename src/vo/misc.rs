use crate::{
    po::misc::{ItemMainType, ItemSubType, Rarity, UseMethod},
    GameData,
};

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
    pub item_ids: &'a [u32; 6],
    pub counts: &'a [u32; 6],
    pub levels: &'a [u8; 6],
    pub ranks: &'a [u8; 6],
}

#[derive(derivative::Derivative)]
#[derivative(Clone, Debug)]
pub struct ItemConfig<'a> {
    #[derivative(Debug = "ignore")]
    pub(crate) game: &'a GameData,
    pub id: u32,
    pub name: &'a str,
    pub main_type: ItemMainType,
    pub sub_type: ItemSubType,
    /// 稀有度（星级）
    pub rarity: Rarity,
    pub desc: &'a str,
    pub bg_desc: &'a str,
    /// 背包中的最大堆叠数
    pub pile_limit: u32,
    /// 使用效果根据 `use_method` 决定, 通过 `use_data_id` 关联到具体效果
    /// 具体效果在不同类型的对象里, 无法在这里简单映射
    pub use_method: Option<UseMethod>,
    /// 使用效果根据 `use_method` 决定, 通过 `use_data_id` 关联到具体效果
    /// 具体效果在不同类型的对象里, 无法在这里简单映射
    /// 2.6 之后似乎消失了?
    pub use_data_id: u32,
}

impl ItemConfig<'_> {}

#[derive(Clone, Debug)]
pub struct ItemUseData<'a> {
    pub id: u32,
    pub use_param: Vec<RewardData<'a>>,
    pub use_multiple_max: u8,
    pub is_auto_use: bool,
}
