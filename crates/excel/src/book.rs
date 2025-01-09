use std::borrow::Cow;

use base::Wiki;

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

#[derive(educe::Educe)]
#[educe(Clone, Debug)]
pub struct BookSeriesConfig<'a, Data: ExcelOutput + ?Sized> {
    #[educe(Debug(ignore))]
    game: &'a Data,
    pub id: u16,
    pub name: &'a str,
    pub comments: &'a str,
    pub num: u8,
    pub world: BookSeriesWorld<'a>,
    pub is_show_in_bookshelf: bool,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for BookSeriesConfig<'a, Data> {
    type Model = model::book::BookSeriesConfig;
    fn from_model(game: &'a Data, model: &Self::Model) -> Self {
        Self {
            game,
            id: model.book_series_id,
            name: game.text(model.book_series),
            comments: game.text(model.book_series_comments),
            num: model.book_series_num,
            world: game.book_series_world(model.book_series_world).unwrap(),
            is_show_in_bookshelf: model.is_show_in_bookshelf,
        }
    }
}

impl<Data: ExcelOutput + format::GameData> Wiki for BookSeriesConfig<'_, Data> {
    fn wiki(&self) -> std::borrow::Cow<'static, str> {
        let mut wiki = String::from("{{书籍");
        let mut formatter = format::Formatter::new(self.game)
            .media_wiki_syntax(true)
            .newline_after_block(true);
        let books = self
            .game
            .localbook_in_book_series(self.id)
            .collect::<Vec<_>>();
        wiki.push_str("\n|书籍名=");
        wiki.push_str(&formatter.format(self.name, &[]));
        wiki.push_str("\n|卷数=");
        wiki.push_str(&self.num.to_string());
        wiki.push_str("\n|编号=");
        wiki.push_str("\n|描述=");
        wiki.push_str(self.comments);
        wiki.push_str("\n|所属=");
        wiki.push_str(&self.world.id.to_string());
        wiki.push_str("\n|类型=");
        let icon = None
            .or_else(|| self.game.item_config(books[0].id))
            .or_else(|| self.game.item_config_book(books[0].id))
            .map(|item| item.icon_path)
            .and_then(|path| path.rsplit_once('/'))
            .map(|(_, file_name)| file_name)
            .and_then(|file_name| file_name.rsplit_once('.'))
            .map(|(id, _)| id)
            .and_then(|id| id.parse::<u32>().ok())
            .unwrap_or_default();
        wiki.push_str(match icon {
            0 => "资料", // 大地图或者剧情中散落的阅读物
            // 按顺序分别是 雅利洛 | 空间站黑塔 | 仙舟罗浮 | 匹诺康尼
            // 特殊图标单独备注
            190001 | 190004 | 190007 | 190016 => "书籍",
            190002 | 190005 | 190008 | 190015 => "资料",
            190003 | 190006 | 190009 => "信件",
            140236 => "信件2", // 目前只有罗浮的《钟珊的来信》和匹诺康尼的《关于财富学院代表的联名投诉信》
            190010 => "石碑",  // 只出现在罗浮
            190011 => "拓印",  // 只出现在罗浮
            190012 => "如意",  // 只出现在罗浮
            190013 => "便条",  // 只出现在匹诺康尼
            190014 => "录像带", // 只出现在匹诺康尼
            _ => unreachable!("可能是新版本新增不同类型的图书 {} {}", self.name, icon),
        });
        wiki.push_str("\n|实装版本=");
        wiki.push_str("\n|相关角色=");
        wiki.push_str("\n|相关任务=");
        wiki.push_str("\n}}\n\n");
        for book in books {
            wiki.push_str("{{书籍/分卷");
            wiki.push_str("\n|名称=");
            wiki.push_str(&formatter.format(book.inside_name, &[]));
            wiki.push_str("\n|卷数=");
            wiki.push_str(&format!(
                "{:01$}",
                &book.inside_id,
                if self.num < 10 { 1 } else { 2 },
            ));
            wiki.push_str("\n|获取方式=");
            wiki.push_str("\n|内容=");
            wiki.push_str(&formatter.format(book.content, &[]));
            if wiki.as_bytes().last() != Some(&b'\n') {
                wiki.push('\n');
            }
            wiki.push_str("}}\n\n");
        }
        Cow::Owned(wiki)
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
pub struct LocalbookConfig<'a, Data: ExcelOutput + ?Sized> {
    pub id: u32,
    pub series: BookSeriesConfig<'a, Data>,
    pub inside_id: u8,
    pub inside_name: &'a str,
    pub content: &'a str,
    pub display_type: BookDisplayType,
}

impl<'a, Data: ExcelOutput> FromModel<'a, Data> for LocalbookConfig<'a, Data> {
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
