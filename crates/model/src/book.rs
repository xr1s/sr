use base::ID;

use crate::Text;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct BookDisplayType {
    #[serde(rename = "BookDisplayTypeID")]
    pub book_display_type_id: u8,
    pub alignment: u8,
}

impl ID for BookDisplayType {
    type ID = u8;
    fn id(&self) -> Self::ID {
        self.book_display_type_id
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct BookSeriesConfig {
    #[serde(rename = "BookSeriesID")]
    pub book_series_id: u16,
    pub book_series: Text,
    pub book_series_comments: Text,
    pub book_series_num: u8,
    pub book_series_world: u8,
    #[serde(default)]
    pub is_show_in_bookshelf: bool,
}

impl ID for BookSeriesConfig {
    type ID = u16;
    fn id(&self) -> Self::ID {
        self.book_series_id
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct BookSeriesWorld {
    pub book_series_world: u8,
    #[serde(rename = "BookSeriesWorldTextmapID")]
    pub book_series_world_textmap_id: Text,
    pub book_series_world_icon_path: String,
    pub book_series_world_background_path: String,
}

impl ID for BookSeriesWorld {
    type ID = u8;
    fn id(&self) -> Self::ID {
        self.book_series_world
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct LocalbookConfig {
    #[serde(rename = "BookID")]
    pub book_id: u32,
    #[serde(rename = "BookSeriesID")]
    pub book_series_id: u16,
    #[serde(rename = "BookSeriesInsideID")]
    pub book_series_inside_id: u8,
    pub book_inside_name: Text,
    pub book_content: Text,
    pub book_display_type: u8,
    pub local_book_image_path: Vec<String>,
}

impl ID for LocalbookConfig {
    type ID = u32;
    fn id(&self) -> Self::ID {
        self.book_id
    }
}
