// --------------------------------------------------
// external
// --------------------------------------------------
use thisenum::Const;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ImapError {
    #[error("Value is below the IMAP minimum.")]
    BelowMinimum,
    #[error("Value is above the IMAP maximum.")]
    AboveMaximum,
    #[error("Value is reserved by MISB, doesn't correspond to anything,")]
    ReservedValue,
    #[error("Value is a user-defined value, and no user-defined decoder is provided to interpret it.")]
    UserDefinedValue,
    #[error("IMAP parsing error: {0}.")]
    ParseError(String),
}

/// Floating Point to Interger Mapping - A
/// 
/// This defines a min, max, and precision for a floating
/// point value to be mapped to, as an integer.
/// 
/// Note that:
/// 
/// * a < b
/// * g < (b - a)
pub struct ImapA<T>
where
    T: std::fmt::Display,
    T: num_traits::Float + num_traits::FromPrimitive,
{
    /// The [`ImapB`], once the length, in bytes, is known
    imapb: ImapB<T>,
}
/// [`ImapA`] implementation
impl<T> ImapA<T>
where
    T: std::fmt::Display,
    T: num_traits::Float + num_traits::FromPrimitive,
{
    /// Creates a new [`ImapA`]
    pub fn new(min: T, max: T, precision: T) -> Option<Self> {
        Self::new_with_user_defined_xcoders(min, max, precision, None, None)
    }

    /// Creates a new [`ImapA`] with a user-defined value decoder / encoder
    pub fn new_with_user_defined_xcoders(
        min: T,
        max: T,
        precision: T,
        user_enc: Option<fn(T) -> Vec<u8>>,
        user_dec: Option<fn(&[u8]) -> T>,
    ) -> Option<Self> {
        match min < max && precision < (max - min) {
            true => {
                match Self::len(&min, &max, &precision) {
                    Some(len) => {
                        let imapb = ImapB::new_with_user_defined_xcoders(min, max, len, user_enc, user_dec)?;
                        // Some(ImapA { min, max, precision, imapb, encoder: user_enc, decoder: user_dec })
                        Some(ImapA { imapb })
                    }
                    None => None
                }
            },
            false => None
        }
    }

    /// Get's the length of the ImapA. To use [`ImapB`] once retrieved.
    fn len(min: &T, max: &T, precision: &T) -> Option<usize> {
        let lbits = (*max - *min).log2().ceil() - precision.log2().floor() + T::one();
        let len = (lbits / Self::eight()).ceil();
        len.to_usize()
    }
    
    /// Returns 8. Didn't want to use T::from(8) here due to `unwrap`
    fn eight() -> T {
        T::one() + T::one() + T::one() + T::one() +
        T::one() + T::one() + T::one() + T::one()
    }

    /// Maps a floating point value to an integer value
    /// 
    /// Returns [`None`] when the value fails to be mapped. Otherwise:
    /// 
    /// * If NaN, return IMAP bytes for [`SpecialValue::PosQuietNan`] or [`SpecialValue::NegQuietNan`]
    /// * If infinite, return IMAP bytes for [`SpecialValue::PosInfinity`] or [`SpecialValue::NegInfinity`]
    /// * If below minimum, return IMAP bytes for [`SpecialValue::ImapBelowMinimum`]
    /// * If above maximum, return IMAP bytes for [`SpecialValue::ImapAboveMaximum`]
    /// * Otherwise, returns the successfully mapped value as IMAP bytes
    pub fn to_imap(&self, x: T) -> Result<Vec<u8>, ImapError> {
        self.imapb.to_imap(x)
    }

    /// Maps an IMAP integer value to a floating point value
    /// 
    /// Returns [`None`] when the value fails to be mapped
    /// 
    /// * If special, return the special value as a floating point
    /// * Otherwise, returns the successfully decoded value
    pub fn from_imap(&self, x: &Vec<u8>) -> Result<T, ImapError> {
        self.imapb.from_imap(x)
    }
}

/// Floating Point to Interger Mapping - B
/// 
/// This defines the min, max, and length for a floating
/// point value to be mapped to, as an integer.
/// 
/// Note that:
/// 
/// * a < b
pub struct ImapB<T>
where 
    T: std::fmt::Display,
    T: num_traits::Float + num_traits::FromPrimitive,
{
    /// The minimum floating point value
    /// 
    /// Also known as a
    min: T,
    /// The maximum floating point value
    /// 
    /// Also known as b
    max: T,
    /// The length, in bytes
    len: usize,
    /// Scaling factor - forward
    s_f: T,
    /// Scaling factor - reverse
    s_r: T,
    /// The zero-point offset
    z_offset: T,
    /// User defined encoder
    encoder: Option<fn(T) -> Vec<u8>>,
    /// User defined decoder
    decoder: Option<fn(&[u8]) -> T>,
}
/// [`ImapB`] implementation
impl<T> ImapB<T>
where
    T: std::fmt::Display,
    T: num_traits::Float + num_traits::FromPrimitive,
{
    /// Creates a new [`ImapB`]
    pub fn new(min: T, max: T, len: usize) -> Option<Self> {
        Self::new_with_user_defined_xcoders(min, max, len, None, None)
    }

    /// Creates a new [`ImapB`] with a user-defined value decoder / encoder
    pub fn new_with_user_defined_xcoders(
        min: T,
        max: T,
        len: usize,
        user_enc: Option<fn(T) -> Vec<u8>>,
        user_dec: Option<fn(&[u8]) -> T>,
    ) -> Option<Self> {
        match min < max {
            true => {
                let b_pow = Self::calc_b_pow(&min, &max)?;
                let d_pow = Self::calc_d_pow(&len);
                let s_f = Self::calc_s_f(&b_pow, &d_pow)?;
                let s_r = Self::calc_s_r(&b_pow, &d_pow)?;
                let z_offset = Self::calc_z_offset(&min, &max, &s_f);
                Some(ImapB { min, max, len, s_f, s_r, z_offset, encoder: user_enc, decoder: user_dec })
            },
            false => None
        }
    }

    /// Calculates `b_pow`, used in calculating [`ImapB::s_f`] and [`ImapB::s_r`]
    fn calc_b_pow(min: &T, max: &T) -> Option<usize> {
        (*max - *min).log2().ceil().to_usize()
    }

    /// Calculates `d_pow`, used in calculating [`ImapB::s_f`] and [`ImapB::s_r`]
    fn calc_d_pow(len: &usize) -> usize {
        (*len * 8) - 1
    }

    /// Calculates [`ImapB::s_f`]
    fn calc_s_f(b_pow: &usize, d_pow: &usize) -> Option<T> {
        T::from_usize(2_usize.pow((*d_pow - *b_pow) as u32))
    }

    /// Calculates [`ImapB::s_r`]
    fn calc_s_r(b_pow: &usize, d_pow: &usize) -> Option<T> {
        // is 2^(b_pow - d_pow), but b_pow will always be less than d_pow
        T::from_f64(1.0 / (1 << (*d_pow - *b_pow)) as f64)
    }

    /// Calculates [`ImapB::z_offset`]
    fn calc_z_offset(min: &T, max: &T, s_f: &T) -> T {
        match *min < T::zero() && *max > T::zero() {
            false => T::zero(),
            true => {
                let sf_x_min = *s_f * *min;
                sf_x_min - sf_x_min.floor()
            },
        }
    }

    /// Maps a floating point value to an integer value
    /// 
    /// Returns [`None`] when the value fails to be mapped. Otherwise:
    /// 
    /// * If NaN, return IMAP bytes for [`SpecialValue::PosQuietNan`] or [`SpecialValue::NegQuietNan`]
    /// * If infinite, return IMAP bytes for [`SpecialValue::PosInfinity`] or [`SpecialValue::NegInfinity`]
    /// * If below minimum, return IMAP bytes for [`SpecialValue::ImapBelowMinimum`]
    /// * If above maximum, return IMAP bytes for [`SpecialValue::ImapAboveMaximum`]
    /// * Otherwise, returns the successfully mapped value as IMAP bytes
    pub fn to_imap(&self, x: T) -> Result<Vec<u8>, ImapError> {
        // --------------------------------------------------
        // Rust's floating-point operations and the IEEE-754 standard
        // typically use quiet NaNs for representing invalid results
        // --------------------------------------------------
        if x.is_nan() {
            return Ok(match x.is_sign_positive() {
                true => Value::Special(SpecialValue::PosQuietNan).to_imap(self.len),
                false => Value::Special(SpecialValue::NegQuietNan).to_imap(self.len),
            })
        }
        if x.is_infinite() {
            return Ok(match x.is_sign_positive() {
                true => Value::Special(SpecialValue::PosInfinity).to_imap(self.len),
                false => Value::Special(SpecialValue::NegInfinity).to_imap(self.len),
            })
        }
        if x < self.min { return Ok(Value::Special(SpecialValue::ImapBelowMinimum).to_imap(self.len)) }
        if x > self.max { return Ok(Value::Special(SpecialValue::ImapAboveMaximum).to_imap(self.len)) }
        // --------------------------------------------------
        // truncate, convert to usize, then convert to len bytes
        // --------------------------------------------------
        let y = match (self.s_f * (x - self.min) + self.z_offset).trunc().to_usize() {
            Some(x) => x,
            None => return Err(ImapError::ParseError(format!("Cannot convert input {} to usize", x))),
        };
        Ok((0..self.len).map(|i| ((y >> (8 * (self.len - 1 - i))) & 0xFF) as u8).collect())
    }

    /// Maps a floating point value to an integer value, using
    /// the custom user-defined encoder first, and then falling
    /// back to the default implementation.
    pub fn to_imap_with_encoder(&self, x: T) -> Result<Vec<u8>, ImapError> {
        match self.encoder {
            Some(enc) => Ok(enc(x)),
            None => self.to_imap(x),
        }
    }

    /// Maps an IMAP integer value to a floating point value
    /// 
    /// Returns [`None`] when the value fails to be mapped
    /// 
    /// * If special, return the special value
    /// * Otherwise, returns the successfully decoded value
    pub fn from_imap(&self, y: &Vec<u8>) -> Result<T, ImapError> {
        if y.len() != self.len { return Err(ImapError::ParseError(format!("Cannot convert {} bytes to {} bytes", y.len(), self.len))) }
        if (y[0] >> 7 & 1) & (y[0] >> 6 & 1) != 0 {
            // --------------------------------------------------
            // special value
            // --------------------------------------------------
            match SpecialValue::from_imap(&y) {
                Some(sval) => match sval {
                    SpecialValue::PosInfinity => return Ok(T::infinity()),
                    SpecialValue::NegInfinity => return Ok(-T::infinity()),
                    SpecialValue::PosQuietNan => return Ok(T::nan()),
                    SpecialValue::NegQuietNan => return Ok(-T::nan()),
                    SpecialValue::PosSignalNan => return Ok(T::nan()),
                    SpecialValue::NegSignalNan => return Ok(-T::nan()),
                    SpecialValue::ImapBelowMinimum => return Ok(self.min),
                    SpecialValue::ImapAboveMaximum => return Ok(self.max),
                    SpecialValue::ReservedSpecial => return Err(ImapError::ReservedValue),
                    SpecialValue::ReservedMisbDefined => return Err(ImapError::ReservedValue),
                    SpecialValue::UserDefined => match self.decoder {
                        Some(dec) => return Ok(dec(&y)),
                        None => return Err(ImapError::UserDefinedValue),
                    },
                },
                None => return Err(ImapError::ParseError(format!("Cannot parse special value: {y:?}"))),
            }
        } else {
            // --------------------------------------------------
            // normal value
            // --------------------------------------------------
            let y = match T::from_u64(u64::from_be_bytes({
                let mut b = [0u8; 8];
                b[8 - y.len().min(8)..].copy_from_slice(&y[..y.len().min(8)]);
                b
            })) {
                Some(x) => x,
                None => return Err(ImapError::ParseError(format!("Cannot parse value: {y:?}"))),
            };
            return Ok(self.s_r * (y - self.z_offset) + self.min)
        }
    }
}

/// IMAP value
pub enum Value {
    Normal(Vec<u8>),
    Special(SpecialValue),
}
/// [`Value`] implementation
impl Value {
    /// To an IMAP value
    pub fn to_imap(self, len: usize) -> Vec<u8> {
        match self {
            Value::Normal(x) => return x,
            Value::Special(x) => x.to_imap(&len),
        }
    }
}

#[derive(Const)]
#[armtype(u8)]
/// MISB Standard 1201 Special Values
/// 
/// * b_n         = 1
/// * b_n-1       = Special bit
/// * b_n-2       = Sign bit
/// * b_n-3       = NaN bit
/// * b_n-4       = Any
/// * b_n-5 - b_0 = Contextual (either user defined, zero-filled, etc.)
/// 
/// See [https://nsgreg.nga.mil/misb.jsp](https://nsgreg.nga.mil/misb.jsp)
pub enum SpecialValue {
    // --------------------------------------------------
    // special values
    // --------------------------------------------------
    #[value = 0b1100_1000]
    PosInfinity,
    #[value = 0b1110_1000]
    NegInfinity,
    #[value = 0b1101_0000]
    PosQuietNan,
    #[value = 0b1111_0000]
    NegQuietNan,
    #[value = 0b1101_1000]
    PosSignalNan,
    #[value = 0b1111_1000]
    NegSignalNan,
    #[value = 0b1100_0000]
    UserDefined,
    #[value = 0b1010_0000]
    ReservedSpecial,
    // --------------------------------------------------
    // MISB defined values
    // --------------------------------------------------
    #[value = 0b1110_0000]
    ImapBelowMinimum,
    #[value = 0b1110_0001]
    ImapAboveMaximum,
    #[value = 0b1110_0010]
    ReservedMisbDefined,
}
/// [`SpecialValue`] implementation
impl SpecialValue {
    /// To a IMAP value
    pub fn to_imap(&self, len: &usize) -> Vec<u8> {
        let mut bytes = vec![0u8; *len];
        bytes[0] = *self.value();
        return bytes;
    }

    /// From a IMAP value
    pub fn from_imap(y: &Vec<u8>) -> Option<SpecialValue> {
        // --------------------------------------------------
        // do not include the 3 trailing bits of the most-significant byte
        // --------------------------------------------------
        match Self::try_from(y[0] & 0b1111_1000) {
            Ok(sval) => match sval {
                SpecialValue::PosInfinity |
                SpecialValue::NegInfinity |
                SpecialValue::ImapBelowMinimum |
                SpecialValue::ImapAboveMaximum => {
                    // --------------------------------------------------
                    // for these values, the trailing bits must be zero
                    // --------------------------------------------------
                    if y[1..] != vec![0u8; y.len() - 1] { return None }
                    return Some(sval)
                },
                SpecialValue::UserDefined => return Some(SpecialValue::UserDefined),
                _ => (),
            }
            Err(_) => (),
        };
        // --------------------------------------------------
        // The default NaN Identifier is a value with all zeros
        // therefore, do not check the trailing bits
        // --------------------------------------------------
        if (y[0] & SpecialValue::PosQuietNan.value()) == *SpecialValue::PosQuietNan.value() { return Some(SpecialValue::PosQuietNan) }
        if (y[0] & SpecialValue::NegQuietNan.value()) == *SpecialValue::NegQuietNan.value() { return Some(SpecialValue::NegQuietNan) }
        if (y[0] & SpecialValue::PosSignalNan.value()) == *SpecialValue::PosSignalNan.value() { return Some(SpecialValue::PosSignalNan) }
        if (y[0] & SpecialValue::NegSignalNan.value()) == *SpecialValue::NegSignalNan.value() { return Some(SpecialValue::NegSignalNan) }
        // --------------------------------------------------
        // if starts with 0b10XX_XXXX, where any X is set, 
        // then the value is a reserved special value
        // --------------------------------------------------
        if (y[0] & 0b1100_0000) == 0b1000_0000
        && (y[0] & 0b0011_1111 != 0 || y[1..].iter().any(|&byte| byte != 0))
        {
            return Some(SpecialValue::ReservedSpecial)
        }
        // --------------------------------------------------
        // if starts with 0b1110_0XXA, where any X is set,
        // and A is 0 or 1, and the remaining bits are zero
        // then the value is a reserved MISB defined value
        // --------------------------------------------------
        if (y[0] & 0b1111_1000) == 0b1110_0000
        && y[0] & 0b0000_0110 != 0
        && y[1..] == vec![0u8; y.len() - 1]
        {
            return Some(SpecialValue::ReservedMisbDefined)
        }
        // --------------------------------------------------
        // if all checks fail, return None
        // --------------------------------------------------
        None
    }
}

#[cfg(test)]
mod test {
    use rand::Rng;
    use core::f64;
    use std::time::{
        Instant,
        Duration,
    };

    use super::ImapA;
    use super::ImapB;
    use super::ImapError;

    #[test]
    fn imap_a_from_spec_0() {
        let example = ImapA::new(-900.0, 19_000.0, 0.5).unwrap();
        
        assert_eq!(example.to_imap(-900.0).unwrap(), vec![0x00, 0x00, 0x00]);
        assert_eq!(example.from_imap(&vec![0x00_u8, 0x00, 0x00]).unwrap(), -900.0);
        
        assert_eq!(example.to_imap(10.0).unwrap(), vec![0x03, 0x8E, 0x00]);
        assert_eq!(example.from_imap(&vec![0x03_u8, 0x8E, 0x00]).unwrap(), 10.0);
        
        assert_eq!(example.to_imap(0.0).unwrap(), vec![0x03, 0x84, 0x00]);
        assert_eq!(example.from_imap(&vec![0x03_u8, 0x84, 0x00]).unwrap(), 0.0);
        
        assert_eq!(example.to_imap(f64::NEG_INFINITY).unwrap(), vec![0xE8, 0x00, 0x00]);
        assert_eq!(example.from_imap(&vec![0xE8_u8, 0x00, 0x00]).unwrap(), f64::NEG_INFINITY);
    }

    #[test]
    fn imap_b_from_spec_0() {
        let example = ImapB::new(0.1, 0.9, 2).unwrap();
        
        assert_eq!(example.to_imap(0.1).unwrap(), vec![0x00, 0x00]);
        assert_eq!(example.from_imap(&vec![0x00_u8, 0x00]).unwrap(), 0.1);
        
        assert_eq!(example.to_imap(0.5).unwrap(), vec![0x33, 0x33]);
        assert_eq!(example.from_imap(&vec![0x33_u8, 0x33]).unwrap(), 0.499993896484375);
        
        assert_eq!(example.to_imap(0.9).unwrap(), vec![0x66, 0x66]);
        assert_eq!(example.from_imap(&vec![0x66_u8, 0x66]).unwrap(), 0.89998779296875);

        assert_eq!(example.to_imap(f64::NEG_INFINITY).unwrap(), vec![0xE8, 0x00]);
        assert_eq!(example.from_imap(&vec![0xE8_u8, 0x00]).unwrap(), f64::NEG_INFINITY);
    }

    #[test]
    fn imap_a_from_spec_1() {
        let example = ImapA::new(0.0, 100.0, 1e-5).unwrap();

        assert_eq!(example.to_imap(0.0).unwrap(), vec![0x00, 0x00, 0x00, 0x00]);
        assert_eq!(example.from_imap(&vec![0x00, 0x00, 0x00, 0x00]).unwrap(), 0.0);

        assert_eq!(example.to_imap(10.1).unwrap(), vec![0x0A, 0x19, 0x99, 0x99]);
        assert!(example.from_imap(&vec![0x0A, 0x19, 0x99, 0x99]).unwrap() - 10.09999996 < 1e-8);
        
        assert_eq!(example.to_imap(20.2).unwrap(), vec![0x14, 0x33, 0x33, 0x33]);
        assert!(example.from_imap(&vec![0x14, 0x33, 0x33, 0x33]).unwrap() - 20.19999999 < 1e-8);
        
        assert_eq!(example.to_imap(30.3).unwrap(), vec![0x1E, 0x4C, 0xCC, 0xCC]);
        assert!(example.from_imap(&vec![0x1E, 0x4C, 0xCC, 0xCC]).unwrap() - 30.29999995 < 1e-8);
        
        assert_eq!(example.to_imap(40.4).unwrap(), vec![0x28, 0x66, 0x66, 0x66]);
        assert!(example.from_imap(&vec![0x28, 0x66, 0x66, 0x66]).unwrap() - 40.39999998 < 1e-8);
        
        assert_eq!(example.to_imap(50.5).unwrap(), vec![0x32, 0x80, 0x00, 0x00]);
        assert_eq!(example.from_imap(&vec![0x32, 0x80, 0x00, 0x00]).unwrap(), 50.5);
        
        assert_eq!(example.to_imap(60.6).unwrap(), vec![0x3C, 0x99, 0x99, 0x99]);
        assert!(example.from_imap(&vec![0x3C, 0x99, 0x99, 0x99]).unwrap() - 60.59999996 < 1e-8);
        
        assert_eq!(example.to_imap(70.7).unwrap(), vec![0x46, 0xB3, 0x33, 0x33]);
        assert!(example.from_imap(&vec![0x46, 0xB3, 0x33, 0x33]).unwrap() - 70.69999999 < 1e-8);
        
        assert_eq!(example.to_imap(80.8).unwrap(), vec![0x50, 0xCC, 0xCC, 0xCC]);
        assert!(example.from_imap(&vec![0x50, 0xCC, 0xCC, 0xCC]).unwrap() -80.79999995 < 1e-8);
        
        assert_eq!(example.to_imap(90.9).unwrap(), vec![0x5A, 0xE6, 0x66, 0x66]);
        assert!(example.from_imap(&vec![0x5A, 0xE6, 0x66, 0x66]).unwrap() - 90.89999998 < 1e-8);
        
        assert_eq!(example.to_imap(100.0).unwrap(), vec![0x64, 0x00, 0x00, 0x00]);
        assert_eq!(example.from_imap(&vec![0x64, 0x00, 0x00, 0x00]).unwrap(), 100.0);
        
        assert_eq!(example.to_imap(f64::NAN).unwrap(), vec![0xD0, 0x00, 0x00, 0x00]);
        assert!(example.from_imap(&vec![0xD0, 0x00, 0x00, 0x00]).unwrap().is_nan());
        
        assert_eq!(example.to_imap(f64::INFINITY).unwrap(), vec![0xC8, 0x00, 0x00, 0x00]);
        assert_eq!(example.from_imap(&vec![0xC8, 0x00, 0x00, 0x00]).unwrap(), f64::INFINITY);
        
        assert_eq!(example.to_imap(f64::NEG_INFINITY).unwrap(), vec![0xE8, 0x00, 0x00, 0x00]);
        assert_eq!(example.from_imap(&vec![0xE8, 0x00, 0x00, 0x00]).unwrap(), f64::NEG_INFINITY);
        
        assert_eq!(example.to_imap(-1.0).unwrap(), vec![0xE0, 0x00, 0x00, 0x00]);
        assert_eq!(example.from_imap(&vec![0xE0, 0x00, 0x00, 0x00]).unwrap_err(), ImapError::BelowMinimum);
        
        assert_eq!(example.to_imap(101.0).unwrap(), vec![0xE1, 0x00, 0x00, 0x00]);
        assert_eq!(example.from_imap(&vec![0xE1, 0x00, 0x00, 0x00]).unwrap_err(), ImapError::AboveMaximum);
    }

    #[test]
    fn main() {
        let mut rng = rand::thread_rng();

        const N_TRIALS: usize = 7_000;

        // Generate random test values and exponents
        let values_f32: Vec<f32> = (0..N_TRIALS).map(|_| rng.gen_range(0.1..100.0)).collect();
        let exponents_f32: Vec<usize> = (0..N_TRIALS).map(|_| rng.gen_range(0..20)).collect();
        let scale_f32: Vec<f32> = exponents_f32.iter().map(|x| 2.0f32.powi(*x as i32)).collect();

        let values_f64: Vec<f64> = (0..N_TRIALS).map(|_| rng.gen_range(0.1..100.0)).collect();
        let exponents_f64: Vec<usize> = (0..N_TRIALS).map(|_| rng.gen_range(0..20)).collect();
        let scale_f64: Vec<f64> = exponents_f64.iter().map(|x| 2.0f64.powi(*x as i32)).collect();

        // Time the standard multiplication approach for f32
        let mut res1_f32 = Vec::new();
        for &value in &values_f32 {
            for &exponent in &exponents_f32 {
                let start = Instant::now();
                let val = value * (2.0f32).powi(exponent as i32);
                let duration = start.elapsed();
                res1_f32.push((val, duration));
            }
        }
        let times: Vec<Duration> = res1_f32.iter().map(|x| x.1).collect();
        let sum: f64 = times.iter().map(Duration::as_secs_f64).sum();
        let mean = sum / times.len() as f64;
        println!("f32 exp multiplication test time: {:?}", mean);
        
        // Time the standard multiplication approach for f32, pre-multiplied
        let mut res2_f32 = Vec::new();
        for &value in &values_f32 {
            for &scale in &scale_f32 {
                let start = Instant::now();
                let val = value * scale;
                let duration = start.elapsed();
                res2_f32.push((val, duration));
            }
        }
        let times: Vec<Duration> = res2_f32.iter().map(|x| x.1).collect();
        let sum: f64 = times.iter().map(Duration::as_secs_f64).sum();
        let mean = sum / times.len() as f64;
        println!("f32 standard / pre-multiplied multiplication test time: {:?}", mean);

        // Time the custom power-of-two multiplication approach for f32
        let mut res3_f32 = Vec::new();
        for &value in &values_f32 {
            for &exponent in &exponents_f32 {
                let start = Instant::now();
                let val = multiply_by_power_of_two_f32(value, exponent);
                let duration = start.elapsed();
                res3_f32.push((val, duration));
            }
        }
        let times: Vec<Duration> = res3_f32.iter().map(|x| x.1).collect();
        let sum: f64 = times.iter().map(Duration::as_secs_f64).sum();
        let mean = sum / times.len() as f64;
        println!("f32 custom power-of-two test time: {:?}", mean);

        // assert all values are within f32::EPSILON
        assert!(res1_f32
            .iter()
            .zip(res2_f32.iter())
            .zip(res3_f32.iter())
            .all(|((a, b), c)| a.0 - b.0 < f32::EPSILON && a.0 - c.0 < f32::EPSILON && b.0 - c.0 < f32::EPSILON));

        // Time the standard multiplication approach for f64
        let mut res1_f64 = Vec::new();
        for &value in &values_f64 {
            for &exponent in &exponents_f64 {
                let start = Instant::now();
                let val = value * (2.0f64).powi(exponent as i32);
                let duration = start.elapsed();
                res1_f64.push((val, duration));
            }
        }
        let times: Vec<Duration> = res1_f64.iter().map(|x| x.1).collect();
        let sum: f64 = times.iter().map(Duration::as_secs_f64).sum();
        let mean = sum / times.len() as f64;
        println!("f64 standard multiplication test time: {:?}", mean);

        // Time the standard multiplication approach for f64, pre-multiplied
        let mut res2_f64 = Vec::new();
        for &value in &values_f64 {
            for &scale in &scale_f64 {
                let start = Instant::now();
                let val = value * scale;
                let duration = start.elapsed();
                res2_f64.push((val, duration));
            }
        }
        let times: Vec<Duration> = res2_f64.iter().map(|x| x.1).collect();
        let sum: f64 = times.iter().map(Duration::as_secs_f64).sum();
        let mean = sum / times.len() as f64;
        println!("f64 standard / pre-multiplied multiplication test time: {:?}", mean);

        // Time the custom power-of-two multiplication approach for f64
        let mut res3_f64 = Vec::new();
        for &value in &values_f64 {
            for &exponent in &exponents_f64 {
                let start = Instant::now();
                let val = multiply_by_power_of_two_f64(value, exponent);
                let duration = start.elapsed();
                res3_f64.push((val, duration));
            }
        }
        let times: Vec<Duration> = res3_f64.iter().map(|x| x.1).collect();
        let sum: f64 = times.iter().map(Duration::as_secs_f64).sum();
        let mean = sum / times.len() as f64;
        println!("f64 custom power-of-two test time: {:?}", mean);

        // assert all values are within f64::EPSILON
        assert!(res1_f64
            .iter()
            .zip(res2_f64.iter())
            .zip(res3_f64.iter())
            .all(|((a, b), c)| a.0 - b.0 < f64::EPSILON && a.0 - c.0 < f64::EPSILON && b.0 - c.0 < f64::EPSILON));

    }
    
    // fn multiply_by_power_of_two_f32(value: f32, exponent: usize) -> f32 {
    //     let shift = exponent as u32;
    //     let bits = value.to_bits();
    //     let exp_bits = ((shift << 23) & 0x7F800000) >> 23;
    //     let new_bits = bits + exp_bits;
    //     f32::from_bits(new_bits)
    // }
    
    // fn multiply_by_power_of_two_f64(value: f64, exponent: usize) -> f64 {
    //     let shift = exponent as u64;
    //     let bits = value.to_bits();
    //     let exp_bits = ((shift << 52) & 0x7FF0000000000000) >> 52;
    //     let new_bits = bits + exp_bits;
    //     f64::from_bits(new_bits)
    // }
    
    fn multiply_by_power_of_two_f32(value: f32, exponent: usize) -> f32 {
        let mut bits = value.to_bits();
        let shift = exponent as u32;
        bits = bits.wrapping_add(shift << 23); // 23 is the number of bits in the exponent part of f32
        f32::from_bits(bits)
    }
    
    fn multiply_by_power_of_two_f64(value: f64, exponent: usize) -> f64 {
        let mut bits = value.to_bits();
        let shift = exponent as u64;
        bits = bits.wrapping_add(shift << 52); // 52 is the number of bits in the exponent part of f64
        f64::from_bits(bits)
    }
}

const F32_BIT_MASK: usize = 23;
const F64_BIT_MASK: usize = 52;