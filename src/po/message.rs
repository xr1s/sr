use std::{num::NonZero, path::PathBuf};

use crate::{vo, GameData, ID, PO};

use super::Text;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum Gender {
    All,
    Female,
    Male,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct EmojiConfig {
    #[serde(rename = "EmojiID")]
    emoji_id: u32,
    gender: Gender,
    #[serde(rename = "EmojiGroupID")]
    emoji_group_id: Option<NonZero<u8>>,
    key_words: Text,
    emoji_path: String,
    same_group_order: Option<NonZero<u8>>,
    gender_link: Option<NonZero<u8>>,
    #[serde(default)]
    is_train_members: bool,
}

impl ID for EmojiConfig {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.emoji_id
    }
}

impl<'a> PO<'a> for EmojiConfig {
    type VO = vo::message::EmojiConfig<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.emoji_id,
            gender: self.gender,
            group: self
                .emoji_group_id
                .map(NonZero::get)
                .map(|id| game.emoji_group(id))
                .map(Option::unwrap),
            keywords: game.text(self.key_words),
            path: &self.emoji_path,
            same_group_order: self.same_group_order.map(NonZero::get).unwrap_or_default(),
            gender_link: self.gender_link.map(NonZero::get).unwrap_or_default(),
            is_train_members: self.is_train_members,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum EmojiGroupType {
    All,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct EmojiGroup {
    #[serde(rename = "EmojiGroupID")]
    emoji_group_id: u8,
    emoji_group_type: EmojiGroupType,
    group_name: Text,
    img_path: PathBuf,
}

impl ID for EmojiGroup {
    type ID = u8;
    fn id(&self) -> Self::ID {
        self.emoji_group_id
    }
}

impl<'a> PO<'a> for EmojiGroup {
    type VO = vo::message::EmojiGroup<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.emoji_group_id,
            r#type: self.emoji_group_type,
            name: game.text(self.group_name),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct MessageContactsCamp {
    contacts_camp: u8,
    name: Text,
    #[serde(rename = "SortID")]
    sort_id: u8,
}

impl ID for MessageContactsCamp {
    type ID = u8;
    fn id(&self) -> Self::ID {
        self.contacts_camp
    }
}

impl<'a> PO<'a> for MessageContactsCamp {
    type VO = vo::message::MessageContactsCamp<'a>;
    fn vo(&self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.contacts_camp,
            name: game.text(self.name),
            sort_id: self.sort_id,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct MessageContactsConfig {
    #[serde(rename = "ID")]
    id: u16,
    name: Text,
    icon_path: PathBuf,
    signature_text: Text,
    contacts_type: Option<NonZero<u8>>, // 只有 1, 2, 3 三种
    contacts_camp: Option<NonZero<u8>>,
}

impl ID for MessageContactsConfig {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.id
    }
}

impl<'a> PO<'a> for MessageContactsConfig {
    type VO = vo::message::MessageContactsConfig<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.id,
            name: game.text(self.name),
            signature_text: game.text(self.signature_text),
            r#type: self
                .contacts_type
                .map(NonZero::get)
                .map(|id| game.message_contacts_type(id))
                .map(Option::unwrap),
            camp: self
                .contacts_camp
                .map(NonZero::get)
                .map(|id| game.message_contacts_camp(id))
                .map(Option::unwrap),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct MessageContactsType {
    contacts_type: u8,
    name: Text,
    #[serde(rename = "SortID")]
    sort_id: u8,
}

impl ID for MessageContactsType {
    type ID = u8;
    fn id(&self) -> Self::ID {
        self.contacts_type
    }
}

impl<'a> PO<'a> for MessageContactsType {
    type VO = vo::message::MessageContactsType<'a>;
    fn vo(&self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.contacts_type,
            name: game.text(self.name),
            sort_id: self.sort_id,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct MessageGroupConfig {
    #[serde(rename = "ID")]
    id: u16,
    #[serde(rename = "MessageContactsID")]
    message_contacts_id: u16,
    #[serde(rename = "MessageSectionIDList")]
    message_section_id_list: Vec<u32>,
    #[serde(rename = "ActivityModuleID")]
    activity_module_id: Option<NonZero<u32>>,
}

impl ID for MessageGroupConfig {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.id
    }
}

impl<'a> PO<'a> for MessageGroupConfig {
    type VO = vo::message::MessageGroupConfig<'a>;
    fn vo(&self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.id,
            contacts: game
                .message_contacts_config(self.message_contacts_id)
                .unwrap(),
            section_list: self
                .message_section_id_list
                .iter()
                .map(|&id| game.message_section_config(id))
                .map(Option::unwrap)
                .collect(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum Sender {
    NPC,
    Player,
    PlayerAuto,
    System,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum ItemType {
    Image,
    /// 目前只有四个，罗浮杂俎新阶段任务小桂子发的短信
    /// MessageItemLink.json 中找对应条目
    Link,
    /// 目前只有两个，穿插在首次抵达罗浮主线时候丹恒视角的故事
    /// MessageItemRaidEntrance.json 中找对应条目
    Raid,
    Sticker,
    Text,
    Video,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct MessageItemConfig {
    #[serde(rename = "ID")]
    id: u32,
    #[serde(rename = "ContactsID")]
    contacts_id: Option<NonZero<u16>>,
    sender: Sender,
    item_type: ItemType,
    main_text: Text,
    #[serde(rename = "ItemContentID")]
    #[serde(alias = "ItemImageID")] // 1.2 之前叫做 ItemImageID
    item_content_id: Option<NonZero<u32>>,
    option_text: Text,
    #[serde(rename = "NextItemIDList")]
    next_item_id_list: Vec<u32>,
    #[serde(rename = "SectionID")]
    section_id: Option<NonZero<u32>>,
}

impl ID for MessageItemConfig {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.id
    }
}

impl<'a> PO<'a> for MessageItemConfig {
    type VO = vo::message::MessageItemConfig<'a>;
    fn vo(&'a self, game: &'a GameData) -> Self::VO {
        Self::VO {
            id: self.id,
            contacts: self
                .contacts_id
                .map(NonZero::get)
                .map(|id| game.message_contacts_config(id))
                .map(Option::unwrap),
            sender: self.sender,
            r#type: self.item_type,
            main_text: game.text(self.main_text),
            content_id: self.item_content_id.map(NonZero::get).unwrap_or_default(),
            option_text: game.text(self.option_text),
            next_item_id_list: &self.next_item_id_list,
            section_id: self.section_id,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
// 另外还有 Link Video 等，字段挺多，不放到一起了吧
pub(crate) struct MessageItemImage {
    #[serde(rename = "ID")]
    id: u32,
    image_path: String,
    female_image_path: Option<String>, // 2.3 版本及之后
}

impl ID for MessageItemImage {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.id
    }
}

impl<'a> PO<'a> for MessageItemImage {
    type VO = vo::message::MessageItemImage<'a>;
    fn vo(&'a self, _game: &GameData) -> Self::VO {
        Self::VO {
            id: self.id,
            image_path: &self.image_path,
            female_image_path: self.female_image_path.as_deref().unwrap_or_default(),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct MessageSectionConfig {
    #[serde(rename = "ID")]
    id: u32,
    #[serde(rename = "StartMessageItemIDList")]
    start_message_item_id_list: Vec<u32>,
    #[serde(default)]
    is_perform_message: bool,
    main_mission_link: Option<NonZero<u32>>,
}

impl ID for MessageSectionConfig {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.id
    }
}

impl<'a> PO<'a> for MessageSectionConfig {
    type VO = vo::message::MessageSectionConfig<'a>;
    fn vo(&self, game: &'a GameData) -> Self::VO {
        Self::VO {
            game,
            id: self.id,
            start_message_item_list: self
                .start_message_item_id_list
                .iter()
                .map(|&id| game.message_item_config(id))
                .map(Option::unwrap)
                .collect(),
            is_perform_message: self.is_perform_message,
            main_mission_link: self
                .main_mission_link
                .map(NonZero::get)
                .map(|id| game.main_mission(id))
                .map(Option::unwrap),
            _contacts: std::cell::OnceCell::new(),
        }
    }
}
