use crate::{po, vo, FnvIndexMap, ID, PO};

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::OnceLock;

pub struct GameData {
    base: PathBuf,
    text_map: std::collections::HashMap<i32, String, fnv::FnvBuildHasher>,

    // item
    /// 道具
    _item_config: OnceLock<FnvIndexMap<u32, po::item::ItemConfig>>,
    /// 道具使用效果
    _item_use_data: OnceLock<FnvIndexMap<u32, po::item::ItemUseData>>,

    // misc
    /// 效果说明，比如模拟宇宙中
    _extra_effect_config: OnceLock<FnvIndexMap<u32, po::misc::ExtraEffectConfig>>,
    _reward_data: OnceLock<FnvIndexMap<u32, po::misc::RewardData>>,

    // monster
    _monster_camp: OnceLock<FnvIndexMap<u8, po::monster::MonsterCamp>>,
    _monster_config: OnceLock<FnvIndexMap<u32, po::monster::MonsterConfig>>,
    _monster_skill_config: OnceLock<FnvIndexMap<u32, po::monster::MonsterSkillConfig>>,
    _monster_template_config: OnceLock<FnvIndexMap<u32, po::monster::MonsterTemplateConfig>>,
    /// 因为存在自引用, 所以只好储存 group_id 到 id 的映射;
    _monster_template_config_group: OnceLock<fnv::FnvHashMap<u32, Vec<u32>>>,
    _npc_monster_data: OnceLock<FnvIndexMap<u32, po::monster::NPCMonsterData>>,

    // rogue
    // 模拟宇宙
    _rogue_handbook_miracle: OnceLock<FnvIndexMap<u16, po::rogue::RogueHandbookMiracle>>,
    _rogue_handbook_miracle_type: OnceLock<FnvIndexMap<u16, po::rogue::RogueHandbookMiracleType>>,
    /// 模拟宇宙祝福
    _rogue_maze_buff: OnceLock<FnvIndexMap<u32, po::rogue::RogueMazeBuff>>,
    /// 模拟宇宙奇物
    _rogue_miracle: OnceLock<FnvIndexMap<u16, po::rogue::RogueMiracle>>,
    _rogue_miracle_display: OnceLock<FnvIndexMap<u16, po::rogue::RogueMiracleDisplay>>,
    _rogue_monster: OnceLock<FnvIndexMap<u32, po::rogue::RogueMonster>>,
    _rogue_monster_group: OnceLock<FnvIndexMap<u32, po::rogue::RogueMonsterGroup>>,

    // rogue magic
    // 模拟宇宙：不可知域
    /// 不可知域奇物
    _rogue_magic_miracle: OnceLock<FnvIndexMap<u16, po::rogue::RogueMiracle>>,

    // rogue tourn 差分宇宙
    /// 差分宇宙文案
    _rogue_tourn_content_display:
        OnceLock<FnvIndexMap<u16, po::rogue::tourn::RogueTournContentDisplay>>,
    /// 差分宇宙方程
    _rogue_tourn_formula: OnceLock<FnvIndexMap<u32, po::rogue::tourn::RogueTournFormula>>,
    _rogue_tourn_formula_display:
        OnceLock<FnvIndexMap<u32, po::rogue::tourn::RogueTournFormulaDisplay>>,
    _rogue_tourn_handbook_miracle:
        OnceLock<FnvIndexMap<u16, po::rogue::tourn::RogueTournHandbookMiracle>>,
    /// 差分宇宙奇物
    _rogue_tourn_miracle: OnceLock<FnvIndexMap<u16, po::rogue::tourn::RogueTournMiracle>>,
    _rogue_tourn_miracle_display: OnceLock<FnvIndexMap<u16, po::rogue::RogueMiracleDisplay>>,
    /// 差分宇宙周期演算
    _rogue_tourn_weekly_challenge:
        OnceLock<FnvIndexMap<u8, po::rogue::tourn::RogueTournWeeklyChallenge>>,
    _rogue_tourn_weekly_display:
        OnceLock<FnvIndexMap<u16, po::rogue::tourn::RogueTournWeeklyDisplay>>,
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
            _extra_effect_config: OnceLock::new(),
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
            _rogue_monster_group: OnceLock::new(),
            _rogue_monster: OnceLock::new(),
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

    pub(crate) fn text(&self, text: po::Text) -> &str {
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
        let file = File::open(dbg!(&path)).unwrap();
        let reader = BufReader::new(file);
        let po: Vec<V> = serde_json::from_reader(reader).unwrap();
        po.into_iter().map(|po| (po.id(), po)).collect()
    }
}

impl GameData {
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
}

macro_rules! impl_field {
    // 入口
    ($field:ident, $typ:ty) => {
        paste::paste! {
            impl_field!($field, $typ, stringify!([<$field:camel>]));
        }
    };

    // 出口
    ($field:ident, $typ:ty, $json:expr) => {
        paste::paste! {
            fn [<_$field>](&self) -> &FnvIndexMap<<po::$typ as ID>::ID, po::$typ> {
                self.[<_ $field>].get_or_init(|| {
                    self.load_to_map(concat!("ExcelOutput/", $json, ".json"))
                })
            }
            #[allow(private_interfaces)]
            pub fn [<$field>](&self, id: <po::$typ as ID>::ID) -> Option<vo::$typ> {
                self.[<_$field>]().get(&id).map(|po| po.vo(self))
            }
            pub fn [<list_$field>](&self) -> Vec<vo::$typ> {
                self.[<_$field>]().values().map(|po| po.vo(self)).collect()
            }
        }
    };
}

#[rustfmt::skip]
impl GameData {
    // 宏效果示例
    /* fn _item_config(&self) -> &FnvIndexMap<u32, po::item::ItemConfig> {
     *      self._item_config.get_or_init(|| {
     *          self.load_to_map(concat!("ExcelOutput/ItemConfig.json"))
     *      })
     *  }
     *  
     *  pub fn item_config(&self, id: u32) -> Option<vo::item::ItemConfig> {
     *      self._item_config().get(&id).map(|po| po.vo(self))
     *  }
     *
     *  pub fn list_item_config(&self) -> Vec<vo::item::ItemConfig> {
     *      self._item_config().values().map(|po| po.vo(self)).collect()
     *  }
     */

    // item
    impl_field!(item_config, item::ItemConfig);
    impl_field!(item_use_data, item::ItemUseData);
    // misc
    impl_field!(extra_effect_config, misc::ExtraEffectConfig);
    impl_field!(reward_data, misc::RewardData);
    // monster
    impl_field!(monster_camp, monster::MonsterCamp);
    impl_field!(monster_config, monster::MonsterConfig);
    impl_field!(monster_skill_config, monster::MonsterSkillConfig);
    impl_field!(monster_template_config, monster::MonsterTemplateConfig);
    impl_field!(npc_monster_data, monster::NPCMonsterData, "NPCMonsterData");
    // rogue
    impl_field!(rogue_handbook_miracle, rogue::RogueHandbookMiracle);
    impl_field!(rogue_handbook_miracle_type, rogue::RogueHandbookMiracleType);
    impl_field!(rogue_maze_buff, rogue::RogueMazeBuff);
    impl_field!(rogue_miracle, rogue::RogueMiracle);
    impl_field!(rogue_miracle_display, rogue::RogueMiracleDisplay);
    impl_field!(rogue_monster, rogue::RogueMonster);
    impl_field!(rogue_monster_group, rogue::RogueMonsterGroup);
    // rogue magic
    impl_field!(rogue_magic_miracle, rogue::RogueMiracle);
    // rogue tourn
    impl_field!(rogue_tourn_content_display, rogue::tourn::RogueTournContentDisplay);
    impl_field!(rogue_tourn_formula, rogue::tourn::RogueTournFormula);
    impl_field!(rogue_tourn_formula_display, rogue::tourn::RogueTournFormulaDisplay);
    impl_field!(rogue_tourn_handbook_miracle, rogue::tourn::RogueTournHandbookMiracle);
    impl_field!(rogue_tourn_miracle, rogue::tourn::RogueTournMiracle);
    impl_field!(rogue_tourn_miracle_display, rogue::RogueMiracleDisplay);
    impl_field!(rogue_tourn_weekly_challenge, rogue::tourn::RogueTournWeeklyChallenge);
    impl_field!(rogue_tourn_weekly_display, rogue::tourn::RogueTournWeeklyDisplay);
}
