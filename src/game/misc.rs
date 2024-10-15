use crate::po;
use crate::vo;
use crate::PO;

type FnvIndexMap<K, V> = indexmap::IndexMap<K, V, fnv::FnvBuildHasher>;

impl super::GameData {
    fn _extra_effect(&self) -> &FnvIndexMap<u32, po::misc::ExtraEffect> {
        self.extra_effect
            .get_or_init(|| self.load_to_map("ExcelOutput/ExtraEffectConfig.json"))
    }
}

impl super::GameData {
    pub fn extra_effect(&self, id: u32) -> Option<vo::misc::ExtraEffect> {
        self._extra_effect()
            .get(&id)
            .map(|po| po.vo(self))
    }

    pub fn list_extra_effect(&self) -> Vec<vo::misc::ExtraEffect> {
        self._extra_effect()
            .values()
            .map(|po| po.vo(self))
            .collect()
    }
}
