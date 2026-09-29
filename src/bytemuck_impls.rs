use bytemuck::{CheckedBitPattern, NoUninit, PodInOption, Zeroable, ZeroableInOption};

use crate::{BigEndian, LittleEndian, SpecificEndian};

// TODO all this is only save if SpecificEndian is implemented as a real endian conversion.
//  If SpecificEndian is doing wiered stuff this is unsafe

unsafe impl<T> Zeroable for BigEndian<T> where T: Zeroable + SpecificEndian<T> {}
unsafe impl<T> NoUninit for BigEndian<T> where T: NoUninit + SpecificEndian<T> {}

unsafe impl<T> CheckedBitPattern for BigEndian<T>
where
    T: CheckedBitPattern + SpecificEndian<T>,
    T::Bits: SpecificEndian<T::Bits>,
{
    type Bits = T::Bits;

    fn is_valid_bit_pattern(bits: &Self::Bits) -> bool {
        let native = T::Bits::from_big_endian(*bits);
        T::Bits::is_valid_bit_pattern(&native)
    }
}

unsafe impl<T> ZeroableInOption for BigEndian<T> where T: ZeroableInOption + SpecificEndian<T> {}
unsafe impl<T> PodInOption for BigEndian<T> where T: PodInOption + SpecificEndian<T> {}

unsafe impl<T> Zeroable for LittleEndian<T> where T: Zeroable + SpecificEndian<T> {}
unsafe impl<T> NoUninit for LittleEndian<T> where T: NoUninit + SpecificEndian<T> {}

unsafe impl<T> CheckedBitPattern for LittleEndian<T>
where
    T: CheckedBitPattern + SpecificEndian<T>,
    T::Bits: SpecificEndian<T::Bits>,
{
    type Bits = T::Bits;

    fn is_valid_bit_pattern(bits: &Self::Bits) -> bool {
        let native = T::Bits::from_big_endian(*bits);
        T::Bits::is_valid_bit_pattern(&native)
    }
}

unsafe impl<T> ZeroableInOption for LittleEndian<T> where T: ZeroableInOption + SpecificEndian<T> {}
unsafe impl<T> PodInOption for LittleEndian<T> where T: PodInOption + SpecificEndian<T> {}
