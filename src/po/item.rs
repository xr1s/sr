use super::Text;
use crate::{vo, GameData, ID, PO};

use std::num::NonZero;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
/// 物品类型，对应打开背包后顶部标签页的分类
pub enum ItemMainType {
    /// 有个图标用来展示, 实际不存在的道具
    /// 出现在黑塔模拟宇宙遗器模板
    Display,
    /// 光锥, 仅出现在 ItemConfigEquipment.json 中
    Equipment,
    /// 各种材料, 非常多非常杂, 参见 ItemSubType
    Material,
    /// 任务道具
    Mission,
    /// 随宠
    Pet,
    /// 可交互的（可消耗、可阅读）
    Usable,
    /// 各种不占据背包格子的数值项
    /// 比如 星琼, 信用点, 经验, 开拓力, 各类活动金币等
    Virtual,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum ItemSubType {
    /// 以太战线技能芯片
    /// 对应 ItemMainType 为 Material
    AetherSkill,
    /// 以太战线宠物
    /// 对应 ItemMainType 为 Material
    AetherSpirit,
    /// 书籍
    /// 对应 ItemMainType 为 Usable
    /// 对应 UseMethod 为 AutoConversionItem 自动转换
    Book,
    /// 对话框
    /// 对应 ItemMainType 为 Usable
    /// 对应 UseMethod 为 AutoConversionItem 自动转换
    ChatBubble,
    /// 黄金与机械骰面
    /// 对应 ItemMainType 为 Usable
    /// 对应 UseMethod 为 AutoConversionItem 自动转换
    ChessRogueDiceSurface,
    /// 星魂, 仅出现在 ItemConfigAvatarRank.json 中
    Eidolon,
    /// 光锥, 仅出现在 ItemConfigEquipment.json 中
    Equipment,
    /// 星天演武仪典技能和饮料
    /// 对应 ItemMainType 为 Material
    FightFestSkill,
    /// 食品
    /// 对应 ItemMainType 为 Usable
    Food,
    /// 一般是领取光锥角色的道具
    /// 对应 ItemMainType 为 Usable
    ForceOpitonalGift,
    /// 合成配方
    /// 对应 ItemMainType 为 Usable
    Formula,
    /// 怪物隐身玩法资源废弃，只在 1.6 及之前出现
    GameplayCounter,
    /// 各种兑换类道具（包括商城礼包）
    /// 对应 ItemMainType 为 Usable
    /// 对应 UseMethod 为大小月卡四种 MonthlyCard, BPUnlock68, BPUnlock128, BPUpgradeFrom68To128
    /// 固定奖励 FixedRewardGift, 随机奖励 RandomRewardGift, 用户选择奖励 PlayerSelectedReward
    Gift,
    /// 非常杂, ItemMainType 为 Material 的剩下的都在里面
    /// 包括但不限于角色, 天赋, 武器的突破材料, 周本材料; 遗器, 角色, 武器经验等
    /// 抽卡用的专票、通票等，各种活动积分等
    Material,
    /// 任务道具, ItemMainType 为 Mission 的都在这里
    Mission,
    /// 东城博物馆活动的展览品
    /// 对应 ItemMainType 为 Material
    MuseumExhibit,
    /// 冬城博物馆活动的员工
    /// 对应 ItemMainType 为 Material
    MuseumStuff,
    /// 随宠
    /// 对应 ItemMainType 为 Pet
    NormalPet,
    /// 帕姆皮肤，派对车厢皮肤
    /// 对应的 ItemMainType 为 Usable
    PamSkin,
    /// 手机主题
    /// 对应 ItemMainType 为 Usable
    /// 对应 UseMethod 为 AutoConversionItem 自动转换
    PhoneTheme,
    /// 图标展示用, 实际不存在的位面饰品套装图，代表任意的位面饰品
    /// 出现在黑塔空间站地图上黑塔办公室传送点的沉浸奖励
    /// 对应 ItemMainType 为 Display
    RelicRarityShowOnly,
    /// 图标展示用, 实际不存在的位面饰品套装图，不是绳球分别的遗器
    /// 出现在模拟宇宙主界面提示每个宇宙能获得哪种套装的沉浸奖励处
    /// 对应 ItemMainType 为 Display
    RelicSetShowOnly,
    /// 差分宇宙概率艺术馆展品, 目前只有「庸者的前路」和「黑塔•典藏版」
    RogueMedal,
    /// 匹诺康尼梦境护照上的贴纸
    /// 对应 ItemMainType 是 Usable
    TravelBrochurePaster,
    /// 各种不占据背包格子的数值项
    /// 比如 星琼, 信用点, 经验, 开拓力, 各类活动金币等
    Virtual,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
/// 物品稀有度（星级）
pub enum Rarity {
    /// 一星
    Normal,
    /// 二星
    NotNormal,
    /// 三星
    Rare,
    /// 五星
    SuperRare,
    /// 四星
    VeryRare,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum UseMethod {
    /// 自动转换为图鉴等道具. 目前有图书, 聊天框, 黄金与机械骰面, 手机主题四种.
    AutoConversionItem,
    /// 128 月卡
    BPUnlock128,
    /// 68 月卡
    BPUnlock68,
    /// 68 月卡升级 128 月卡道具
    BPUpgradeFrom68To128,
    /// 食物效果, 往往是战斗增益或减益.
    /// 当 UseMethod 为此时, 会通过 UseDataID 关联到 ItemUseData.json
    /// 再通过对应 ItemUseData 对象的 UerParam 作为主键关联到 ItemBuffData.json
    /// 具体数值通过 ItemBuffData 的 MazeBuffID 关联到 MazeBuff.json
    ExternalSystemFoodBenefit,
    /// 只有一个星天演武仪典的纪念道具
    FightFestMemorialPaper,
    /// 固定奖励
    /// 当 UseMethod 为此时, 会通过 UseDataID 关联到 ItemUseData.json
    /// 再通过对应 ItemUseData 对象的 UerParam 作为主键关联到 RewardData.json
    /// 再通过 RewardData 的 ItemID_* 作为主键关联到角色、光锥或道具
    FixedRewardGift,
    /// 30 月卡
    MonthlyCard,
    /// 随宠
    PetSummonRecall,
    /// 用户多选一, 一般是活动的角色或者光锥奖励
    /// 当 UseMethod 为此时, 会通过 UseDataID 关联到 ItemUseData.json
    /// 再通过对应 ItemUseData 对象的 UerParam 作为主键关联到 RewardData.json
    /// 再通过 RewardData 的 ItemID_* 作为主键关联到角色、光锥或道具
    PlayerSelectedReward,
    /// 随机多选一, 机制不明
    RandomRewardGift,
    /// 合成台配方
    Recipe,
    /// 食物效果, 往往是战斗增益或减益.
    /// 当 UseMethod 为此时, 会通过 UseDataID 关联到 ItemUseData.json
    /// 再通过对应 ItemUseData 对象的 UerParam 作为主键关联到 ItemBuffData.json
    /// 具体数值通过 ItemBuffData 的 MazeBuffID 关联到 MazeBuff.json
    TeamSpecificFoodBenefit,
    /// 匹诺康尼梦境护照上的剪贴纸
    TravelBrochurePasterUse,
    /// 梦境护照本身（只有一个道具）
    TravelBrochureUse,
    /// 藏宝图, 一般是使用后带有额外文字或图片的书籍或者相册, 非消耗品
    /// 当 UseMethod 为此时, 会通过 UseDataID 关联到 ItemCureInfoData.json
    TreasureMap,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum UseType {
    Food,
    Formula,
    Gift,
    Treasure,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum SellType {
    Destroy,
    Sell,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct ItemList {
    #[serde(rename = "ItemID")]
    pub(crate) item_id: u32,
    #[serde(rename = "ItemNum")]
    pub(crate) item_num: Option<NonZero<u16>>,
}

impl<'a> PO<'a> for ItemList {
    type VO = vo::item::ItemList<'a>;
    fn vo(&self, game: &'a GameData) -> Self::VO {
        Self::VO {
            item: None
                .or_else(|| game.item_config(self.item_id))
                .or_else(|| game.item_config_avatar_rank(self.item_id))
                .or_else(|| game.item_config_equipment(self.item_id))
                .unwrap(),
            num: self.item_num.map(NonZero::get).unwrap_or_default(),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct ItemConfig {
    #[serde(rename = "ID")]
    id: u32,
    item_main_type: ItemMainType,
    item_sub_type: ItemSubType,
    inventory_display_tag: u8, // 只有 1 2 3
    rarity: Rarity,
    purpose_type: Option<NonZero<u8>>,
    #[serde(rename = "isVisible", default)]
    is_visible: bool,
    item_name: Text,
    item_desc: Text,
    #[serde(rename = "ItemBGDesc")]
    item_bg_desc: Text,
    item_icon_path: PathBuf,
    item_figure_icon_path: PathBuf,
    item_currency_icon_path: PathBuf,
    item_avatar_icon_path: PathBuf,
    #[serde(default)]
    is_auto_use: bool, // 1.3 及之前，后面大概合并为 UseMethod 了
    pile_limit: u32,
    use_method: Option<UseMethod>,
    use_type: Option<UseType>, // 1.3 及之前，后面大概合并为 UseMethod 了
    #[serde(rename = "UseDataID")]
    use_data_id: Option<NonZero<u32>>,
    custom_data_list: Vec<u16>,
    #[serde(rename = "ReturnItemIDList")]
    return_item_id_list: Vec<ItemList>,
    item_group: Option<NonZero<u16>>,
    sell_type: Option<SellType>,
    #[serde(default)]
    is_show_red_dot: bool,
}

impl ID for ItemConfig {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.id
    }
}

impl<'a> PO<'a> for ItemConfig {
    type VO = vo::item::ItemConfig<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        Self::VO {
            game,
            id: self.id,
            name: game.text(self.item_name),
            main_type: self.item_main_type,
            sub_type: self.item_sub_type,
            rarity: self.rarity,
            desc: game.text(self.item_desc),
            bg_desc: game.text(self.item_bg_desc),
            pile_limit: self.pile_limit,
            use_method: self.use_method,
            use_data_id: self.use_data_id.map(NonZero::get).unwrap_or_default(),
            return_item_id_list: self
                .return_item_id_list
                .iter()
                .map(|item| item.vo(game))
                .collect(),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct ItemUseData {
    #[serde(rename = "UseDataID")]
    use_data_id: u32,
    use_param: Vec<u32>,
    use_multiple_max: u8,
    #[serde(default)]
    is_auto_use: bool,
}

impl ID for ItemUseData {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.use_data_id
    }
}

impl<'a> PO<'a> for ItemUseData {
    type VO = vo::item::ItemUseData<'a>;
    fn vo(&self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.use_data_id,
            use_param: self
                .use_param
                .iter()
                .map(|&param| game.reward_data(param))
                .map(Option::unwrap)
                .collect(),
            use_multiple_max: self.use_multiple_max,
            is_auto_use: self.is_auto_use,
        }
    }
}
