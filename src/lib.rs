#![feature(debug_closure_helpers)]
#![feature(iter_intersperse)]

pub mod format;
pub mod game;
pub mod po;
pub mod vo;

pub use game::GameData;

pub(crate) type FnvIndexMap<K, V> = indexmap::IndexMap<K, V, fnv::FnvBuildHasher>;
pub(crate) type FnvMultiMap<K, V> = multimap::MultiMap<K, V, fnv::FnvBuildHasher>;

pub trait ID {
    type ID: Eq + PartialEq + std::hash::Hash;
    fn id(&self) -> Self::ID;
}

pub trait GroupID {
    type GroupID: Eq + PartialEq + std::hash::Hash;
    type InnerID: Eq + PartialEq + std::hash::Hash;
    fn group_id(&self) -> Self::GroupID;
    fn inner_id(&self) -> Self::InnerID;
}

trait PO<'a> {
    type VO;
    fn vo(&'a self, game: &'a GameData) -> Self::VO;
}

pub trait Name {
    fn name(&self) -> &str;
    fn wiki_name(&self) -> std::borrow::Cow<'_, str>;
}

impl<L: Name, R: Name> Name for either::Either<L, R> {
    fn name(&self) -> &str {
        match self {
            Self::Left(l) => l.name(),
            Self::Right(r) => r.name(),
        }
    }
    fn wiki_name(&self) -> std::borrow::Cow<'_, str> {
        match self {
            Self::Left(l) => l.wiki_name(),
            Self::Right(r) => r.wiki_name(),
        }
    }
}

pub trait Wiki {
    fn wiki(&self) -> std::borrow::Cow<'static, str>;
}
