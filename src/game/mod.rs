mod misc;
mod monster;
mod rogue;
mod rogue_tourn;

use crate::po;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::OnceLock;

type FnvIndexMap<K, V> = indexmap::IndexMap<K, V, fnv::FnvBuildHasher>;

pub struct GameData {
    base: PathBuf,
    text_map: std::collections::HashMap<i32, String, fnv::FnvBuildHasher>,

    // misc
    extra_effect: OnceLock<FnvIndexMap<u32, po::misc::ExtraEffectConfig>>,

    // monster
    monster_template_config: OnceLock<FnvIndexMap<u32, po::monster::MonsterTemplateConfig>>,
    monster_config: OnceLock<FnvIndexMap<u32, po::monster::MonsterConfig>>,
    npc_monster_data: OnceLock<FnvIndexMap<u32, po::monster::NPCMonsterData>>,
    monster_skill_config: OnceLock<FnvIndexMap<u32, po::monster::MonsterSkillConfig>>,
    monster_camp: OnceLock<FnvIndexMap<u8, po::monster::MonsterCamp>>,

    // rogue
    /// 模拟宇宙祝福
    rogue_maze_buff: OnceLock<FnvIndexMap<u32, po::rogue::RogueMazeBuff>>,

    // rogue_tourn
    rogue_tourn_content_display:
        OnceLock<FnvIndexMap<u16, po::rogue_tourn::RogueTournContentDisplay>>,
    /// 差分宇宙周期演算
    rogue_tourn_weekly_challenge: OnceLock<Vec<po::rogue_tourn::RogueTournWeeklyChallenge>>,
    rogue_tourn_weekly_display:
        OnceLock<FnvIndexMap<u16, po::rogue_tourn::RogueTournWeeklyDisplay>>,
    /// 差分宇宙奇物
    rogue_tourn_miracle: OnceLock<FnvIndexMap<u16, po::rogue_tourn::RogueTournMiracle>>,
    rogue_tourn_miracle_display:
        OnceLock<FnvIndexMap<u16, po::rogue_tourn::RogueTournMiracleDisplay>>,
    rogue_tourn_handbook_miracle:
        OnceLock<FnvIndexMap<u16, po::rogue_tourn::RogueTournHandbookMiracle>>,
    /// 差分宇宙方程
    rogue_tourn_formula: OnceLock<FnvIndexMap<u32, po::rogue_tourn::RogueTournFormula>>,
    rogue_tourn_formula_display:
        OnceLock<FnvIndexMap<u32, po::rogue_tourn::RogueTournFormulaDisplay>>,
}

impl GameData {
    pub fn new(base: impl Into<PathBuf>) -> Self {
        let base = base.into();
        let text_map_reader =
            BufReader::new(File::open(base.join("TextMap/TextMapCHS.json")).unwrap());
        GameData {
            base,
            text_map: serde_json::from_reader(text_map_reader).unwrap(),
            // misc
            extra_effect: OnceLock::new(),
            // monster
            monster_template_config: OnceLock::new(),
            monster_config: OnceLock::new(),
            npc_monster_data: OnceLock::new(),
            monster_skill_config: OnceLock::new(),
            monster_camp: OnceLock::new(),
            // rogue
            rogue_maze_buff: OnceLock::new(),
            // rogue_tourn
            rogue_tourn_content_display: OnceLock::new(),
            rogue_tourn_weekly_challenge: OnceLock::new(),
            rogue_tourn_weekly_display: OnceLock::new(),
            rogue_tourn_miracle: OnceLock::new(),
            rogue_tourn_miracle_display: OnceLock::new(),
            rogue_tourn_handbook_miracle: OnceLock::new(),
            rogue_tourn_formula: OnceLock::new(),
            rogue_tourn_formula_display: OnceLock::new(),
        }
    }

    pub(crate) fn text(&self, text: &po::Text) -> &str {
        self.text_map
            .get(&text.hash)
            .map(|s| &**s)
            .unwrap_or_default()
    }

    fn load_to_map<K, V>(&self, dir: impl Into<std::path::PathBuf>) -> FnvIndexMap<K, V>
    where
        K: std::cmp::Eq + std::hash::Hash,
        for<'a> V: serde::Deserialize<'a> + crate::ID<ID = K>,
    {
        let path = self.base.join(dir.into());
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let po: Vec<V> = serde_json::from_reader(reader).unwrap();
        po.into_iter().map(|po| (po.id(), po)).collect()
    }
}
