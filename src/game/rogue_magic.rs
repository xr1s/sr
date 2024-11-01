use crate::{po, vo, FnvIndexMap, GameData, PO};

impl GameData {
    fn _rogue_magic_miracle(&self) -> &FnvIndexMap<u16, po::rogue::RogueMiracle> {
        self._rogue_miracle
            .get_or_init(|| self.load_to_map("ExcelOutput/RogueMagicMiracle.json"))
    }
}

impl GameData {
    pub fn rogue_magic_miracle(&self, id: u16) -> Option<vo::rogue::RogueMiracle> {
        self._rogue_magic_miracle().get(&id).map(|po| po.vo(self))
    }

    pub fn list_rogue_magic_miracle(&self) -> Vec<vo::rogue::RogueMiracle> {
        self._rogue_magic_miracle()
            .values()
            .map(|po| po.vo(self))
            .collect()
    }
}
