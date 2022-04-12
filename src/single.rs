macro_rules! to_primitive_impl {
    ($to_primitive:ident, $primitive:ty) => {
        pub fn $to_primitive(self) -> Option<$primitive> {
            self.version.$to_primitive()
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
pub struct VersionSingle<I: num::Integer + num::cast::ToPrimitive> {
    pub(crate) version: I,
}

impl<I: num::Integer + num::cast::ToPrimitive> VersionSingle<I> {
    pub fn new(version: I) -> Self {
        Self { version }
    }

    to_primitives!();
}
