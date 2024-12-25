pub mod guide;

use std::borrow::Cow;
use std::num::NonZero;

use base::{FnvIndexMap, Name, Wiki};
pub use model::monster::{
    DebuffResistKey, MonsterCampType, MonsterCharacterType, MonsterRank, MonsterSubType,
};
use model::Element;

use crate::{ExcelOutput, FromModel};

#[derive(Clone, Debug)]
pub struct MonsterCamp<'a> {
    pub id: u8,
    pub sort_id: u8,
    pub name: &'a str,
    pub r#type: Option<MonsterCampType>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for MonsterCamp<'a> {
    type Model = model::monster::MonsterCamp;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            id: model.id,
            sort_id: model.sort_id,
            name: game.text(model.name),
            r#type: model.camp_type,
        }
    }
}

#[derive(educe::Educe)]
#[educe(Clone, Debug)]
/// 对应游戏中的每种怪物
/// 和 Template 的差别是会随着环境修改具体的属性数值
/// 比如在深渊里的会适当调高降低属性等
pub struct MonsterConfig<'a, Data: ExcelOutput + ?Sized> {
    #[educe(Debug(ignore))]
    game: &'a Data,
    pub id: u32,
    pub template: Option<MonsterTemplateConfig<'a, Data>>,
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
    pub skill_list: Vec<MonsterSkillConfig<'a>>,
    pub custom_values: FnvIndexMap<&'a str, i32>,
    pub debuff_resist: FnvIndexMap<DebuffResistKey, f32>,
    pub custom_value_tags: Vec<&'a str>,
    pub stance_weak_list: &'a [Element],
    pub damage_type_resistance: fnv::FnvHashMap<Element, f32>,
    pub ability_name_list: Vec<&'a str>,
    pub override_ai_skill_sequence: Vec<MonsterSkillConfig<'a>>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for MonsterConfig<'a, Data> {
    type Model = model::monster::MonsterConfig;
    fn from_model(game: &'a Data, model: &'a Self::Model) -> Self {
        Self {
            game,
            id: model.monster_id,
            // 1.0~1.3, 2.0 存在几个数据，会导致 panic
            template: None
                .or_else(|| game.monster_template_config(model.monster_template_id))
                .or_else(|| game.monster_template_unique_config(model.monster_template_id)),
            name: game.text(model.monster_name),
            introduction: game.text(model.monster_introduction),
            battle_introduction: model
                .monster_battle_introduction
                .map(|hash| game.text(hash))
                .unwrap_or_default(),
            attack_modify_ratio: model.attack_modify_ratio.value,
            defence_modify_ratio: model.defence_modify_ratio.value,
            hp_modify_ratio: model.hp_modify_ratio.value,
            speed_modify_ratio: model.speed_modify_ratio.value,
            stance_modify_ratio: model.stance_modify_ratio.value,
            speed_modify_value: model.speed_modify_value.unwrap_or_default().value,
            stance_modify_value: model.stance_modify_value.unwrap_or_default().value,
            skill_list: model
                .skill_list
                .iter()
                .map(|&id| {
                    None.or_else(|| game.monster_skill_config(id))
                        .or_else(|| game.monster_skill_unique_config(id))
                })
                .map(Option::unwrap)
                .collect(),
            custom_values: model
                .custom_values
                .iter()
                .map(|o| (o.key.as_str(), o.value))
                .collect(),
            debuff_resist: model
                .debuff_resist
                .iter()
                .map(|o| (o.key, o.value.value))
                .collect(),
            custom_value_tags: model.custom_value_tags.iter().map(String::as_str).collect(),
            stance_weak_list: &model.stance_weak_list,
            damage_type_resistance: model
                .damage_type_resistance
                .iter()
                .map(|o| (o.damage_type, o.value.value))
                .collect(),
            ability_name_list: model.ability_name_list.iter().map(String::as_str).collect(),
            override_ai_skill_sequence: model
                .override_ai_skill_sequence
                .iter()
                .map(|seq| game.monster_skill_config(seq.id))
                .map(Option::unwrap)
                .collect(),
        }
    }
}

impl<Data: ExcelOutput> MonsterConfig<'_, Data> {
    pub fn prototype(&self) -> MonsterConfig<'_, Data> {
        // 不确定 unwrap 会不会挂，总之先试试
        self.template
            .as_ref()
            .map(|template| template.id)
            .map(|id| self.game.monster_config(id))
            .map(Option::unwrap)
            .unwrap_or_else(|| self.clone())
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
    pub fn phase_skill(&self, phase: u8) -> Vec<&MonsterSkillConfig> {
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
        self.template
            .as_ref()
            .map(|template| template.speed_base)
            .unwrap_or_default() as f32
            * self.speed_modify_ratio
            + self.speed_modify_value as f32
    }

    /// 满级速度（指 `speed` 函数的 86 级版本）
    pub fn max_speed(&self) -> f32 {
        self.speed() * 1.32
    }

    /// 韧性
    pub fn stance(&self) -> f32 {
        (self
            .template
            .as_ref()
            .map(|template| template.stance_base)
            .unwrap_or_default() as f32
            * self.stance_modify_ratio
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
    pub fn summons(&self) -> Vec<MonsterConfig<Data>> {
        self.custom_values
            .iter()
            .filter_map(|(_, &id)| self.game.monster_config(id as _))
            .collect()
    }

    /// 小怪，排除精英、剧情 Boss、周本 Boss
    pub fn is_minion(&self) -> bool {
        let rank = self
            .template
            .as_ref()
            .map(|template| template.rank)
            .unwrap_or(MonsterRank::MinionLv2);
        rank == MonsterRank::Minion || rank == MonsterRank::MinionLv2
    }
}

// {{特殊敌方}} 模板，出现在混沌回忆和虚构叙事的一览页面
impl<Data: ExcelOutput> MonsterConfig<'_, Data> {
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
        assert!(self.template.is_some(), "普通怪物不能没有模板");
        let template = self.template.as_ref().unwrap();
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
                let ratio = self.speed() / template.speed_base as f32;
                let ratio = f32::round(ratio * 10000.) / 100.;
                attr_change.push(format!("速度：{}", ratio));
            }
            if self.stance_modify_ratio != 1. || self.stance_modify_value != 0 {
                let ratio = self.stance() * 3. / template.stance_base as f32;
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

impl<Data: ExcelOutput> Name for MonsterConfig<'_, Data> {
    fn name(&self) -> &str {
        self.name
    }
    fn wiki_name(&self) -> Cow<'_, str> {
        wiki_name(self.name)
    }
}

impl<Data: ExcelOutput> Wiki for MonsterConfig<'_, Data> {
    fn wiki(&self) -> Cow<'static, str> {
        let mut wiki = String::new();
        // 名称
        wiki.push_str("{{敌人\n|名称=");
        wiki.push_str(&self.wiki_name());
        wiki.push_str("\n|实装版本=");
        wiki.push_str("\n|系列=");
        // 分类（阵营）
        wiki.push_str("\n|分类=");
        let camp = self
            .template
            .as_ref()
            .map(|template| template.camp())
            .unwrap_or_default();
        wiki.push_str(camp);
        wiki.push_str(
            "<!-- 选填：反物质军团、裂界造物、雅利洛-Ⅵ、仙舟「罗浮」、虫群、星际和平公司、\
             惊梦剧团、忆域迷因、模拟宇宙、星核猎手、银河 -->",
        );
        // 类型（周本Boss、剧情Boss等，这里没法获取全部，需要手动处理）
        let mut typ = match self.template.as_ref().map(|template| template.rank) {
            Some(MonsterRank::BigBoss) => "周本BOSS",
            Some(MonsterRank::Elite) => "强敌",
            Some(MonsterRank::LittleBoss) => "剧情BOSS",
            Some(MonsterRank::Minion | MonsterRank::MinionLv2) => "普通",
            None => "",
        };
        let group_id = self
            .template
            .as_ref()
            .map(|template| template.group_id)
            .unwrap_or_default();
        if self.template.is_some() && group_id == 0 && !self.name.contains("扑满") {
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
            .map(MonsterConfig::wiki_name)
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
    pub character_type: MonsterCharacterType,
    pub sub_type: MonsterSubType,
    pub rank: MonsterRank,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for NPCMonsterData<'a> {
    type Model = model::monster::NPCMonsterData;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            id: model.id,
            name: game.text(model.npc_name),
            title: game.text(model.npc_title),
            character_type: model.character_type,
            sub_type: model.sub_type,
            rank: model.rank,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MonsterSkillConfig<'a> {
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

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for MonsterSkillConfig<'a> {
    type Model = model::monster::SkillConfig;
    fn from_model(game: &'a Data, model: &'a Self::Model) -> Self {
        let params = format::Argument::from_array(&model.param_list);
        Self {
            id: model.skill_id,
            name: game.text(model.skill_name),
            desc: format::format(game.text(model.skill_desc), &params),
            type_desc: game.text(model.skill_type_desc),
            tag: model
                .skill_tag
                .map(|hash| game.text(hash))
                .unwrap_or_default(),
            phase_list: &model.phase_list,
            is_threat: model.is_threat,
            extra_effect_list: model
                .extra_effect_id_list
                .iter()
                .map(|&id| game.extra_effect_config(id))
                .map(Option::unwrap)
                .collect(),
            damage_type: model.damage_type,
            skill_trigger_key: model.skill_trigger_key.as_str(),
            sp_hit_base: model.sp_hit_base.unwrap_or_default().value,
        }
    }
}

#[derive(educe::Educe)]
#[educe(Clone, Debug)]
/// 对应一种怪物类型，不同的怪物类型可能是同一个种族（建模头像相同），但是一般数值上会有差异
pub struct MonsterTemplateConfig<'a, Data: ExcelOutput + ?Sized> {
    #[educe(Debug(ignore))]
    game: &'a Data,
    pub id: u32,
    /// 怪物种族，一般同一个 group 中的怪物的建模、头像是一样的
    /// 举例来说错误、完整是两个 Template，但是它们 TemplateGroupID 是相等的
    /// 无 TemplateGroupID 的大多是召唤物，但也有一系列模拟宇宙扑满（存护扑满、毁灭扑满）
    pub group_id: u32,
    pub name: &'a str,
    /// 阵营名字，如果不存在阵营名，可以将 id 保留到十位查询 group 的阵营
    pub camp_name: &'a str,
    /// 怪物稀有度（周本Boss、Boss、精英怪、小怪）
    pub rank: MonsterRank,
    pub icon_path: &'a str,
    pub round_icon_path: &'a str,
    pub image_path: &'a str,
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
    /// 不明，目前所有怪物中该值缺少物理属性
    pub stance_type: Option<Element>,
    /// 不明
    pub npc_monster_list: Vec<NPCMonsterData<'a>>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for MonsterTemplateConfig<'a, Data> {
    type Model = model::monster::MonsterTemplateConfig;
    fn from_model(game: &'a Data, model: &'a Self::Model) -> Self {
        #[rustfmt::skip]
        const MISSING_NPC_MONSTER: &[u32] = &[
            1005010, 1012010, 3024012, 8022020,
            // 1.2 缺漏数据
            2013011,
        ];
        let camp = model
            .monster_camp_id
            .map(NonZero::get)
            .map(|id| game.monster_camp(id))
            .map(Option::unwrap);
        Self {
            game,
            id: model.monster_template_id,
            group_id: model
                .template_group_id
                .map(NonZero::get)
                .unwrap_or_default(),
            name: game.text(model.monster_name),
            camp_name: camp.map(|camp| camp.name).unwrap_or_default(),
            rank: model.rank,
            icon_path: &model.icon_path,
            round_icon_path: &model.round_icon_path,
            image_path: &model.image_path,
            attack_base: model.attack_base.value,
            defence_base: model
                .defence_base
                .map(|v| v.value.get())
                .unwrap_or_default(),
            hp_base: model.hp_base.value,
            speed_base: model.speed_base.map(|v| v.value.get()).unwrap_or_default(),
            stance_base: model.stance_base.map(|v| v.value.get()).unwrap_or_default(),
            critical_damage_base: model
                .critical_damage_base
                .map(|v| v.value)
                .unwrap_or_default(),
            status_resistance_base: model.status_resistance_base.unwrap_or_default().value,
            minimum_fatigue_ratio: model.minimum_fatigue_ratio.value,
            stance_count: model.stance_count.map(NonZero::get).unwrap_or_default(),
            initial_delay_ratio: model.initial_delay_ratio.unwrap_or_default().value,
            npc_monster_list: model
                .npc_monster_list
                .iter()
                .filter(|&id| !MISSING_NPC_MONSTER.contains(id)) // TODO: 疑似缺数据
                .map(|&id| game.npc_monster_data(id))
                .map(Option::unwrap)
                .collect(),
            stance_type: model.stance_type,
        }
    }
}

impl<Data: ExcelOutput> MonsterTemplateConfig<'_, Data> {
    /// 同种族敌人（头像、建模相同的敌人）
    /// 在 WIKI 上被称为「系列」
    fn group(&self) -> impl Iterator<Item = MonsterTemplateConfig<Data>> {
        self.game.monster_template_config_group(self.group_id)
    }

    /// 找到 group 的原型，原型上会多一些信息
    pub fn prototype(&self) -> MonsterTemplateConfig<Data> {
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
