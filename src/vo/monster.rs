use crate::po::monster::{
    AbilityName, CampType, CharacterType, CustomValueTag, DebuffResistKey,
    MonsterConfigCustomValueKey, Rank, SkillTriggerKey, StanceType, SubType,
};
use crate::po::Element;
use std::borrow::Cow;

type FnvIndexMap<K, V> = indexmap::IndexMap<K, V, fnv::FnvBuildHasher>;

#[derive(Clone, Debug)]
pub struct MonsterTemplateConfig<'a> {
    pub id: u32,
    pub name: &'a str,
    pub camp_name: &'a str,
    pub rank: Rank,
    pub attack_base: u16,
    pub defence_base: u16,
    pub hp_base: f32,
    pub speed_base: u16,
    pub stance_base: u16,
    pub critical_damage_base: f32,
    pub status_resistance_base: f32,
    pub minimum_fatigue_ratio: f32,
    pub stance_count: u8,
    pub initial_delay_ratio: f32,
    pub stance_type: Option<StanceType>,
    pub npc_monster_list: Vec<NPCMonsterData<'a>>,
}

#[derive(Clone, Debug)]
pub struct NPCMonsterData<'a> {
    pub id: u32,
    pub name: &'a str,
    pub title: &'a str,
    pub character_type: CharacterType,
    pub sub_type: SubType,
    pub rank: Rank,
}

#[derive(derivative::Derivative)]
#[derivative(Clone, Debug)]
pub struct MonsterConfig<'a> {
    #[derivative(Debug = "ignore")]
    pub(crate) game: &'a crate::GameData,
    pub id: u32,
    pub template: MonsterTemplateConfig<'a>,
    pub name: &'a str,
    pub introduction: &'a str,
    pub battle_introduction: &'a str,
    pub attack_modify_ratio: f32,
    pub defence_modify_ratio: f32,
    pub hp_modify_ratio: f32,
    pub speed_modify_value: i16,
    pub stance_modify_value: i16,
    pub skill_list: Vec<MonsterSkillConfig<'a>>,
    pub custom_values: FnvIndexMap<MonsterConfigCustomValueKey, i32>,
    pub debuff_resist: FnvIndexMap<DebuffResistKey, f32>,
    pub custom_value_tags: &'a [CustomValueTag],
    pub stance_weak_list: &'a [Element],
    pub damage_type_resistance: fnv::FnvHashMap<Element, f32>,
    pub ability_name_list: &'a [AbilityName],
    pub override_ai_skill_sequence: Vec<MonsterSkillConfig<'a>>,
}

impl<'a> MonsterConfig<'a> {
    pub fn phase(&self) -> u8 {
        self.skill_list
            .iter()
            .flat_map(|skill| skill.phase_list)
            .copied()
            .max()
            .unwrap_or(1)
    }

    pub fn phase_skill(&'a self, phase: u8) -> Vec<&'a MonsterSkillConfig<'a>> {
        let mut skills = self
            .skill_list
            .iter()
            .filter(|skill| skill.phase_list.contains(&phase))
            .collect::<Vec<_>>();
        skills.sort_by(|lhs, rhs| match (lhs.is_threat, rhs.is_threat) {
            (true, true) => std::cmp::Ordering::Equal,
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            (false, false) => std::cmp::Ordering::Equal,
        });
        skills
    }

    pub fn speed(&self) -> u16 {
        self.template.speed_base + self.speed_modify_value as u16
    }

    pub fn stance(&self) -> u16 {
        (self.template.stance_base + self.stance_modify_value as u16) / 3
    }

    pub fn damage_types(&self) -> Vec<Element> {
        self.skill_list
            .iter()
            .filter_map(|skill| skill.damage_type)
            .collect::<fnv::FnvHashSet<_>>()
            .drain()
            .collect()
    }

    pub fn summons(&self) -> Vec<MonsterConfig> {
        const NON_SUMMON_CUSTOM_VALUE_KEYS: [MonsterConfigCustomValueKey; 10] = [
            MonsterConfigCustomValueKey::CocoliaChangePhaseInsertController,
            MonsterConfigCustomValueKey::FlopSide,
            MonsterConfigCustomValueKey::HardLevel,
            MonsterConfigCustomValueKey::IsWeeklyBoss,
            MonsterConfigCustomValueKey::MonsterAMLElite0100AICounter01,
            MonsterConfigCustomValueKey::MonsterCount,
            MonsterConfigCustomValueKey::MonsterRO015SummonID,
            MonsterConfigCustomValueKey::MonsterXPElite0201AIFlag,
            MonsterConfigCustomValueKey::TV01RandomPoolID,
            MonsterConfigCustomValueKey::TV01EliteChance,
        ];
        self.custom_values
            .iter()
            .filter(|(&key, _)| !NON_SUMMON_CUSTOM_VALUE_KEYS.contains(&key))
            .filter_map(|(_, &id)| self.game.monster_config(id as u32))
            .collect()
    }
}

impl<'a> crate::Wiki for MonsterConfig<'a> {
    fn wiki(&self) -> Cow<'static, str> {
        let mut wiki = String::new();
        // 名称
        wiki.push_str("{{敌人\n|名称=");
        wiki.push_str(self.name);
        wiki.push_str("\n|实装版本=");
        wiki.push_str("\n|系列=");
        // 分类（阵营）
        wiki.push_str("\n|分类=");
        wiki.push_str(self.template.camp_name);
        wiki.push_str("<!-- 选填：反物质军团、裂界造物、雅利洛-Ⅵ、仙舟「罗浮」、虫群、星际和平公司、惊梦剧团、忆域迷因、模拟宇宙、星核猎手、银河 -->");
        // 类型（周本Boss、剧情Boss等，这里没法获取全部，需要手动处理）
        let typ = match self.template.rank {
            Rank::BigBoss => "周本BOSS",
            Rank::Elite => "强敌",
            Rank::LittleBoss => "剧情BOSS",
            Rank::Minion => "普通",
            Rank::MinionLv2 => "普通",
        };
        wiki.push_str("\n|类型=");
        wiki.push_str(typ);
        wiki.push_str(
            "<!-- 选填：普通、强敌、剧情BOSS、周本BOSS、模拟宇宙精英、模拟宇宙首领、召唤物 -->",
        );
        // 介绍
        wiki.push_str("\n|介绍=");
        wiki.push_str(&self.introduction.replace("\\n", "<br />"));
        // 别称
        wiki.push_str("\n|别称=");
        // 弱点
        let weaknesses = self
            .stance_weak_list
            .iter()
            .map(Element::wiki)
            .intersperse(Cow::Borrowed("、"))
            .collect::<String>();
        wiki.push_str("\n|弱点=");
        wiki.push_str(&weaknesses);
        // 攻击属性
        let damage_types = self
            .damage_types()
            .iter()
            .map(Element::wiki)
            .intersperse(Cow::Borrowed("、"))
            .collect::<String>();
        wiki.push_str("\n|攻击属性=");
        wiki.push_str(&damage_types);
        wiki.push_str("\n|出现地点=");
        wiki.push_str("\n|掉落系列素材=");
        wiki.push_str("\n|掉落素材=");
        wiki.push_str("\n|掉落期望=");
        wiki.push_str("\n|TAG=");
        wiki.push_str("\n|韧性=");
        wiki.push_str(&self.stance().to_string());
        // 召唤物
        wiki.push_str("\n|召唤物=");
        let summon_names = self
            .summons()
            .into_iter()
            .map(|monster| monster.name)
            .intersperse("、")
            .collect::<String>();
        wiki.push_str(&summon_names);
        wiki.push_str("\n|血量档位=");
        wiki.push_str("\n|血量比例总=");
        wiki.push_str("\n|血量1名字=");
        wiki.push_str("\n|血量1比例=");
        wiki.push_str("\n|血量2名字=");
        wiki.push_str("\n|血量2比例=");
        wiki.push_str("\n|血量3名字=");
        wiki.push_str("\n|血量3比例=");
        wiki.push_str("\n|血量4名字=");
        wiki.push_str("\n|血量4比例=");
        wiki.push_str("\n|血量5名字=");
        wiki.push_str("\n|血量5比例=");
        // 属性抗性
        const RESISTANCE_ELEMENT: [(Element, &str); 7] = [
            (Element::Physical, "物"),
            (Element::Fire, "火"),
            (Element::Ice, "冰"),
            (Element::Thunder, "雷"),
            (Element::Wind, "风"),
            (Element::Imaginary, "虚数"),
            (Element::Quantum, "量子"),
        ];
        for (element, name) in RESISTANCE_ELEMENT {
            wiki.push_str("\n|");
            wiki.push_str(name);
            wiki.push_str("抗=");
            let v = self
                .damage_type_resistance
                .get(&element)
                .copied()
                .unwrap_or_default()
                * 100.;
            wiki.push_str(&(v as u32).to_string());
            wiki.push('%');
        }
        wiki.push_str("\n|抗性备注=");
        wiki.push_str("\n|属性抗性=");
        let element_resistance = self
            .damage_type_resistance
            .iter()
            .filter(|(_, &resistance)| resistance > 0.2)
            .map(|(element, _)| format!("{}属性抗性", element.wiki()))
            .intersperse("、".to_string())
            .collect::<String>();
        wiki.push_str(&element_resistance);
        // 状态抵抗
        wiki.push_str("\n|状态抵抗=");
        let resists = self
            .debuff_resist
            .iter()
            .map(|(debuff, _)| format!("{}抵抗", debuff.wiki()))
            .intersperse("、".to_string())
            .collect::<String>();
        wiki.push_str(&resists);
        // 技能
        let phase_count = self.phase();
        wiki.push_str("\n|阶段数=");
        wiki.push_str(&phase_count.to_string());
        // 减少重复代码
        fn phase_text(wiki: &mut String, phase: u8) {
            wiki.push_str("\n|");
            if phase != 1 {
                wiki.push_str("阶段");
                wiki.push_str(&phase.to_string());
            }
        }
        for phase in 1..=phase_count {
            let skills = self.phase_skill(phase);
            // 技能数
            phase_text(&mut wiki, phase);
            wiki.push_str("技能数=");
            wiki.push_str(&skills.len().to_string());
            // 大招
            let threat_count = skills.iter().filter(|skill| skill.is_threat).count();
            if threat_count != 0 {
                phase_text(&mut wiki, phase);
                wiki.push_str("大招=");
                wiki.push_str(&threat_count.to_string());
            }

            for (index, skill) in skills.iter().enumerate() {
                let sp_hit = if skill.sp_hit_base == 0 {
                    String::new()
                } else {
                    skill.sp_hit_base.to_string()
                };
                let skill_kvs: [(&'static str, &str, &'static str); 5] = [
                    ("名称", skill.name, ""),
                    ("TAG", skill.tag, ""),
                    ("能量", &sp_hit, ""),
                    ("GIF", "", ""),
                    ("描述", &skill.desc.replace("\\n", "<br />"), ""),
                ];
                let index_str = (index + 1).to_string();
                for (key, val, comment) in skill_kvs {
                    phase_text(&mut wiki, phase);
                    wiki.push_str("技能");
                    wiki.push_str(&index_str);
                    wiki.push_str(key);
                    wiki.push('=');
                    wiki.push_str(val);
                    wiki.push_str(comment);
                }
            }
        }

        wiki.push_str("\n}}");
        std::borrow::Cow::Owned(wiki)
    }
}

#[derive(Clone, Debug)]
pub struct MonsterSkillConfig<'a> {
    pub name: &'a str,
    pub desc: String,
    pub type_desc: &'a str,
    pub tag: &'a str,
    pub phase_list: &'a [u8],
    pub is_threat: bool,
    pub extra_effect_list: Vec<super::misc::ExtraEffect<'a>>,
    pub damage_type: Option<Element>,
    pub skill_trigger_key: SkillTriggerKey,
    pub sp_hit_base: u16,
}

#[derive(Clone, Debug)]
pub struct MonsterCamp<'a> {
    pub id: u8,
    pub sort_id: u8,
    pub name: &'a str,
    pub r#type: CampType,
}
