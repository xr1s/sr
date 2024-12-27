use std::borrow::Cow;
use std::num::NonZero;

use base::Wiki;
pub use model::message::{EmojiGender, EmojiGroupType, MessageItemType, MessageSender};

use crate::{ExcelOutput, FromModel};

#[derive(Clone, Debug)]
pub struct EmojiConfig<'a> {
    pub id: u32,
    pub gender: EmojiGender,
    pub group: Option<EmojiGroup<'a>>,
    pub keywords: &'a str,
    pub path: &'a str,
    pub same_group_order: u8,
    pub gender_link: u8,
    pub is_train_members: bool,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for EmojiConfig<'a> {
    type Model = model::message::EmojiConfig;
    fn from_model(game: &'a Data, model: &'a Self::Model) -> Self {
        Self {
            id: model.emoji_id,
            gender: model.gender,
            group: model
                .emoji_group_id
                .map(NonZero::get)
                .map(|id| game.emoji_group(id))
                .map(Option::unwrap),
            keywords: game.text(model.key_words),
            path: &model.emoji_path,
            same_group_order: model.same_group_order.map(NonZero::get).unwrap_or_default(),
            gender_link: model.gender_link.map(NonZero::get).unwrap_or_default(),
            is_train_members: model.is_train_members,
        }
    }
}

impl Wiki for EmojiConfig<'_> {
    fn wiki(&self) -> std::borrow::Cow<'static, str> {
        // 毫无办法，Wiki 上顺序是乱的，只能硬编码了
        // 除了玩家不可用的隐藏表情包之外，其它表情包均按照游戏内顺序在 match 中排序
        fn wiki_id(group: u32, order: u32) -> Cow<'static, str> {
            Cow::Owned(format!("{:02}-{:02}", group, order))
        }
        match self.id {
            // 隐藏
            30101 => Cow::Borrowed("00-01"), // 布洛妮娅（短信对象）-干杯
            30102 => Cow::Borrowed("00-02"), // 布洛妮娅（短信对象）-鲜花
            30001 => Cow::Borrowed("00-03"), // 艾丝妲（短信对象）-坏笑
            30002 => Cow::Borrowed("00-04"), // 阿兰（短信对象）-威胁
            30014 => Cow::Borrowed("00-05"), // 花火（短信对象）-照片
            30015 => Cow::Borrowed("00-06"), // 花火（短信对象）-照片
            30016 => Cow::Borrowed("00-07"), // 花火（短信对象）-照片
            // 帕姆展览馆第1弹 三月七
            101002 => Cow::Borrowed("01-01"), // 能量发射
            101003 => Cow::Borrowed("01-14"), // 点赞
            101005 => Cow::Borrowed("01-16"), // 悄悄话
            30006 => Cow::Borrowed("01-15"),  // 生气
            101006 => Cow::Borrowed("01-11"), // 盯
            30004 => Cow::Borrowed("01-09"),  // 骄傲
            101007 => Cow::Borrowed("01-10"), // 加油
            30005 => Cow::Borrowed("01-07"),  // 哭
            30003 => Cow::Borrowed("01-03"),  // 暗中观察
            // 帕姆展览馆第2弹 漫游测试篇
            102001 => Cow::Borrowed("02-01"), // 丹恒-思考
            102002 => Cow::Borrowed("02-02"), // 姬子-笑
            30007 => Cow::Borrowed("02-11"),  // 银狼-吹泡泡
            // 帕姆展览馆第3弹 跃迁测试篇
            30201 => Cow::Borrowed("03-05"),  // 刃-来了
            103002 => Cow::Borrowed("03-11"), // 彦卿-哼
            103004 => Cow::Borrowed("03-15"), // 星-吃瓜
            103007 => Cow::Borrowed("03-09"), // 驭空-叹气
            103013 => Cow::Borrowed("03-07"), // 素裳-冲鸭
            // 帕姆展览馆第4弹
            id @ 104001..=104999 => wiki_id(id / 1000 - 100, id % 1000),
            // 帕姆展览馆第5弹 呜呜伯
            106001 => Cow::Borrowed("05-01"), // 委屈
            106002 => Cow::Borrowed("05-02"), // 惊吓
            106003 => Cow::Borrowed("05-03"), // 开心
            106004 => Cow::Borrowed("05-10"), // 睡觉
            106005 => Cow::Borrowed("05-12"), // 比心
            106006 => Cow::Borrowed("05-15"), // 无奈
            106007 => Cow::Borrowed("05-09"), // 无语
            106008 => Cow::Borrowed("05-11"), // 点赞
            106009 => Cow::Borrowed("05-16"), // 晕
            106010 => Cow::Borrowed("05-05"), // 气气
            106011 => Cow::Borrowed("05-06"), // 疑惑
            106012 => Cow::Borrowed("05-14"), // 恶作剧
            106013 => Cow::Borrowed("05-08"), // 坏笑
            106014 => Cow::Borrowed("05-07"), // 期待
            106015 => Cow::Borrowed("05-04"), // 认真
            106016 => Cow::Borrowed("05-13"), // 敲黑板
            // 帕姆展览馆第7弹 帕姆篇
            20001 => Cow::Borrowed("07-01"), // 嗨
            20002 => Cow::Borrowed("07-07"), // 比心
            20003 => Cow::Borrowed("07-08"), // 不可以
            20004 => Cow::Borrowed("07-03"), // 收到
            20005 => Cow::Borrowed("07-04"), // 哭哭
            20006 => Cow::Borrowed("07-02"), // 点赞
            20007 => Cow::Borrowed("07-05"), // 疑惑
            20008 => Cow::Borrowed("07-06"), // 震惊
            // 新的帕姆表情包顺序都是对的
            107001..=107007 => {
                let group = self.group.as_ref().unwrap();
                wiki_id(group.id as u32 - 100, self.same_group_order as u32)
            }
            // 帕姆展览馆第8弹 一部分 Wiki 顺序错位，具体而言
            // 1. 108011 本来在 11 位，被提前到了 9 位，108009 和 108010 顺序延后
            id @ 108009..=108010 => wiki_id(id / 1000 - 100, id % 1000 + 1),
            108011 => Cow::Borrowed("08-09"), // 玲可-冲
            // 2. 108012 (12 位) 和 108013 (13 位) 错位
            108012 => Cow::Borrowed("08-13"), // 艾丝妲-发红包
            108013 => Cow::Borrowed("08-12"), // 白露-吐泡泡
            // 帕姆展览馆第11弹 后大半部分 Wiki 顺序错位，具体而言：
            // 1. 111005 (5 位) 被移动到了 13 位
            // 2. 111006 (6 位) 被移动到了 16 位
            // 3. 其它顺序往前
            //
            // 注意 30008, 30009, 30010, 30011, 30012, 30013 对应「11」中的一部分表情包
            30008 => Cow::Borrowed("11-01"),
            30009 => Cow::Borrowed("11-02"),
            30010 => Cow::Borrowed("11-03"),
            30012 | 111005 => Cow::Borrowed("11-13"),
            30013 => Cow::Borrowed("11-04"),
            30011 | 111006 => Cow::Borrowed("11-16"),
            id @ 111007..=111014 => wiki_id(id / 1000 - 100, id % 1000 - 2),
            id @ 111015..=111016 => wiki_id(id / 1000 - 100, id % 1000 - 1),
            // 帕姆展览馆第8弹 108001..=108016
            // 帕姆展览馆第9弹 109001..=109016
            // 帕姆展览馆第10弹 110001..=110016
            // 帕姆展览馆第11弹 111001..=111016
            // 帕姆展览馆第12弹 121001..=121016
            105001..=105999 | 108001..=111004 | 121001..=121999 => {
                let group = self.group.as_ref().unwrap();
                wiki_id(group.id as u32 - 100, self.same_group_order as u32)
            }
            // 从「帕姆展览馆第13弹」开始
            // 因为 Wiki 中间插入了一个不属于游戏的表情包系列（来自微信）
            // 因此需要 group.id 要从 -100 变成 -99 才是 Wiki 顺序
            //
            // 帕姆展览馆第13弹 131001..=131016
            // 帕姆展览馆第14弹 114001..=114016
            // 帕姆展览馆第15弹 30017 | 30018 | 115001..=115016
            // 帕姆展览馆第16弹 116001..=116016
            // 帕姆展览馆第17弹 30024 | 117001..=117016
            // 帕姆展览馆第18弹 30025 | 30028 | 118001..=118016
            //
            // 注意 30017 和 30018 分别对应「帕姆展览馆第15弹」的 115003 和 115004，完全相同
            // 注意 30024 对应「17」的 117002
            // 注意 30025, 30027, 30026 和 30028 分别对应「18」的 118001, 118003, 118002 和 118007
            // 注意 30029, 30030 和 30031 分别对应「19」的 119005, 119007 和 119010
            // 注意「帕姆展览馆第16弹」第一次出现 emoji.id % 1000 != emoji.same_order (除了靠前的)
            // 注意「帕姆展览馆第18弹」仍未实装
            30017 => Cow::Borrowed("16-03"),
            30018 => Cow::Borrowed("16-04"),
            30021 => Cow::Borrowed("17-04"),
            30024 => Cow::Borrowed("18-02"),
            30025 => Cow::Borrowed("19-01"),
            30026 => Cow::Borrowed("19-02"),
            30027 => Cow::Borrowed("19-03"),
            30028 => Cow::Borrowed("19-07"),
            30029 => Cow::Borrowed("20-05"),
            30030 => Cow::Borrowed("20-07"),
            30031 => Cow::Borrowed("20-10"),
            114001..=119999 | 131001..=131999 => {
                let group = self.group.as_ref().unwrap();
                wiki_id(group.id as u32 - 99, self.same_group_order as u32)
            }
            _ => Cow::Owned(format!(
                "表情未匹配 wiki，id: {}, name: {}, path: {:?}",
                self.id, self.keywords, self.path
            )),
        }
    }
}

#[derive(Clone, Debug)]
pub struct EmojiGroup<'a> {
    pub id: u8,
    pub r#type: EmojiGroupType,
    pub name: &'a str,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for EmojiGroup<'a> {
    type Model = model::message::EmojiGroup;
    fn from_model(game: &'a Data, model: &'a Self::Model) -> Self {
        Self {
            id: model.emoji_group_id,
            r#type: model.emoji_group_type,
            name: game.text(model.group_name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MessageContactsCamp<'a> {
    pub id: u8,
    pub name: &'a str,
    pub sort_id: u8,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for MessageContactsCamp<'a> {
    type Model = model::message::MessageContactsCamp;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            id: model.contacts_camp,
            name: game.text(model.name),
            sort_id: model.sort_id,
        }
    }
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for MessageContactsConfig<'a, Data> {
    type Model = model::message::MessageContactsConfig;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            game,
            id: model.id,
            name: game.text(model.name),
            signature_text: game.text(model.signature_text),
            r#type: model
                .contacts_type
                .map(NonZero::get)
                .map(|id| game.message_contacts_type(id))
                .map(Option::unwrap),
            camp: model
                .contacts_camp
                .map(NonZero::get)
                .map(|id| game.message_contacts_camp(id))
                .map(Option::unwrap),
        }
    }
}

#[derive(educe::Educe)]
#[educe(Clone, Debug)]
pub struct MessageContactsConfig<'a, Data: ExcelOutput + ?Sized> {
    #[educe(Debug(ignore))]
    game: &'a Data,
    pub id: u16,
    pub name: &'a str,
    pub signature_text: &'a str,
    pub r#type: Option<MessageContactsType<'a>>,
    pub camp: Option<MessageContactsCamp<'a>>,
}

impl<Data: ExcelOutput + format::GameData> Wiki for MessageContactsConfig<'_, Data> {
    fn wiki(&self) -> std::borrow::Cow<'static, str> {
        let mut wiki = String::new();
        wiki.push_str("{{#subobject:");
        wiki.push_str(self.name);
        wiki.push_str("-短信内容");
        wiki.push_str("\n|@category=短信头像");
        wiki.push_str("\n|名称=");
        wiki.push_str(self.name);
        let camp_name = self.camp.as_ref().map(|camp| camp.name).unwrap_or_default();
        wiki.push_str("\n|阵营=");
        wiki.push_str(camp_name);
        let contacts_type = self.r#type.as_ref().map(|typ| typ.name).unwrap_or_default();
        wiki.push_str("\n|类型=");
        wiki.push_str(contacts_type);
        wiki.push_str("\n}}");
        for section in self.game.message_section_in_contacts(self.id) {
            if section.contacts().id != self.id {
                continue;
            }
            wiki.push_str("\n\n{{#subobject:");
            wiki.push_str(self.name);
            wiki.push('-');
            wiki.push_str("<!-- 填入标题，ID=");
            wiki.push_str(&section.id.to_string());
            wiki.push_str(" -->");
            wiki.push_str("\n|@category=短信内容");
            wiki.push_str("\n|人物=");
            wiki.push_str(self.name);
            wiki.push_str("\n|短信标题=<!-- 填入相关事件或任务 -->");
            wiki.push_str("\n|版本=<!-- 填入版本 -->");
            wiki.push_str("\n|内容=");
            let mut indent = String::from("\n  ");
            wiki.push_str(&indent);
            section.wiki_message_template(&mut wiki, &mut indent);
            wiki.push_str("\n}}");
        }

        Cow::Owned(wiki)
    }
}

#[derive(Clone, Debug)]
pub struct MessageContactsType<'a> {
    pub id: u8,
    pub name: &'a str,
    pub sort_id: u8,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for MessageContactsType<'a> {
    type Model = model::message::MessageContactsType;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            id: model.contacts_type,
            name: game.text(model.name),
            sort_id: model.sort_id,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MessageGroupConfig<'a, Data: ExcelOutput + ?Sized> {
    pub id: u16,
    pub contacts: MessageContactsConfig<'a, Data>,
    pub section_list: Vec<MessageSectionConfig<'a, Data>>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for MessageGroupConfig<'a, Data> {
    type Model = model::message::MessageGroupConfig;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            id: model.id,
            contacts: game
                .message_contacts_config(model.message_contacts_id)
                .unwrap(),
            section_list: model
                .message_section_id_list
                .iter()
                .map(|&id| game.message_section_config(id))
                .map(Option::unwrap)
                .collect(),
        }
    }
}

#[derive(educe::Educe)]
#[educe(Clone, Debug)]
pub struct MessageItemConfig<'a, Data: ExcelOutput + ?Sized> {
    pub id: u32,
    pub contacts: Option<MessageContactsConfig<'a, Data>>,
    pub sender: MessageSender,
    pub r#type: MessageItemType,
    pub main_text: &'a str,
    pub content_id: u32,
    pub option_text: &'a str,
    pub next_item_id_list: &'a [u32],
    pub section_id: Option<NonZero<u32>>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for MessageItemConfig<'a, Data> {
    type Model = model::message::MessageItemConfig;
    fn from_model(game: &'a Data, model: &'a Self::Model) -> Self {
        Self {
            id: model.id,
            contacts: model
                .contacts_id
                .map(NonZero::get)
                .map(|id| game.message_contacts_config(id))
                .map(Option::unwrap),
            sender: model.sender,
            r#type: model.item_type,
            main_text: game.text(model.main_text),
            content_id: model.item_content_id.map(NonZero::get).unwrap_or_default(),
            option_text: game.text(model.option_text),
            next_item_id_list: &model.next_item_id_list,
            section_id: model.section_id,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MessageItemImage<'a> {
    pub id: u32,
    pub image_path: &'a str,
    pub female_image_path: &'a str,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for MessageItemImage<'a> {
    type Model = model::message::MessageItemImage;
    fn from_model(_game: &Data, model: &'a Self::Model) -> Self {
        Self {
            id: model.id,
            image_path: &model.image_path,
            female_image_path: model.female_image_path.as_deref().unwrap_or_default(),
        }
    }
}

#[derive(educe::Educe)]
#[educe(Clone, Debug)]
pub struct MessageSectionConfig<'a, Data: ExcelOutput + ?Sized> {
    #[educe(Debug(ignore))]
    game: &'a Data,
    pub id: u32,
    pub start_message_item_list: Vec<MessageItemConfig<'a, Data>>,
    pub is_perform_message: bool,
    pub main_mission_link: Option<crate::mission::MainMission<'a>>,
    _contacts: std::sync::OnceLock<MessageContactsConfig<'a, Data>>,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for MessageSectionConfig<'a, Data> {
    type Model = model::message::MessageSectionConfig;
    fn from_model(game: &'a Data, model: &'a Self::Model) -> Self {
        Self {
            game,
            id: model.id,
            start_message_item_list: model
                .start_message_item_id_list
                .iter()
                .map(|&id| game.message_item_config(id))
                .map(Option::unwrap)
                .collect(),
            is_perform_message: model.is_perform_message,
            main_mission_link: model
                .main_mission_link
                .map(NonZero::get)
                .map(|id| game.main_mission(id))
                .map(Option::unwrap),
            _contacts: std::sync::OnceLock::new(),
        }
    }
}

impl<Data: ExcelOutput> MessageSectionConfig<'_, Data> {
    pub fn contacts(&self) -> &MessageContactsConfig<'_, Data> {
        self._contacts
            .get_or_init(|| self.game.message_contacts_of_section(self.id).unwrap())
    }

    /// 寻找该节点的下一个聚合点 MessageItemConfig 的 ID
    /// 对于没有分支（不是多路选项）的单句对话，下一句就是当前节点的聚合点
    /// 对于有分支的的对话，需要所有分支都会抵达的首个节点才是聚合点
    /// 聚合点的定义有点类似于最近公共祖先
    ///
    /// 聚合点可能为空，比如最后一句对话、没有共同对话的最后一条选项等。为空返回 0
    pub fn next_convergence_node(&self, next_ids: &[u32]) -> Option<MessageItemConfig<Data>> {
        let branches = next_ids.len();
        if branches == 1 {
            let next = self.game.message_item_config(next_ids[0]).unwrap();
            return Some(next);
        }
        if branches == 0 {
            return None;
        }
        // TODO: 可以记忆化，但是从数据生存周期看应该挂到 GameData 下，有点令人难受
        let mut queue = next_ids
            .iter()
            .map(|&id| self.game.message_item_config(id))
            .map(Option::unwrap)
            .map(Some)
            .collect::<Vec<_>>();
        let mut visit = fnv::FnvHashMap::<u32, usize>::default();
        // 当前节点分支下的节点分别往前找各自节点的聚合点，记录每个被找到聚合点的被访问次数
        // 若是某个聚合点被访问次数等于分支数，说明该节点为当前节点的聚合点
        loop {
            for next in &mut queue {
                if next.is_none() {
                    // 某条分支已经迭代到整个对话列表最后一句了，这条分支不继续找了
                    // 但是有可能另一分支速度较慢，仍未到聚合点，所以需要继续迭代找完为止
                    continue;
                }
                *next = self.next_convergence_node(next.as_ref().unwrap().next_item_id_list);
                if next.is_none() {
                    continue;
                }
                let entry = visit.entry(next.as_ref().unwrap().id).or_default();
                *entry += 1;
                if *entry == branches {
                    // 找到聚合点直接返回
                    return next.clone();
                }
            }
            if queue.iter().all(Option::is_none) {
                // 如果所有分支都到了终点而未找到聚合点
                // 说明当前就是最后一个选项且不存在聚合点，直接返回
                return None;
            }
        }
    }

    fn try_get_emoji(game: &Data, content_id: u32) -> EmojiConfig {
        game.emoji_config(content_id).unwrap_or({
            // 1.2 版本及之前没有解包 emoji 信息，尝试一下手动组装
            // 因为没有 group 因此出错会 panic
            EmojiConfig {
                id: content_id,
                gender: EmojiGender::All,
                group: None,
                keywords: "",
                path: "",
                same_group_order: 0,
                gender_link: 8,
                is_train_members: false,
            }
        })
    }
}

impl<Data: ExcelOutput + format::GameData> MessageSectionConfig<'_, Data> {
    fn wiki_message_single_item_content(
        &self,
        wiki: &mut String,
        prefix: &str,
        formatter: &mut format::Formatter<Data>,
        message: &MessageItemConfig<Data>,
    ) {
        fn dialogue(wiki: &mut String, direction: char, contacts: &str, r#type: &str, text: &str) {
            wiki.push_str("{{角色对话|");
            wiki.push(direction);
            wiki.push('|');
            wiki.push_str(contacts);
            wiki.push('|');
            wiki.push_str(r#type);
            wiki.push('|');
            wiki.push_str(text);
            wiki.push_str("}}");
        }
        fn image_text<Data: ExcelOutput>(
            game: &Data,
            message: &MessageItemConfig<Data>,
        ) -> Cow<'static, str> {
            let path = game
                .message_item_image(message.content_id)
                .unwrap()
                .image_path;
            // 处理一下图片路径，目录全部用小写，文件名保持大写，方便拿去查询
            // 解包数据中
            let segments = path.split('/').collect::<Vec<_>>();
            let path = segments[..segments.len() - 1]
                .iter()
                .map(|segment| segment.to_ascii_lowercase())
                .map(Cow::Owned)
                .chain([Cow::Borrowed(segments[segments.len() - 1])])
                .intersperse(Cow::Borrowed("/"))
                .collect::<String>();
            Cow::Owned(format!("{}<!-- {} -->", message.main_text, path))
        }
        let text = match message.r#type {
            MessageItemType::Image => image_text(self.game, message),
            MessageItemType::Link => Cow::Borrowed("<!-- ItemType::Link -->"),
            MessageItemType::Raid => Cow::Borrowed("<!-- ItemType::Raid -->"),
            MessageItemType::Sticker => Self::try_get_emoji(self.game, message.content_id).wiki(),
            MessageItemType::Text => {
                if message.content_id != 0 {
                    // 1.2 及之前图片没有
                    image_text(self.game, message)
                } else {
                    Cow::Owned(formatter.format(message.main_text, &[]))
                }
            }
            MessageItemType::Video => Cow::Borrowed("<!-- ItemType::Video -->"),
        };
        let r#type = match message.r#type {
            MessageItemType::Image => "图片",
            MessageItemType::Link => "图片",
            MessageItemType::Raid => "图片",
            MessageItemType::Sticker => "表情",
            MessageItemType::Text => "文本",
            MessageItemType::Video => "图片",
        };
        wiki.push_str(prefix);
        match message.sender {
            MessageSender::NPC => {
                let mut contacts = message
                    .contacts
                    .as_ref()
                    .map(|contacts| contacts.name)
                    .unwrap_or_else(|| self.contacts().name);
                if contacts == "{NICKNAME}" {
                    contacts = "开拓者";
                }
                dialogue(wiki, '左', contacts, r#type, &text);
            }
            MessageSender::Player | MessageSender::PlayerAuto => {
                let contacts = message
                    .contacts
                    .as_ref()
                    .map(|contacts| contacts.name)
                    .unwrap_or("开拓者");
                dialogue(wiki, '右', contacts, r#type, &text);
            }
            MessageSender::System => {
                wiki.push_str("{{短信警告|");
                wiki.push_str(&formatter.format(message.main_text, &[]));
                wiki.push_str("}}");
            }
        };
    }

    fn wiki_message_single_selection_content(
        &self,
        wiki: &mut String,
        prefix: &mut String,
        formatter: &mut format::Formatter<Data>,
        selections: &[MessageItemConfig<Data>],
    ) -> Option<MessageItemConfig<Data>> {
        wiki.push_str(prefix);
        wiki.push_str("{{短信选项");
        // 存在选项为表情的情况，需要在聊天记录中再手动发一条表情
        let is_sticker_selection = selections
            .iter()
            .all(|message| message.r#type == MessageItemType::Sticker);
        if is_sticker_selection {
            wiki.push_str("|表情");
        }
        let next_ids = selections
            .iter()
            .map(|message| message.id)
            .collect::<Vec<_>>();
        let convergence = self.next_convergence_node(&next_ids);
        let convergence_id = convergence
            .as_ref()
            .map(|message| message.id)
            .unwrap_or_default();
        for (index, message) in selections.iter().enumerate() {
            let index = index + 1;
            wiki.push_str(prefix);
            wiki.push_str("|选项");
            wiki.push_str(&index.to_string());
            wiki.push('=');
            wiki.push_str(&if is_sticker_selection {
                Self::try_get_emoji(self.game, message.content_id).wiki()
            } else {
                Cow::Owned(formatter.format(message.option_text, &[]))
            });
            wiki.push_str(prefix);
            wiki.push_str("|剧情");
            wiki.push_str(&index.to_string());
            wiki.push('=');
            prefix.push_str(Self::INDENT);
            self.wiki_next_message(wiki, prefix, formatter, message, convergence_id);
            prefix.truncate(prefix.len() - Self::INDENT_LENGTH);
        }
        wiki.push_str(prefix);
        wiki.push_str("}}");
        convergence
    }

    const INDENT: &'static str = "  ";
    const INDENT_LENGTH: usize = Self::INDENT.len();
    fn wiki_next_message(
        &self,
        wiki: &mut String,
        prefix: &mut String,
        formatter: &mut format::Formatter<Data>,
        message: &MessageItemConfig<Data>,
        convergence_id: u32,
    ) {
        if message.id == convergence_id {
            return;
        }
        self.wiki_message_single_item_content(wiki, prefix, formatter, message);
        if message.next_item_id_list.len() == 1 {
            // 快速判断，不需要求聚合点，直接往后走即可
            // 快速判断不是完全准确，具体就是唯一的 next 是选项的情况，在下面处理了
            // 准确的判断需要先获取所有后继节点
            let next = self
                .game
                .message_item_config(message.next_item_id_list[0])
                .unwrap();
            if next.id != convergence_id && !next.option_text.is_empty() {
                // 存在只有一个选项的情况，这里直接写死进去吧,
                // 具体条件是：后续没有多路分支，而且后续是选项节点
                // 那就不需要求聚合点
                //
                // 但是考虑到这个节点恰好是其它分支的聚合点（存在的，见花火短信 2012400）
                // 因此需要额外加上判断 next 是不是聚合点
                wiki.push_str(prefix);
                wiki.push_str("{{短信选项|选项1=");
                wiki.push_str(&formatter.format(next.option_text, &[]));
                wiki.push_str("}}");
            }
            self.wiki_next_message(wiki, prefix, formatter, &next, convergence_id);
        }
        if message.next_item_id_list.len() > 1 {
            // 需要求聚合点的情况
            let selections = message
                .next_item_id_list
                .iter()
                .map(|&id| self.game.message_item_config(id))
                .map(Option::unwrap)
                .collect::<Vec<_>>();
            if let Some(next) =
                self.wiki_message_single_selection_content(wiki, prefix, formatter, &selections)
            {
                self.wiki_next_message(wiki, prefix, formatter, &next, convergence_id);
            }
        }
    }

    fn wiki_message_template(&self, wiki: &mut String, prefix: &mut String) {
        let mut formatter = format::Formatter::new(self.game).output_wiki(true);
        let contacts = self.contacts();
        wiki.push_str("{{角色对话|模板开始|");
        wiki.push_str(&formatter.format(contacts.name, &[]));
        // 非自机角色的短信签名需要作为参数传入（自机角色由模板自动查询了）
        let contacts_type = contacts
            .r#type
            .as_ref()
            .map(|r#type| r#type.name)
            .unwrap_or_default();
        if contacts_type != "角色" && !contacts.signature_text.is_empty() {
            wiki.push('|');
            wiki.push_str(contacts.signature_text);
        }
        wiki.push_str("}}");
        prefix.push_str(Self::INDENT);
        if self.start_message_item_list.len() == 1 {
            self.wiki_next_message(
                wiki,
                prefix,
                &mut formatter,
                &self.start_message_item_list[0],
                0,
            );
        }
        if self.start_message_item_list.len() > 1 {
            if let Some(next) = self.wiki_message_single_selection_content(
                wiki,
                prefix,
                &mut formatter,
                &self.start_message_item_list,
            ) {
                self.wiki_next_message(wiki, prefix, &mut formatter, &next, 0);
            }
        }
        if let Some(mission) = &self.main_mission_link {
            wiki.push_str(prefix);
            wiki.push_str("{{接取任务|");
            wiki.push_str(&mission.r#type.wiki());
            wiki.push('|');
            wiki.push_str(mission.name);
            wiki.push_str("}}");
        }
        prefix.truncate(prefix.len() - Self::INDENT_LENGTH);
        wiki.push_str(prefix);
        wiki.push_str("{{角色对话|模板结束}}");
        prefix.truncate(prefix.len() - Self::INDENT_LENGTH);
    }
}

impl<Data: ExcelOutput + format::GameData> Wiki for MessageSectionConfig<'_, Data> {
    fn wiki(&self) -> std::borrow::Cow<'static, str> {
        let mut wiki = String::from("{{#subobject:");
        let mut indent = String::from("\n");
        self.wiki_message_template(&mut wiki, &mut indent);
        Cow::Owned(wiki)
    }
}
