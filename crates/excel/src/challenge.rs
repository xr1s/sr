use std::{borrow::Cow, collections::HashMap, num::NonZero};

use base::{FnvIndexMap, FnvMultiMap, Name, Wiki};
pub use model::{
    challenge::{ChallengeGroupType, ChallengeStoryType, ChallengeTargetType},
    monster::MonsterRank,
    Element,
};

use crate::{ExcelOutput, FromModel};

#[derive(educe::Educe)]
#[educe(Clone, Debug)]
pub struct ChallengeGroupConfig<'a, Data: ExcelOutput + ?Sized> {
    #[educe(Debug(ignore))]
    game: &'a Data,
    pub id: u16,
    pub name: &'a str,
    pub reward_line_group: Vec<ChallengeRewardLine<'a>>,
    pub pre_mission: crate::mission::MainMission<'a>,
    pub global_schedule: Option<crate::misc::ScheduleDataGlobal>,
    // 两个常驻混沌回忆没有 schedule_data
    pub schedule_data: Option<crate::misc::ScheduleData>,
    pub maze_buff: Option<crate::misc::MazeBuff<'a>>,
    pub map_entrance: Option<crate::map::MapEntrance<'a>>,
    pub mapping_info: Vec<crate::map::MappingInfo<'a, Data>>,
    pub world: Option<crate::map::WorldDataConfig<'a>>,
    pub r#type: ChallengeGroupType,
    // cache
    _extra: std::sync::OnceLock<ChallengeGroupExtra<'a>>,
    _mazes: std::sync::OnceLock<Vec<ChallengeMazeConfig<'a, Data>>>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for ChallengeGroupConfig<'a, Data> {
    type Model = model::challenge::ChallengeGroupConfig;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            game,
            id: model.group_id,
            name: game.text(model.group_name),
            reward_line_group: match model.challenge_group_type {
                ChallengeGroupType::Memory => Data::challenge_maze_reward_line,
                ChallengeGroupType::Story => Data::challenge_story_reward_line,
                ChallengeGroupType::Boss => Data::challenge_boss_reward_line,
            }(game, model.reward_line_group_id),
            pre_mission: game.main_mission(model.pre_mission_id).unwrap(),
            global_schedule: model
                .global_schedule_id
                .map(NonZero::get)
                .map(|id| game.schedule_data_global(id))
                .map(Option::unwrap),
            schedule_data: model
                .schedule_data_id
                .map(NonZero::get)
                .map(|id| match model.challenge_group_type {
                    ChallengeGroupType::Memory => game.schedule_data_challenge_maze(id),
                    ChallengeGroupType::Story => game.schedule_data_challenge_story(id),
                    ChallengeGroupType::Boss => game.schedule_data_challenge_boss(id),
                })
                .map(Option::unwrap),
            maze_buff: model
                .maze_buff_id
                .map(NonZero::get)
                .map(|id| game.maze_buff(id))
                .map(Vec::into_iter)
                .map(|mut iter| iter.next())
                .map(Option::unwrap),
            map_entrance: model
                .map_entrance_id
                .map(NonZero::get)
                .map(|id| game.map_entrance(id))
                .map(Option::unwrap),
            mapping_info: model
                .mapping_info_id
                .map(NonZero::get)
                .filter(|&id| id != 1220) // TODO: 疑似缺数据
                .map(|id| game.mapping_info(id))
                .unwrap_or_default(),
            world: model
                .world_id
                .map(NonZero::get)
                .map(|id| game.world_data_config(id))
                .map(Option::unwrap),
            r#type: model.challenge_group_type,
            _extra: std::sync::OnceLock::new(),
            _mazes: std::sync::OnceLock::new(),
        }
    }
}

impl<'a, Data: ExcelOutput> ChallengeGroupConfig<'a, Data> {
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
            ChallengeGroupType::Memory => match self.id {
                // 100 是常驻「永屹之城的遗秘」
                // 900 是常驻「天艟求仙迷航录」
                // 101 ~ 119 是开服前和开服后 1.x 版本的
                100..=109 | 900 => 0,
                116..=119 => self.id - 116,
                110..=115 => self.id - 106,
                // 1.3 迄今的混沌回忆
                _ => self.id - 991,
            },
            ChallengeGroupType::Story => self.id - 2000,
            ChallengeGroupType::Boss => self.id - 3000,
        }
    }

    pub fn mazes(&self) -> &[ChallengeMazeConfig<'a, Data>] {
        self._mazes
            .get_or_init(|| self.game.challenge_maze_in_group(self.id))
    }

    pub fn extra(&self) -> &ChallengeGroupExtra<'_> {
        self._extra.get_or_init(|| {
            (match self.r#type {
                ChallengeGroupType::Memory => ExcelOutput::challenge_maze_group_extra,
                ChallengeGroupType::Story => ExcelOutput::challenge_story_group_extra,
                ChallengeGroupType::Boss => ExcelOutput::challenge_boss_group_extra,
            })(self.game, self.id)
            .unwrap()
        })
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

    fn special_monster_wiki(
        &self,
        specials: FnvIndexMap<u32, &crate::monster::MonsterConfig<Data>>,
        mut floors: HashMap<u32, Vec<u8>>,
    ) -> Cow<'static, str> {
        let mut wiki = String::new();
        for floors in floors.values_mut() {
            floors.sort();
            floors.dedup();
        }
        if !specials.is_empty() {
            wiki.push_str("\n{{折叠面板（特殊敌方）|tsdf");
            wiki.push_str(&format!("{:03}", self.issue()));
            wiki.push('|');
            for (id, monster) in specials {
                let special_wiki = monster.special_wiki(self.name, &floors[&id]);
                wiki.push_str(&special_wiki);
                wiki.push('\n');
            }
            wiki.push_str("}}");
        }
        Cow::Owned(wiki)
    }

    fn monster_score(&self, monster: &crate::monster::MonsterConfig<Data>) -> u32 {
        const STRATEGY: &[[u32; 4]] = &[[500, 2000, 4000, 8000], [500, 2500, 3000, 5000]];
        let strategy = &match self.issue() {
            1..=3 => STRATEGY[0],
            4..=6 => STRATEGY[1],
            7..=8 => STRATEGY[0],
            9..=10 => STRATEGY[1],
            // 猜测，如果错了手动改
            _ => STRATEGY[0],
        };
        let story_type = self
            .extra()
            .story_type
            .unwrap_or(ChallengeStoryType::Normal);
        if story_type == ChallengeStoryType::Normal
            && (monster.name() == "王下一桶" || monster.name() == "序列扑满")
        {
            return strategy[1];
        }
        match monster.template.as_ref().map(|template| template.rank) {
            Some(MonsterRank::Minion | MonsterRank::MinionLv2) => strategy[0],
            Some(MonsterRank::Elite) => strategy[2],
            Some(MonsterRank::LittleBoss | MonsterRank::BigBoss) => strategy[3],
            None => unreachable!(),
        }
    }
}

// 混沌回忆相关方法
impl<Data: ExcelOutput + format::GameData> ChallengeGroupConfig<'_, Data> {
    fn wiki_write_buff(
        &self,
        wiki: &mut String,
        formatter: &mut format::Formatter<Data>,
        title: &str,
        buff: Option<&crate::misc::MazeBuff>,
    ) {
        if let Some(buff) = buff {
            wiki.push_str("\n|");
            wiki.push_str(title);
            wiki.push('=');
            wiki.push_str(&formatter.format(buff.desc, &buff.params));
        }
    }

    fn memory_wiki_write_event(
        &self,
        wiki: &mut String,
        floor: u8,
        team: u8,
        event: &crate::battle::StageConfig<Data>,
        weaknesses: &[Element],
    ) {
        let floor_team = format!("\n|其{}{}队", Self::CHNUM[floor as usize - 1], team);
        for (wave_no, wave) in event.monster_list.iter().enumerate() {
            wiki.push_str(&floor_team);
            wiki.push_str(&(wave_no + 1).to_string());
            wiki.push_str("波=");
            let monster_names: String = wave
                .iter()
                .map(crate::monster::MonsterConfig::wiki_name)
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

    fn memory_wiki(&self) -> Cow<'static, str> {
        let mut formatter = format::Formatter::new(self.game).media_wiki_syntax(true);
        if self.id < 1007 {
            // 更早以往的混沌回忆机制不太一样
            // 比如同一层存在两个敌方 Boss 左右分立（具体见游戏内常驻回忆「永屹之城的遗秘」回忆其六）
            // 左右分立对应到数据上是 event_list_1.len() == 2
            return Cow::Borrowed("<!-- 过旧数据，不考虑兼容 -->");
        }
        let mut wiki = String::from("{{混沌回忆单期3");
        wiki.push_str("\n|期数=");
        wiki.push_str(&format!("{:03}", self.issue()));
        wiki.push_str("\n|名称=");
        wiki.push_str(self.name);
        self.wiki_write_sched(&mut wiki);
        self.wiki_write_buff(
            &mut wiki,
            &mut formatter,
            "记忆紊流",
            self.maze_buff.as_ref(),
        );
        let mazes = self.mazes();
        for maze in mazes {
            assert_eq!(maze.event_list_1.len(), 1, "上半场景中无分立敌方首领");
            assert_eq!(maze.event_list_2.len(), 1, "下半场景中无分立敌方首领");
        }
        let mut specials = FnvIndexMap::default();
        let mut floors = HashMap::<_, Vec<u8>>::new();
        for (index, maze) in mazes.iter().enumerate() {
            let floor = if maze.floor != 0 {
                maze.floor
            } else {
                index as u8 + 1
            };
            self.memory_wiki_write_event(
                &mut wiki,
                floor,
                1,
                &maze.event_list_1[0],
                maze.damage_type_1,
            );
            self.memory_wiki_write_event(
                &mut wiki,
                floor,
                2,
                &maze.event_list_2[0],
                maze.damage_type_2,
            );
            [&maze.event_list_1[0], &maze.event_list_2[0]]
                .into_iter()
                .flat_map(|event| &event.monster_list)
                .flatten()
                .filter(|monster| monster.is_special())
                .for_each(|monster| {
                    floors.entry(monster.id).or_default().push(floor);
                    specials.insert(monster.id, monster);
                });
        }
        wiki.push_str("\n}}");
        wiki.push_str(&self.special_monster_wiki(specials, floors));
        wiki.push_str("<br />\n<br />\n----");
        Cow::Owned(wiki)
    }
}

// 虚构叙事相关方法
impl<Data: ExcelOutput + format::GameData> ChallengeGroupConfig<'_, Data> {
    fn aggregate_monster<F>(
        monster: &[crate::monster::MonsterConfig<Data>],
        monster_counts: &FnvIndexMap<Cow<'_, str>, usize>,
        is_elite: F,
    ) -> String
    where
        F: Fn(&crate::monster::MonsterConfig<Data>) -> bool,
    {
        monster
            .iter()
            .filter(|&monster| is_elite(monster))
            .map(crate::monster::MonsterConfig::wiki_name)
            .collect::<indexmap::IndexSet<_, fnv::FnvBuildHasher>>()
            .into_iter()
            .map(|name| format!("{name}:{}", monster_counts[&name]))
            .map(Cow::Owned)
            .intersperse(Cow::Borrowed("、"))
            .collect()
    }

    fn story_wiki_assertions(&self, mazes: &[ChallengeMazeConfig<Data>]) {
        for maze in mazes {
            assert_eq!(maze.event_list_1.len(), 1, "上半场景中只有一个 NPC 敌方");
            assert_eq!(maze.event_list_2.len(), 1, "下半场景中只有一个 NPC 敌方");
            for wave in maze.event_list_1[0].infinite_group().unwrap().wave_list {
                assert_eq!(wave.monster_group_list.len(), 1, "上半每一波只有一个敌人组");
                for monster in &wave.monster_group_list[0].monster_list {
                    assert_eq!(
                        monster.hard_level_group[0].id, maze.event_list_1[0].hard_level_group.id,
                        "monster 的 hard_level_group 和 stage 的 hard_level_group 应当是同一个"
                    );
                }
            }
            for wave in maze.event_list_2[0].infinite_group().unwrap().wave_list {
                assert_eq!(wave.monster_group_list.len(), 1, "下半每一波只有一个敌人组");
                for monster in &wave.monster_group_list[0].monster_list {
                    assert_eq!(
                        monster.hard_level_group[0].id, maze.event_list_2[0].hard_level_group.id,
                        "monster 的 hard_level_group 和 stage 的 hard_level_group 应当是同一个"
                    );
                }
            }
        }
    }

    /// story_write_event 解析一层中每一波次的敌人的信息
    /// 如第四层下半第 3 波的信息，会往 wiki 中写入 "|其四2队3波敌人=......"
    fn story_wiki_write_event(
        &self,
        wiki: &mut String,
        floor: u8,
        team: u8,
        event: &crate::battle::StageConfig<Data>,
        weakness: &[Element],
    ) {
        let floor_team = format!("\n|其{}{}队", Self::CHNUM[floor as usize - 1], team);
        let infinite_group = event.infinite_group().unwrap();
        let mut weakness_scores =
            fnv::FnvHashMap::from_iter(Self::ELEMENTS.into_iter().map(|element| (element, 0)));
        let mut max_teammate = [0; 3];
        let story_type = self
            .extra()
            .story_type
            .unwrap_or(ChallengeStoryType::Normal);
        for (wave_no, wave) in infinite_group.wave_list.iter().enumerate() {
            max_teammate[wave_no] = wave.max_teammate_count;
            let wave_no = (wave_no + 1).to_string();
            let monsters = &wave.monster_group_list[0].monster_list;
            let monster_counts = monsters
                .iter()
                .map(|monster| (monster.wiki_name(), ()))
                .collect::<FnvMultiMap<_, _>>()
                .into_iter()
                .map(|(name, vec)| (name, vec.len()))
                .collect::<FnvIndexMap<_, _>>();
            if story_type == ChallengeStoryType::Normal {
                // 计算每一种弱点在每一波中能拿到的分数
                // 就是假如只将含有某种弱点的怪物全部击败，所能获得的总分
                for monster in monsters {
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
            }
            let normal_names =
                Self::aggregate_monster(monsters, &monster_counts, |monster| match story_type {
                    ChallengeStoryType::Normal => self.monster_score(monster) == 500,
                    ChallengeStoryType::Fever => monster.is_minion(),
                });
            wiki.push_str(&floor_team);
            wiki.push_str(&wave_no);
            wiki.push_str("波=");
            wiki.push_str(&normal_names);
            let elite_names =
                Self::aggregate_monster(monsters, &monster_counts, |monster| match story_type {
                    ChallengeStoryType::Normal => self.monster_score(monster) > 500,
                    ChallengeStoryType::Fever => !monster.is_minion(),
                });
            wiki.push_str(&floor_team);
            wiki.push_str(&wave_no);
            wiki.push_str("波特殊敌方=");
            wiki.push_str(&elite_names);
        }
        wiki.push_str(&floor_team);
        wiki.push_str("敌方上限=");
        if max_teammate[0] == max_teammate[1] && max_teammate[1] == max_teammate[2] {
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
        let weaknesses: String = weakness
            .iter()
            .map(Element::wiki)
            .intersperse(Cow::Borrowed("、"))
            .collect();
        wiki.push_str(&floor_team);
        wiki.push_str("建议属性=");
        wiki.push_str(&weaknesses);
        if story_type == ChallengeStoryType::Normal {
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
                            return u32::cmp(&r.1, &l.1); // 注意排序是逆向的
                        }
                        u8::cmp(&(l.0 as u8), &(r.0 as u8))
                    }
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (Some(l_index), Some(r_index)) => {
                        if l.1 != r.1 {
                            return u32::cmp(&r.1, &l.1); // 注意排序是逆向的
                        }
                        usize::cmp(&l_index, &r_index)
                    }
                }
            });
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
        event: &crate::battle::StageConfig<Data>,
    ) {
        let floor_team = format!("\n|其{}{}队", Self::CHNUM[floor as usize - 1], team);
        let infinite_group = event.infinite_group().unwrap();
        for (wave_no, wave) in infinite_group.wave_list.iter().enumerate() {
            match wave.ability {
                "FantasticStory_Wave_Ability_0001" => {
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
                "" => (), // 旧版虚构叙事，懒得处理了
                // 虚构叙事定制的 Ability，目前只有给怪物增幅攻击和生命。
                // 具体配置文件见 Config/ConfigAbility/BattleEvent/FantasticStory_Wave_Ability.json
                // assert 的 ability 的效果是按 param_list 分别增幅攻击和生命上限
                // 解析 Config 非常复杂，先写死，如果有改动的话，就人工处理吧
                _ => unreachable!(),
            }
        }
    }

    fn story_special_monster_wiki(
        &self,
        mazes: &[ChallengeMazeConfig<Data>],
        infinite_groups: &[crate::battle::StageInfiniteGroup<Data>],
    ) -> Cow<'static, str> {
        let mut specials = FnvIndexMap::default();
        let mut floors = HashMap::<_, Vec<u8>>::new();
        for (index, maze) in mazes.iter().enumerate() {
            let floor = if maze.floor != 0 {
                maze.floor
            } else {
                index as u8 + 1
            };
            infinite_groups[index * 2..index * 2 + 2]
                .iter()
                .flat_map(|group| &group.wave_list)
                .flat_map(|wave| &wave.monster_group_list)
                .flat_map(|group| &group.monster_list)
                .filter(|monster| monster.is_special())
                .for_each(|monster| {
                    floors.entry(monster.id).or_default().push(floor);
                    specials.insert(monster.id, monster);
                });
        }
        self.special_monster_wiki(specials, floors)
    }

    fn story_reinforce_wiki(&self, mazes: &[ChallengeMazeConfig<Data>]) -> String {
        let mut wiki = String::from("{{虚构叙事增援序列");
        fn handle<Data: ExcelOutput>(
            wiki: &mut String,
            event: &crate::battle::StageConfig<Data>,
            floor: u8,
            team: u8,
        ) {
            let infinite_group = event.infinite_group().unwrap();
            for (wave_no, wave) in infinite_group.wave_list.into_iter().enumerate() {
                let wave_no = (wave_no + 1).to_string();
                let monsters = &wave.monster_group_list[0].monster_list;
                let reinforces: String = monsters
                    .iter()
                    .map(crate::monster::MonsterConfig::wiki_name)
                    .intersperse(Cow::Borrowed("、"))
                    .collect();
                wiki.push_str("\n|其");
                wiki.push_str(ChallengeGroupConfig::<Data>::CHNUM[floor as usize - 1]);
                wiki.push_str(if team == 1 { "上半" } else { "下半" });
                wiki.push('第');
                wiki.push_str(&wave_no);
                wiki.push_str("波=");
                wiki.push_str(&reinforces);
            }
        }
        for (index, maze) in mazes.iter().enumerate() {
            let floor = if maze.floor != 0 {
                maze.floor
            } else {
                index as u8 + 1
            };
            handle(&mut wiki, &maze.event_list_1[0], floor, 1);
            handle(&mut wiki, &maze.event_list_2[0], floor, 2);
        }
        wiki.push_str("\n}}");
        wiki
    }

    /// 用来确保 monster_score 返回的值是正确的
    /// monster_score 的值每个版本都有可能不同，但是没法从数据集中提取出来，都是自己试出来的
    fn story_wiki_score_assertions(
        &self,
        mazes: &[ChallengeMazeConfig<Data>],
        extra: &ChallengeGroupExtra,
    ) {
        fn handle<Data: ExcelOutput>(
            story: &ChallengeGroupConfig<Data>,
            event: &crate::battle::StageConfig<Data>,
        ) -> u32 {
            event
                .infinite_group()
                .unwrap()
                .wave_list
                .into_iter()
                .flat_map(|infs| infs.monster_group_list)
                .flat_map(|mons| mons.monster_list)
                .map(|mon| story.monster_score(&mon))
                .sum()
        }
        match extra.story_type.unwrap() {
            ChallengeStoryType::Normal => {
                assert_eq!(handle(self, &mazes[0].event_list_1[0]), 40000, "第一层上半");
                assert_eq!(handle(self, &mazes[0].event_list_2[0]), 40000, "第一层下半");
                assert_eq!(handle(self, &mazes[1].event_list_1[0]), 40000, "第二层上半");
                assert_eq!(handle(self, &mazes[1].event_list_2[0]), 40000, "第二层下半");
                assert_eq!(handle(self, &mazes[2].event_list_1[0]), 40000, "第三层上半");
                assert_eq!(handle(self, &mazes[2].event_list_2[0]), 40000, "第三层下半");
                assert_eq!(handle(self, &mazes[3].event_list_1[0]), 40000, "第四层上半");
                assert_eq!(handle(self, &mazes[3].event_list_2[0]), 40000, "第四层下半");
            }
            ChallengeStoryType::Fever => (), // 还不知道怎么处理
        }
    }

    fn story_wiki_elite_group_in_comments(
        &self,
        mazes: &[ChallengeMazeConfig<Data>],
    ) -> Cow<'static, str> {
        let mut wiki = String::new();
        // 下面的注释用于提示虚构叙事敌方阵容是否换代
        let mut elite_groups = std::collections::HashMap::<u16, crate::monster::EliteGroup>::new();
        let mut elite_application = multimap::MultiMap::<u16, String>::new();
        for maze in mazes {
            for (half, event) in [
                ("上半", &maze.event_list_1[0]),
                ("下半", &maze.event_list_2[0]),
            ] {
                for (wave_no, wave) in event.infinite_group().unwrap().wave_list.iter().enumerate()
                {
                    const EMPTY_ELITE_GROUP: crate::monster::EliteGroup =
                        crate::monster::EliteGroup {
                            id: 0,
                            attack_ratio: 1.,
                            defence_ratio: 1.,
                            hp_ratio: 1.,
                            speed_ratio: 1.,
                            stance_ratio: 1.,
                        };
                    let group = wave.monster_group_list[0]
                        .elite_group
                        .as_ref()
                        .unwrap_or(&EMPTY_ELITE_GROUP);
                    elite_application.insert(
                        group.id,
                        format!("第{}层{}第{}波", maze.floor, half, wave_no + 1),
                    );
                    elite_groups.insert(group.id, group.clone());
                }
            }
        }
        if elite_groups.len() == 1 {
            wiki.push_str(&format!(
                "<!-- 当期敌方属性增幅（降幅）： {:?} -->",
                elite_groups.into_values().next().unwrap()
            ));
        } else {
            wiki.push_str("<!-- 当期敌方属性增幅（降幅）：\n");
            for (id, group) in &elite_groups {
                let applied = elite_application.get_vec(id).unwrap().join("、");
                wiki.push_str(&format!("{applied}：{group:?}\n"));
            }
            wiki.push_str("-->");
        }
        Cow::Owned(wiki)
    }

    fn story_wiki(&self) -> Cow<'static, str> {
        let mut formatter = format::Formatter::new(self.game).media_wiki_syntax(true);
        let mazes = self.mazes();
        // 开头两个 assert 确保数据一致性
        self.story_wiki_assertions(mazes);
        self.story_wiki_score_assertions(mazes, self.extra());
        let extra = self.extra();
        assert_eq!(extra.buff_list.len(), 3, "虚构叙事固定三个增益");
        if extra.story_type == Some(ChallengeStoryType::Fever) {
            assert_eq!(
                extra.sub_maze_buff_list.len(),
                3,
                "新版虚构虚构叙事固定三个战意机制"
            );
        }

        let mut wiki = String::from("{{虚构叙事单期");
        if extra.story_type == Some(ChallengeStoryType::Fever) {
            // 虚构叙事第 11 期开始更新为新版虚构叙事
            // 主要是新增战意和 Boss 共享血量
            // BWIKI 同时换模板
            wiki.push('2');
        }
        wiki.push_str("\n|期数=");
        wiki.push_str(&format!("{:03}", self.issue()));
        wiki.push_str("\n|名称=");
        wiki.push_str(self.name);
        self.wiki_write_sched(&mut wiki);
        match extra.story_type.unwrap_or(ChallengeStoryType::Normal) {
            ChallengeStoryType::Normal => {
                self.wiki_write_buff(
                    &mut wiki,
                    &mut formatter,
                    "怪诞逸闻",
                    self.maze_buff.as_ref(),
                );
            }
            ChallengeStoryType::Fever => {
                // 0 => 战意机制
                // 1 => 战熄潮平
                // 2 => 战意汹涌
                wiki.push_str("\n|战意机制名称=");
                wiki.push_str(extra.sub_maze_buff_list[0].name);
                wiki.push_str("\n|战意机制效果=");
                let buff = &extra.sub_maze_buff_list[0];
                wiki.push_str(&formatter.format(buff.desc, &buff.params));
                wiki.push_str("\n|战熄潮平=");
                let buff = &extra.sub_maze_buff_list[1];
                wiki.push_str(&formatter.format(buff.desc, &buff.params));
                wiki.push_str("\n|战意汹涌=");
                let buff = &extra.sub_maze_buff_list[2];
                wiki.push_str(&formatter.format(buff.desc, &buff.params));
            }
        }
        for (index, buff) in extra.buff_list.iter().enumerate() {
            wiki.push_str("\n|荒腔");
            wiki.push_str(&(index + 1).to_string());
            wiki.push('=');
            wiki.push_str(buff.name);
            wiki.push_str("\n|荒腔走板其");
            wiki.push_str(Self::CHNUM[index]);
            wiki.push('=');
            wiki.push_str(&formatter.format(buff.desc, &buff.params));
            if extra.story_type == Some(ChallengeStoryType::Fever) {
                wiki.push_str("\n|荒腔走板其");
                wiki.push_str(Self::CHNUM[index]);
                wiki.push_str("战意机制");
                wiki.push('=');
                wiki.push_str(&formatter.format(buff.simple_desc, &buff.params));
            }
        }
        for maze in mazes {
            assert_eq!(maze.event_list_1.len(), 1, "只有一个 event");
            assert_eq!(maze.event_list_2.len(), 1, "只有一个 event");
        }
        for maze in mazes {
            self.story_wiki_write_event(
                &mut wiki,
                maze.floor,
                1,
                &maze.event_list_1[0],
                maze.damage_type_1,
            );
            self.story_wiki_write_event(
                &mut wiki,
                maze.floor,
                2,
                &maze.event_list_2[0],
                maze.damage_type_2,
            );
        }
        for maze in mazes {
            Self::story_wiki_write_wave_ability(&mut wiki, maze.floor, 1, &maze.event_list_1[0]);
            Self::story_wiki_write_wave_ability(&mut wiki, maze.floor, 2, &maze.event_list_2[0]);
        }
        wiki.push_str("\n}}");
        let infinite_groups = mazes
            .iter()
            .flat_map(|maze| [&maze.event_list_1[0], &maze.event_list_2[0]])
            .map(crate::battle::StageConfig::infinite_group)
            .map(Option::unwrap)
            .collect::<Vec<_>>();
        wiki.push_str(&self.story_special_monster_wiki(mazes, &infinite_groups));
        wiki.push_str("\n<br />\n<br />\n----\n\n");
        wiki.push_str(&self.story_reinforce_wiki(mazes));
        wiki.push_str("\n\n");
        wiki.push_str(&self.story_wiki_elite_group_in_comments(mazes));
        Cow::Owned(wiki)
    }
}

impl<Data: ExcelOutput + format::GameData> ChallengeGroupConfig<'_, Data> {
    fn boss_wiki_assertions(&self, mazes: &[ChallengeMazeConfig<Data>]) {
        for maze in mazes {
            assert_eq!(mazes[0].maze_buff.id, maze.maze_buff.id, "同期增益相同");
            // 上半
            assert_eq!(maze.event_list_1.len(), 1);
            let event = &maze.event_list_1[0];
            assert_eq!(event.monster_list.len(), 1, "上半只有一波怪物");
            // 敌方可能会召唤随从，随从会出现在 monster_list 中，我们直接无视非首项
            // 目前唯一会召唤随从的特例：可可利亚会召唤杰帕德
            assert!(event.monster_list[0][0].template.is_some(), "怪物模板非空");
            let event0 = &mazes[0].event_list_1[0];
            let template0 = event0.monster_list[0][0].template.as_ref().unwrap();
            let template = event.monster_list[0][0].template.as_ref().unwrap();
            assert_eq!(template0.id, template.id, "上半同期怪物模板相同");
            // 下半
            assert_eq!(maze.event_list_2.len(), 1);
            let event0 = &mazes[0].event_list_2[0];
            let event = &maze.event_list_2[0];
            assert_eq!(event.monster_list.len(), 1, "下半只有一波怪物");
            assert!(event.monster_list[0][0].template.is_some(), "怪物模板非空");
            let template0 = event0.monster_list[0][0].template.as_ref().unwrap();
            let template = event.monster_list[0][0].template.as_ref().unwrap();
            assert_eq!(template0.id, template.id, "下半同期怪物模板相同");
        }
    }

    fn boss_wiki_write_tags(
        &self,
        wiki: &mut String,
        team: u8,
        event: &crate::battle::StageConfig<Data>,
        extra: &ChallengeGroupExtra,
    ) {
        let mut formatter = format::Formatter::new(self.game).media_wiki_syntax(true);
        let half = if team == 1 { "上半" } else { "下半" };
        wiki.push_str("\n|");
        wiki.push_str(half);
        wiki.push_str("名称=");
        wiki.push_str(event.name);
        let monster = &event.monster_list[0][0];
        wiki.push_str("\n|");
        wiki.push_str(half);
        wiki.push_str("首领=");
        wiki.push_str(&monster.wiki_name());
        let guide = self.game.monster_guide_config(monster.id).unwrap();
        for (tag_no, tag) in guide.tag_list.iter().enumerate() {
            let tag_no = (tag_no + 1).to_string();
            wiki.push_str("\n|");
            wiki.push_str(half);
            wiki.push_str("特性");
            wiki.push_str(&tag_no);
            wiki.push_str("类型=");
            if tag_no == "1" || tag_no == "2" {
                wiki.push_str("固有特性");
            } else {
                wiki.push_str("难度");
                wiki.push_str(&tag_no);
                wiki.push_str("增加");
            }
            wiki.push_str("\n|");
            wiki.push_str(half);
            wiki.push_str("特性");
            wiki.push_str(&tag_no);
            wiki.push_str("名称=");
            wiki.push_str(tag.name);
            wiki.push_str("\n|");
            wiki.push_str(half);
            wiki.push_str("特性");
            wiki.push_str(&tag_no);
            wiki.push('=');
            let mut description = formatter.format(tag.brief_description, &tag.parameter_list);
            let mut effect_explain = String::new();
            for effect in &tag.effect {
                let effect_wiki = String::from("{{效果说明|") + effect.name + "}}";
                description = description.replace(&effect_wiki, &format!("<u>{}</u>", effect.name));
                effect_explain.push_str("<br />'''· ");
                effect_explain.push_str(effect.name);
                effect_explain.push_str("'''<br />");
                effect_explain.push_str(effect.desc);
            }
            wiki.push_str(&description);
            if !effect_explain.is_empty() {
                wiki.push_str("<br />");
                wiki.push_str(&effect_explain);
            }
        }
        let buff_list = if team == 1 {
            extra.buff_list_1.as_slice()
        } else {
            extra.buff_list_2.as_slice()
        };
        assert_eq!(buff_list.len(), 3, "末日幻影固定 3 个增益");
        for (buff_no, buff) in buff_list.iter().enumerate() {
            let no = (team as usize * 3 + buff_no - 2).to_string();
            wiki.push_str("\n|终焉公理");
            wiki.push_str(&no);
            wiki.push_str("名称=");
            wiki.push_str(buff.name);
            wiki.push_str("\n|终焉公理");
            wiki.push_str(&no);
            wiki.push('=');
            wiki.push_str(&formatter.format(buff.desc, &buff.params));
        }
    }

    fn boss_wiki(&self) -> Cow<'static, str> {
        let mut formatter = format::Formatter::new(self.game).media_wiki_syntax(true);
        let mazes = self.mazes();
        self.boss_wiki_assertions(mazes);

        let mut wiki = String::from("{{末日幻影单期");
        wiki.push_str("\n|期数=");
        wiki.push_str(&format!("{:03}", self.issue()));
        wiki.push_str("\n|名称=");
        wiki.push_str(self.name);

        self.wiki_write_sched(&mut wiki);
        self.wiki_write_buff(
            &mut wiki,
            &mut formatter,
            "末法余烬",
            Some(&mazes[0].maze_buff),
        );
        let maze = &mazes[2];
        let extra = self.extra();
        self.boss_wiki_write_tags(&mut wiki, 1, &maze.event_list_1[0], extra);
        self.boss_wiki_write_tags(&mut wiki, 2, &maze.event_list_2[0], extra);
        wiki.push_str("\n}}\n<br />\n<br />\n----");
        Cow::Owned(wiki)
    }
}

impl<Data: ExcelOutput + format::GameData> Wiki for ChallengeGroupConfig<'_, Data> {
    /// 备注一下各个深渊计算敌方属性的方法
    ///
    /// - 混沌回忆，敌方属性来源于敌方基础数据
    ///   再乘以当前层上下半精英组别增益，乘以当前层上下半成长曲线数据。
    ///   需要注意的是不能乘以怪物模板中敌方本身的成长曲线，
    ///   这个数据会被混沌回忆自带的成长曲线数据覆盖。
    ///   具体公式是：
    ///   ``` rust
    ///   let hp = monster.template.hp_base * monster.hp_modify_ratio \
    ///       * maze.event_list_1[0].elite_group.hp_ratio \
    ///       * maze.event_list_1[0].hard_level_group.hp_ratio
    ///   ```
    /// - 虚构叙事，敌方属性可以直接取自敌方在当前层等级下的生命
    ///   再乘以当前波次的 1 + param_list\[1\] 即可。
    ///   这意味着虚构叙事下敌方模板中本身的成长曲线和各自层里的是一致的。
    ///   顺便一提 param_list\[0\] 是攻击增益。
    ///   具体公式是：
    ///   ```rust
    ///   let hp = monster.hp_at(maze.event_list_1[0].level) \
    ///       * (1. + wave.param_list.get(1).unwrap_or_default())
    ///   ```
    /// - 末日幻影，敌方属性可以直接取自敌方在当前层等级下的生命
    ///   再乘以当前层上下半的精英组别增益即可。
    ///   具体公式是：
    ///   ```rust
    ///   let hp = monster.hp_at(maze.event_list_1[0].level) \
    ///       * maze.event_list_1[0].elite_group.hp_ratio
    ///   ```
    fn wiki(&self) -> Cow<'static, str> {
        match self.r#type {
            ChallengeGroupType::Memory => self.memory_wiki(),
            ChallengeGroupType::Story => self.story_wiki(),
            ChallengeGroupType::Boss => self.boss_wiki(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ChallengeGroupExtra<'a> {
    pub id: u16,
    pub sub_maze_buff_list: Vec<crate::misc::MazeBuff<'a>>,
    pub story_type: Option<ChallengeStoryType>,
    pub buff_list: Vec<crate::misc::MazeBuff<'a>>,
    pub buff_list_1: Vec<crate::misc::MazeBuff<'a>>,
    pub buff_list_2: Vec<crate::misc::MazeBuff<'a>>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for ChallengeGroupExtra<'a> {
    type Model = model::challenge::ChallengeGroupExtra;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        let assemble = |buffs: [u32; 3]| buffs.iter().flat_map(|&id| game.maze_buff(id)).collect();
        Self {
            id: model.group_id,
            sub_maze_buff_list: model
                .sub_maze_buff_list
                .as_deref()
                .unwrap_or_default()
                .iter()
                .flat_map(|&id| game.maze_buff(id))
                .collect(),
            story_type: model.story_type,
            buff_list: model.buff_list.map(assemble).unwrap_or_default(),
            buff_list_1: model.buff_list_1.map(assemble).unwrap_or_default(),
            buff_list_2: model.buff_list_2.map(assemble).unwrap_or_default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ChallengeMazeConfig<'a, Data: ExcelOutput + ?Sized> {
    pub id: u16,
    pub name: &'a str,
    pub group: ChallengeGroupConfig<'a, Data>,
    pub map_entrance: crate::map::MapEntrance<'a>,
    pub map_entrance_2: Option<crate::map::MapEntrance<'a>>,
    pub pre_level: u8,
    pub pre_challenge_maze_id: u16,
    /// 本 Maze 所在层数
    /// 混沌回忆一期共有 12 层 Maze
    /// 虚构叙事一期共有 4 层 Maze
    /// 末日幻影一期共有 4 层 Maze
    pub floor: u8,
    pub reward: crate::misc::RewardData<'a>,
    /// 上半层敌方主要弱点属性（官方推荐属性）
    pub damage_type_1: &'a [Element],
    /// 下半层敌方主要弱点属性（官方推荐属性）
    pub damage_type_2: &'a [Element],
    pub target: [ChallengeTargetConfig<'a>; 3],
    pub stage_num: u8,
    /// 不清楚，大部分是空的，存在非空的情况，但是似乎和当期敌人无关
    pub monster_1: Vec<crate::monster::MonsterConfig<'a, Data>>,
    /// 不清楚，大部分是空的，存在非空的情况，但是似乎和当期敌人无关
    pub monster_2: Vec<crate::monster::MonsterConfig<'a, Data>>,
    /// 回合数内打倒敌人，仅出现在混沌回忆中
    pub challenge_count_down: u8,
    /// 上半场地图上站桩的敌人
    pub npc_monster_id_list_1: Vec<crate::monster::NPCMonsterData<'a>>,
    /// 深渊同层上半信息
    /// 1.6 版本开始混沌回忆、所有虚构叙事、所有末日幻影中，该字段长度均为 1
    /// 此前的旧混沌回忆和常驻回忆中存在一些同一层两个首领左右分立的情况，此时长度为 2
    pub event_list_1: Vec<crate::battle::StageConfig<'a, Data>>,
    /// 下半场地图上站桩的敌人
    pub npc_monster_id_list_2: Vec<crate::monster::NPCMonsterData<'a>>,
    /// 深渊同层下半信息
    /// 1.6 版本开始混沌回忆、所有虚构叙事、所有末日幻影中，该字段长度均为 1
    /// 此前的旧混沌回忆和常驻回忆中存在一些同一层两个首领左右分立的情况，此时长度为 2
    /// 入门的常驻回忆前几层没有下半，此时长度为 0
    pub event_list_2: Vec<crate::battle::StageConfig<'a, Data>>,
    pub maze_buff: crate::misc::MazeBuff<'a>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for ChallengeMazeConfig<'a, Data> {
    type Model = model::challenge::ChallengeMazeConfig;
    fn from_model(game: &'a Data, model: &'a Self::Model) -> Self {
        let group = None
            .or_else(|| game.challenge_group_config(model.group_id))
            .or_else(|| game.challenge_story_group_config(model.group_id))
            .or_else(|| game.challenge_boss_group_config(model.group_id))
            .unwrap();
        let group_type = group.r#type;
        Self {
            id: model.id,
            name: game.text(model.name),
            group,
            map_entrance: game.map_entrance(model.map_entrance_id).unwrap(),
            map_entrance_2: model
                .map_entrance_id_2
                .map(NonZero::get)
                .map(|id| game.map_entrance(id))
                .map(Option::unwrap),
            pre_level: model.pre_level.map(NonZero::get).unwrap_or_default(),
            pre_challenge_maze_id: model
                .pre_challenge_maze_id
                .map(NonZero::get)
                .unwrap_or_default(),
            floor: model.floor.map(NonZero::get).unwrap_or_default(),
            reward: game.reward_data(model.reward_id).unwrap(),
            damage_type_1: &model.damage_type_1,
            damage_type_2: &model.damage_type_2,
            target: std::array::from_fn(|index| {
                (match group_type {
                    ChallengeGroupType::Memory => Data::challenge_target_config,
                    ChallengeGroupType::Story => Data::challenge_story_target_config,
                    ChallengeGroupType::Boss => Data::challenge_boss_target_config,
                })(game, model.challenge_target_id[index])
                .unwrap()
            }),
            stage_num: model.stage_num,
            monster_1: model
                .monster_id_1
                .iter()
                .map(|&id| game.monster_config(id))
                .map(Option::unwrap)
                .collect(),
            monster_2: model
                .monster_id_2
                .iter()
                .map(|&id| game.monster_config(id))
                .map(Option::unwrap)
                .collect(),
            challenge_count_down: model
                .challenge_count_down
                .map(NonZero::get)
                .unwrap_or_default(),
            npc_monster_id_list_1: model
                .npc_monster_id_list_1
                .iter()
                .map(|&id| game.npc_monster_data(id))
                .map(Option::unwrap)
                .collect(),
            event_list_1: model
                .event_id_list_1
                .iter()
                .map(|&id| game.stage_config(id))
                .map(Option::unwrap)
                .collect(),
            npc_monster_id_list_2: model
                .npc_monster_id_list_2
                .iter()
                .map(|&id| game.npc_monster_data(id))
                .map(Option::unwrap)
                .collect(),
            event_list_2: model
                .event_id_list_2
                .iter()
                .map(|&id| game.stage_config(id))
                .map(Option::unwrap)
                .collect(),
            maze_buff: game
                .maze_buff(model.maze_buff_id)
                .into_iter()
                .next()
                .unwrap(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ChallengeMazeExtra<'a, Data: ExcelOutput + ?Sized> {
    pub id: u16,
    pub turn_limit: u8,
    pub monster_1: Option<crate::monster::MonsterConfig<'a, Data>>,
    pub monster_2: Option<crate::monster::MonsterConfig<'a, Data>>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for ChallengeMazeExtra<'a, Data> {
    type Model = model::challenge::ChallengeMazeExtra;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            id: model.id,
            turn_limit: model.turn_limit.map(NonZero::get).unwrap_or_default(),
            monster_1: model
                .monster_id_1
                .map(NonZero::get)
                .map(|id| game.monster_config(id))
                .map(Option::unwrap),
            monster_2: model
                .monster_id_2
                .map(NonZero::get)
                .map(|id| game.monster_config(id))
                .map(Option::unwrap),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ChallengeRewardLine<'a> {
    pub group_id: u16,
    pub star_count: u8,
    pub reward: crate::misc::RewardData<'a>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for ChallengeRewardLine<'a> {
    type Model = model::challenge::RewardLine;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            group_id: model.group_id,
            star_count: model.star_count,
            reward: game.reward_data(model.reward_id).unwrap(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ChallengeTargetConfig<'a> {
    pub id: u16,
    pub r#type: ChallengeTargetType,
    pub name: &'a str,
    pub params: [format::Argument<'a>; 1],
    /// 不明，不是 RewardData
    /// 只有 ChallengeBossTargetConfig.json 没有 RewardID
    pub reward_id: u32,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for ChallengeTargetConfig<'a> {
    type Model = model::challenge::ChallengeTargetConfig;
    fn from_model(game: &'a Data, model: &'a Self::Model) -> Self {
        Self {
            id: model.id,
            r#type: model.challenge_target_type,
            name: game.text(model.challenge_target_name),
            params: [format::Argument::from(model.challenge_target_param_1)],
            reward_id: model.reward_id.map(NonZero::get).unwrap_or_default(),
        }
    }
}
