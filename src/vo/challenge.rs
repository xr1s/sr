use std::{borrow::Cow, collections::HashMap};

use crate::{
    po::{
        challenge::{GroupType, TargetType},
        monster::Rank,
        Element,
    },
    vo, GameData, Name, Wiki,
};

#[derive(derivative::Derivative)]
#[derivative(Clone, Debug)]
pub struct GroupConfig<'a> {
    #[derivative(Debug = "ignore")]
    pub(crate) game: &'a GameData,
    pub id: u16,
    pub name: &'a str,
    pub reward_line_group: Vec<RewardLine<'a>>,
    pub pre_mission: vo::mission::MainMission<'a>,
    pub global_schedule: Option<vo::misc::ScheduleDataGlobal>,
    // 两个常驻混沌回忆没有 schedule_data
    pub schedule_data: Option<vo::misc::ScheduleData>,
    pub maze_buff: Option<vo::misc::MazeBuff<'a>>,
    pub map_entrance: Option<vo::map::MapEntrance<'a>>,
    pub mapping_info: Option<vo::map::MappingInfo<'a>>,
    pub world: Option<vo::map::WorldDataConfig<'a>>,
    pub r#type: GroupType,
}

impl GroupConfig<'_> {
    const ELEMENTS: [Element; 7] = [
        Element::Fire,
        Element::Ice,
        Element::Imaginary,
        Element::Physical,
        Element::Quantum,
        Element::Thunder,
        Element::Wind,
    ];
    const CHNUM: [&'static str; 12] = [
        "一", "二", "三", "四", "五", "六", "七", "八", "九", "十", "十一", "十二",
    ];

    // 第几期，混沌回忆特殊一些
    pub fn issue(&self) -> u16 {
        match self.r#type {
            GroupType::Memory => match self.id {
                // 100 是常驻「永屹之城的遗秘」
                // 101 ~ 119 是开服前和开服后 1.x 版本的
                100..200 => 0,
                900 => 0, // 900 是常驻「天艟求仙迷航录」
                // 上述都懒得计算了
                // 1.3 迄今的混沌回忆
                _ => self.id - 991,
            },
            GroupType::Story => self.id - 2000,
            GroupType::Boss => self.id - 3000,
        }
    }

    pub fn mazes(&self) -> Vec<MazeConfig> {
        self.game.challenge_group_maze(self.id)
    }

    fn wiki_write_sched(&self, wiki: &mut String) {
        if let Some(sched) = &self.schedule_data {
            wiki.push_str("\n|开始时间=");
            wiki.push_str(&sched.begin_time.format("%Y/%m/%d %H:%M").to_string());
            wiki.push_str("\n|结束时间=");
            let end_time = sched.end_time - chrono::TimeDelta::nanoseconds(1);
            wiki.push_str(&end_time.format("%Y/%m/%d %H:%M").to_string());
        }
    }

    fn wiki_write_buff(&self, wiki: &mut String, title: &str, buff: Option<&vo::misc::MazeBuff>) {
        if let Some(buff) = buff {
            wiki.push_str("\n|");
            wiki.push_str(title);
            wiki.push('=');
            wiki.push_str(&crate::format::format_wiki(&buff.desc));
        }
    }

    fn special_monster_wiki(
        &self,
        specials: &indexmap::IndexMap<u32, vo::monster::MonsterConfig>,
        special_floors: &mut HashMap<u32, Vec<u8>>,
    ) -> Cow<'static, str> {
        let mut wiki = String::new();
        for floors in special_floors.values_mut() {
            floors.dedup();
        }
        if !specials.is_empty() {
            wiki.push_str("\n{{折叠面板（特殊敌方）|tsdf");
            wiki.push_str(&format!("{:03}", self.issue()));
            wiki.push('|');
            for (id, monster) in specials {
                let special_wiki = monster.special_wiki(self.name, &special_floors[id]);
                wiki.push_str(&special_wiki);
                wiki.push('\n');
            }
            wiki.push_str("}}");
        }
        Cow::Owned(wiki)
    }
}

// 混沌回忆相关方法
impl GroupConfig<'_> {
    fn memory_wiki_write_event(
        &self,
        wiki: &mut String,
        floor: u8,
        team: u8,
        events: &[vo::battle::StageConfig],
        weaknesses: &[Element],
    ) {
        let floor_team = format!("\n|其{}{}队", Self::CHNUM[floor as usize - 1], team);
        for event in events {
            for (wave_no, wave) in event.monster_list.iter().enumerate() {
                wiki.push_str(&floor_team);
                wiki.push_str(&(wave_no + 1).to_string());
                wiki.push_str("波=");
                let monster_names: String = wave
                    .iter()
                    .map(vo::monster::MonsterConfig::wiki_name)
                    .intersperse(Cow::Borrowed("、"))
                    .collect();
                wiki.push_str(&monster_names);
            }
            let weakness: String = weaknesses
                .iter()
                .map(Element::wiki)
                .intersperse(Cow::Borrowed("、"))
                .collect();
            wiki.push_str(&floor_team);
            wiki.push_str("建议属性=");
            wiki.push_str(&weakness);
        }
    }

    fn memory_wiki(&self) -> Cow<'static, str> {
        let mut wiki = String::from("{{混沌回忆单期3");
        wiki.push_str("\n|期数=");
        wiki.push_str(&format!("{:03}", self.issue()));
        wiki.push_str("\n|名称=");
        wiki.push_str(self.name);
        self.wiki_write_sched(&mut wiki);
        self.wiki_write_buff(&mut wiki, "怪诞逸闻", self.maze_buff.as_ref());
        let mazes = self.mazes();
        let mut specials = indexmap::IndexMap::new();
        let mut special_floors = HashMap::<_, Vec<u8>>::new();
        for maze in mazes {
            self.memory_wiki_write_event(
                &mut wiki,
                maze.floor,
                1,
                &maze.event_list_1,
                maze.damage_type_1,
            );
            self.memory_wiki_write_event(
                &mut wiki,
                maze.floor,
                2,
                &maze.event_list_2,
                maze.damage_type_2,
            );
            for event in maze.event_list_1 {
                for monster in event.monster_list.into_iter().flatten() {
                    if !monster.is_special() {
                        continue;
                    }
                    special_floors
                        .entry(monster.id)
                        .or_default()
                        .push(maze.floor);
                    specials.insert(monster.id, monster);
                }
            }
        }
        wiki.push_str("\n}}");
        wiki.push_str(&self.special_monster_wiki(&specials, &mut special_floors));
        Cow::Owned(wiki)
    }
}

// 虚构叙事相关方法
impl GroupConfig<'_> {
    fn monster_score(&self, monster: &vo::monster::MonsterConfig) -> u16 {
        if self.issue() < 9 {
            if monster.name() == "王下一桶" || monster.name() == "序列扑满" {
                return 2000;
            }
            return match monster.template.rank {
                Rank::Minion | Rank::MinionLv2 => 500,
                Rank::Elite => 4000,
                Rank::LittleBoss | Rank::BigBoss => 8000,
            };
        }
        if monster.name() == "王下一桶" || monster.name() == "序列扑满" {
            return 2500;
        }
        match monster.template.rank {
            Rank::Minion | Rank::MinionLv2 => 500,
            Rank::Elite => 3000,
            Rank::LittleBoss | Rank::BigBoss => 5000,
        }
    }

    fn aggregate_monster<F>(
        monster: &[vo::monster::MonsterConfig],
        monster_counts: &HashMap<Cow<'_, str>, usize>,
        is_elite: F,
    ) -> String
    where
        F: Fn(&vo::monster::MonsterConfig) -> bool,
    {
        monster
            .iter()
            .filter(|&monster| is_elite(monster))
            .map(vo::monster::MonsterConfig::wiki_name)
            .collect::<indexmap::IndexSet<_, fnv::FnvBuildHasher>>()
            .into_iter()
            .map(|name| format!("{name}:{}", monster_counts[&name]))
            .map(Cow::Owned)
            .intersperse(Cow::Borrowed("、"))
            .collect()
    }

    /// story_write_event 解析一层中每一波次的敌人的信息
    /// 如第四层下半第 3 波的信息，会往 wiki 中写入 "|其四2队3波敌人=......"
    fn story_wiki_write_event(
        &self,
        wiki: &mut String,
        floor: u8,
        team: u8,
        events: &[vo::battle::StageConfig],
        weakness: &[Element],
    ) {
        let floor_team = format!("\n|其{}{}队", Self::CHNUM[floor as usize - 1], team);
        for event in events {
            let infinite_group = event.infinite_group().unwrap();
            let mut weakness_scores = fnv::FnvHashMap::from_iter(
                Self::ELEMENTS.into_iter().map(|element| (element, 0u16)),
            );
            let mut max_teammate = [0; 3];
            for (wave_no, wave) in infinite_group.wave_list.iter().enumerate() {
                max_teammate[wave_no] = wave.max_teammate_count;
                let wave_no = (wave_no + 1).to_string();
                for group in &wave.monster_group_list {
                    let monster_counts = itertools::Itertools::counts_by(
                        group.monster_list.iter(),
                        vo::monster::MonsterConfig::wiki_name,
                    );
                    // 计算每一种弱点在每一波中能拿到的分数
                    // 就是假如只将含有某种弱点的怪物全部击败，所能获得的总分
                    for monster in &group.monster_list {
                        let weaknesses = if monster.wiki_name() == "王下一桶" {
                            &Self::ELEMENTS // 「王下一桶」作为全属性弱点对待
                        } else {
                            monster.stance_weak_list
                        };
                        for weakness in weaknesses {
                            *weakness_scores.entry(*weakness).or_default() +=
                                self.monster_score(monster)
                        }
                    }
                    let normal_names =
                        Self::aggregate_monster(&group.monster_list, &monster_counts, |monster| {
                            self.monster_score(monster) == 500
                        });
                    wiki.push_str(&floor_team);
                    wiki.push_str(&wave_no);
                    wiki.push_str("波=");
                    wiki.push_str(&normal_names);
                    let elite_names =
                        Self::aggregate_monster(&group.monster_list, &monster_counts, |monster| {
                            self.monster_score(monster) > 500
                        });
                    wiki.push_str(&floor_team);
                    wiki.push_str(&wave_no);
                    wiki.push_str("波特殊敌方=");
                    wiki.push_str(&elite_names);
                }
            }
            wiki.push_str(&floor_team);
            wiki.push_str("敌方上限=");
            if max_teammate[0] != max_teammate[1] && max_teammate[1] == max_teammate[2] {
                wiki.push_str(&max_teammate[0].to_string());
            } else {
                let max_teammate: String = max_teammate
                    .iter()
                    .map(u8::to_string)
                    .map(Cow::Owned)
                    .intersperse(Cow::Borrowed("/"))
                    .collect();
                wiki.push_str(&max_teammate);
            }
            // 按顺序打印出所有弱点积分
            let mut weakness_scores = weakness_scores.into_iter().collect::<Vec<_>>();
            // 按照弱点积分从大到小排序
            // 建议属性无条件排到前面。当积分一致时候，在建议属性中的按照建议顺序排序，否则按照英文字典序
            weakness_scores.sort_by(|l, r| {
                let l_in_weak = weakness.iter().position(|&weakness| weakness == l.0);
                let r_in_weak = weakness.iter().position(|&weakness| weakness == r.0);
                match (l_in_weak, r_in_weak) {
                    (None, None) => {
                        if l.1 != r.1 {
                            return u16::cmp(&r.1, &l.1); // 注意排序是逆向的
                        }
                        u8::cmp(&(l.0 as u8), &(r.0 as u8))
                    }
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (Some(l_index), Some(r_index)) => {
                        if l.1 != r.1 {
                            return u16::cmp(&r.1, &l.1); // 注意排序是逆向的
                        }
                        usize::cmp(&l_index, &r_index)
                    }
                }
            });
            let weaknesses: String = weakness
                .iter()
                .map(Element::wiki)
                .intersperse(Cow::Borrowed("、"))
                .collect();
            wiki.push_str(&floor_team);
            wiki.push_str("建议属性=");
            wiki.push_str(&weaknesses);
            let weakness_scores: String = weakness_scores
                .into_iter()
                .map(|(weakness, score)| format!("{}:{score}", weakness.wiki()))
                .map(Cow::Owned)
                .intersperse(Cow::Borrowed("、"))
                .collect();
            wiki.push_str(&floor_team);
            wiki.push_str("弱点分数=");
            wiki.push_str(&weakness_scores);
        }
    }

    fn story_wiki_write_wave_ability(
        wiki: &mut String,
        floor: u8,
        team: u8,
        events: &[vo::battle::StageConfig],
    ) {
        let floor_team = format!("\n|其{}{}队", Self::CHNUM[floor as usize - 1], team);
        // 虚构叙事定制的 Ability，目前只有给怪物增幅攻击和生命。
        // 具体配置文件见 Config/ConfigAbility/BattleEvent/FantasticStory_Wave_Ability.json
        // assert 的 ability 的效果是按 param_list 分别增幅攻击和生命上限
        // 解析 Config 非常复杂，先写死，如果有改动的话，就人工处理吧
        for event in events {
            let infinite_group = event.infinite_group().unwrap();
            for (wave_no, wave) in infinite_group.wave_list.iter().enumerate() {
                assert_eq!(wave.ability, "FantasticStory_Wave_Ability_0001");
                let wave_no = (wave_no + 1).to_string();
                wiki.push_str(&floor_team);
                wiki.push_str(&wave_no);
                wiki.push_str("波攻击增幅=");
                let attack_ratio = f32::round(100. * wave.param_list[0]) as u16;
                if attack_ratio != 0 {
                    wiki.push_str(&attack_ratio.to_string());
                    wiki.push('%');
                }
                wiki.push_str(&floor_team);
                wiki.push_str(&wave_no);
                wiki.push_str("波生命增幅=");
                let hp_ratio = f32::round(100. * wave.param_list[1]) as u16;
                if hp_ratio != 0 {
                    wiki.push_str(&hp_ratio.to_string());
                    wiki.push('%');
                }
            }
        }
    }

    fn story_special_monster_wiki(&self, mazes: &[MazeConfig]) -> Cow<'static, str> {
        let mut specials = indexmap::IndexMap::new();
        let mut special_floors = HashMap::<_, Vec<u8>>::new();
        for maze in mazes {
            macro_rules! handle {
                ($events:expr) => {
                    for event in $events {
                        let infinite_group = event.infinite_group().unwrap();
                        for wave in infinite_group.wave_list {
                            for group in wave.monster_group_list {
                                for monster in group.monster_list {
                                    if !monster.is_special() {
                                        continue;
                                    }
                                    special_floors
                                        .entry(monster.id)
                                        .or_default()
                                        .push(maze.floor);
                                    specials.insert(monster.id, monster);
                                }
                            }
                        }
                    }
                };
            }
            handle!(&maze.event_list_1);
            handle!(&maze.event_list_2);
        }
        self.special_monster_wiki(&specials, &mut special_floors)
    }

    fn story_reinforce(&self, mazes: &[MazeConfig]) -> String {
        let mut wiki = String::from("{{虚构叙事增援序列");
        fn handle(wiki: &mut String, events: &[vo::battle::StageConfig], floor: u8, team: u8) {
            for event in events {
                let infinite_group = event.infinite_group().unwrap();
                for (wave_no, wave) in infinite_group.wave_list.into_iter().enumerate() {
                    let wave_no = (wave_no + 1).to_string();
                    for group in wave.monster_group_list {
                        let reinforces: String = group
                            .monster_list
                            .iter()
                            .map(vo::monster::MonsterConfig::wiki_name)
                            .intersperse(Cow::Borrowed("、"))
                            .collect();
                        wiki.push_str("\n|其");
                        wiki.push_str(GroupConfig::CHNUM[floor as usize - 1]);
                        wiki.push_str(if team == 1 { "上半" } else { "下半" });
                        wiki.push('第');
                        wiki.push_str(&wave_no);
                        wiki.push_str("波=");
                        wiki.push_str(&reinforces);
                    }
                }
            }
        }
        for maze in mazes {
            handle(&mut wiki, &maze.event_list_1, maze.floor, 1);
            handle(&mut wiki, &maze.event_list_2, maze.floor, 2);
        }
        wiki.push_str("\n}}");
        wiki
    }

    /// 用来确保 monster_score 返回的值是正确的
    /// monster_score 的值每个版本都有可能不同，但是没法从数据集中提取出来，都是自己试出来的
    fn story_assert_score(&self, mazes: &[MazeConfig]) {
        macro_rules! handle {
            ($events:expr) => {{
                let mut score = 0u16;
                for event in $events {
                    let infinite_group = event.infinite_group().unwrap();
                    for wave in infinite_group.wave_list {
                        for group in wave.monster_group_list {
                            for monster in group.monster_list {
                                score += self.monster_score(&monster);
                            }
                        }
                    }
                }
                score
            }};
        }
        assert_eq!(handle!(&mazes[0].event_list_1), 40000, "第一层上半");
        assert_eq!(handle!(&mazes[0].event_list_2), 40000, "第一层下半");
        assert_eq!(handle!(&mazes[1].event_list_1), 40000, "第二层上半");
        assert_eq!(handle!(&mazes[1].event_list_2), 40000, "第二层下半");
        assert_eq!(handle!(&mazes[2].event_list_1), 40000, "第三层上半");
        assert_eq!(handle!(&mazes[2].event_list_2), 40000, "第三层下半");
        assert_eq!(handle!(&mazes[3].event_list_1), 40000, "第四层上半");
        assert_eq!(handle!(&mazes[3].event_list_2), 40000, "第四层下半");
    }

    fn story_wiki(&self) -> Cow<'static, str> {
        use crate::format::format_wiki;
        let mut wiki = String::from("{{虚构叙事单期");
        wiki.push_str("\n|期数=");
        wiki.push_str(&format!("{:03}", self.issue()));
        wiki.push_str("\n|名称=");
        wiki.push_str(self.name);
        self.wiki_write_sched(&mut wiki);
        self.wiki_write_buff(&mut wiki, "记忆紊流", self.maze_buff.as_ref());
        let extra = self.game.challenge_story_group_extra(self.id).unwrap();
        if let Some(buff_list) = extra.buff_list {
            for (index, buff) in buff_list.iter().enumerate() {
                wiki.push_str("\n|荒腔");
                wiki.push_str(&(index + 1).to_string());
                wiki.push('=');
                wiki.push_str(buff.name);
                wiki.push_str("\n|荒腔走板其");
                wiki.push_str(Self::CHNUM[index]);
                wiki.push('=');
                wiki.push_str(&format_wiki(&buff.desc));
            }
        }
        let mut mazes = self.mazes();
        mazes.sort_by_key(|maze| maze.floor);
        for maze in &mazes {
            self.story_wiki_write_event(
                &mut wiki,
                maze.floor,
                1,
                &maze.event_list_1,
                maze.damage_type_1,
            );
            self.story_wiki_write_event(
                &mut wiki,
                maze.floor,
                2,
                &maze.event_list_2,
                maze.damage_type_2,
            );
        }
        for maze in &mazes {
            Self::story_wiki_write_wave_ability(&mut wiki, maze.floor, 1, &maze.event_list_1);
            Self::story_wiki_write_wave_ability(&mut wiki, maze.floor, 2, &maze.event_list_2);
        }
        wiki.push_str("\n}}");
        wiki.push_str(&self.story_special_monster_wiki(&mazes));
        wiki.push_str("\n<br />\n<br />\n----\n\n");
        self.story_assert_score(&mazes);
        wiki.push_str(&self.story_reinforce(&mazes));
        Cow::Owned(wiki)
    }
}

impl Wiki for GroupConfig<'_> {
    fn wiki(&self) -> Cow<'static, str> {
        match self.r#type {
            GroupType::Memory => self.memory_wiki(),
            GroupType::Story => self.story_wiki(),
            GroupType::Boss => Cow::Borrowed(""),
        }
    }
}

#[derive(Clone, Debug)]
pub struct GroupExtra<'a> {
    pub id: u16,
    pub buff_list: Option<[vo::misc::MazeBuff<'a>; 3]>,
    pub buff_list_1: Option<[vo::misc::MazeBuff<'a>; 3]>,
    pub buff_list_2: Option<[vo::misc::MazeBuff<'a>; 3]>,
}

#[derive(Clone, Debug)]
pub struct MazeConfig<'a> {
    pub id: u16,
    pub name: &'a str,
    pub group: GroupConfig<'a>,
    pub map_entrance: vo::map::MapEntrance<'a>,
    pub map_entrance_2: vo::map::MapEntrance<'a>,
    pub pre_level: u8,
    pub pre_challenge_maze_id: u16,
    pub floor: u8,
    pub reward: vo::misc::RewardData<'a>,
    pub damage_type_1: &'a [Element],
    pub damage_type_2: &'a [Element],
    pub target: [TargetConfig; 3],
    pub stage_num: u8,
    pub monster_id_1: Vec<vo::monster::MonsterConfig<'a>>,
    pub monster_id_2: Vec<vo::monster::MonsterConfig<'a>>,
    /// 回合数内打倒敌人，仅出现在混沌回忆中
    pub challenge_count_down: u8,
    pub npc_monster_id_list_1: Vec<vo::monster::NPCMonsterData<'a>>,
    pub event_list_1: Vec<vo::battle::StageConfig<'a>>,
    pub npc_monster_id_list_2: Vec<vo::monster::NPCMonsterData<'a>>,
    pub event_list_2: Vec<vo::battle::StageConfig<'a>>,
    pub maze_buff: vo::misc::MazeBuff<'a>,
}

#[derive(Clone, Debug)]
pub struct MazeExtra<'a> {
    pub id: u16,
    pub turn_limit: u8,
    pub monster_1: Option<vo::monster::MonsterConfig<'a>>,
    pub monster_2: Option<vo::monster::MonsterConfig<'a>>,
}

#[derive(Clone, Debug)]
pub struct RewardLine<'a> {
    pub group_id: u16,
    pub star_count: u8,
    pub reward: vo::misc::RewardData<'a>,
}

#[derive(Clone, Debug)]
pub struct TargetConfig {
    pub id: u16,
    pub r#type: TargetType,
    pub name: String,
    /// 不明，不是 RewardData
    /// 只有 ChallengeBossTargetConfig.json 没有 RewardID
    pub reward_id: u32,
}
