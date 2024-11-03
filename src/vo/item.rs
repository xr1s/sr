use crate::po::item::{ItemMainType, ItemSubType, Rarity, UseMethod};
use crate::{vo, GameData};

#[derive(Clone, Debug)]
pub struct ItemList<'a> {
    pub item: ItemConfig<'a>,
    pub num: u16,
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
    /// 道具拆分效果，比如光锥、遗器
    pub return_item_id_list: Vec<ItemList<'a>>,
}

#[derive(Clone, Debug)]
pub struct ItemUseData<'a> {
    pub id: u32,
    pub use_param: Vec<vo::misc::RewardData<'a>>,
    pub use_multiple_max: u8,
    pub is_auto_use: bool,
}
