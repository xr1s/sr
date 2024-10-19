use crate::{po, vo, FnvIndexMap, GameData, PO};

impl GameData {
    fn _rogue_tourn_content_display(
        &self,
    ) -> &FnvIndexMap<u16, po::rogue_tourn::RogueTournContentDisplay> {
        self._rogue_tourn_content_display
            .get_or_init(|| self.load_to_map("ExcelOutput/RogueTournContentDisplay.json"))
    }

    fn _rogue_tourn_weekly_challenge(&self) -> &[po::rogue_tourn::RogueTournWeeklyChallenge] {
        self._rogue_tourn_weekly_challenge.get_or_init(|| {
            let path = self.base.join("ExcelOutput/RogueTournWeeklyChallenge.json");
            let file = std::fs::File::open(path).unwrap();
            let reader = std::io::BufReader::new(file);
            serde_json::from_reader(reader).unwrap()
        })
    }

    fn _rogue_tourn_weekly_display(
        &self,
    ) -> &FnvIndexMap<u16, po::rogue_tourn::RogueTournWeeklyDisplay> {
        self._rogue_tourn_weekly_display
            .get_or_init(|| self.load_to_map("ExcelOutput/RogueTournWeeklyDisplay.json"))
    }

    fn _rogue_tourn_miracle(&self) -> &FnvIndexMap<u16, po::rogue_tourn::RogueTournMiracle> {
        self._rogue_tourn_miracle
            .get_or_init(|| self.load_to_map("ExcelOutput/RogueTournMiracle.json"))
    }

    fn _rogue_tourn_handbook_miracle(
        &self,
    ) -> &FnvIndexMap<u16, po::rogue_tourn::RogueTournHandbookMiracle> {
        self._rogue_tourn_handbook_miracle
            .get_or_init(|| self.load_to_map("ExcelOutput/RogueTournHandbookMiracle.json"))
    }

    fn _rogue_tourn_miracle_display(
        &self,
    ) -> &FnvIndexMap<u16, po::rogue_tourn::RogueTournMiracleDisplay> {
        self._rogue_tourn_miracle_display
            .get_or_init(|| self.load_to_map("ExcelOutput/RogueTournMiracleDisplay.json"))
    }

    fn _rogue_tourn_formula(&self) -> &FnvIndexMap<u32, po::rogue_tourn::RogueTournFormula> {
        self._rogue_tourn_formula
            .get_or_init(|| self.load_to_map("ExcelOutput/RogueTournFormula.json"))
    }

    fn _rogue_tourn_formula_display(
        &self,
    ) -> &FnvIndexMap<u32, po::rogue_tourn::RogueTournFormulaDisplay> {
        self._rogue_tourn_formula_display
            .get_or_init(|| self.load_to_map("ExcelOutput/RogueTournFormulaDisplay.json"))
    }
}

impl GameData {
    pub fn rogue_tourn_weekly_challenge(
        &self,
        id: u16,
    ) -> Option<vo::rogue_tourn::RogueTournWeeklyChallenge> {
        self._rogue_tourn_weekly_challenge()
            .get(id as usize - 1)
            .map(|po| po.vo(self))
    }

    pub fn list_rogue_tourn_weekly_challenge(
        &self,
    ) -> Vec<vo::rogue_tourn::RogueTournWeeklyChallenge> {
        self._rogue_tourn_weekly_challenge()
            .iter()
            .map(|po| po.vo(self))
            .collect()
    }

    pub fn rogue_tourn_weekly_display(
        &self,
        id: u16,
    ) -> Option<vo::rogue_tourn::RogueTournWeeklyDisplay> {
        self._rogue_tourn_weekly_display()
            .get(&id)
            .map(|po| po.vo(self))
    }

    pub fn rogue_tourn_miracle_display(
        &self,
        id: u16,
    ) -> Option<vo::rogue_tourn::RogueTournMiracleDisplay> {
        self._rogue_tourn_miracle_display()
            .get(&id)
            .map(|po| po.vo(self))
    }

    pub fn rogue_tourn_miracle(&self, id: u16) -> Option<vo::rogue_tourn::RogueTournMiracle> {
        self._rogue_tourn_miracle().get(&id).map(|po| po.vo(self))
    }

    pub fn list_rogue_tourn_miracle(&self) -> Vec<vo::rogue_tourn::RogueTournMiracle> {
        self._rogue_tourn_miracle()
            .values()
            .map(|po| po.vo(self))
            .collect()
    }

    pub fn rogue_tourn_handbook_miracle(
        &self,
        id: u16,
    ) -> Option<vo::rogue_tourn::RogueTournHandbookMiracle> {
        self._rogue_tourn_handbook_miracle()
            .get(&id)
            .map(|po| po.vo(self))
    }

    pub fn rogue_tourn_content_display(
        &self,
        id: u16,
    ) -> Option<vo::rogue_tourn::RogueTournContentDisplay> {
        self._rogue_tourn_content_display()
            .get(&id)
            .map(|po| po.vo(self))
    }

    pub fn rogue_tourn_formula(&self, id: u32) -> Option<vo::rogue_tourn::RogueTournFormula> {
        self._rogue_tourn_formula().get(&id).map(|po| po.vo(self))
    }

    pub fn list_rogue_tourn_formula(&self) -> Vec<vo::rogue_tourn::RogueTournFormula> {
        self._rogue_tourn_formula()
            .values()
            .map(|po| po.vo(self))
            .collect()
    }

    pub fn rogue_tourn_formula_display(
        &self,
        id: u32,
    ) -> Option<vo::rogue_tourn::RogueTournFormulaDisplay> {
        self._rogue_tourn_formula_display()
            .get(&id)
            .map(|po| po.vo(self))
    }
}
