pub mod media;

pub mod structured_content;
pub mod user_post;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Response<T> {
    pub data: T,
    pub message: String,
    pub retcode: i32,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Hash,
    serde_repr::Deserialize_repr,
    serde_repr::Serialize_repr,
)]
#[repr(u8)]
pub enum GameId {
    /// 崩坏3
    HonkaiImpact3 = 1,
    /// 原神
    GenshinImpact = 2,
    /// 崩坏学院2
    GunsGirlSchoolDayZ = 3,
    /// 未定事件簿
    TearsOfThemis = 4,
    /// 大别野
    Villa = 5,
    /// 崩坏：星穹铁道
    HonkaiStarRail = 6,
    /// 绝区零
    ZenlessZoneZero = 8,
}

mod json_twice {
    use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S, T>(value: T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize,
    {
        serializer.serialize_str(&serde_json::to_string(&value).map_err(serde::ser::Error::custom)?)
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        for<'owned> T: Deserialize<'owned>,
    {
        serde_json::from_str(&String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

mod optional_json_twice {
    use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize,
    {
        match value {
            Some(value) => super::json_twice::serialize(value, serializer),
            None => serializer.serialize_str(""),
        }
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        for<'owned> T: Deserialize<'owned>,
    {
        let str = String::deserialize(deserializer)?;
        if str.is_empty() {
            return Ok(None);
        }
        serde_json::from_str(&str).map_err(serde::de::Error::custom)
    }
}

mod vec_http_uri {
    pub fn serialize<S>(uris: &[http::Uri], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        struct Uri<'a>(&'a http::Uri);
        impl serde::Serialize for Uri<'_> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                http_serde::uri::serialize(self.0, serializer)
            }
        }
        serializer.collect_seq(uris.iter().map(Uri))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<http::Uri>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct Uri(#[serde(deserialize_with = "http_serde::uri::deserialize")] http::Uri);
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Vec<http::Uri>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("uri")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut uris = Vec::new();
                while let Some(uri) = seq.next_element::<Uri>()? {
                    uris.push(uri.0);
                }
                Ok(uris)
            }
        }
        deserializer.deserialize_seq(Visitor)
    }
}
