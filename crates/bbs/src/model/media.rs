#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize,
)]
#[serde(rename_all = "lowercase")]
#[allow(clippy::upper_case_acronyms)]
pub enum ImageFormat {
    #[default]
    #[serde(rename = "")]
    None,
    GIF,
    JPG,
    PNG,
    WebP,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EntityType {
    ImgEntityPost,
    ImgEntityUnknown,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ImageCrop {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    #[serde(with = "http_serde::uri")]
    pub url: http::Uri,
}

#[serde_with::serde_as]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Image {
    #[serde(with = "http_serde::uri")]
    pub url: http::Uri,
    pub height: u16,
    pub width: u16,
    pub format: ImageFormat,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub size: u32,
    pub crop: Option<ImageCrop>,
    pub is_user_set_cover: bool,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub image_id: u32,
    pub entity_type: EntityType,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub entity_id: u64,
    pub is_deleted: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "UPPERCASE")]
#[allow(clippy::upper_case_acronyms)]
pub enum VideoFormat {
    #[serde(alias = ".mp4")]
    MP4,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
#[allow(clippy::upper_case_acronyms)]
pub enum VideoCodec {
    H264,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum VideoResolutionDefinition {
    #[serde(rename = "480P")]
    _480P,
    #[serde(rename = "720P")]
    _720P,
    #[serde(rename = "1080P")]
    _1080P,
    #[serde(rename = "2K")]
    _2K,
}

mod num_maybe_str {
    pub fn serialize<S, I>(int: &I, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
        I: num::traits::AsPrimitive<u64>,
    {
        serializer.serialize_u64(int.as_())
    }

    struct Visitor<I>(std::marker::PhantomData<I>);
    impl<I> serde::de::Visitor<'_> for Visitor<I>
    where
        I: Copy + std::fmt::Display + num::Num + 'static,
        <I as num::Num>::FromStrRadixErr: std::fmt::Display,
        u64: num::traits::AsPrimitive<I>,
    {
        type Value = I;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("integer")
        }
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            I::from_str_radix(v, 10).map_err(serde::de::Error::custom)
        }
        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(num::traits::AsPrimitive::as_(v))
        }
    }

    pub fn deserialize<'de, D, I>(deserializer: D) -> Result<I, D::Error>
    where
        D: serde::Deserializer<'de>,
        I: Copy + std::fmt::Display + num::Num + 'static,
        <I as num::Num>::FromStrRadixErr: std::fmt::Display,
        u64: num::traits::AsPrimitive<I>,
    {
        deserializer.deserialize_any(Visitor::<I>(std::marker::PhantomData))
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct VideoResolution {
    #[serde(with = "http_serde::uri")]
    pub url: http::Uri,
    pub definition: VideoResolutionDefinition,
    pub height: u16,
    pub width: u16,
    pub bitrate: u32,
    #[serde(with = "num_maybe_str")]
    pub size: u32,
    pub format: VideoFormat,
    pub label: VideoResolutionDefinition,
    pub codec: VideoCodec,
}

#[serde_with::serde_as]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Video {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub id: u64,
    #[serde_as(as = "serde_with::DurationMilliSeconds<i64>")]
    pub duration: chrono::TimeDelta,
    #[serde(with = "http_serde::uri")]
    pub cover: http::Uri,
    pub resolutions: Vec<VideoResolution>,
    pub view_num: u32,
    pub transcoding_status: u8,      // 未知枚举
    pub review_status: u8,           // 未知枚举
    pub brief_intro: Option<String>, // 只有空串
}
