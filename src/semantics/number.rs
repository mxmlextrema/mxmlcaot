use crate::ns::*;

use num_traits::FromPrimitive;
// use num_traits::{One, Zero};
// use num_bigint::BigInt;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Rem, Shl, Shr, Sub};

/// Represents a numeric value represented as one of the data types
/// `Number`, `float`, `uint`, or `int`.
#[derive(Clone, PartialEq, PartialOrd)]
pub enum Number {
    Number(f64),
    Float(f32),
    Int(i32),
    Uint(u32),
}

impl Add for Number {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::Float(v) => {
                let rhs = rhs.force_float();
                Self::Float(v + rhs)
            },
            Self::Number(v) => {
                let rhs = rhs.force_double();
                Self::Number(v + rhs)
            },
            Self::Int(v) => {
                let rhs = rhs.force_int();
                Self::Int(v.checked_add(rhs).unwrap_or(0))
            },
            Self::Uint(v) => {
                let rhs = rhs.force_uint();
                Self::Uint(v.checked_add(rhs).unwrap_or(0))
            },
        }
    }
}

impl Sub for Number {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Self::Float(v) => {
                let rhs = rhs.force_float();
                Self::Float(v - rhs)
            },
            Self::Number(v) => {
                let rhs = rhs.force_double();
                Self::Number(v - rhs)
            },
            Self::Int(v) => {
                let rhs = rhs.force_int();
                Self::Int(v.checked_sub(rhs).unwrap_or(0))
            },
            Self::Uint(v) => {
                let rhs = rhs.force_uint();
                Self::Uint(v.checked_sub(rhs).unwrap_or(0))
            },
        }
    }
}

impl Mul for Number {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Self::Float(v) => {
                let rhs = rhs.force_float();
                Self::Float(v * rhs)
            },
            Self::Number(v) => {
                let rhs = rhs.force_double();
                Self::Number(v * rhs)
            },
            Self::Int(v) => {
                let rhs = rhs.force_int();
                Self::Int(v.checked_mul(rhs).unwrap_or(0))
            },
            Self::Uint(v) => {
                let rhs = rhs.force_uint();
                Self::Uint(v.checked_mul(rhs).unwrap_or(0))
            },
        }
    }
}

impl Div for Number {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Self::Float(v) => {
                let rhs = rhs.force_float();
                Self::Float(v / rhs)
            },
            Self::Number(v) => {
                let rhs = rhs.force_double();
                Self::Number(v / rhs)
            },
            Self::Int(v) => {
                let rhs = rhs.force_int();
                Self::Int(v.checked_div(rhs).unwrap_or(0))
            },
            Self::Uint(v) => {
                let rhs = rhs.force_uint();
                Self::Uint(v.checked_div(rhs).unwrap_or(0))
            },
        }
    }
}

impl Rem for Number {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        match self {
            Self::Float(v) => {
                let rhs = rhs.force_float();
                Self::Float(v % rhs)
            },
            Self::Number(v) => {
                let rhs = rhs.force_double();
                Self::Number(v % rhs)
            },
            Self::Int(v) => {
                let rhs = rhs.force_int();
                Self::Int(v.checked_rem(rhs).unwrap_or(0))
            },
            Self::Uint(v) => {
                let rhs = rhs.force_uint();
                Self::Uint(v.checked_rem(rhs).unwrap_or(0))
            },
        }
    }
}

impl std::ops::Neg for Number {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Self::Float(v) => Self::Float(-v),
            Self::Number(v) => Self::Number(-v),
            Self::Int(v) => Self::Int(-v),
            Self::Uint(v) => Self::Uint(v),
        }
    }
}

impl BitAnd for Number {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        match self {
            Self::Float(v) => {
                let rhs = rhs.force_float();
                Self::Float(f32::from_u32(unsafe { v.to_int_unchecked::<u32>() } & unsafe { rhs.to_int_unchecked::<u32>() }).unwrap_or(0.0))
            },
            Self::Number(v) => {
                let rhs = rhs.force_double();
                Self::Number(f64::from_u32(unsafe { v.to_int_unchecked::<u32>() } & unsafe { rhs.to_int_unchecked::<u32>() }).unwrap_or(0.0))
            },
            Self::Int(v) => {
                let rhs = rhs.force_int();
                Self::Int(v & rhs)
            },
            Self::Uint(v) => {
                let rhs = rhs.force_uint();
                Self::Uint(v & rhs)
            },
        }
    }
}

impl BitXor for Number {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        match self {
            Self::Float(v) => {
                let rhs = rhs.force_float();
                Self::Float(f32::from_u32(unsafe { v.to_int_unchecked::<u32>() } ^ unsafe { rhs.to_int_unchecked::<u32>() }).unwrap_or(0.0))
            },
            Self::Number(v) => {
                let rhs = rhs.force_double();
                Self::Number(f64::from_u32(unsafe { v.to_int_unchecked::<u32>() } ^ unsafe { rhs.to_int_unchecked::<u32>() }).unwrap_or(0.0))
            },
            Self::Int(v) => {
                let rhs = rhs.force_int();
                Self::Int(v ^ rhs)
            },
            Self::Uint(v) => {
                let rhs = rhs.force_uint();
                Self::Uint(v ^ rhs)
            },
        }
    }
}

impl BitOr for Number {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        match self {
            Self::Float(v) => {
                let rhs = rhs.force_float();
                Self::Float(f32::from_u32(unsafe { v.to_int_unchecked::<u32>() } | unsafe { rhs.to_int_unchecked::<u32>() }).unwrap_or(0.0))
            },
            Self::Number(v) => {
                let rhs = rhs.force_double();
                Self::Number(f64::from_u32(unsafe { v.to_int_unchecked::<u32>() } | unsafe { rhs.to_int_unchecked::<u32>() }).unwrap_or(0.0))
            },
            Self::Int(v) => {
                let rhs = rhs.force_int();
                Self::Int(v | rhs)
            },
            Self::Uint(v) => {
                let rhs = rhs.force_uint();
                Self::Uint(v | rhs)
            },
        }
    }
}

impl Shl for Number {
    type Output = Self;
    fn shl(self, rhs: Self) -> Self::Output {
        match self {
            Self::Float(v) => {
                let rhs = rhs.force_float();
                Self::Float(f32::from_u32(unsafe { v.to_int_unchecked::<u32>() }.checked_shl(unsafe { rhs.to_int_unchecked::<u32>() }).unwrap_or(0)).unwrap_or(0.0))
            },
            Self::Number(v) => {
                let rhs = rhs.force_double();
                Self::Number(f64::from_u32(unsafe { v.to_int_unchecked::<u32>() }.checked_shl(unsafe { rhs.to_int_unchecked::<u32>() }).unwrap_or(0)).unwrap_or(0.0))
            },
            Self::Int(v) => {
                let rhs = rhs.force_int();
                Self::Int(v.checked_shl(rhs.try_into().unwrap_or(0)).unwrap_or(0))
            },
            Self::Uint(v) => {
                let rhs = rhs.force_uint();
                Self::Uint(v.checked_shl(rhs).unwrap_or(0))
            },
        }
    }
}

impl Shr for Number {
    type Output = Self;
    fn shr(self, rhs: Self) -> Self::Output {
        match self {
            Self::Float(v) => {
                let rhs = rhs.force_float();
                Self::Float(f32::from_u32(unsafe { v.to_int_unchecked::<u32>() }.checked_shr(unsafe { rhs.to_int_unchecked::<u32>() }).unwrap_or(0)).unwrap_or(0.0))
            },
            Self::Number(v) => {
                let rhs = rhs.force_double();
                Self::Number(f64::from_u32(unsafe { v.to_int_unchecked::<u32>() }.checked_shr(unsafe { rhs.to_int_unchecked::<u32>() }).unwrap_or(0)).unwrap_or(0.0))
            },
            Self::Int(v) => {
                let rhs = rhs.force_int();
                Self::Int(v.checked_shr(rhs.try_into().unwrap_or(0)).unwrap_or(0))
            },
            Self::Uint(v) => {
                let rhs = rhs.force_uint();
                Self::Uint(v.checked_shr(rhs).unwrap_or(0))
            },
        }
    }
}

impl Number {
    pub fn zero(type_thing: &Entity, host: &Database) -> Self {
        if type_thing == &host.number_type() {
            Self::Number(0.0)
        } else if type_thing == &host.int_type() {
            Self::Int(0)
        } else if type_thing == &host.uint_type() {
            Self::Uint(0)
        /*
        } else if type_thing == &host.big_int_type() {
            Self::BigInt(BigInt::zero())
        */
        } else if type_thing == &host.float_type() {
            Self::Float(0.0)
        } else {
            panic!()
        }
    }

    pub fn nan(type_thing: &Entity, host: &Database) -> Self {
        if type_thing == &host.number_type() {
            Self::Number(f64::NAN)
        } else if type_thing == &host.float_type() {
            Self::Float(f32::NAN)
        } else {
            panic!("Type does not support NaN.");
        }
    }

    pub fn one(type_thing: &Entity, host: &Database) -> Self {
        if type_thing == &host.number_type() {
            Self::Number(1.0)
        } else if type_thing == &host.int_type() {
            Self::Int(1)
        } else if type_thing == &host.uint_type() {
            Self::Uint(1)
        /*
        } else if type_thing == &host.big_int_type() {
            Self::BigInt(BigInt::one())
        */
        } else if type_thing == &host.float_type() {
            Self::Float(1.0)
        } else {
            panic!()
        }
    }

    pub fn minimum_value(type_thing: &Entity, host: &Database) -> Self {
        if type_thing == &host.number_type() {
            Self::Number(f64::NEG_INFINITY)
        } else if type_thing == &host.int_type() {
            Self::Int(i32::MIN)
        } else if type_thing == &host.uint_type() {
            Self::Uint(0)
        /*
        } else if type_thing == &host.big_int_type() {
            panic!("BigInt has no minimum value")
        */
        } else if type_thing == &host.float_type() {
            Self::Float(f32::NEG_INFINITY)
        } else {
            panic!()
        }
    }

    pub fn maximum_value(type_thing: &Entity, host: &Database) -> Self {
        if type_thing == &host.number_type() {
            Self::Number(f64::INFINITY)
        } else if type_thing == &host.int_type() {
            Self::Int(i32::MAX)
        } else if type_thing == &host.uint_type() {
            Self::Uint(u32::MAX)
        /*
        } else if type_thing == &host.big_int_type() {
            panic!("BigInt has no maximum value")
        */
        } else if type_thing == &host.float_type() {
            Self::Float(f32::INFINITY)
        } else {
            panic!()
        }
    }

    pub fn is_zero(&self) -> bool {
        match self {
            Self::Float(v) => v == &0.0,
            Self::Number(v) => v == &0.0,
            // Self::BigInt(v) => v.is_zero(),
            Self::Int(v) => v == &0,
            Self::Uint(v) => v == &0,
        }
    }

    pub fn is_one(&self) -> bool {
        match self {
            Self::Float(v) => v == &1.0,
            Self::Number(v) => v == &1.0,
            // Self::BigInt(v) => v.is_one(),
            Self::Int(v) => v == &1,
            Self::Uint(v) => v == &1,
        }
    }

    pub fn multiply_per_two(&self) -> Self {
        match self {
            Self::Float(v) => Self::Float(v * 2.0),
            Self::Number(v) => Self::Number(v * 2.0),
            // Self::BigInt(v) => Self::BigInt(v * 2),
            Self::Int(v) => Self::Int(v * 2),
            Self::Uint(v) => Self::Uint(v * 2),
        }
    }

    pub fn increase_by_one(&self) -> Self {
        match self {
            Self::Float(v) => Self::Float(v + 1.0),
            Self::Number(v) => Self::Number(v + 1.0),
            // Self::BigInt(v) => Self::BigInt(v + 1),
            Self::Int(v) => Self::Int(v + 1),
            Self::Uint(v) => Self::Uint(v + 1),
        }
    }

    /// Performs bitwise OR if `value` is true or erases bits with the `erase_bits()` method otherwise.
    pub fn apply_bits(&self, bits: &Self, value: bool) -> Self {
        if value { self.clone() | bits.clone() } else { self.erase_bits(bits) }
    }

    /// Erases bits if all of such bits are included in the base value.
    pub fn erase_bits(&self, bits: &Self) -> Self {
        if self.includes_bits(bits) { self.clone() ^ bits.clone() } else { self.clone() }
    }

    pub fn bitwise_not(&self) -> Self {
        match self {
            Self::Float(v) => Self::Float(f32::from_u32(unsafe { !v.to_int_unchecked::<u32>() }).unwrap_or(0.0)),
            Self::Number(v) => Self::Number(f64::from_u32(unsafe { !v.to_int_unchecked::<u32>() }).unwrap_or(0.0)),
            Self::Int(v) => Self::Int(!v),
            Self::Uint(v) => Self::Uint(!v),
        }
    }
    
    pub fn shift_right_unsigned(&self, rhs: &Self) -> Self {
        match self {
            Self::Float(v) => {
                let rhs = rhs.force_float();
                Self::Float(f32::from_u32(unsafe { v.to_int_unchecked::<u32>() }.checked_shr(unsafe { rhs.to_int_unchecked::<u32>() }).unwrap_or(0)).unwrap_or(0.0))
            },
            Self::Number(v) => {
                let rhs = rhs.force_double();
                Self::Number(f64::from_u32(unsafe { v.to_int_unchecked::<u32>() }.checked_shr(unsafe { rhs.to_int_unchecked::<u32>() }).unwrap_or(0)).unwrap_or(0.0))
            },
            Self::Int(v) => {
                let rhs = rhs.force_int();
                let uint1: u32 = (*v).try_into().unwrap_or(0);
                let uint2: u32 = rhs.try_into().unwrap_or(0);
                let v = uint1.checked_shr(uint2).unwrap_or(0);
                let v: i32 = v.try_into().unwrap_or(0);
                Self::Int(v)
            },
            Self::Uint(v) => {
                let rhs = rhs.force_uint();
                Self::Uint(v.checked_shr(rhs).unwrap_or(0))
            },
        }
    }

    pub fn includes_bits(&self, rhs: &Self) -> bool {
        match self {
            Self::Float(v) => {
                let Self::Float(rhs) = rhs else { panic!(); };
                (unsafe { v.to_int_unchecked::<u32>() } & unsafe { rhs.to_int_unchecked::<u32>() } != 0)
            },
            Self::Number(v) => {
                let Self::Number(rhs) = rhs else { panic!(); };
                (unsafe { v.to_int_unchecked::<u32>() } & unsafe { rhs.to_int_unchecked::<u32>() } != 0)
            },
            /*
            Self::BigInt(v) => {
                let Self::BigInt(ref rhs) = rhs else { panic!(); };
                !(v.clone() & rhs.clone()).is_zero()
            },
            */
            Self::Int(v) => {
                let Self::Int(rhs) = rhs else { panic!(); };
                v & rhs != 0
            },
            Self::Uint(v) => {
                let Self::Uint(rhs) = rhs else { panic!(); };
                v & rhs != 0
            },
        }
    }

    pub fn is_power_of_two(&self) -> bool {
        // Based on https://stackoverflow.com/a/600306
        match self {
            Self::Float(v) => {
                let v = unsafe { v.to_int_unchecked::<u32>() };
                (v != 0) && ((v & (v - 1)) == 0)
            },
            Self::Number(v) => {
                let v = unsafe { v.to_int_unchecked::<u32>() };
                (v != 0) && ((v & (v - 1)) == 0)
            },
            /*
            Self::BigInt(v) => {
                !v.is_zero() && ((v & (v - BigInt::one())).is_zero())
            },
            */
            Self::Int(v) => (v != &0) && ((v & (v - 1)) == 0),
            Self::Uint(v) => (v != &0) && ((v & (v - 1)) == 0),
        }
    }

    pub fn convert_type(&self, target_type: &Entity, host: &Database) -> Result<Self, DeferError> {
        let number_type = host.number_type().defer()?;
        let float_type = host.float_type().defer()?;
        let int_type = host.int_type().defer()?;
        let uint_type = host.int_type().defer()?;

        Ok(if target_type == &number_type {
            Self::Number(self.force_double())
        } else if target_type == &float_type {
            Self::Float(self.force_float())
        } else if target_type == &int_type {
            Self::Int(self.force_int())
        } else if target_type == &uint_type {
            Self::Uint(self.force_uint())
        } else {
            panic!()
        })
    }

    pub fn is_nan(&self) -> bool {
        match self {
            Self::Number(f) => f.is_nan(),
            Self::Float(f) => f.is_nan(),
            _ => false,
        }
    }

    pub fn is_negative_infinity(&self) -> bool {
        match self {
            Self::Number(f) => f == &f64::NEG_INFINITY,
            Self::Float(f) => f == &f32::NEG_INFINITY,
            _ => false,
        }
    }

    pub fn is_positive_infinity(&self) -> bool {
        match self {
            Self::Number(f) => f == &f64::INFINITY,
            Self::Float(f) => f == &f32::INFINITY,
            _ => false,
        }
    }

    pub fn as_double(&self) -> Option<f64> {
        if let Number::Number(v) = self { Some(*v) } else { None }
    }

    pub fn as_float(&self) -> Option<f32> {
        if let Number::Float(v) = self { Some(*v) } else { None }
    }

    pub fn as_int(&self) -> Option<i32> {
        if let Number::Int(v) = self { Some(*v) } else { None }
    }

    pub fn as_uint(&self) -> Option<u32> {
        if let Number::Uint(v) = self { Some(*v) } else { None }
    }

    pub fn force_double(&self) -> f64 {
        match self {
            Self::Number(v) => *v,
            Self::Float(v) => *v as f64,
            Self::Int(v) => {
                let v: Result<i32, _> = (*v).try_into();
                v.map(|v| v as f64).unwrap_or(f64::NAN)
            },
            Self::Uint(v) => {
                let v: Result<u32, _> = (*v).try_into();
                v.map(|v| v as f64).unwrap_or(f64::NAN)
            },
        }
    }

    pub fn force_float(&self) -> f32 {
        match self {
            Self::Float(v) => *v,
            Self::Number(v) => *v as f32,
            Self::Int(v) => {
                let v: Result<i32, _> = (*v).try_into();
                v.map(|v| v as f32).unwrap_or(f32::NAN)
            },
            Self::Uint(v) => {
                let v: Result<u32, _> = (*v).try_into();
                v.map(|v| v as f32).unwrap_or(f32::NAN)
            },
        }
    }

    pub fn force_int(&self) -> i32 {
        match self {
            Self::Float(v) =>
                if v.is_infinite() {
                    if v.is_sign_negative() { i32::MIN } else { i32::MAX }
                } else if v.is_nan() { 0 } else { unsafe { v.to_int_unchecked() } },
            Self::Number(v) =>
                if v.is_infinite() {
                    if v.is_sign_negative() { i32::MIN } else { i32::MAX }
                } else if v.is_nan() { 0 } else { unsafe { v.to_int_unchecked() } },
            Self::Int(v) => (*v).try_into().unwrap_or(0),
            Self::Uint(v) => (*v).try_into().unwrap_or(0),
        }
    }

    pub fn force_uint(&self) -> u32 {
        match self {
            Self::Float(v) =>
                if v.is_infinite() {
                    if v.is_sign_negative() { u32::MIN } else { u32::MAX }
                } else if v.is_nan() { 0 } else { unsafe { v.to_int_unchecked() } },
            Self::Number(v) =>
                if v.is_infinite() {
                    if v.is_sign_negative() { u32::MIN } else { u32::MAX }
                } else if v.is_nan() { 0 } else { unsafe { v.to_int_unchecked() } },
            // Self::BigInt(v) => Self::Int(v.try_into().unwrap_or(0)),
            Self::Int(v) => (*v).try_into().unwrap_or(0),
            Self::Uint(v) => (*v).try_into().unwrap_or(0),
        }
    }
}