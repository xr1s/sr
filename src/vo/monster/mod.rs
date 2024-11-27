pub mod guide;

use std::borrow::Cow;

use crate::po::monster::{CampType, CharacterType, DebuffResistKey, Rank, StanceType, SubType};
use crate::po::Element;
use crate::{FnvIndexMap, GameData, Name, Wiki};

#[derive(Clone, Debug)]
pub struct Camp<'a> {
    pub id: u8,
    pub sort_id: u8,
    pub name: &'a str,
    pub r#type: CampType,
}

#[derive(derivative::Derivative)]
#[derivative(Clone, Debug)]
/// 对应游戏中的每种怪物
/// 和 Template 的差别是会随着环境修改具体的属性数值
/// 比如在深渊里的会适当调高降低属性等
pub struct Config<'a> {
    #[derivative(Debug = "ignore")]
    pub(crate) game: &'a GameData,
    pub id: u32,
    pub template: TemplateConfig<'a>,
    pub name: &'a str,
    pub introduction: &'a str,
    pub battle_introduction: &'a str,
    pub attack_modify_ratio: f32,
    pub defence_modify_ratio: f32,
    pub hp_modify_ratio: f32,
    /// 目前该值只有 1
    pub speed_modify_ratio: f32,
    /// 目前该值只有 1
    pub stance_modify_ratio: f32,
    pub speed_modify_value: i16,
    pub stance_modify_value: i16,
    pub skill_list: Vec<SkillConfig<'a>>,
    pub custom_values: FnvIndexMap<&'a str, i32>,
    pub debuff_resist: FnvIndexMap<DebuffResistKey, f32>,
    pub custom_value_tags: Vec<&'a str>,
    pub stance_weak_list: &'a [Element],
    pub damage_type_resistance: fnv::FnvHashMap<Element, f32>,
    pub ability_name_list: Vec<&'a str>,
    pub override_ai_skill_sequence: Vec<SkillConfig<'a>>,
}

impl Config<'_> {
    pub fn prototype(&self) -> Config {
        // 不确定 unwrap 会不会挂，总之先试试
        self.game.monster_config(self.template.id).unwrap()
    }

    pub fn phase(&self) -> u8 {
        self.skill_list
            .iter()
            .flat_map(|skill| skill.phase_list)
            .copied()
            .max()
            .unwrap_or(1)
    }

    /// 列出某一阶段的技能
    pub fn phase_skill(&self, phase: u8) -> Vec<&SkillConfig> {
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

    /// 基础速度
    /// 65 级开始，速度 = 基础速度 * 1.10
    /// 78 级开始，速度 = 基础速度 * 1.20
    /// 86 级开始，速度 = 基础速度 * 1.32
    pub fn speed(&self) -> f32 {
        self.template.speed_base as f32 * self.speed_modify_ratio + self.speed_modify_value as f32
    }

    /// 满级速度（指 `speed` 函数的 86 级版本）
    pub fn max_speed(&self) -> f32 {
        self.speed() as f32 * 1.32
    }

    /// 韧性
    pub fn stance(&self) -> f32 {
        (self.template.stance_base as f32 * self.stance_modify_ratio
            + self.stance_modify_value as f32)
            / 3.
    }

    /// 敌人所有技能的伤害属性
    pub fn damage_types(&self) -> Vec<Element> {
        self.skill_list
            .iter()
            .filter_map(|skill| skill.damage_type)
            .collect::<indexmap::IndexSet<_, fnv::FnvBuildHasher>>()
            .drain(..)
            .collect()
    }

    /// 召唤物，不过这大概不完整，目前没找到能完整列出召唤物的手段
    pub fn summons(&self) -> Vec<Config> {
        self.custom_values
            .iter()
            .filter_map(|(_, &id)| self.game.monster_config(id as _))
            .collect()
    }
}

// {{特殊敌方}} 模板，出现在混沌回忆和虚构叙事的一览页面
impl Config<'_> {
    pub fn is_special(&self) -> bool {
        let mut is_attr_change = false;
        is_attr_change |= self.attack_modify_ratio != 1.;
        is_attr_change |= self.defence_modify_ratio != 1.;
        is_attr_change |= self.hp_modify_ratio != 1.;
        is_attr_change |= self.speed_modify_ratio != 1.;
        is_attr_change |= self.stance_modify_ratio != 1.;
        is_attr_change |= self.speed_modify_value != 0;
        is_attr_change |= self.stance_modify_value != 0;
        if is_attr_change {
            return true;
        }
        let proto = self.prototype();
        self.damage_type_resistance != proto.damage_type_resistance
    }

    pub fn special_wiki(&self, abyss_name: &str, floors: &[u8]) -> String {
        let mut is_attr_change = false;
        is_attr_change |= self.attack_modify_ratio != 1.;
        is_attr_change |= self.defence_modify_ratio != 1.;
        is_attr_change |= self.hp_modify_ratio != 1.;
        is_attr_change |= self.speed_modify_ratio != 1.;
        is_attr_change |= self.stance_modify_ratio != 1.;
        is_attr_change |= self.speed_modify_value != 0;
        is_attr_change |= self.stance_modify_value != 0;
        let proto = self.prototype();
        let is_resist_change = self.damage_type_resistance != proto.damage_type_resistance;
        if !is_attr_change && !is_resist_change {
            return String::new();
        }
        let mut wiki = String::from("{{特殊敌方");
        wiki.push_str("\n|深渊名=");
        wiki.push_str(abyss_name);
        wiki.push_str("\n|层数=");
        let floors: String = floors
            .iter()
            .map(u8::to_string)
            .map(Cow::Owned)
            .intersperse(Cow::Borrowed("、"))
            .collect();
        wiki.push_str(&floors);
        wiki.push_str("\n|敌方名称=");
        wiki.push_str(&self.wiki_name());
        if is_attr_change {
            let mut attr_change = Vec::with_capacity(5);
            if self.attack_modify_ratio != 1. {
                let ratio = f32::round(self.attack_modify_ratio * 10000.) / 100.;
                attr_change.push(format!("攻击：{}", ratio));
            }
            if self.defence_modify_ratio != 1. {
                let ratio = f32::round(self.defence_modify_ratio * 10000.) / 100.;
                attr_change.push(format!("防御：{}", ratio));
            }
            if self.hp_modify_ratio != 1. {
                let ratio = f32::round(self.hp_modify_ratio * 10000.) / 100.;
                attr_change.push(format!("生命：{}", ratio));
            }
            if self.speed_modify_ratio != 1. || self.speed_modify_value != 0 {
                let ratio = self.speed() / self.template.speed_base as f32;
                let ratio = f32::round(ratio * 10000.) / 100.;
                attr_change.push(format!("速度：{}", ratio));
            }
            if self.stance_modify_ratio != 1. || self.stance_modify_value != 0 {
                let ratio = self.stance() * 3. / self.template.stance_base as f32;
                let ratio = f32::round(ratio * 10000.) / 100.;
                attr_change.push(format!("韧性：{}", ratio));
            }
            wiki.push_str("\n|是否属性变化=是\n|属性变化=");
            wiki.push_str(&attr_change.join("、"));
        }
        if is_resist_change {
            let mut resist_upper = Vec::with_capacity(7);
            let mut resist_lower = Vec::with_capacity(7);
            for (element, &resist) in &self.damage_type_resistance {
                let proto_resist = proto
                    .damage_type_resistance
                    .get(element)
                    .copied()
                    .unwrap_or_default();
                if resist == proto_resist {
                    continue;
                }
                let resist_change = format!(
                    "{}：{}",
                    element.wiki(),
                    f32::abs(resist - proto_resist) * 100.
                );
                match resist.total_cmp(&proto_resist) {
                    std::cmp::Ordering::Less => resist_lower.push(resist_change),
                    std::cmp::Ordering::Equal => unreachable!(),
                    std::cmp::Ordering::Greater => resist_upper.push(resist_change),
                }
            }
            // 出现在原型怪物元素抗性中而不出现在当前怪物元素抗性中，作为降为零处理
            for (element, resist) in &proto.damage_type_resistance {
                if !self.damage_type_resistance.contains_key(element) {
                    resist_lower.push(format!("{}：{}", element.wiki(), resist * 100.));
                }
            }
            wiki.push_str("\n|是否弱点或抗性变化=是");
            if !resist_upper.is_empty() {
                wiki.push_str("\n|抗性提高=");
                wiki.push_str(&resist_upper.join("、"));
            }
            if !resist_lower.is_empty() {
                wiki.push_str("\n|抗性降低=");
                wiki.push_str(&resist_lower.join("、"));
            }
            let self_weakness: indexmap::IndexSet<Element> =
                self.stance_weak_list.iter().copied().collect();
            let proto_weakness: indexmap::IndexSet<Element> =
                proto.stance_weak_list.iter().copied().collect();
            let more_weakness = &self_weakness - &proto_weakness;
            let less_weakness = &proto_weakness - &self_weakness;
            if !more_weakness.is_empty() {
                let weaknesses: String = more_weakness
                    .iter()
                    .map(Element::wiki)
                    .intersperse(Cow::Borrowed("、"))
                    .collect();
                wiki.push_str("\n增加弱点=");
                wiki.push_str(&weaknesses);
            }
            if !less_weakness.is_empty() {
                let weaknesses: String = less_weakness
                    .iter()
                    .map(Element::wiki)
                    .intersperse(Cow::Borrowed("、"))
                    .collect();
                wiki.push_str("\n减少弱点=");
                wiki.push_str(&weaknesses)
            }
        }
        wiki.push_str("\n}}");
        wiki
    }
}

pub(crate) fn wiki_name(name: &str) -> Cow<'_, str> {
    use std::borrow::Borrow;
    // 和 NPC 或者自机角色同名的敌方
    const NPC_COLLIDE_NAME: &[&str] = &["可可利亚", "杰帕德", "布洛妮娅", "史瓦罗", "银枝"];
    let mut name = Cow::Borrowed(name);
    if NPC_COLLIDE_NAME.contains(&name.borrow()) {
        name = Cow::Owned(name.to_string() + "（敌方）");
    }
    // 不知为何 WIKI 上自动机兵都使用「•」做分隔符而非保留原来的
    if let Some(strip_name) = name.strip_prefix("自动机兵「") {
        let lend = strip_name.find('」').unwrap();
        let rbeg = lend + "」".len();
        let (l, r) = (&strip_name[..lend], &strip_name[rbeg..]);
        name = Cow::Owned("自动机兵•".to_string() + l + r);
    }
    // 仅出现在「入魔机巧」系列魔物中
    if name.contains('\u{a0}') {
        name = Cow::Owned(name.replace('\u{a0}', ""));
    }
    // WIKI 中大量使用「、」作为分隔符，因此当怪物名称中出现「、」时需要额外转义
    // 仅出现在「昔在、今在、永在的剧目」系列魔物中
    if name.contains('、') {
        name = Cow::Owned(name.replace('、', "&#x3001;"));
    }
    name
}

impl Name for Config<'_> {
    fn name(&self) -> &str {
        self.name
    }
    fn wiki_name(&self) -> Cow<'_, str> {
        wiki_name(self.name)
    }
}

impl crate::Wiki for Config<'_> {
    fn wiki(&self) -> Cow<'static, str> {
        let mut wiki = String::new();
        // 名称
        wiki.push_str("{{敌人\n|名称=");
        wiki.push_str(&self.wiki_name());
        wiki.push_str("\n|实装版本=");
        wiki.push_str("\n|系列=");
        // 分类（阵营）
        wiki.push_str("\n|分类=");
        wiki.push_str(self.template.camp());
        wiki.push_str("<!-- 选填：反物质军团、裂界造物、雅利洛-Ⅵ、仙舟「罗浮」、虫群、星际和平公司、惊梦剧团、忆域迷因、模拟宇宙、星核猎手、银河 -->");
        // 类型（周本Boss、剧情Boss等，这里没法获取全部，需要手动处理）
        let mut typ = match self.template.rank {
            Rank::BigBoss => "周本BOSS",
            Rank::Elite => "强敌",
            Rank::LittleBoss => "剧情BOSS",
            Rank::Minion | Rank::MinionLv2 => "普通",
        };
        if self.template.group_id == 0 && !self.name.contains("扑满") {
            typ = "召唤物";
        }
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
        let mut tags = Vec::<&'static str>::new();
        let summons = self.summons();
        if !summons.is_empty() {
            tags.push("召唤");
        }
        if self.name().ends_with("（完整）") {
            tags.push("完整");
        }
        if self.name().ends_with("（错误）") {
            tags.push("错误");
        }
        wiki.push_str(&tags.join("、"));
        wiki.push_str("\n|速度=");
        let speed = f32::ceil(self.speed()) as u16;
        wiki.push_str(&speed.to_string());
        if speed != 0 {
            wiki.push('~');
            let max_speed = self.max_speed() as u16;
            wiki.push_str(&max_speed.to_string());
        }
        wiki.push_str("\n|韧性=");
        let stance = f32::round(self.stance()) as u16;
        wiki.push_str(&stance.to_string());
        // 召唤物
        wiki.push_str("\n|召唤物=");
        let summon_names = summons
            .iter()
            .map(Config::wiki_name)
            .intersperse(Cow::Borrowed("、"))
            .collect::<String>();
        wiki.push_str(&summon_names);
        // 不明，需要去看下 wiki 模板这几个参数是干什么的
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
            .map(Cow::Owned)
            .intersperse(Cow::Borrowed("、"))
            .collect::<String>();
        wiki.push_str(&element_resistance);
        // 状态抵抗
        wiki.push_str("\n|状态抵抗=");
        let resists = self
            .debuff_resist
            .iter()
            .map(|(debuff, _)| debuff.wiki())
            .intersperse(Cow::Borrowed("、"))
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
                    ("GIF", "", "<!-- 无技能动画填0 -->"),
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
        wiki.push_str("\n}}{{WIKI底部导航|角色图鉴=展开}}");
        std::borrow::Cow::Owned(wiki)
    }
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

#[derive(Clone, Debug)]
pub struct SkillConfig<'a> {
    pub id: u32,
    pub name: &'a str,
    pub desc: String,
    /// 目前只有两种 天赋、技能
    pub type_desc: &'a str,
    /// 技能分类，单攻、群攻、扩散等
    pub tag: &'a str,
    /// 技能在角色的哪个阶段会出现
    pub phase_list: &'a [u8],
    /// 大招
    pub is_threat: bool,
    /// 技能的特殊效果说明
    pub extra_effect_list: Vec<super::misc::ExtraEffectConfig<'a>>,
    /// 技能造成的元素伤害类型
    pub damage_type: Option<Element>,
    pub skill_trigger_key: &'a str,
    /// 技能命中我方角色后为对应角色的充能增加多少
    pub sp_hit_base: u16,
}

#[derive(derivative::Derivative)]
#[derivative(Clone, Debug)]
/// 对应一种怪物类型，不同的怪物类型可能是同一个种族（建模头像相同），但是一般数值上会有差异
pub struct TemplateConfig<'a> {
    #[derivative(Debug = "ignore")]
    pub(crate) game: &'a GameData,
    pub id: u32,
    /// 怪物种族，一般同一个 group 中的怪物的建模、头像是一样的
    /// 举例来说错误、完整是两个 Template，但是它们 TemplateGroupID 是相等的
    /// 无 TemplateGroupID 的大多是召唤物，但也有一系列模拟宇宙扑满（存护扑满、毁灭扑满）
    pub group_id: u32,
    pub name: &'a str,
    /// 阵营名字，如果不存在阵营名，可以将 id 保留到十位查询 group 的阵营
    pub camp_name: &'a str,
    // 怪物稀有度（周本Boss、Boss、精英怪、小怪）
    pub rank: Rank,
    /// 基础攻击值
    /// 在具体的 MonsterConfig 中会按对应 modify_ratio 增长
    /// 也会随着等级提升成长
    pub attack_base: u16,
    /// 基础防御值
    /// 在具体的 MonsterConfig 中会按对应 modify_ratio 增长
    /// 也会随着等级提升成长
    pub defence_base: u16,
    /// 基础生命值
    /// 在具体的 MonsterConfig 中会按对应 modify_ratio 增长
    /// 也会随着剧情中敌方的等级成长
    pub hp_base: f32,
    /// 基础速度值
    /// 在具体的 MonsterConfig 中会按对应 modify_value 增长
    /// 也会随着剧情中敌方的等级成长
    pub speed_base: u16,
    /// 基础韧性值
    /// 在具体的 MonsterConfig 中会按对应 modify_value 增长
    pub stance_base: u16,
    /// 基础暴击伤害值
    pub critical_damage_base: f32,
    /// 基础效果抗性值
    pub status_resistance_base: f32,
    /// 不明
    pub minimum_fatigue_ratio: f32,
    /// 不明
    pub stance_count: u8,
    /// 首动提前多少（绝大部分怪物该数值小于 1）
    /// 如等于 0 表示进入战斗后立即行动
    /// 如等于 0.2 表示提前 80%，以此类推
    pub initial_delay_ratio: f32,
    /// 不明，目前所有怪物中该值缺少物理和雷两种属性
    pub stance_type: Option<StanceType>,
    /// 不明
    pub npc_monster_list: Vec<NPCMonsterData<'a>>,
}

impl TemplateConfig<'_> {
    /// 同种族敌人（头像、建模相同的敌人）
    /// 在 WIKI 上被称为「系列」
    fn group(&self) -> impl Iterator<Item = TemplateConfig> {
        self.game.monster_template_config_group(self.group_id)
    }

    /// 找到 group 的原型，原型上会多一些信息
    pub fn prototype(&self) -> TemplateConfig {
        // 必须要有，因为 self 就是一个满足条件的结果
        self.group()
            // 这里有一个假设, 就是原型的 ID 等于 GroupID
            .find(|monster| monster.id == self.group_id)
            .unwrap_or_else(|| self.clone())
    }

    /// 游戏图鉴中的阵营，如「反物质军团」、「惊梦剧团」等
    pub fn camp(&self) -> &str {
        if !self.camp_name.is_empty() {
            return self.camp_name;
        }
        // 一个坑，有些怪物的 camp_name 不在原型上, 因此还是只能把所有的都找过去
        self.group()
            .find(|monster| !monster.camp_name.is_empty())
            .map(|monster| monster.camp_name)
            .unwrap_or_default()
    }
}
