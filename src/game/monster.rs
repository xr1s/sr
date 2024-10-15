use crate::po;
use crate::vo;
use crate::PO;

type FnvIndexMap<K, V> = indexmap::IndexMap<K, V, fnv::FnvBuildHasher>;

impl super::GameData {
    fn _monster_template_config(&self) -> &FnvIndexMap<u32, po::monster::MonsterTemplateConfig> {
        self.monster_template_config
            .get_or_init(|| self.load_to_map("ExcelOutput/MonsterTemplateConfig.json"))
    }

    fn _monster_config(&self) -> &FnvIndexMap<u32, po::monster::MonsterConfig> {
        self.monster_config
            .get_or_init(|| self.load_to_map("ExcelOutput/MonsterConfig.json"))
    }

    fn _npc_monster_data(&self) -> &FnvIndexMap<u32, po::monster::NPCMonsterData> {
        self.npc_monster_data
            .get_or_init(|| self.load_to_map("ExcelOutput/NPCMonsterData.json"))
    }

    fn _monster_skill_config(&self) -> &FnvIndexMap<u32, po::monster::MonsterSkillConfig> {
        self.monster_skill_config
            .get_or_init(|| self.load_to_map("ExcelOutput/MonsterSkillConfig.json"))
    }

    fn _monster_camp(&self) -> &FnvIndexMap<u8, po::monster::MonsterCamp> {
        self.monster_camp
            .get_or_init(|| self.load_to_map("ExcelOutput/MonsterCamp.json"))
    }
}

impl super::GameData {
    pub fn monster_config(&self, id: u32) -> Option<vo::monster::MonsterConfig> {
        self._monster_config()
            .get(&id)
            .map(|po| po.vo(self))
    }

    pub fn list_monster_config(&self) -> Vec<vo::monster::MonsterConfig> {
        self._monster_config()
            .values()
            .map(|po| po.vo(self))
            .collect()
    }

    pub fn monster_template_config(&self, id: u32) -> Option<vo::monster::MonsterTemplateConfig> {
        self._monster_template_config()
            .get(&id)
            .map(|po| po.vo(self))
    }

    pub fn list_monster_template_config(&self) -> Vec<vo::monster::MonsterTemplateConfig> {
        self._monster_template_config()
            .values()
            .map(|po| po.vo(self))
            .collect()
    }

    pub fn npc_monster_data(&self, id: u32) -> Option<vo::monster::NPCMonsterData> {
        self._npc_monster_data()
            .get(&id)
            .map(|po| po.vo(self))
    }

    pub fn monster_skill_config(&self, id: u32) -> Option<vo::monster::MonsterSkillConfig> {
        self._monster_skill_config()
            .get(&id)
            .map(|po| po.vo(self))
    }

    pub fn list_monster_skill_config(&self) -> Vec<vo::monster::MonsterSkillConfig> {
        self._monster_skill_config()
            .values()
            .map(|po| po.vo(self))
            .collect()
    }

    pub fn monster_camp(&self, id: u8) -> Option<vo::monster::MonsterCamp> {
        self._monster_camp()
            .get(&id)
            .map(|po| po.vo(self))
    }
}
