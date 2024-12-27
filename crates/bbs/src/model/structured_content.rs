use std::num::NonZero;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Align {
    Center,
    Justify,
    Right,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ColorName {
    Black,
    Gray,
    Red,
    Blue,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ColorSpecialHtml {
    Inherit,
    WindowText,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum Color {
    Hex(base::serde::Color),
    Name(ColorName),
    Special(ColorSpecialHtml),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TextAttributesList {
    Bullet,
}

#[serde_with::serde_as]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TextAttributes {
    pub align: Option<Align>,
    #[serde(default)]
    pub bold: bool,
    pub color: Option<Color>,
    pub header: Option<NonZero<u8>>,
    #[serde(default)]
    pub italic: bool,
    // 没有用，不知道干什么的
    pub list: Option<TextAttributesList>,
    /// 特么的官方帖错百度网盘链接了，只能用 DefaultOnError
    /// https://www.miyoushe.com/sr/article/54339944
    #[serde_as(as = "serde_with::DefaultOnError<Option<serde_with::DisplayFromStr>>")]
    #[serde(default)]
    pub link: Option<http::Uri>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Divider {
    /// 中间为米游兔（应用 Logo 上的吉祥物），两侧虚线
    /// https://upload-bbs.miyoushe.com/upload/2021/01/05/40eb5281cb24042bf34a9f1bcc61eaf5.png
    #[serde(rename = "line_1")]
    Line1,
    /// 中间为米游兔（应用 Logo 上的吉祥物），两侧渐隐实线
    /// https://upload-bbs.miyoushe.com/upload/2021/01/05/477d4c535e965bec1791203aecdfa8e6.png
    #[serde(rename = "line_2")]
    Line2,
    /// 花里胡哨的，左边米游姬，右边米游兔，中间用类似链条的十字和星球串起来
    /// https://upload-bbs.miyoushe.com/upload/2021/01/05/e7047588e912d60ff87a975e037c7606.png
    #[serde(rename = "line_3")]
    Line3,
    /// 非常朴素的一条实线
    /// https://upload-bbs.miyoushe.com/upload/2022/07/13/line_4.png
    #[serde(rename = "line_4")]
    Line4,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct InsertDivider {
    pub divider: Divider,
}

#[serde_with::serde_as]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct InsertImage {
    // 实际就是 http::Uri，但是存在脏数据错误的链接，因为图裂，直接用 None
    // 顺带一提，这会导致 ImageAttributes 中的数据也出现错误
    #[serde_as(as = "serde_with::DefaultOnError<Option<serde_with::DisplayFromStr>>")]
    pub image: Option<http::Uri>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ImageAttributes {
    pub align: Option<Align>,
    // bold 和 color 都是多余的，实际页面中不会生效，估计是以前不规范导致的问题
    #[serde(default)]
    pub bold: bool,
    pub color: Option<Color>,
    // 以下四个数据必然是存在的，除非是脏数据图片链接错误
    // 脏数据参见 https://www.miyoushe.com/bh3/article/1866812
    #[serde(default)]
    pub height: u16,
    #[serde(default)]
    pub width: u16,
    #[serde(default)]
    pub size: u32,
    #[serde(default)]
    pub ext: crate::model::media::ImageFormat,
}

#[serde_with::serde_as]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct LinkCard {
    pub link_type: u8, // 未知枚举
    #[serde(with = "http_serde::uri")]
    pub origin_url: http::Uri,
    #[serde(with = "http_serde::uri")]
    pub landing_url: http::Uri,
    #[serde(with = "http_serde::uri")]
    pub cover: http::Uri,
    pub title: String,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub card_id: u64,
    pub card_status: u8,              // 未知枚举
    pub market_price: Option<String>, // 目前看未定事件簿专属，带价格的米游铺链接
    pub price: Option<String>,        // 目前看未定事件簿专属，带价格的米游铺链接
    pub button_text: Option<String>,  // 目前看未定事件簿专属，带价格的米游铺链接
    pub landing_url_type: u8,         // 未知枚举
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct InsertLinkCard {
    link_card: LinkCard,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Lottery {
    pub id: String,
    pub toast: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct InsertLottery {
    pub backup_text: String,
    pub lottery: Lottery,
}

#[serde_with::serde_as]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Fold {
    #[serde(with = "crate::model::json_twice")]
    pub title: Vec<StructuredContent>,
    #[serde(with = "crate::model::json_twice")]
    pub content: Vec<StructuredContent>,
    pub id: uuid::Uuid,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub size: u16,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct InsertFold {
    pub backup_text: String,
    pub fold: Fold,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Mention {
    pub uid: u32,
    pub nickname: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct InsertMention {
    pub mention: Mention,
}

#[serde_with::serde_as]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReceptionCardPreRegisterCount {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    count: u32,
}

struct DotSeparator;

impl serde_with::formats::Separator for DotSeparator {
    fn separator() -> &'static str {
        "."
    }
}

#[serde_with::serde_as]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReceptionCardPkg {
    #[serde(with = "http_serde::uri")]
    pub android_url: http::Uri,
    #[serde_as(as = "serde_with::StringWithSeparator<DotSeparator, String>")]
    pub pkg_name: Vec<String>,
    #[serde_as(as = "serde_with::StringWithSeparator<DotSeparator, u8>")]
    pub pkg_version: Vec<u8>,
    #[serde(with = "http_serde::uri")]
    pub ios_url: http::Uri,
    pub pkg_length: u32,
    #[serde(with = "hex::serde")]
    pub pkg_md5: [u8; 16],
    pub pkg_version_code: u32,
    #[serde_as(as = "serde_with::StringWithSeparator<DotSeparator, u8>")]
    pub ios_version: Vec<u8>,
    pub filename: String,
    pub ios_scheme_url: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReceptionCardUserStatus {
    pub is_device_support: bool,
    pub pre_register_status: u8, // 未知枚举
    pub has_qualification: bool,
    pub qualify_type: u8, // 未知枚举
}

#[serde_with::serde_as]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReceptionCard {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub id: u8,
    pub game_id: crate::model::GameId,
    pub name: String,
    #[serde(with = "http_serde::uri")]
    pub icon: http::Uri,
    pub game_reception_status: u8, // 未知枚举
    pub pre_register_count: ReceptionCardPreRegisterCount,
    pub prompt: String,
    pub custom_toast: String,
    pub pkg: ReceptionCardPkg,
    pub user_status: ReceptionCardUserStatus,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct InsertReceptionCard {
    backup_text: String,
    reception_card: ReceptionCard,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct InsertVideo {
    #[serde(with = "http_serde::uri")]
    pub video: http::Uri,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct InsertVillaRoomCard {
    villa_room_card: VillaRoomCard,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum VillaRoomType {
    RoomTypeChatRoom,
}

#[serde_with::serde_as]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct VillaRoomCard {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    room_id: u32,
    room_name: String,
    room_type: VillaRoomType,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    villa_id: u16,
    villa_name: String,
    #[serde(with = "http_serde::uri")]
    villa_avatar_url: http::Uri,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    active_member_num: u16,
    #[serde(with = "crate::model::vec_http_uri")]
    active_user_avatar: Vec<http::Uri>,
    is_available: bool,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct InsertVod {
    pub vod: crate::model::media::Video,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum StructuredContent {
    /// 文本
    Text {
        /// 文本内容
        insert: String,
        /// 超链接、颜色、加粗等样式
        attributes: Option<TextAttributes>,
    },
    /// 类似 <hr> 的分隔线，具体见 enum
    Divider {
        insert: InsertDivider,
        attributes: Option<TextAttributes>,
    },
    /// 图片
    Image {
        insert: InsertImage,
        attributes: ImageAttributes,
    },
    LinkCard {
        insert: InsertLinkCard,
    },
    Lottery {
        insert: InsertLottery,
        attributes: Option<TextAttributes>,
    },
    // 折叠内容
    Fold {
        insert: InsertFold,
    },
    /// at 某用户
    Mention {
        insert: InsertMention,
        attributes: Option<TextAttributes>,
    },
    /// 游戏卡片，点击下载那种
    /// 参考 https://www.miyoushe.com/sr/article/54510644 底部预下载按钮
    ReceptionCard {
        insert: InsertReceptionCard,
    },
    Video {
        insert: InsertVideo,
    },
    VillaRoomCard {
        insert: InsertVillaRoomCard,
    },
    Vod {
        insert: InsertVod,
    },
}
