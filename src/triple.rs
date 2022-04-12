use crate::single::VersionSingle;

macro_rules! to_primitive_impl {
    ($to_primitive:ident, $primitive:ty) => {
        pub fn $to_primitive(self) -> Option<($primitive, $primitive, $primitive)> {
            let major = self.major.$to_primitive()?;
            let minor = self.minor.$to_primitive()?;
            let patch = self.patch.$to_primitive()?;
            Some((major, minor, patch))
        }
    };
}

macro_rules! to_primitives {
    () => {
        to_primitive_impl!(to_i8, i8);
        to_primitive_impl!(to_i16, i16);
        to_primitive_impl!(to_i32, i32);
        to_primitive_impl!(to_i64, i64);

        to_primitive_impl!(to_u8, u8);
        to_primitive_impl!(to_u16, u16);
        to_primitive_impl!(to_u32, u32);
        to_primitive_impl!(to_u64, u64);
    };
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct VersionTriple<I: num::Integer + num::cast::ToPrimitive + Copy> {
    major: I,
    minor: I,
    patch: I,
}

impl<I: num::Integer + num::cast::ToPrimitive + num::Zero + Copy> VersionTriple<I> {
    pub fn new(major: I, minor: I, patch: I) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    pub fn major(&self) -> I {
        self.major
    }

    pub fn minor(&self) -> I {
        self.minor
    }

    pub fn patch(&self) -> I {
        self.patch
    }

    to_primitives!();
}

impl<I: num::Integer + num::cast::ToPrimitive + num::Zero + Copy> From<VersionSingle<I>>
    for VersionTriple<I>
{
    fn from(single: VersionSingle<I>) -> VersionTriple<I> {
        Self {
            major: single.version(),
            minor: I::zero(),
            patch: I::zero(),
        }
    }
}
