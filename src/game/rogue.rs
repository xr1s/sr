use crate::po;
use crate::vo;
use crate::PO;

use super::GameData;

type FnvIndexMap<K, V> = indexmap::IndexMap<K, V, fnv::FnvBuildHasher>;

impl GameData {
    fn _rogue_maze_buff(&self) -> &FnvIndexMap<u32, po::rogue::RogueMazeBuff> {
        self.rogue_maze_buff
            .get_or_init(|| self.load_to_map("ExcelOutput/RogueMazeBuff.json"))
    }
}

impl GameData {
    pub fn rogue_maze_buff(&self, id: u32) -> Option<vo::rogue::RogueMazeBuff> {
        self._rogue_maze_buff().get(&id).map(|po| po.vo(self))
    }
}
