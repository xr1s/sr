use crate::{ExcelOutput, FromModel};

#[derive(Clone, Debug)]
pub struct BookDisplayType {
    pub id: u8,
    pub alignment: u8,
}

impl<Data: ExcelOutput> FromModel<'_, Data> for BookDisplayType {
    type Model = model::book::BookDisplayType;
    fn from_model(_game: &Data, model: &Self::Model) -> Self {
        Self {
            id: model.book_display_type_id,
            alignment: model.alignment,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BookSeriesConfig<'a> {
    pub id: u16,
    pub name: &'a str,
    pub comments: &'a str,
    pub num: u8,
    pub world: BookSeriesWorld<'a>,
    pub is_show_in_bookshelf: bool,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for BookSeriesConfig<'a> {
    type Model = model::book::BookSeriesConfig;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            id: model.book_series_id,
            name: game.text(model.book_series),
            comments: game.text(model.book_series_comments),
            num: model.book_series_num,
            world: game.book_series_world(model.book_series_world).unwrap(),
            is_show_in_bookshelf: model.is_show_in_bookshelf,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BookSeriesWorld<'a> {
    pub id: u8,
    pub name: &'a str,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for BookSeriesWorld<'a> {
    type Model = model::book::BookSeriesWorld;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            id: model.book_series_world,
            name: game.text(model.book_series_world_textmap_id),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LocalbookConfig<'a> {
    pub id: u32,
    pub series: BookSeriesConfig<'a>,
    pub inside_id: u8,
    pub inside_name: &'a str,
    pub content: &'a str,
    pub display_type: BookDisplayType,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for LocalbookConfig<'a> {
    type Model = model::book::LocalbookConfig;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            id: model.book_id,
            series: game.book_series_config(model.book_series_id).unwrap(),
            inside_id: model.book_series_inside_id,
            inside_name: game.text(model.book_inside_name),
            content: game.text(model.book_content),
            display_type: game.book_display_type(model.book_display_type).unwrap(),
        }
    }
}
