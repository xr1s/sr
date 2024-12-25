use std::num::NonZero;

pub use model::item::{ItemMainType, ItemRarity, ItemSubType, ItemUseMethod};

use crate::{ExcelOutput, FromModel};

#[derive(Clone, Debug)]
pub struct ItemList<'a> {
    pub item: ItemConfig<'a>,
    pub num: u16,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for ItemList<'a> {
    type Model = model::item::ItemList;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            item: None
                .or_else(|| game.item_config(model.item_id))
                .or_else(|| game.item_config_avatar_rank(model.item_id))
                .or_else(|| game.item_config_equipment(model.item_id))
                .unwrap(),
            num: model.item_num.map(NonZero::get).unwrap_or_default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ItemConfig<'a> {
    pub id: u32,
    pub name: &'a str,
    pub main_type: ItemMainType,
    pub sub_type: ItemSubType,
    /// 稀有度（星级）
    pub rarity: ItemRarity,
    pub desc: &'a str,
    pub bg_desc: &'a str,
    pub icon_path: &'a str,
    pub figure_icon_path: &'a str,
    pub currency_icon_path: &'a str,
    pub avatar_icon_path: &'a str,
    /// 背包中的最大堆叠数
    pub pile_limit: u32,
    /// 使用效果根据 `use_method` 决定, 通过 `use_data_id` 关联到具体效果
    /// 具体效果在不同类型的对象里, 无法在这里简单映射
    pub use_method: Option<ItemUseMethod>,
    /// 使用效果根据 `use_method` 决定, 通过 `use_data_id` 关联到具体效果
    /// 具体效果在不同类型的对象里, 无法在这里简单映射
    /// 2.6 之后似乎消失了?
    pub use_data_id: u32,
    /// 道具拆分效果，比如光锥、遗器
    pub return_item_id_list: Vec<ItemList<'a>>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for ItemConfig<'a> {
    type Model = model::item::ItemConfig;
    fn from_model(game: &'a Data, model: &'a Self::Model) -> Self {
        Self {
            id: model.id,
            name: game.text(model.item_name),
            main_type: model.item_main_type,
            sub_type: model.item_sub_type,
            rarity: model.rarity,
            desc: game.text(model.item_desc),
            bg_desc: game.text(model.item_bg_desc),
            icon_path: &model.item_icon_path,
            figure_icon_path: &model.item_figure_icon_path,
            currency_icon_path: &model.item_currency_icon_path,
            avatar_icon_path: &model.item_avatar_icon_path,
            pile_limit: model.pile_limit,
            use_method: model.use_method,
            use_data_id: model.use_data_id.map(NonZero::get).unwrap_or_default(),
            return_item_id_list: model
                .return_item_id_list
                .iter()
                .map(|item| ItemList::from_model(game, item))
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ItemUseData<'a> {
    pub id: u32,
    pub use_param: Vec<crate::misc::RewardData<'a>>,
    pub use_multiple_max: u8,
    pub is_auto_use: bool,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for ItemUseData<'a> {
    type Model = model::item::ItemUseData;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            id: model.use_data_id,
            use_param: model
                .use_param
                .iter()
                .map(|&param| game.reward_data(param))
                .map(Option::unwrap)
                .collect(),
            use_multiple_max: model.use_multiple_max,
            is_auto_use: model.is_auto_use,
        }
    }
}
