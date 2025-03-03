pub mod tourn;

use std::{borrow::Cow, num::NonZero};

use base::{Name, Wiki};
pub use model::rogue::RogueBuffCategory;

use crate::{ExcelOutput, FromModel};

#[derive(educe::Educe)]
#[educe(Clone, Debug)]
/// 模拟宇宙奇物
pub struct RogueBuff<'a, Data: ExcelOutput + ?Sized> {
    #[educe(Debug(ignore))]
    game: &'a Data,
    pub id: u32,
    pub level: u8,
    pub buff: Option<crate::misc::MazeBuff<'a>>,
    pub r#type: RogueBuffType<'a>,
    pub category: Option<RogueBuffCategory>,
    pub tag: u32,
    pub extra_effect_list: Vec<crate::misc::ExtraEffectConfig<'a>>,
    pub handbook_unlock_desc: &'a str,
    pub aeon_cross_icon: &'a str,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for RogueBuff<'a, Data> {
    type Model = model::rogue::RogueBuff;
    fn from_model(game: &'a Data, model: &'a Self::Model) -> Self {
        Self {
            game,
            id: model.maze_buff_id,
            level: model.maze_buff_level,
            r#type: game.rogue_buff_type(model.rogue_buff_type).unwrap(),
            buff: game
                .rogue_maze_buff(model.maze_buff_id)
                .into_iter()
                .nth(model.maze_buff_level as usize - 1),
            category: model
                .rogue_buff_rarity
                .and_then(|rarity| match rarity.get() {
                    1 => Some(RogueBuffCategory::Common),
                    3 => Some(RogueBuffCategory::Legendary),
                    2 => Some(RogueBuffCategory::Rare),
                    4 => None,
                    _ => unreachable!(),
                })
                .or(model.rogue_buff_category),
            tag: model.rogue_buff_tag,
            extra_effect_list: model
                .extra_effect_id_list
                .iter()
                .map(|&id| {
                    None.or_else(|| game.extra_effect_config(id))
                        .or_else(|| game.rogue_extra_config(id))
                })
                .map(Option::unwrap)
                .collect(),
            handbook_unlock_desc: model
                .handbook_unlock_desc
                .map(|text| game.text(text))
                .unwrap_or_default(),
            aeon_cross_icon: model.aeon_cross_icon.as_deref().unwrap_or_default(),
        }
    }
}

impl<Data: ExcelOutput + format::GameData> Name for RogueBuff<'_, Data> {
    fn name(&self) -> &str {
        self.buff.as_ref().map(|buff| buff.name).unwrap_or_default()
    }

    fn wiki_name(&self) -> std::borrow::Cow<'_, str> {
        Cow::Owned(
            self.name()
                .replace("\u{00A0}", "")
                .replace("<unbreak>", "")
                .replace("</unbreak>", ""),
        )
    }
}

impl<Data: ExcelOutput + format::GameData> Wiki for RogueBuff<'_, Data> {
    fn wiki(&self) -> Cow<'static, str> {
        if self.level != 1 || self.buff.is_none() {
            return Cow::Borrowed("");
        }
        let buff = self.buff.as_ref().unwrap();
        let mut formatter = format::Formatter::new(self.game).media_wiki_syntax(true);
        let mut wiki = String::new();
        let tourn = self.game.rogue_tourn_buff_by_name(buff.name);
        wiki.push_str("{{模拟宇宙祝福");
        wiki.push_str("\n|名称=");
        wiki.push_str(&self.wiki_name());
        if let Some(category) = self.category {
            wiki.push_str("\n|稀有度=");
            wiki.push_str(&category.wiki());
        }
        let path = &self.r#type.title[3..9];
        wiki.push_str("\n|命途=");
        wiki.push_str(path);
        wiki.push_str("\n|模式=模拟宇宙");
        wiki.push_str(match path {
            "繁育" => "、寰宇蝗灾、黄金与机械",
            "智识" => "、黄金与机械",
            _ => "、寰宇蝗灾、黄金与机械",
        });
        if tourn.is_some() {
            wiki.push_str("、差分宇宙");
        }
        wiki.push_str("\n|效果=");
        let desc = formatter.format(buff.desc, &buff.params);
        wiki.push_str(&desc);
        let mut upgrade_desc = String::new();
        if let Some(upgrade) = self.game.rogue_maze_buff(self.id).get(1) {
            wiki.push_str("\n|强化后效果=");
            upgrade_desc = formatter.format(upgrade.desc, &upgrade.params);
            wiki.push_str(&upgrade_desc);
        }
        if let Some(tourn) = self.game.rogue_tourn_buff_by_name(buff.name) {
            if self.buff.as_ref().map(|buff| buff.id).unwrap_or_default() != tourn.buff.id {
                // 如果 id 相同，那么描述肯定相同，直接跳过
                let divergent_desc = formatter.format(tourn.buff.desc, &tourn.buff.params);
                let divergent_upgrade = self.game.rogue_maze_buff(tourn.id);
                let divergent_upgrade_desc = divergent_upgrade
                    .get(1)
                    .map(|upgrade| formatter.format(upgrade.desc, &upgrade.params))
                    .unwrap_or_default();
                // 否则判断文案完全相同才继续录入（可能存在升级前相同但是升级后不同，所以需要完整比较）
                if desc != divergent_desc || upgrade_desc != divergent_upgrade_desc {
                    wiki.push_str("\n|差分效果=");
                    wiki.push_str(&divergent_desc);
                    if let Some(upgrade) = self.game.rogue_maze_buff(tourn.id).get(1) {
                        wiki.push_str("\n|差分强化效果=");
                        wiki.push_str(&formatter.format(upgrade.desc, &upgrade.params));
                    }
                }
            }
        }
        wiki.push_str("\n|TAG=");
        wiki.push_str("\n|实装版本=");
        wiki.push_str("\n|类型=");
        wiki.push_str(match self.category {
            None => "",
            Some(RogueBuffCategory::Legendary) if self.name().starts_with("命途回响：") => "1",
            Some(RogueBuffCategory::Legendary) if self.name().starts_with("回响构音：") => "2",
            Some(RogueBuffCategory::Legendary) if self.name().starts_with("回响交错：") => "3",
            Some(RogueBuffCategory::Legendary) => "4",
            Some(RogueBuffCategory::Rare) => "5",
            Some(RogueBuffCategory::Common) => "6",
        });
        wiki.push_str("\n|排序=");
        wiki.push_str("\n}}");
        Cow::Owned(wiki)
    }
}

#[derive(Clone, Debug)]
pub struct RogueBuffType<'a> {
    pub id: u8,
    pub text: &'a str,
    pub title: &'a str,
    pub sub_title: &'a str,
    pub hint_desc: &'a str,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for RogueBuffType<'a> {
    type Model = model::rogue::RogueBuffType;
    fn from_model(game: &'a Data, model: &'a Self::Model) -> Self {
        Self {
            id: model.rogue_buff_type,
            text: game.text(model.rogue_buff_type_textmap_id),
            title: game.text(model.rogue_buff_type_title),
            sub_title: model
                .rogue_buff_type_sub_title
                .map(|text| game.text(text))
                .unwrap_or_default(),
            hint_desc: model
                .hint_desc
                .map(|text| game.text(text))
                .unwrap_or_default(),
        }
    }
}

#[derive(Clone, Debug)]
/// 模拟宇宙奇物
pub struct RogueMiracle<'a> {
    pub id: u16,
    /// 1.2 及之前版本的 display 为空
    pub display: RogueMiracleDisplay<'a>,
    pub effect_display: Option<RogueMiracleEffectDisplay<'a>>,
    pub desc: &'a str,
    pub desc_params: Vec<format::Argument<'a>>,
    /// 没有 unlock_handbook 的一般是可以同时携带多个、效果不同的奇物
    /// 如分裂咕咕钟、绝对失败处方
    pub unlock_handbook: Option<RogueHandbookMiracle<'a>>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for RogueMiracle<'a> {
    type Model = model::rogue::RogueMiracle;
    fn from_model(game: &'a Data, model: &'a Self::Model) -> Self {
        Self {
            id: model.miracle_id,
            // 存在一些奇物, 图鉴中展示的是模拟宇宙的效果, 游戏过程中展示的是差分宇宙的效果
            // 这一类奇物主要是差分宇宙新增的奇物和商店相关奇物 (邪恶机械卫星#900和「中等念头」群体机)
            display: if let Some(id) = model.miracle_display_id {
                None.or_else(|| game.rogue_miracle_display(id.get()))
                    .or_else(|| game.rogue_tourn_miracle_display(id.get()))
                    .unwrap()
            } else {
                RogueMiracleDisplay {
                    id: 0,
                    name: model
                        .miracle_name
                        .map(|text| game.text(text))
                        .unwrap_or_default(),
                    desc: model
                        .miracle_desc
                        .map(|hash| game.text(hash))
                        .unwrap_or_default(),
                    desc_params: model
                        .desc_param_list
                        .as_deref()
                        .map(format::Argument::from_array)
                        .unwrap_or_default(),
                    extra_effect: Vec::new(),
                    bg_desc: model
                        .miracle_bg_desc
                        .map(|text| game.text(text))
                        .unwrap_or_default(),
                    tag: model
                        .miracle_tag
                        .map(|text| game.text(text))
                        .unwrap_or_default(),
                    icon_path: model.miracle_icon_path.as_deref().unwrap_or_default(),
                    figure_icon_path: model
                        .miracle_figure_icon_path
                        .as_deref()
                        .unwrap_or_default(),
                }
            },
            effect_display: model
                .miracle_effect_display_id
                .map(NonZero::get)
                .map(|id| game.rogue_miracle_effect_display(id))
                .map(Option::unwrap),
            desc: model
                .miracle_desc
                .map(|hash| game.text(hash))
                .unwrap_or_default(),
            desc_params: model
                .desc_param_list
                .as_deref()
                .map(format::Argument::from_array)
                .unwrap_or_default(),
            unlock_handbook: model
                .unlock_handbook_miracle_id
                .map(NonZero::get)
                .map(|id| game.rogue_handbook_miracle(id))
                .map(Option::unwrap),
        }
    }
}

impl Name for RogueMiracle<'_> {
    fn name(&self) -> &str {
        self.display.name
    }
    fn wiki_name(&self) -> Cow<'_, str> {
        Cow::Borrowed(self.name())
    }
}

#[derive(Clone, Debug)]
/// 模拟宇宙奇物展示数据（效果、背景故事等）
pub struct RogueMiracleEffectDisplay<'a> {
    pub id: u16,
    pub desc: &'a str,
    pub simple_desc: &'a str,
    pub desc_params: Vec<format::Argument<'a>>,
    pub extra_effect: Vec<crate::misc::ExtraEffectConfig<'a>>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for RogueMiracleEffectDisplay<'a> {
    type Model = model::rogue::RogueMiracleEffectDisplay;
    fn from_model(game: &'a Data, model: &'a Self::Model) -> Self {
        Self {
            id: model.miracle_effect_display_id,
            desc: model
                .miracle_desc
                .map(|hash| game.text(hash))
                .unwrap_or_default(),
            simple_desc: model
                .miracle_simple_desc
                .map(|hash| game.text(hash))
                .unwrap_or_default(),
            desc_params: format::Argument::from_array(&model.desc_param_list),
            extra_effect: model
                .extra_effect
                .iter()
                .map(|&id| game.extra_effect_config(id))
                .map(Option::unwrap)
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
/// 模拟宇宙奇物展示数据（效果、背景故事等）
pub struct RogueMiracleDisplay<'a> {
    pub id: u16,
    /// 名称
    pub name: &'a str,
    /// 奇物效果
    pub desc: &'a str,
    pub desc_params: Vec<format::Argument<'a>>,
    /// 奇物效果中，带有下划线的特殊效果的详细介绍
    pub extra_effect: Vec<crate::misc::ExtraEffectConfig<'a>>,
    /// 背景故事
    pub bg_desc: &'a str,
    /// 无意义，目前只有空字符串
    pub tag: &'a str,
    // 图标
    pub icon_path: &'a str,
    pub figure_icon_path: &'a str,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for RogueMiracleDisplay<'a> {
    type Model = model::rogue::RogueMiracleDisplay;
    fn from_model(game: &'a Data, model: &'a Self::Model) -> Self {
        Self {
            id: model.miracle_display_id,
            name: game.text(model.miracle_name),
            desc: model
                .miracle_desc
                .map(|hash| game.text(hash))
                .unwrap_or_default(),
            desc_params: model
                .desc_param_list
                .as_deref()
                .map(format::Argument::from_array)
                .unwrap_or_default(),
            extra_effect: model
                .extra_effect
                .as_deref()
                .unwrap_or_default()
                .iter()
                .map(|&id| game.extra_effect_config(id))
                .map(Option::unwrap)
                .collect(),
            bg_desc: model
                .miracle_bg_desc
                .map(|hash| game.text(hash))
                .unwrap_or_default(),
            tag: model
                .miracle_tag
                .map(|hash| game.text(hash))
                .unwrap_or_default(),
            icon_path: &model.miracle_icon_path,
            figure_icon_path: &model.miracle_figure_icon_path,
        }
    }
}

#[derive(Clone, Debug)]
// 模拟宇宙奇物图鉴信息（解锁奖励、在哪些 DLC 中出现等）
pub struct RogueHandbookMiracle<'a> {
    pub id: u16,
    pub reward: crate::misc::RewardData<'a>,
    pub type_list: Vec<RogueHandbookMiracleType<'a>>,
    pub display: RogueMiracleDisplay<'a>,
    pub order: u8,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for RogueHandbookMiracle<'a> {
    type Model = model::rogue::RogueHandbookMiracle;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            id: model.miracle_handbook_id,
            reward: game.reward_data(model.miracle_reward).unwrap(),
            type_list: model
                .miracle_type_list
                .iter()
                .map(|&typ| game.rogue_handbook_miracle_type(typ))
                .map(Option::unwrap)
                .collect(),
            // 存在一些奇物, 图鉴中展示的是模拟宇宙的效果, 游戏过程中展示的是差分宇宙的效果
            // 这一类奇物主要是差分宇宙新增的奇物和商店相关奇物 (邪恶机械卫星#900和「中等念头」群体机)
            display: None
                .or_else(|| game.rogue_miracle_display(model.miracle_dispaly_id))
                .or_else(|| game.rogue_tourn_miracle_display(model.miracle_dispaly_id))
                .unwrap(),
            order: model.order,
        }
    }
}

#[derive(Clone, Debug)]
// 模拟宇宙奇物图鉴所属 DLC
pub struct RogueHandbookMiracleType<'a> {
    pub id: u16,
    pub title: &'a str,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for RogueHandbookMiracleType<'a> {
    type Model = model::rogue::RogueHandbookMiracleType;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            id: model.rogue_handbook_miracle_type,
            title: game.text(model.rogue_miracle_type_title),
        }
    }
}

#[derive(Clone, Debug)]
// 模拟宇宙一轮战斗的敌人，目前只用于差分宇宙周期演算 Boss
pub struct RogueMonsterGroup<'a> {
    pub id: u32,
    pub list_and_weight: Vec<(RogueMonster<'a>, u8)>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for RogueMonsterGroup<'a> {
    type Model = model::rogue::RogueMonsterGroup;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            id: model.rogue_monster_group_id,
            list_and_weight: model
                .rogue_monster_list_and_weight
                .iter()
                .map(|&(id, weight)| (game.rogue_monster(id).unwrap(), weight))
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RogueMonster<'a> {
    pub id: u32,
    pub npc_monster: crate::monster::NPCMonsterData<'a>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for RogueMonster<'a> {
    type Model = model::rogue::RogueMonster;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            id: model.rogue_monster_id,
            npc_monster: game.npc_monster_data(model.npc_monster_id).unwrap(),
        }
    }
}

impl Name for RogueMonster<'_> {
    fn name(&self) -> &str {
        self.npc_monster.name
    }
    fn wiki_name(&self) -> Cow<'_, str> {
        crate::monster::wiki_name(self.name())
    }
}
