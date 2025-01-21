// TODO: Define a new `SaturatingU16` type. --done
//   It should hold a `u16` value. --done
//   It should be possible to print its debug representation. --done
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct SaturatingU16 {
    value: u16,
}

//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`. --done
impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16 { value }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        SaturatingU16 {
            value: value as u16,
        }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        SaturatingU16 { value: *value }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        SaturatingU16 {
            value: *value as u16,
        }
    }
}

//      It should support addition with a right-hand side of type SaturatingU16, u16, &u16, and &SaturatingU16.
//      Addition should saturate at the maximum value for `u16`.

impl Add<SaturatingU16> for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: SaturatingU16) -> Self::Output {
        Self {
            value: u16::saturating_add(self.value, rhs.value),
        }
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: u16) -> Self::Output {
        Self {
            value: u16::saturating_add(self.value, rhs),
        }
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: &u16) -> Self::Output {
        Self {
            value: u16::saturating_add(self.value, *rhs),
        }
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        Self {
            value: u16::saturating_add(self.value, rhs.value),
        }
    }
}

//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}

impl PartialEq<SaturatingU16> for u16 {
    fn eq(&self, other: &SaturatingU16) -> bool {
        *self == other.value
    }
}
