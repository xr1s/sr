use std::num::NonZero;

pub enum Argument<'a> {
    String(&'a str),
    Signed(i64),
    Unsigned(u64),
    Floating(f64),
}

macro_rules! impl_from_for_argument {
    ($typ:ty, $var:ident) => {
        impl<'a> From<$typ> for Argument<'a> {
            fn from(value: $typ) -> Self {
                Self::$var(value as _)
            }
        }
    };
}

impl_from_for_argument!(i64, Signed);
impl_from_for_argument!(i32, Signed);
impl_from_for_argument!(i16, Signed);
impl_from_for_argument!(i8, Signed);
impl_from_for_argument!(u64, Unsigned);
impl_from_for_argument!(u32, Unsigned);
impl_from_for_argument!(u16, Unsigned);
impl_from_for_argument!(u8, Unsigned);
impl_from_for_argument!(f32, Floating);
impl_from_for_argument!(f64, Floating);
impl_from_for_argument!(&'a str, String);

macro_rules! impl_from_option_nonzero_for_argument {
    ($typ:ty, $var:ident) => {
        impl<'a> From<Option<NonZero<$typ>>> for Argument<'a> {
            fn from(value: Option<NonZero<$typ>>) -> Self {
                Self::$var(value.map(NonZero::get).unwrap_or_default() as _)
            }
        }
    };
}

impl_from_option_nonzero_for_argument!(i64, Signed);
impl_from_option_nonzero_for_argument!(i32, Signed);
impl_from_option_nonzero_for_argument!(i16, Signed);
impl_from_option_nonzero_for_argument!(i8, Signed);
impl_from_option_nonzero_for_argument!(u64, Unsigned);
impl_from_option_nonzero_for_argument!(u32, Unsigned);
impl_from_option_nonzero_for_argument!(u16, Unsigned);
impl_from_option_nonzero_for_argument!(u8, Unsigned);

impl<T: Copy> From<&T> for Argument<'_>
where
    for<'a> Argument<'a>: From<T>,
{
    fn from(value: &T) -> Self {
        Argument::from(*value)
    }
}

impl<T> From<model::Value<T>> for Argument<'_>
where
    for<'a> Argument<'a>: From<T>,
{
    fn from(value: model::Value<T>) -> Self {
        Argument::from(value.value)
    }
}

impl Argument<'_> {
    pub fn from_array<'t, T>(values: &'t [T]) -> Vec<Self>
    where
        for<'a> Argument<'a>: From<&'t T>,
    {
        values.iter().map(Argument::from).collect()
    }
}

impl crate::formattable::Formattable for Argument<'_> {
    fn write_raw(&self, f: &mut crate::Formatter, percent: bool) {
        match self {
            Argument::String(s) => s.write_raw(f, percent),
            Argument::Signed(n) => n.write_raw(f, percent),
            Argument::Unsigned(n) => n.write_raw(f, percent),
            Argument::Floating(n) => n.write_raw(f, percent),
        }
    }

    fn write_int(&self, f: &mut crate::Formatter, percent: bool) {
        match self {
            Argument::String(s) => s.write_int(f, percent),
            Argument::Signed(n) => n.write_int(f, percent),
            Argument::Unsigned(n) => n.write_int(f, percent),
            Argument::Floating(n) => n.write_int(f, percent),
        }
    }

    fn write_float(&self, f: &mut crate::Formatter, prec: usize, percent: bool) {
        match self {
            Argument::String(s) => s.write_float(f, prec, percent),
            Argument::Signed(n) => n.write_float(f, prec, percent),
            Argument::Unsigned(n) => n.write_float(f, prec, percent),
            Argument::Floating(n) => n.write_float(f, prec, percent),
        }
    }
}
