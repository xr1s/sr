use crate::{po, vo, FnvIndexMap, GameData, PO};

impl GameData {
    fn _extra_effect_config(&self) -> &FnvIndexMap<u32, po::misc::ExtraEffectConfig> {
        self._extra_effect
            .get_or_init(|| self.load_to_map("ExcelOutput/ExtraEffectConfig.json"))
    }

    fn _reward_data(&self) -> &FnvIndexMap<u32, po::misc::RewardData> {
        self._reward_data
            .get_or_init(|| self.load_to_map("ExcelOutput/RewardData.json"))
    }
}

impl GameData {
    pub fn extra_effect_config(&self, id: u32) -> Option<vo::misc::ExtraEffectConfig> {
        self._extra_effect_config().get(&id).map(|po| po.vo(self))
    }

    pub fn list_extra_effect_config(&self) -> Vec<vo::misc::ExtraEffectConfig> {
        self._extra_effect_config()
            .values()
            .map(|po| po.vo(self))
            .collect()
    }

    pub fn reward_data(&self, id: u32) -> Option<vo::misc::RewardData> {
        self._reward_data().get(&id).map(|po| po.vo(self))
    }

    pub fn list_reward_data(&self) -> Vec<vo::misc::RewardData> {
        self._reward_data().values().map(|po| po.vo(self)).collect()
    }
}
