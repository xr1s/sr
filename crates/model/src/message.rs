use std::{num::NonZero, path::PathBuf};

use base::ID;

use super::Text;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum EmojiGender {
    All,
    Female,
    Male,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct EmojiConfig {
    #[serde(rename = "EmojiID")]
    pub emoji_id: u32,
    pub gender: EmojiGender,
    #[serde(rename = "EmojiGroupID")]
    pub emoji_group_id: Option<NonZero<u8>>,
    pub key_words: Text,
    pub emoji_path: String,
    pub same_group_order: Option<NonZero<u8>>,
    pub gender_link: Option<NonZero<u8>>,
    #[serde(default)]
    pub is_train_members: bool,
}

impl ID for EmojiConfig {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.emoji_id
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum EmojiGroupType {
    All,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct EmojiGroup {
    #[serde(rename = "EmojiGroupID")]
    pub emoji_group_id: u8,
    pub emoji_group_type: EmojiGroupType,
    pub group_name: Text,
    pub img_path: PathBuf,
}

impl ID for EmojiGroup {
    type ID = u8;
    fn id(&self) -> Self::ID {
        self.emoji_group_id
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct MessageContactsCamp {
    pub contacts_camp: u8,
    pub name: Text,
    #[serde(rename = "SortID")]
    pub sort_id: u8,
}

impl ID for MessageContactsCamp {
    type ID = u8;
    fn id(&self) -> Self::ID {
        self.contacts_camp
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct MessageContactsConfig {
    #[serde(rename = "ID")]
    pub id: u16,
    pub name: Text,
    pub icon_path: PathBuf,
    pub signature_text: Text,
    pub contacts_type: Option<NonZero<u8>>, // 只有 1, 2, 3 三种
    pub contacts_camp: Option<NonZero<u8>>,
}

impl ID for MessageContactsConfig {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.id
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct MessageContactsType {
    pub contacts_type: u8,
    pub name: Text,
    #[serde(rename = "SortID")]
    pub sort_id: u8,
}

impl ID for MessageContactsType {
    type ID = u8;
    fn id(&self) -> Self::ID {
        self.contacts_type
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct MessageGroupConfig {
    #[serde(rename = "ID")]
    pub id: u16,
    #[serde(rename = "MessageContactsID")]
    pub message_contacts_id: u16,
    #[serde(rename = "MessageSectionIDList")]
    pub message_section_id_list: Vec<u32>,
    #[serde(rename = "ActivityModuleID")]
    activity_module_id: Option<NonZero<u32>>,
}

impl ID for MessageGroupConfig {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.id
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum MessageSender {
    NPC,
    Player,
    PlayerAuto,
    System,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum MessageItemType {
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
pub struct MessageItemConfig {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "ContactsID")]
    pub contacts_id: Option<NonZero<u16>>,
    pub sender: MessageSender,
    pub item_type: MessageItemType,
    pub main_text: Text,
    #[serde(rename = "ItemContentID")]
    #[serde(alias = "ItemImageID")] // 1.2 之前叫做 ItemImageID
    pub item_content_id: Option<NonZero<u32>>,
    pub option_text: Text,
    #[serde(rename = "NextItemIDList")]
    pub next_item_id_list: Vec<u32>,
    #[serde(rename = "SectionID")]
    pub section_id: Option<NonZero<u32>>,
}

impl ID for MessageItemConfig {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.id
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
// 另外还有 Link Video 等，字段挺多，不放到一起了吧
pub struct MessageItemImage {
    #[serde(rename = "ID")]
    pub id: u32,
    pub image_path: String,
    pub female_image_path: Option<String>, // 2.3 版本及之后
}

impl ID for MessageItemImage {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.id
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct MessageSectionConfig {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "StartMessageItemIDList")]
    pub start_message_item_id_list: Vec<u32>,
    #[serde(default)]
    pub is_perform_message: bool,
    pub main_mission_link: Option<NonZero<u32>>,
}

impl ID for MessageSectionConfig {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.id
    }
}
