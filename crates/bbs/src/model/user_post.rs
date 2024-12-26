use chrono::{DateTime, FixedOffset, Utc};

use crate::model::structured_content::StructuredContent;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct PostStatus {
    pub is_top: bool,
    pub is_good: bool,
    pub is_official: bool,
    pub post_status: u8, // 未知枚举
}

mod empty_http_uri {
    use serde::Deserialize;

    pub fn serialize<S>(uri: &Option<http::Uri>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match uri {
            None => serializer.serialize_str(""),
            Some(uri) => http_serde::uri::serialize(uri, serializer),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<http::Uri>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[serde_with::serde_as]
        #[derive(serde::Deserialize)]
        struct OptionUri(#[serde_as(as = "serde_with::NoneAsEmptyString")] Option<http::Uri>);
        OptionUri::deserialize(deserializer).map(|uri| uri.0)
    }
}

#[serde_with::serde_as]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct MetaContentVods {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: u64,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct MetaContent {
    describe: Option<Vec<StructuredContent>>,
    vods: Option<Vec<MetaContentVods>>,
    #[serde(rename = "ActivityMeta")]
    activity_meta: Option<()>, // 只有空值
}

#[serde_with::serde_as]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Post {
    pub game_id: crate::model::GameId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub post_id: u32,
    pub f_forum_id: u8,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub uid: u32,
    /// 标题
    pub subject: String,
    /// 预览
    pub content: String,
    #[serde(with = "empty_http_uri")]
    pub cover: Option<http::Uri>,
    pub view_type: u8, // 未知枚举
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "crate::model::vec_http_uri")]
    pub images: Vec<http::Uri>,
    pub post_status: PostStatus,
    pub topic_ids: Vec<u32>, // getTopicFullInfo?gids=6&id=
    pub view_status: u8,     // 未知枚举
    pub max_floor: u32,
    pub is_original: u8,             // 未知枚举
    pub republish_authorization: u8, // 未知枚举
    #[serde(with = "base::serde::datetime")]
    pub reply_time: DateTime<FixedOffset>,
    pub is_deleted: u8,
    pub is_interactive: bool,
    #[serde(with = "crate::model::optional_json_twice")]
    pub structured_content: Option<Vec<StructuredContent>>,
    pub structured_content_rows: Vec<()>,
    pub review_id: u8,
    pub is_profit: bool,
    pub is_in_profit: bool,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub deleted_at: chrono::DateTime<chrono::Utc>,
    pub pre_pub_status: u8, // 未知枚举
    pub cate_id: u8,
    pub profit_post_status: i8, // 未知枚举
    pub audit_status: u8,       // 未知枚举
    #[serde(with = "crate::model::optional_json_twice")]
    pub meta_content: Option<MetaContent>,
    pub is_missing: bool,
    pub block_reply_img: u8, // 未知枚举
    pub is_showing_missing: bool,
    pub block_latest_reply_time: u8, // 未知枚举
    pub selected_comment: u8,
    pub is_mentor: bool,
}

#[serde_with::serde_as]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ForumCate {
    pub id: u8,
    pub name: String,
    pub forum_id: u8,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Forum {
    pub id: u8,
    pub name: String,
    #[serde(with = "http_serde::uri")]
    pub icon: http::Uri,
    pub game_id: crate::model::GameId,
    pub forum_cate: Option<ForumCate>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Topic {
    pub id: u16,
    pub name: String,
    #[serde(with = "http_serde::uri")]
    pub cover: http::Uri,
    pub is_top: bool,
    pub is_good: bool,
    pub is_interactive: bool,
    pub game_id: u8,
    pub content_type: u8, // 未知枚举
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UserCertification {
    pub r#type: u8,
    pub label: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UserLevelExp {
    pub level: u8,
    pub exp: u32,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UserAvatarExt {
    pub avatar_type: u8,          // 未知枚举
    pub avatar_assets_id: String, // 只有空串
    pub resources: Vec<()>,       // 只有空串
    pub hd_resources: Vec<()>,    // 只有空串
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct SelfOpration {
    pub attitude: u8, // 未知枚举
    pub is_collected: bool,
    pub upvote_type: u8, // 未知枚举
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct PostUpvoteStat {
    pub upvote_type: u8, // 未知枚举，大概是不同表情点赞
    pub upvote_cnt: u32,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stat {
    pub view_num: u32,
    pub reply_num: u32,
    pub like_num: u32,
    pub bookmark_num: u16,
    pub forward_num: u32,
    pub original_like_num: u32,
    pub post_upvote_stat: Vec<PostUpvoteStat>,
}

#[serde_with::serde_as]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct User {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub uid: u32,
    pub nickname: String,
    pub introduce: String,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub avatar: u32,
    pub gender: u8, // 大概率是 0 未知 1 男 2 女
    pub certification: UserCertification,
    pub level_exp: UserLevelExp,
    pub is_following: bool,
    pub is_followed: bool,
    #[serde(with = "http_serde::uri")]
    pub avatar_url: http::Uri,
    /// 头像装扮
    #[serde(with = "empty_http_uri")]
    pub pendant: Option<http::Uri>,
    pub certifications: Vec<UserCertification>,
    pub is_creator: bool,
    pub avatar_ext: UserAvatarExt,
}

#[serde_with::serde_as]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UserPost {
    pub post: Post,
    pub forum: Option<Forum>, // 很古老的帖子可能为 null
    pub topics: Vec<Topic>,
    pub user: User,
    pub self_operation: SelfOpration,
    pub stat: Stat,
    pub help_sys: Option<()>, // 只有空值
    pub cover: Option<crate::model::media::Image>,
    pub image_list: Vec<crate::model::media::Image>,
    pub is_official_master: bool,
    pub is_user_master: bool,
    pub hot_reply_exist: bool,
    pub vote_count: u8,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub last_modify_time: DateTime<Utc>,
    pub recommend_type: String, // 只有空串
    pub collection: Option<()>, // 只有空值
    pub vod_list: Vec<crate::model::media::Video>,
    pub is_block_on: bool,
    pub forum_rank_info: Option<()>,  // 只有空值
    pub link_card_list: Vec<()>,      // 只有空串
    pub news_meta: Option<()>,        // 只有空值
    pub recommend_reason: Option<()>, // 只有空值
    pub villa_card: Option<()>,       // 只有空值
    pub is_mentor: bool,
    pub villa_room_card: Option<()>,          // 只有空值
    pub reply_avatar_action_info: Option<()>, // 只有空值
    pub challenge: Option<()>,                // 只有空值
    pub hot_reply_list: Vec<()>,              // 只有空串
    pub villa_msg_image_list: Vec<()>,        // 只有空串
    pub contribution_act: Option<()>,         // 只有空值
    pub is_has_vote: bool,
    pub is_has_lottery: bool,
}

#[serde_with::serde_as]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UserPostList {
    pub list: Vec<UserPost>,
    pub is_last: bool,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub next_offset: u32,
}
