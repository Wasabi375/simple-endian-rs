//! Comparison ops.
#[allow(unused_imports)]
use core::cmp::Ordering;

#[allow(unused_imports)]
use super::*;

impl<V, B> PartialOrd for BigEndian<V, B>
where
    V: SpecificEndian<B> + PartialOrd,
    B: Copy,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_native().partial_cmp(&other.to_native())
    }
}

impl<V, B> PartialOrd for LittleEndian<V, B>
where
    V: SpecificEndian<B> + PartialOrd,
    B: Copy,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_native().partial_cmp(&other.to_native())
    }
}

impl<V, B> Ord for BigEndian<V, B>
where
    V: SpecificEndian<B> + Ord,
    B: Copy,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_native().cmp(&other.to_native())
    }
}

impl<V, B> Ord for LittleEndian<V, B>
where
    V: SpecificEndian<B> + Ord,
    B: Copy,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_native().cmp(&other.to_native())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn equality_test() {
        let be1 = BigEndian::from(12345);
        let be2 = BigEndian::from(12345);
        assert!(be1 == be2);
    }

    #[test]
    fn not_equality_test() {
        let be1 = BigEndian::from(12345);
        let be2 = BigEndian::from(34565);
        assert!(be1 != be2);
    }

    #[test]
    fn lt_test() {
        let be1 = BigEndian::from(12345);
        let be2 = BigEndian::from(34565);
        assert!(be1 < be2);
    }

    #[test]
    fn gt_test() {
        let be1 = BigEndian::from(34565);
        let be2 = BigEndian::from(12345);
        assert!(be1 > be2);
    }

    #[test]
    fn lt_fp_be() {
        let be1 = BigEndian::<f64, u64>::from(1234.5678);
        let be2 = BigEndian::<f64, u64>::from(6234.5678);
        assert!(be1 < be2);
    }
}
