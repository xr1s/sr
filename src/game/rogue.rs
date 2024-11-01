use crate::{po, vo, FnvIndexMap, GameData, PO};

impl GameData {
    fn _rogue_maze_buff(&self) -> &FnvIndexMap<u32, po::rogue::RogueMazeBuff> {
        self._rogue_maze_buff
            .get_or_init(|| self.load_to_map("ExcelOutput/RogueMazeBuff.json"))
    }

    fn _rogue_miracle(&self) -> &FnvIndexMap<u16, po::rogue::RogueMiracle> {
        self._rogue_miracle
            .get_or_init(|| self.load_to_map("ExcelOutput/RogueMiracle.json"))
    }

    fn _rogue_miracle_display(&self) -> &FnvIndexMap<u16, po::rogue::RogueMiracleDisplay> {
        self._rogue_miracle_display
            .get_or_init(|| self.load_to_map("ExcelOutput/RogueMiracleDisplay.json"))
    }

    fn _rogue_handbook_miracle(&self) -> &FnvIndexMap<u16, po::rogue::RogueHandbookMiracle> {
        self._rogue_handbook_miracle
            .get_or_init(|| self.load_to_map("ExcelOutput/RogueHandbookMiracle.json"))
    }

    fn _rogue_handbook_miracle_type(
        &self,
    ) -> &FnvIndexMap<u16, po::rogue::RogueHandbookMiracleType> {
        self._rogue_handbook_miracle_type
            .get_or_init(|| self.load_to_map("ExcelOutput/RogueHandbookMiracleType.json"))
    }

    fn _rogue_monster_group(&self) -> &FnvIndexMap<u32, po::rogue::RogueMonsterGroup> {
        self._rogue_monster_group
            .get_or_init(|| self.load_to_map("ExcelOutput/RogueMonsterGroup.json"))
    }

    fn _rogue_monster(&self) -> &FnvIndexMap<u32, po::rogue::RogueMonster> {
        self._rogue_monster
            .get_or_init(|| self.load_to_map("ExcelOutput/RogueMonster.json"))
    }
}

impl GameData {
    pub fn rogue_maze_buff(&self, id: u32) -> Option<vo::rogue::RogueMazeBuff> {
        self._rogue_maze_buff().get(&id).map(|po| po.vo(self))
    }

    pub fn rogue_miracle(&self, id: u16) -> Option<vo::rogue::RogueMiracle> {
        self._rogue_miracle().get(&id).map(|po| po.vo(self))
    }

    pub fn list_rogue_miracle(&self) -> Vec<vo::rogue::RogueMiracle> {
        self._rogue_miracle()
            .values()
            .map(|po| po.vo(self))
            .collect()
    }

    pub fn rogue_miracle_display(&self, id: u16) -> Option<vo::rogue::RogueMiracleDisplay> {
        self._rogue_miracle_display().get(&id).map(|po| po.vo(self))
    }

    pub fn rogue_handbook_miracle(&self, id: u16) -> Option<vo::rogue::RogueHandbookMiracle> {
        self._rogue_handbook_miracle()
            .get(&id)
            .map(|po| po.vo(self))
    }

    pub fn rogue_handbook_miracle_type(
        &self,
        id: u16,
    ) -> Option<vo::rogue::RogueHandbookMiracleType> {
        self._rogue_handbook_miracle_type()
            .get(&id)
            .map(|po| po.vo(self))
    }

    pub fn rogue_monster_group(&self, id: u32) -> Option<vo::rogue::RogueMonsterGroup> {
        self._rogue_monster_group().get(&id).map(|po| po.vo(self))
    }

    pub fn rogue_monster(&self, id: u32) -> Option<vo::rogue::RogueMonster> {
        self._rogue_monster().get(&id).map(|po| po.vo(self))
    }
}
