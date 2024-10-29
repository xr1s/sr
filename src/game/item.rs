use crate::{po, vo, FnvIndexMap, PO};

use super::GameData;

impl GameData {
    fn _item_config(&self) -> &FnvIndexMap<u32, po::item::ItemConfig> {
        self._item_config
            .get_or_init(|| self.load_to_map("ExcelOutput/ItemConfig.json"))
    }

    fn _item_use_data(&self) -> &FnvIndexMap<u32, po::item::ItemUseData> {
        self._item_use_data
            .get_or_init(|| self.load_to_map("ExcelOutput/ItemUseData.json"))
    }
}

impl GameData {
    pub fn item_config(&self, id: u32) -> Option<vo::item::ItemConfig> {
        self._item_config().get(&id).map(|po| po.vo(self))
    }

    pub fn list_item_config(&self) -> Vec<vo::item::ItemConfig> {
        self._item_config().values().map(|po| po.vo(self)).collect()
    }

    pub fn item_use_data(&self, id: u32) -> Option<vo::item::ItemUseData> {
        self._item_use_data().get(&id).map(|po| po.vo(self))
    }
}
