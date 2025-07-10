use core::marker::PhantomData;

/// Any object implementing `SpecificEndian<T>` can be converted between big and little endian.  Implement this trait to allow for endian conversion by this crate.
pub trait SpecificEndian<T>
where
    Self: Clone + Copy,
    T: Clone + Copy,
{
    fn to_big_endian(&self) -> T;
    fn to_little_endian(&self) -> T;
    fn from_big_endian(value: T) -> Self;
    fn from_little_endian(value: T) -> Self;
}

#[cfg(feature = "byte_impls")]
mod byte_impls {
    use super::*;
    /// A macro implementing `SpecificEndian<T>` for simple data types where big and little endian forms are the same.
    macro_rules! make_specific_endian_single_byte {
        ($wrap_ty:ty) => {
            impl SpecificEndian<$wrap_ty> for $wrap_ty {
                fn to_big_endian(&self) -> Self {
                    *self
                }
                fn to_little_endian(&self) -> Self {
                    *self
                }
                fn from_big_endian(value: Self) -> Self {
                    value
                }
                fn from_little_endian(value: Self) -> Self {
                    value
                }
            }
        };
    }

    make_specific_endian_single_byte!(u8);
    make_specific_endian_single_byte!(i8);
    // If bool ends up being represented by something other than a byte, this might not work right.
    make_specific_endian_single_byte!(bool);
}

#[cfg(feature = "integer_impls")]
mod integer_impls {
    use super::*;
    /// A macro for implementing `SpecificEndian<T>` on types that have endian conversions built into Rust.  Currently, this is the primitive integer types.
    macro_rules! make_specific_endian_integer {
        ($wrap_ty:ty) => {
            impl SpecificEndian<$wrap_ty> for $wrap_ty {
                fn to_big_endian(&self) -> Self {
                    self.to_be()
                }
                fn to_little_endian(&self) -> Self {
                    self.to_le()
                }
                fn from_big_endian(value: Self) -> Self {
                    Self::from_be(value)
                }
                fn from_little_endian(value: Self) -> Self {
                    Self::from_le(value)
                }
            }
        };
    }

    make_specific_endian_integer!(u16);
    make_specific_endian_integer!(i16);
    make_specific_endian_integer!(u32);
    make_specific_endian_integer!(i32);
    make_specific_endian_integer!(u64);
    make_specific_endian_integer!(i64);
    make_specific_endian_integer!(u128);
    make_specific_endian_integer!(i128);
    make_specific_endian_integer!(usize);
    make_specific_endian_integer!(isize);
}

#[cfg(feature = "non_zero_impls")]
mod non_zero {
    use super::*;
    use core::num::{
        NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU16, NonZeroU32,
        NonZeroU64, NonZeroU8, NonZeroUsize,
    };

    macro_rules! make_specific_non_zero {
        ($wrap_ty:ty, $primitive:ty) => {
            impl SpecificEndian<$wrap_ty> for $wrap_ty {
                fn to_big_endian(&self) -> $wrap_ty {
                    unsafe {
                        // Safety: endian conversion can only lead to 0 if self is already 0,
                        // but self is NonZero
                        Self::new_unchecked(self.get().to_be())
                    }
                }

                fn to_little_endian(&self) -> $wrap_ty {
                    unsafe {
                        // Safety: endian conversion can only lead to 0 if self is already 0,
                        // but self is NonZero
                        Self::new_unchecked(self.get().to_le())
                    }
                }

                fn from_big_endian(value: $wrap_ty) -> Self {
                    unsafe {
                        // Safety: endian conversion can only lead to 0 if self is already 0,
                        // but self is NonZero
                        Self::new_unchecked(<$primitive>::from_big_endian(value.get()))
                    }
                }

                fn from_little_endian(value: $wrap_ty) -> Self {
                    unsafe {
                        // Safety: endian conversion can only lead to 0 if self is already 0,
                        // but self is NonZero
                        Self::new_unchecked(<$primitive>::from_little_endian(value.get()))
                    }
                }
            }
        };
    }

    make_specific_non_zero!(NonZeroU8, u8);
    make_specific_non_zero!(NonZeroU16, u16);
    make_specific_non_zero!(NonZeroU32, u32);
    make_specific_non_zero!(NonZeroU64, u64);
    make_specific_non_zero!(NonZeroUsize, usize);
    make_specific_non_zero!(NonZeroI8, i8);
    make_specific_non_zero!(NonZeroI16, i16);
    make_specific_non_zero!(NonZeroI32, i32);
    make_specific_non_zero!(NonZeroI64, i64);
    make_specific_non_zero!(NonZeroIsize, isize);
}

#[cfg(feature = "float_impls")]
mod float_impls {
    use super::*;
    /// Uses .from_bits() and .to_bits() to implement SpecificEndian<T> with Integer types.  Can be used with any type having these methods, but mainly for use with the floats.
    macro_rules! make_specific_endian_float {
        ($value_ty:ty, $bit_ty:ty) => {
            impl SpecificEndian<$bit_ty> for $value_ty {
                fn to_big_endian(&self) -> $bit_ty {
                    self.to_bits().to_be()
                }
                fn to_little_endian(&self) -> $bit_ty {
                    self.to_bits().to_le()
                }
                fn from_big_endian(value: $bit_ty) -> Self {
                    <$value_ty>::from_bits(<$bit_ty>::from_big_endian(value))
                }
                fn from_little_endian(value: $bit_ty) -> Self {
                    <$value_ty>::from_bits(<$bit_ty>::from_little_endian(value))
                }
            }
        };
    }

    make_specific_endian_float!(f32, u32);
    make_specific_endian_float!(f64, u64);
}

/// A big-endian representation of type `B` that implements `SpecificEndian<B>`.  Data stored in the struct must be converted to big-endian using `::from()` or `.into()`.
#[derive(Copy, Clone, Debug, Default, Hash)]
#[repr(transparent)]
pub struct BigEndian<V: SpecificEndian<B>, B: Copy = V>(pub(crate) B, pub(crate) PhantomData<V>);

impl<V, B> BigEndian<V, B>
where
    V: SpecificEndian<B>,
    B: Copy,
{
    /// Returns the raw data stored in the struct.
    pub const fn to_bits(&self) -> B {
        self.0
    }
    /// Imports the data raw into a BigEndian<B> struct.
    pub const fn from_bits(v: B) -> Self {
        Self(v, PhantomData)
    }
    /// Converts the data to the same type B in host-native endian.
    pub fn to_native(&self) -> V {
        V::from_big_endian(self.0)
    }
}

impl<V: SpecificEndian<B>, B: Copy> From<V> for BigEndian<V, B> {
    fn from(v: V) -> BigEndian<V, B> {
        BigEndian::<V, B>(v.to_big_endian(), PhantomData)
    }
}

impl<V: SpecificEndian<B> + PartialEq, B: Copy> PartialEq for BigEndian<V, B> {
    fn eq(&self, other: &Self) -> bool {
        self.to_native() == other.to_native()
    }
}
impl<V: SpecificEndian<B> + Eq, B: Copy> Eq for BigEndian<V, B> {}

/// A little-endian representation of type `T` that implements `SpecificEndian<T>`.  Data stored in the struct must be converted to little-endian using `::from()` or `.into()`.
#[derive(Copy, Clone, Debug, Default, Hash)]
#[repr(transparent)]
pub struct LittleEndian<V: SpecificEndian<B>, B: Copy = V>(pub(crate) B, pub(crate) PhantomData<V>);

impl<V, B> LittleEndian<V, B>
where
    V: SpecificEndian<B>,
    B: Copy,
{
    /// Returns the raw data stored in the struct.
    pub const fn to_bits(&self) -> B {
        self.0
    }
    /// Imports the data raw into a LittleEndian<T> struct.
    pub const fn from_bits(v: B) -> Self {
        Self(v, PhantomData)
    }
    /// Converts the data to the same type T in host-native endian.
    pub fn to_native(&self) -> V {
        V::from_little_endian(self.0)
    }
}

impl<V: SpecificEndian<B>, B: Copy> From<V> for LittleEndian<V, B> {
    fn from(v: V) -> LittleEndian<V, B> {
        LittleEndian::<V, B>(v.to_little_endian(), PhantomData)
    }
}

impl<V: SpecificEndian<B> + PartialEq, B: Copy> PartialEq for LittleEndian<V, B> {
    fn eq(&self, other: &Self) -> bool {
        self.to_native() == other.to_native()
    }
}

impl<V: SpecificEndian<B> + Eq, B: Copy> Eq for LittleEndian<V, B> {}

#[cfg(feature = "big_endian")]
mod big_endian_primatives {
    #[allow(unused_imports)]
    use super::*;
    // Rust's orphan trait rule prevents us from using a generic implementation on the primitive types, so we do this:
    #[allow(unused_macros)]
    macro_rules! make_primitive_type_from_be {
        ($value_ty:ty, $bit_ty:ty) => {
            impl From<BigEndian<$value_ty, $bit_ty>> for $value_ty {
                fn from(v: BigEndian<$value_ty, $bit_ty>) -> $value_ty {
                    <$value_ty>::from_big_endian(v.0)
                }
            }
        };
    }

    #[cfg(feature = "integer_impls")]
    make_primitive_type_from_be!(bool, bool);
    #[cfg(feature = "integer_impls")]
    make_primitive_type_from_be!(u8, u8);
    #[cfg(feature = "integer_impls")]
    make_primitive_type_from_be!(i8, i8);
    #[cfg(feature = "integer_impls")]
    make_primitive_type_from_be!(u16, u16);
    #[cfg(feature = "integer_impls")]
    make_primitive_type_from_be!(i16, i16);
    #[cfg(feature = "integer_impls")]
    make_primitive_type_from_be!(u32, u32);
    #[cfg(feature = "integer_impls")]
    make_primitive_type_from_be!(i32, i32);
    #[cfg(feature = "integer_impls")]
    make_primitive_type_from_be!(u64, u64);
    #[cfg(feature = "integer_impls")]
    make_primitive_type_from_be!(i64, i64);
    #[cfg(feature = "integer_impls")]
    make_primitive_type_from_be!(u128, u128);
    #[cfg(feature = "integer_impls")]
    make_primitive_type_from_be!(i128, i128);
    #[cfg(feature = "integer_impls")]
    make_primitive_type_from_be!(usize, usize);
    #[cfg(feature = "integer_impls")]
    make_primitive_type_from_be!(isize, isize);
    #[cfg(feature = "float_impls")]
    make_primitive_type_from_be!(f32, u32);
    #[cfg(feature = "float_impls")]
    make_primitive_type_from_be!(f64, u64);
}

#[cfg(feature = "little_endian")]
mod little_endian_primatives {
    #[allow(unused_imports)]
    use super::*;
    // Rust's orphan trait rule prevents us from using a generic implementation on the primitive types, so we do this:
    #[allow(unused_macros)]
    macro_rules! make_primitive_type_from_le {
        ($value_ty:ty, $bit_ty:ty) => {
            impl From<LittleEndian<$value_ty, $bit_ty>> for $value_ty {
                fn from(v: LittleEndian<$value_ty, $bit_ty>) -> $value_ty {
                    <$value_ty>::from_little_endian(v.0)
                }
            }
        };
    }

    #[cfg(feature = "integer_impls")]
    make_primitive_type_from_le!(bool, bool);
    #[cfg(feature = "integer_impls")]
    make_primitive_type_from_le!(u8, u8);
    #[cfg(feature = "integer_impls")]
    make_primitive_type_from_le!(i8, i8);
    #[cfg(feature = "integer_impls")]
    make_primitive_type_from_le!(u16, u16);
    #[cfg(feature = "integer_impls")]
    make_primitive_type_from_le!(i16, i16);
    #[cfg(feature = "integer_impls")]
    make_primitive_type_from_le!(u32, u32);
    #[cfg(feature = "integer_impls")]
    make_primitive_type_from_le!(i32, i32);
    #[cfg(feature = "integer_impls")]
    make_primitive_type_from_le!(u64, u64);
    #[cfg(feature = "integer_impls")]
    make_primitive_type_from_le!(i64, i64);
    #[cfg(feature = "integer_impls")]
    make_primitive_type_from_le!(u128, u128);
    #[cfg(feature = "integer_impls")]
    make_primitive_type_from_le!(i128, i128);
    #[cfg(feature = "integer_impls")]
    make_primitive_type_from_le!(usize, usize);
    #[cfg(feature = "integer_impls")]
    make_primitive_type_from_le!(isize, isize);
    #[cfg(feature = "float_impls")]
    make_primitive_type_from_le!(f32, u32);
    #[cfg(feature = "float_impls")]
    make_primitive_type_from_le!(f64, u64);
}

#[cfg(feature = "both_endian")]
mod both_endian_primatives {
    use super::*;
    /// Allow conversion directly from `LittleEndian<T>` to `BigEndian<T>` without manually going through native endian.
    impl<V: SpecificEndian<B>, B: Copy> From<LittleEndian<V, B>> for BigEndian<V, B> {
        fn from(v: LittleEndian<V, B>) -> BigEndian<V, B> {
            BigEndian::<V, B>::from_bits(v.to_native().to_big_endian())
        }
    }

    /// Allow conversion directly from `BigEndian<T>` to `LittleEndian<T>` without manually going through native endian.
    impl<V: SpecificEndian<B>, B: Copy> From<BigEndian<V, B>> for LittleEndian<V, B> {
        fn from(v: BigEndian<V, B>) -> LittleEndian<V, B> {
            LittleEndian::<V, B>::from_bits(v.to_native().to_little_endian())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use core::mem::size_of;

    #[test]
    fn declare_all() {
        let _a: BigEndian<i16, i16> = 0xfe.into();
        let _a: LittleEndian<i16, i16> = 0xfe.into();
        let _a: BigEndian<u16, u16> = 0xfe.into();
        let _a: LittleEndian<u16, u16> = 0xfe.into();

        let _a: BigEndian<i32, i32> = 0xfe.into();
        let _a: LittleEndian<i32, i32> = 0xfe.into();
        let _a: BigEndian<u32, u32> = 0xfe.into();
        let _a: LittleEndian<u32, u32> = 0xfe.into();

        let _a: BigEndian<i64, i64> = 0xfe.into();
        let _a: LittleEndian<i64, i64> = 0xfe.into();
        let _a: BigEndian<u64, u64> = 0xfe.into();
        let _a: LittleEndian<u64, u64> = 0xfe.into();

        let _a: BigEndian<i128, i128> = 0xfe.into();
        let _a: LittleEndian<i128, i128> = 0xfe.into();
        let _a: BigEndian<u128, u128> = 0xfe.into();
        let _a: LittleEndian<u128, u128> = 0xfe.into();
    }

    #[test]
    fn make_struct() {
        #[repr(C)]
        struct Foo(
            BigEndian<i16, i16>,
            LittleEndian<i16, i16>,
            BigEndian<u16, u16>,
            LittleEndian<u16, u16>,
            BigEndian<i32, i32>,
            LittleEndian<i32, i32>,
            BigEndian<u32, u32>,
            LittleEndian<u32, u32>,
            BigEndian<i64, i64>,
            LittleEndian<i64, i64>,
            BigEndian<u64, u64>,
            LittleEndian<u64, u64>,
            BigEndian<i128, i128>,
            LittleEndian<i128, i128>,
            BigEndian<u128, u128>,
            LittleEndian<u128, u128>,
            BigEndian<f32, u32>,
            LittleEndian<f32, u32>,
            BigEndian<f64, u64>,
            LittleEndian<f64, u64>,
        );

        let _foo = Foo(
            0.into(),
            1.into(),
            2.into(),
            3.into(),
            4.into(),
            5.into(),
            6.into(),
            7.into(),
            8.into(),
            9.into(),
            10.into(),
            11.into(),
            12.into(),
            13.into(),
            14.into(),
            15.into(),
            (0.1).into(),
            (123.5).into(),
            (7.8).into(),
            (12345.4567).into(),
        );
    }

    #[test]
    fn store_be() {
        let be: BigEndian<u64, u64> = 0xfe.into();
        if cfg!(target_endian = "big") {
            assert_eq!(be.to_bits(), 0xfe);
        } else {
            assert_eq!(be.to_bits(), 0xfe00000000000000);
        }
    }

    #[test]
    fn same_size() {
        assert_eq!(size_of::<u64be>(), size_of::<u64>());
    }

    #[test]
    fn store_le() {
        let le: LittleEndian<u64, u64> = 0xfe.into();
        if cfg!(target_endian = "big") {
            assert_eq!(le.to_bits(), 0xfe00000000000000);
        } else {
            assert_eq!(le.to_bits(), 0xfe);
        }
    }

    #[test]
    fn cast() {
        let be = BigEndian::<u64, u64>::from(12345);
        let ne: u64 = be.into();
        assert_eq!(ne, 12345);
    }

    #[test]
    fn convert_back() {
        let be = BigEndian::<u64, u64>::from(12345);
        println!("{}", u64::from(be));
    }

    #[test]
    fn convert_to_native() {
        let be = BigEndian::from(0xfe);
        println!("{:x}, {:x}", be.0, be.to_native());
        assert_eq!(0xfe, be.to_native());
    }

    #[test]
    fn store_fp_be() {
        let be1 = BigEndian::<f64, u64>::from(1234.5678);
        if cfg!(target_endian = "little") {
            assert_ne!(1234.5678, f64::from_bits(be1.to_bits()));
        }
        assert_eq!(1234.5678, f64::from(be1));
    }

    #[test]
    fn store_fp_le() {
        let le1 = LittleEndian::<f64, u64>::from(1234.5678);
        if cfg!(target_endian = "big") {
            assert_ne!(1234.5678, f64::from_bits(le1.to_bits()));
        }
        assert_eq!(1234.5678, f64::from(le1));
    }

    #[test]
    fn inferred_type() {
        let mut be1 = BigEndian::from(1234);
        be1 &= BigEndian::from(5678);
        println!("{} {} {}", be1, be1.to_bits(), be1.to_native());
        assert_eq!(be1, 1026.into());
    }

    #[test]
    fn inferred_type_bigger() {
        let mut be1 = BigEndian::from(0x0feeddcc);
        be1 &= BigEndian::from(0xff00);
        println!("{} {} {}", be1, be1.to_bits(), be1.to_native());
        assert_eq!(be1, 0xdd00.into());
    }

    #[test]
    fn mixed_endian_big() {
        let be = BigEndian::from(100);
        let le = LittleEndian::from(200);
        let me = be + le.into();
        assert_eq!(me, 300.into());
    }

    #[test]
    fn mixed_endian_little() {
        let be = BigEndian::from(100);
        let le = LittleEndian::from(200);
        let me = le + be.into();
        assert_eq!(me, 300.into());
    }

    #[test]
    fn custom_type() {
        #[derive(Copy, Clone, Debug)]
        enum EndianAwareExample {
            BigEndianFunction(u64),
            LittleEndianFunction(u64),
        }
        impl SpecificEndian<EndianAwareExample> for EndianAwareExample {
            fn to_big_endian(&self) -> Self {
                match self {
                    EndianAwareExample::BigEndianFunction(_) => *self,
                    EndianAwareExample::LittleEndianFunction(v) => {
                        EndianAwareExample::BigEndianFunction(v.to_big_endian())
                    }
                }
            }
            fn to_little_endian(&self) -> Self {
                match self {
                    EndianAwareExample::LittleEndianFunction(_0) => *self,
                    EndianAwareExample::BigEndianFunction(v) => {
                        EndianAwareExample::BigEndianFunction(v.to_little_endian())
                    }
                }
            }
            fn from_big_endian(value: Self) -> Self {
                match value {
                    EndianAwareExample::BigEndianFunction(_0) => value,
                    EndianAwareExample::LittleEndianFunction(v) => {
                        EndianAwareExample::BigEndianFunction(v.to_big_endian())
                    }
                }
            }
            fn from_little_endian(value: Self) -> Self {
                match value {
                    EndianAwareExample::LittleEndianFunction(_) => value,
                    EndianAwareExample::BigEndianFunction(v) => {
                        EndianAwareExample::BigEndianFunction(v.to_little_endian())
                    }
                }
            }
        }
        let foo: BigEndian<EndianAwareExample> =
            EndianAwareExample::LittleEndianFunction(0xf0).into();
        #[allow(unused_assignments)]
        let mut value = 0;
        match foo.to_native() {
            EndianAwareExample::BigEndianFunction(v) => {
                println!("be: {:x}", v);
                value = v
            }
            EndianAwareExample::LittleEndianFunction(v) => {
                println!("le: {:x}", v);
                value = 0
            }
        }
        assert_eq!(value, 0x0f000000000000000);
    }
}
