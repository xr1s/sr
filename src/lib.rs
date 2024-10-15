#![feature(iter_intersperse)]

pub mod po;
pub mod vo;

pub mod format;
pub mod game;

pub use game::GameData;

pub trait ID {
    type ID: Eq + PartialEq + std::hash::Hash;
    fn id(&self) -> Self::ID;
}

trait PO<'a> {
    type VO;
    fn vo(&'a self, game: &'a GameData) -> Self::VO;
}

pub trait Name {
    fn name(&self) -> &str;
}

impl<L: Name, R: Name> Name for either::Either<L, R> {
    fn name(&self) -> &str {
        match self {
            Self::Left(l) => l.name(),
            Self::Right(r) => r.name(),
        }
    }
}

pub trait Wiki {
    fn wiki(&self) -> std::borrow::Cow<'static, str>;
}
