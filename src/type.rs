use std::fmt;
use std::ops;
use crate::parse::Data;
pub struct StrCode(Vec<(usize, Data)>);

/// This is a Copy of [Serde_json::index](https://docs.serde.rs/src/serde_json/value/index.rs.html)
pub trait Index: private::Sealed {
    #[doc(hidden)]
    fn index_into<'v>(&self, v: &'v StrCode) -> Option<&'v StrCode>;

    #[doc(hidden)]
    fn index_into_mut<'v>(&self, v: &'v mut StrCode) -> Option<&'v mut StrCode>;

    #[doc(hidden)]
    fn index_or_insert<'v>(&self, v: &'v mut StrCode) -> &'v mut StrCode;
}

impl Index for usize {
    fn index_into<'v>(&self, v: &'v StrCode) -> Option<&'v StrCode> {
        match *v {
            StrCode::Vector(ref vec) => vec.0.get(*self),
            StrCode::List(ref vec) => vec.0.get(*self),
            _ => None,
        }
    }
    fn index_into_mut<'v>(&self, v: &'v mut StrCode) -> Option<&'v mut StrCode> {
        match *v {
            StrCode::Vector(ref mut vec) => vec.0.get_mut(*self),
            StrCode::List(ref mut vec) => vec.0.get_mut(*self),
            _ => None,
        }
    }
    fn index_or_insert<'v>(&self, v: &'v mut StrCode) -> &'v mut StrCode {
        match *v {
            StrCode::Vector(ref mut vec) => {
                let len = vec.0.len();
                vec.0.get_mut(*self).unwrap_or_else(|| {
                    panic!(
                        "cannot access index {} of StrCode array of length {}",
                        self, len
                    )
                })
            }
            _ => panic!("cannot access index {} of StrCode {}", self, Type(v)),
        }
    }
}

// Prevent users from implementing the Index trait.
mod private {
    pub trait Sealed {}
    impl Sealed for usize {}
    impl Sealed for str {}
    impl Sealed for String {}
    impl<'a, T: ?Sized> Sealed for &'a T where T: Sealed {}
}
struct Type<'a>(&'a StrCode);

impl<'a> fmt::Display for Type<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            StrCode::Empty => formatter.write_str("empty"),
            StrCode::Nil => formatter.write_str("null"),
            StrCode::Bool(_) => formatter.write_str("boolean"),
            StrCode::Int(_) => formatter.write_str("integer"),
            StrCode::UInt(_) => formatter.write_str("integer"),
            StrCode::Str(_) => formatter.write_str("string"),
            StrCode::Vector(_) => formatter.write_str("vector"),
            StrCode::Set(_) => formatter.write_str("set"),
            StrCode::List(_) => formatter.write_str("list"),
            StrCode::Map(_) => formatter.write_str("map"),
            StrCode::Key(_) => formatter.write_str("key"),
            StrCode::Char(_) => formatter.write_str("char"),
            StrCode::Symbol(_) => formatter.write_str("symbol"),
            StrCode::Double(_) => formatter.write_str("double"),
            StrCode::Inst(_) => formatter.write_str("inst"),
            StrCode::Rational(_) => formatter.write_str("rational"),
        }
    }
}

impl<I> ops::Index<I> for StrCode
where
    I: Index,
{
    type Output = StrCode;
    fn index(&self, index: I) -> &StrCode {
        index.index_into(self).unwrap()
    }
}

impl<I> ops::IndexMut<I> for StrCode
where
    I: Index,
{
    fn index_mut(&mut self, index: I) -> &mut StrCode {
        index.index_or_insert(self)
    }
}