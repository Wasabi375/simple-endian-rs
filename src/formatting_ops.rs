/*!
Implementations for formatting the various types.
*/
use core::fmt::{Binary, Display, Formatter, LowerHex, Octal, Result, UpperHex};

use super::*;

impl<V: UpperHex + SpecificEndian<B>, B: Copy> UpperHex for BigEndian<V, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:X}", self.to_native()) // delegate to i32's implementation
    }
}

impl<V: UpperHex + SpecificEndian<B>, B: Copy> UpperHex for LittleEndian<V, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:X}", self.to_native()) // delegate to i32's implementation
    }
}

impl<V: LowerHex + SpecificEndian<B>, B: Copy> LowerHex for BigEndian<V, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:x}", self.to_native()) // delegate to i32's implementation
    }
}

impl<V: LowerHex + SpecificEndian<B>, B: Copy> LowerHex for LittleEndian<V, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:x}", self.to_native()) // delegate to i32's implementation
    }
}

impl<V: Octal + SpecificEndian<B>, B: Copy> Octal for BigEndian<V, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:o}", self.to_native()) // delegate to i32's implementation
    }
}

impl<V: Octal + SpecificEndian<B>, B: Copy> Octal for LittleEndian<V, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:o}", self.to_native()) // delegate to i32's implementation
    }
}

impl<V: Binary + SpecificEndian<B>, B: Copy> Binary for BigEndian<V, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:b}", self.to_native()) // delegate to i32's implementation
    }
}

impl<V: Binary + SpecificEndian<B>, B: Copy> Binary for LittleEndian<V, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:b}", self.to_native()) // delegate to i32's implementation
    }
}

impl<V: Display + SpecificEndian<B>, B: Copy> Display for BigEndian<V, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_native()) // delegate to i32's implementation
    }
}

impl<V: Display + SpecificEndian<B>, B: Copy> Display for LittleEndian<V, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_native()) // delegate to i32's implementation
    }
}
