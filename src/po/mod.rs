use std::borrow::Cow;

use crate::Wiki;

pub mod challenge;
pub mod misc;
pub mod monster;
pub mod rogue;
pub mod rogue_tourn;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
/// 元素
pub enum Element {
    /// 火
    Fire,
    /// 冰
    Ice,
    /// 虚数
    Imaginary,
    /// 物理
    Physical,
    /// 量子
    Quantum,
    /// 雷
    Thunder,
    /// 风
    Wind,
}

impl Wiki for Element {
    fn wiki(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Element::Fire => "火",
            Element::Ice => "冰",
            Element::Imaginary => "虚数",
            Element::Physical => "物理",
            Element::Quantum => "量子",
            Element::Thunder => "雷",
            Element::Wind => "风",
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
/// 命途，目前只有模拟宇宙在用
pub enum Path {
    /// 存护
    Preservation,
    /// 记忆
    Remembrance,
    /// 虚无
    Nihility,
    /// 丰饶
    Abundance,
    /// 巡猎
    Hunt,
    /// 毁灭
    Destruction,
    /// 欢愉
    Elation,
    /// 繁育
    Propagation,
    /// 智识
    Erudition,
}

impl Wiki for Path {
    fn wiki(&self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Path::Preservation => "存护",
            Path::Remembrance => "记忆",
            Path::Nihility => "虚无",
            Path::Abundance => "丰饶",
            Path::Hunt => "巡猎",
            Path::Destruction => "毁灭",
            Path::Elation => "欢愉",
            Path::Propagation => "繁育",
            Path::Erudition => "智识",
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Text {
    pub hash: i32,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Value<T> {
    pub value: T,
}
