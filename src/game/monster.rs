use crate::{po, vo, FnvIndexMap, GameData, ID, PO};

impl GameData {
    fn _monster_template_config(&self) -> &FnvIndexMap<u32, po::monster::MonsterTemplateConfig> {
        self._monster_template_config
            .get_or_init(|| self.load_to_map("ExcelOutput/MonsterTemplateConfig.json"))
    }

    fn _monster_template_config_group(&self) -> &fnv::FnvHashMap<u32, Vec<u32>> {
        self._monster_template_config_group.get_or_init(|| {
            let mut multimap = fnv::FnvHashMap::<u32, Vec<u32>>::default();
            for monster in self._monster_template_config().values() {
                if let Some(group_id) = monster.template_group_id {
                    multimap
                        .entry(group_id.get())
                        .or_default()
                        .push(monster.id());
                }
            }
            multimap
        })
    }

    fn _monster_config(&self) -> &FnvIndexMap<u32, po::monster::MonsterConfig> {
        self._monster_config
            .get_or_init(|| self.load_to_map("ExcelOutput/MonsterConfig.json"))
    }

    fn _npc_monster_data(&self) -> &FnvIndexMap<u32, po::monster::NPCMonsterData> {
        self._npc_monster_data
            .get_or_init(|| self.load_to_map("ExcelOutput/NPCMonsterData.json"))
    }

    fn _monster_skill_config(&self) -> &FnvIndexMap<u32, po::monster::MonsterSkillConfig> {
        self._monster_skill_config
            .get_or_init(|| self.load_to_map("ExcelOutput/MonsterSkillConfig.json"))
    }

    fn _monster_camp(&self) -> &FnvIndexMap<u8, po::monster::MonsterCamp> {
        self._monster_camp
            .get_or_init(|| self.load_to_map("ExcelOutput/MonsterCamp.json"))
    }
}

impl GameData {
    pub fn monster_config(&self, id: u32) -> Option<vo::monster::MonsterConfig> {
        self._monster_config().get(&id).map(|po| po.vo(self))
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

    pub fn monster_template_config_group(
        &self,
        id: u32,
    ) -> impl Iterator<Item = vo::monster::MonsterTemplateConfig> {
        if id == 0 {
            return either::Either::Left(std::iter::empty());
        }
        either::Either::Right(
            self._monster_template_config_group()
                .get(&id)
                .map(Vec::as_slice)
                .unwrap_or_default()
                .iter()
                .map(|&id| self.monster_template_config(id))
                .map(Option::unwrap), // map 的值就是从 monster_template_config 生成的
                                      // 所以这里不会 panic
        )
    }

    pub fn npc_monster_data(&self, id: u32) -> Option<vo::monster::NPCMonsterData> {
        self._npc_monster_data().get(&id).map(|po| po.vo(self))
    }

    pub fn monster_skill_config(&self, id: u32) -> Option<vo::monster::MonsterSkillConfig> {
        self._monster_skill_config().get(&id).map(|po| po.vo(self))
    }

    pub fn list_monster_skill_config(&self) -> Vec<vo::monster::MonsterSkillConfig> {
        self._monster_skill_config()
            .values()
            .map(|po| po.vo(self))
            .collect()
    }

    pub fn monster_camp(&self, id: u8) -> Option<vo::monster::MonsterCamp> {
        self._monster_camp().get(&id).map(|po| po.vo(self))
    }
}
