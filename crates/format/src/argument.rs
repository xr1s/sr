use std::num::NonZero;

#[derive(Copy, Clone, Debug)]
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

impl<Data: crate::data::GameData> crate::formattable::Formattable<Argument<'_>>
    for crate::Formatter<'_, Data>
{
    fn write_raw(&mut self, value: &Argument<'_>, percent: bool) {
        match value {
            Argument::String(s) => {
                <Self as crate::formattable::Formattable<&str>>::write_raw(self, s, percent);
            }
            Argument::Signed(n) => {
                <Self as crate::formattable::Formattable<i64>>::write_raw(self, n, percent);
            }
            Argument::Unsigned(n) => {
                <Self as crate::formattable::Formattable<u64>>::write_raw(self, n, percent);
            }
            Argument::Floating(n) => {
                <Self as crate::formattable::Formattable<f64>>::write_raw(self, n, percent);
            }
        }
    }

    fn write_int(&mut self, value: &Argument<'_>, percent: bool) {
        match value {
            Argument::String(s) => {
                <Self as crate::formattable::Formattable<&str>>::write_int(self, s, percent);
            }
            Argument::Signed(n) => {
                <Self as crate::formattable::Formattable<i64>>::write_int(self, n, percent);
            }
            Argument::Unsigned(n) => {
                <Self as crate::formattable::Formattable<u64>>::write_int(self, n, percent);
            }
            Argument::Floating(n) => {
                <Self as crate::formattable::Formattable<f64>>::write_int(self, n, percent);
            }
        }
    }

    fn write_float(&mut self, value: &Argument<'_>, prec: u32, percent: bool) {
        match value {
            Argument::String(s) => {
                <Self as crate::formattable::Formattable<&str>>::write_float(
                    self, s, prec, percent,
                );
            }
            Argument::Signed(n) => {
                <Self as crate::formattable::Formattable<i64>>::write_float(self, n, prec, percent);
            }
            Argument::Unsigned(n) => {
                <Self as crate::formattable::Formattable<u64>>::write_float(self, n, prec, percent);
            }
            Argument::Floating(n) => {
                <Self as crate::formattable::Formattable<f64>>::write_float(self, n, prec, percent);
            }
        }
    }
}
