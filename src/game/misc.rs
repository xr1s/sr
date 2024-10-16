use crate::po;
use crate::vo;
use crate::PO;

type FnvIndexMap<K, V> = indexmap::IndexMap<K, V, fnv::FnvBuildHasher>;

impl super::GameData {
    fn _extra_effect_config(&self) -> &FnvIndexMap<u32, po::misc::ExtraEffectConfig> {
        self.extra_effect
            .get_or_init(|| self.load_to_map("ExcelOutput/ExtraEffectConfig.json"))
    }
}

impl super::GameData {
    pub fn extra_effect_config(&self, id: u32) -> Option<vo::misc::ExtraEffectConfig> {
        self._extra_effect_config()
            .get(&id)
            .map(|po| po.vo(self))
    }

    pub fn list_extra_effect_config(&self) -> Vec<vo::misc::ExtraEffectConfig> {
        self._extra_effect_config()
            .values()
            .map(|po| po.vo(self))
            .collect()
    }
}
