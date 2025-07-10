/*!
Provides a bunch of short type names for easier declaration.  All follow a pattern of LittleEndian<BASETY> = BASETYPEle and BigEndian<BASETYPE, BASETY> = BASETYPEle and BigEndian<BASETYPE> = BASETYPEbe
*/

#![allow(non_camel_case_types)]
use super::*;
/// Shorthand for `LittleEndian<u16, u16>`
pub type u16le = LittleEndian<u16, u16>;
/// Shorthand for `BigEndian<u16, u16>`
pub type u16be = BigEndian<u16, u16>;
/// Shorthand for `LittleEndian<u32, u32>`
pub type u32le = LittleEndian<u32, u32>;
/// Shorthand for `BigEndian<u32, u32>`
pub type u32be = BigEndian<u32, u32>;
/// Shorthand for `LittleEndian<u64, u64>`
pub type u64le = LittleEndian<u64, u64>;
/// Shorthand for `BigEndian<u64, u64>`
pub type u64be = BigEndian<u64, u64>;
/// Shorthand for `LittleEndian<u128, u128>`
pub type u128le = LittleEndian<u128, u128>;
/// Shorthand for `BigEndian<u128, u128>`
pub type u128be = BigEndian<u128, u128>;
/// Shorthand for `LittleEndian<usize, usize>`
pub type usizele = LittleEndian<usize, usize>;
/// Shorthand for `BigEndian<usize, usize>`
pub type usizebe = BigEndian<usize, usize>;

/// Shorthand for `LittleEndian<i16, i16>`
pub type i16le = LittleEndian<i16, i16>;
/// Shorthand for `BigEndian<i16, i16>`
pub type i16be = BigEndian<i16, i16>;
/// Shorthand for `LittleEndian<i32, i32>`
pub type i32le = LittleEndian<i32, i32>;
/// Shorthand for `BigEndian<i32, i32>`
pub type i32be = BigEndian<i32, i32>;
/// Shorthand for `LittleEndian<i64, i64>`
pub type i64le = LittleEndian<i64, i64>;
/// Shorthand for `BigEndian<i64, i64>`
pub type i64be = BigEndian<i64, i64>;
/// Shorthand for `LittleEndian<i128, i128>`
pub type i128le = LittleEndian<i128, i128>;
/// Shorthand for `BigEndian<i128, i128>`
pub type i128be = BigEndian<i128, i128>;
/// Shorthand for `LittleEndian<isize, isize>`
pub type isizele = LittleEndian<isize, isize>;
/// Shorthand for `BigEndian<isize, isize>`
pub type isizebe = BigEndian<isize, isize>;

/// Shorthand for `LittleEndian<f32, f32>`
pub type f32le = LittleEndian<f32, u32>;
/// Shorthand for `BigEndian<f32, f32>`
pub type f32be = BigEndian<f32, u32>;

/// Shorthand for `LittleEndian<f64, f64>`
pub type f64le = LittleEndian<f64, u64>;
/// Shorthand for `BigEndian<f64, f64>`
pub type f64be = BigEndian<f64, u64>;
