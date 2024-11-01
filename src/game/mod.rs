mod item;
mod misc;
mod monster;
mod rogue;
mod rogue_magic;
mod rogue_tourn;

use crate::{po, FnvIndexMap};

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::OnceLock;

pub struct GameData {
    base: PathBuf,
    text_map: std::collections::HashMap<i32, String, fnv::FnvBuildHasher>,

    // item
    _item_config: OnceLock<FnvIndexMap<u32, po::item::ItemConfig>>,
    _item_use_data: OnceLock<FnvIndexMap<u32, po::item::ItemUseData>>,

    // misc
    _extra_effect: OnceLock<FnvIndexMap<u32, po::misc::ExtraEffectConfig>>,
    _reward_data: OnceLock<FnvIndexMap<u32, po::misc::RewardData>>,

    // monster
    _monster_template_config: OnceLock<FnvIndexMap<u32, po::monster::MonsterTemplateConfig>>,
    /// 因为存在自引用, 所以只好储存 group_id 到 id 的映射;
    /// 考虑到以后可能还会做名称到对象的映射，未来可能会全部重构到 Arc 中.
    /// TODO: Rust 似乎没有多索引映射表, 要不要考虑自己实现一个?
    _monster_template_config_group: OnceLock<fnv::FnvHashMap<u32, Vec<u32>>>,
    _monster_config: OnceLock<FnvIndexMap<u32, po::monster::MonsterConfig>>,
    _npc_monster_data: OnceLock<FnvIndexMap<u32, po::monster::NPCMonsterData>>,
    _monster_skill_config: OnceLock<FnvIndexMap<u32, po::monster::MonsterSkillConfig>>,
    _monster_camp: OnceLock<FnvIndexMap<u8, po::monster::MonsterCamp>>,

    // rogue
    /// 模拟宇宙祝福
    _rogue_maze_buff: OnceLock<FnvIndexMap<u32, po::rogue::RogueMazeBuff>>,
    /// 模拟宇宙奇物
    _rogue_miracle: OnceLock<FnvIndexMap<u16, po::rogue::RogueMiracle>>,
    _rogue_miracle_display: OnceLock<FnvIndexMap<u16, po::rogue::RogueMiracleDisplay>>,
    _rogue_handbook_miracle: OnceLock<FnvIndexMap<u16, po::rogue::RogueHandbookMiracle>>,
    _rogue_handbook_miracle_type: OnceLock<FnvIndexMap<u16, po::rogue::RogueHandbookMiracleType>>,

    // rogue_magic
    /// 不可知域
    _rogue_magic_miracle: OnceLock<FnvIndexMap<u16, po::rogue::RogueMiracle>>,

    // rogue_tourn
    _rogue_tourn_content_display:
        OnceLock<FnvIndexMap<u16, po::rogue_tourn::RogueTournContentDisplay>>,
    /// 差分宇宙周期演算
    _rogue_tourn_weekly_challenge: OnceLock<Vec<po::rogue_tourn::RogueTournWeeklyChallenge>>,
    _rogue_tourn_weekly_display:
        OnceLock<FnvIndexMap<u16, po::rogue_tourn::RogueTournWeeklyDisplay>>,
    /// 差分宇宙奇物
    _rogue_tourn_miracle: OnceLock<FnvIndexMap<u16, po::rogue_tourn::RogueTournMiracle>>,
    _rogue_tourn_miracle_display: OnceLock<FnvIndexMap<u16, po::rogue::RogueMiracleDisplay>>,
    _rogue_tourn_handbook_miracle:
        OnceLock<FnvIndexMap<u16, po::rogue_tourn::RogueTournHandbookMiracle>>,
    /// 差分宇宙方程
    _rogue_tourn_formula: OnceLock<FnvIndexMap<u32, po::rogue_tourn::RogueTournFormula>>,
    _rogue_tourn_formula_display:
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
            _extra_effect: OnceLock::new(),
            _reward_data: OnceLock::new(),
            // item
            _item_config: OnceLock::new(),
            _item_use_data: OnceLock::new(),
            // monster
            _monster_template_config: OnceLock::new(),
            _monster_template_config_group: OnceLock::new(),
            _monster_config: OnceLock::new(),
            _npc_monster_data: OnceLock::new(),
            _monster_skill_config: OnceLock::new(),
            _monster_camp: OnceLock::new(),
            // rogue
            _rogue_maze_buff: OnceLock::new(),
            _rogue_miracle: OnceLock::new(),
            _rogue_miracle_display: OnceLock::new(),
            _rogue_handbook_miracle: OnceLock::new(),
            _rogue_handbook_miracle_type: OnceLock::new(),
            // rogue_magic_miracle
            _rogue_magic_miracle: OnceLock::new(),
            // rogue_tourn
            _rogue_tourn_content_display: OnceLock::new(),
            _rogue_tourn_weekly_challenge: OnceLock::new(),
            _rogue_tourn_weekly_display: OnceLock::new(),
            _rogue_tourn_miracle: OnceLock::new(),
            _rogue_tourn_miracle_display: OnceLock::new(),
            _rogue_tourn_handbook_miracle: OnceLock::new(),
            _rogue_tourn_formula: OnceLock::new(),
            _rogue_tourn_formula_display: OnceLock::new(),
        }
    }

    pub(crate) fn text(&self, text: &po::Text) -> &str {
        self.text_map
            .get(&text.hash)
            .map(String::as_str)
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
