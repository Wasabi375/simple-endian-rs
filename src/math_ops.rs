//! The math operations.  These all have some cost because they require conversion to native endian.
#[allow(unused_imports)]
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[allow(unused_imports)]
use super::*;

#[allow(unused_macros)]
macro_rules! add_math_ops {
    ($value_ty:ident) => {
        impl<V, B> Add for $value_ty<V, B>
        where
            V: Add<Output = V> + SpecificEndian<B>,
            B: Copy,
        {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                Self::from(self.to_native() + other.to_native())
            }
        }

        impl<V, B> AddAssign for $value_ty<V, B>
        where
            V: Add<Output = V> + SpecificEndian<B>,
            B: Copy,
        {
            fn add_assign(&mut self, other: Self) {
                *self = *self + other;
            }
        }

        impl<V, B> Mul for $value_ty<V, B>
        where
            V: Mul<Output = V> + SpecificEndian<B>,
            B: Copy,
        {
            type Output = Self;

            fn mul(self, other: Self) -> Self {
                Self::from(self.to_native() * other.to_native())
            }
        }

        impl<V, B> MulAssign for $value_ty<V, B>
        where
            V: Mul<Output = V> + SpecificEndian<B>,
            B: Copy,
        {
            fn mul_assign(&mut self, other: Self) {
                *self = *self * other;
            }
        }

        impl<V, B> Div for $value_ty<V, B>
        where
            V: Div<Output = V> + SpecificEndian<B>,
            B: Copy,
        {
            type Output = Self;

            fn div(self, other: Self) -> Self {
                Self::from(self.to_native() / other.to_native())
            }
        }

        impl<V, B> DivAssign for $value_ty<V, B>
        where
            V: Div<Output = V> + SpecificEndian<B>,
            B: Copy,
        {
            fn div_assign(&mut self, other: Self) {
                *self = *self / other;
            }
        }

        impl<V, B> Sub for $value_ty<V, B>
        where
            V: Sub<Output = V> + SpecificEndian<B>,
            B: Copy,
        {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                Self::from(self.to_native() - other.to_native())
            }
        }

        impl<V, B> SubAssign for $value_ty<V, B>
        where
            V: Sub<Output = V> + SpecificEndian<B>,
            B: Copy,
        {
            fn sub_assign(&mut self, other: Self) {
                *self = *self - other;
            }
        }
    };
}

add_math_ops!(LittleEndian);
add_math_ops!(BigEndian);

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn add_fp_be() {
        let mut be1 = f64be::from(1234.5678);
        be1 += 1.0.into();
        be1 += 1.0.into();
        assert_eq!(be1, 1236.5678.into());
    }

    #[test]
    fn subtract_fp_be() {
        let mut be1 = f64be::from(1234.5678);
        be1 -= 1.0.into();
        be1 -= 1.0.into();
        assert_eq!(be1, 1232.5678.into());
    }

    #[test]
    fn mul_fp_be() {
        let mut be1 = f64be::from(1234.5678);
        be1 *= 10.0.into();
        be1 *= 10.0.into();
        assert_eq!(be1, 123456.78.into());
    }

    #[test]
    fn div_fp_be() {
        let mut ne1: f64 = 1234.5678;
        let mut be1 = f64be::from(ne1);
        be1 /= 10.0.into();
        ne1 /= 10.0;
        be1 /= 10.0.into();
        ne1 /= 10.0;
        assert_eq!(ne1, be1.into());
    }
}
