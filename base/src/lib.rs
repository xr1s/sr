pub type FnvIndexMap<K, V> = indexmap::IndexMap<K, V, fnv::FnvBuildHasher>;
pub type FnvMultiMap<K, V> = multimap::MultiMap<K, V, fnv::FnvBuildHasher>;

// ID
pub trait ID {
    type ID: Eq + PartialEq + std::hash::Hash;
    fn id(&self) -> Self::ID;
}

pub trait MainSubID {
    type ID: Eq + PartialEq + std::hash::Hash;
    type SubID: Eq + PartialEq + std::hash::Hash;
    fn id(&self) -> Self::ID;
    fn sub_id(&self) -> Self::SubID;
}

pub trait Name {
    fn name(&self) -> &str;
    fn wiki_name(&self) -> std::borrow::Cow<'_, str>;
}

pub trait Wiki {
    fn wiki(&self) -> std::borrow::Cow<'static, str>;
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
