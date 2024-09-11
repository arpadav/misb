// --------------------------------------------------
// external
// --------------------------------------------------
use thisenum::Const;
use thiserror::Error;

// --------------------------------------------------
// external
// --------------------------------------------------
pub trait ImapFloat: std::fmt::Debug + std::fmt::Display + num_traits::Float + num_traits::FromPrimitive {}
impl<T> ImapFloat for T where T: std::fmt::Debug + std::fmt::Display + num_traits::Float + num_traits::FromPrimitive {}

#[derive(Error, Debug, PartialEq)]
// #[armtype(&str)]
/// Error type when utilizing floating-integer mapping for MISB
pub enum ImapError<T: ImapFloat> {
    // #[value = "Value is below the IMAP minimum."]
    #[error("Value is below the IMAP minimum.")]
    BelowMinimum,
    // #[value = "Value is above the IMAP maximum."]
    #[error("Value is above the IMAP maximum.")]
    AboveMaximum,
    // #[value = "Value is reserved by MISB, doesn't correspond to anything,"]
    #[error("Value is reserved by MISB, doesn't correspond to anything,")]
    ReservedValue,
    // #[value = "Value is a user-defined value, and no user-defined decoder is provided to interpret it."]
    #[error("Value is a user-defined value, and no user-defined decoder is provided to interpret it.")]
    UserDefinedValue,
    // #[value = "IMAP parsing error"]
    #[error("IMAP parsing error: {0}.")]
    ParseError(String),
    // #[value = "Invalid input arguments: note that min < max AND precision < (max - min)."]
    #[error("Invalid input arguments: note that min < max AND precision < (max - min). Got: min = {0}, max = {1}, len = {2:?}, precision = {3:?}.")]
    InvalidInputArgs(T, T, Option<usize>, Option<T>),
    // #[value = "Unable to convert initialize IMAP"]
    #[error("Unable to convert initialize IMAP: {0}")]
    InitError(String),
}
/// [`winnow::error::StrContext`] implementation of [`From`] for [`ImapError`]
impl<T: ImapFloat> From<ImapError<T>> for winnow::error::StrContext {
    fn from(input: ImapError<T>) -> Self {
        winnow::error::StrContext::Label(match input {
            ImapError::BelowMinimum => "Value is below the IMAP minimum.",
            ImapError::AboveMaximum => "Value is above the IMAP maximum.",
            ImapError::ReservedValue => "Value is reserved by MISB, doesn't correspond to anything,",
            ImapError::UserDefinedValue => "Value is a user-defined value, and no user-defined decoder is provided to interpret it.",
            ImapError::ParseError(_) => "IMAP parsing error",
            ImapError::InvalidInputArgs(_, _, _, _) => "Invalid input arguments: note that min < max AND precision < (max - min).",
            ImapError::InitError(_) => "Unable to convert initialize IMAP",
        })
    }
}

/// Floating Point to Interger Mapping - Starting Point A
/// 
/// This defines a min, max, and precision for a floating
/// point value to be mapped to, as an integer.
/// 
/// Note that:
/// 
/// * a < b
/// * g < (b - a)
/// 
/// Where a = min, b = max, g = precision
pub struct ImapA<T: ImapFloat> {
    /// The [`ImapB`], once the length, in bytes, is known
    imapb: ImapB<T>,
}
/// [`ImapA`] implementation
impl<T: ImapFloat> ImapA<T> {
    /// Creates a new [`ImapA`]
    pub fn new(min: T, max: T, precision: T) -> Result<Self, ImapError<T>> {
        Self::new_with_user_defined_xcoders(min, max, precision, None, None)
    }

    /// Creates a new [`ImapA`] with a user-defined value decoder / encoder
    pub fn new_with_user_defined_xcoders(
        min: T,
        max: T,
        precision: T,
        user_enc: Option<fn(T) -> Vec<u8>>,
        user_dec: Option<fn(&[u8]) -> T>,
    ) -> Result<Self, ImapError<T>> {
        match min < max && precision < (max - min) {
            true => Self::len(&min, &max, &precision).and_then(|len|
                Ok(ImapA { imapb: ImapB::new_with_user_defined_xcoders(min, max, len, user_enc, user_dec)? })
            ),
            false => Err(ImapError::InvalidInputArgs(min, max, None, Some(precision)))
        }
    }

    /// Get's the length of the ImapA. To use [`ImapB`] once retrieved.
    /// 
    /// The `unwrap` on `T::from_f32(8.0)` should never panic due to trait bounds
    fn len(min: &T, max: &T, precision: &T) -> Result<usize, ImapError<T>> {
        let lbits = (*max - *min).log2().ceil() - precision.log2().floor() + T::one();
        let len = (lbits / T::from_f32(8.0).unwrap()).ceil();
        len.to_usize().ok_or(ImapError::InitError(format!("Cannot convert len {len} to usize")))
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
    pub fn to_imap(&self, x: T) -> Result<Vec<u8>, ImapError<T>> {
        self.imapb.to_imap(x)
    }

    /// Maps an IMAP integer value to a floating point value
    /// 
    /// Returns [`None`] when the value fails to be mapped
    /// 
    /// * If special, return the special value as a floating point
    /// * Otherwise, returns the successfully decoded value
    pub fn from_imap(&self, x: &[u8]) -> Result<T, ImapError<T>> {
        self.imapb.from_imap(x)
    }
}

#[derive(Debug)]
/// Floating Point to Interger Mapping - Starting Point B
/// 
/// This defines the min, max, and length for a floating
/// point value to be mapped to, as an integer.
/// 
/// Note that:
/// 
/// * a < b
/// 
/// Where a = min, b = max
pub struct ImapB<T: ImapFloat> {
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
impl<T: ImapFloat> ImapB<T> {
    /// Creates a new [`ImapB`]
    pub fn new(min: T, max: T, len: usize) -> Result<Self, ImapError<T>> {
        Self::new_with_user_defined_xcoders(min, max, len, None, None)
    }

    /// Creates a new [`ImapB`] with a user-defined value decoder / encoder
    pub fn new_with_user_defined_xcoders(
        min: T,
        max: T,
        len: usize,
        user_enc: Option<fn(T) -> Vec<u8>>,
        user_dec: Option<fn(&[u8]) -> T>,
    ) -> Result<Self, ImapError<T>> {
        match min < max {
            true => {
                let b_pow = Self::calc_b_pow(&min, &max)?;
                let d_pow = Self::calc_d_pow(&len);
                let s_f = Self::calc_s_f(&b_pow, &d_pow)?;
                let s_r = Self::calc_s_r(&b_pow, &d_pow)?;
                let z_offset = Self::calc_z_offset(&min, &max, &s_f);
                Ok(ImapB { min, max, len, s_f, s_r, z_offset, encoder: user_enc, decoder: user_dec })
            },
            false => Err(ImapError::InvalidInputArgs(min, max, Some(len), None)),
        }
    }

    /// Calculates `b_pow`, used in calculating [`ImapB::s_f`] and [`ImapB::s_r`]
    fn calc_b_pow(min: &T, max: &T) -> Result<usize, ImapError<T>> {
        let b_pow = (*max - *min).log2().ceil();
        b_pow.to_usize().ok_or(ImapError::ParseError(format!("Unable to convert `b_pow` {} to `usize`", b_pow)))
    }

    /// Calculates `d_pow`, used in calculating [`ImapB::s_f`] and [`ImapB::s_r`]
    fn calc_d_pow(len: &usize) -> usize {
        (*len * 8) - 1
    }

    /// Calculates [`ImapB::s_f`]
    fn calc_s_f(b_pow: &usize, d_pow: &usize) -> Result<T, ImapError<T>> {
        let s_f = 2_usize.pow((*d_pow - *b_pow) as u32);
        T::from_usize(s_f).ok_or(ImapError::ParseError(format!("Unable to convert forward scale factor {} to input floating-point precision", s_f)))
    }

    /// Calculates [`ImapB::s_r`]
    fn calc_s_r(b_pow: &usize, d_pow: &usize) -> Result<T, ImapError<T>> {
        // is `2^(b_pow - d_pow)`, but `b_pow` will always be less than `d_pow`
        let s_r = 1.0 / (1 << (*d_pow - *b_pow)) as f64;
        T::from_f64(s_r).ok_or(ImapError::ParseError(format!("Unable to convert reverse scale factor {} to input floating-point precision", s_r)))
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
    pub fn to_imap(&self, x: T) -> Result<Vec<u8>, ImapError<T>> {
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
    pub fn to_imap_with_encoder(&self, x: T) -> Result<Vec<u8>, ImapError<T>> {
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
    pub fn from_imap(&self, y: &[u8]) -> Result<T, ImapError<T>> {
        if y.len() != self.len { return Err(ImapError::ParseError(format!("Cannot convert {} bytes to {} bytes", y.len(), self.len))) }
        if (y[0] >> 7 & 1) & (y[0] >> 6 & 1) != 0 {
            // --------------------------------------------------
            // special value
            // --------------------------------------------------
            match SpecialValue::from_imap(y) {
                Ok(sval) => match sval {
                    SpecialValue::PosInfinity => return Ok(T::infinity()),
                    SpecialValue::NegInfinity => return Ok(-T::infinity()),
                    SpecialValue::PosQuietNan |
                    SpecialValue::PosSignalNan => return Ok(T::nan()),
                    SpecialValue::NegQuietNan |
                    SpecialValue::NegSignalNan => return Ok(-T::nan()),
                    SpecialValue::ImapBelowMinimum => return Err(ImapError::BelowMinimum),
                    SpecialValue::ImapAboveMaximum => return Err(ImapError::AboveMaximum),
                    SpecialValue::ReservedSpecial => return Err(ImapError::ReservedValue),
                    SpecialValue::ReservedMisbDefined => return Err(ImapError::ReservedValue),
                    SpecialValue::UserDefined => match self.decoder {
                        Some(dec) => return Ok(dec(&y)),
                        None => return Err(ImapError::UserDefinedValue),
                    },
                },
                Err(e) => return Err(e),
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
// impl<T: ImapFloat> Send for ImapA<T> {}

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
    pub fn from_imap<T: ImapFloat>(y: &[u8]) -> Result<SpecialValue, ImapError<T>> {
        // --------------------------------------------------
        // check the entire first byte to see if IMAP error
        // (below min or above max)
        // --------------------------------------------------
        match Self::try_from(y[0]) {
            Ok(sval) => match sval {
                SpecialValue::ImapBelowMinimum |
                SpecialValue::ImapAboveMaximum => {
                    // --------------------------------------------------
                    // for these values, the trailing bits must be zero
                    // --------------------------------------------------
                    if y[1..] == vec![0u8; y.len() - 1] { return Ok(sval) }
                },
                _ => (),
            }
            _ => (),
        }
        // --------------------------------------------------
        // do not include the 3 trailing bits of the most-significant byte
        // for positive infinity, negative infinity, and user defined
        // --------------------------------------------------
        match Self::try_from(y[0] & 0b1111_1000) {
            Ok(sval) => match sval {
                SpecialValue::PosInfinity |
                SpecialValue::NegInfinity => {
                    // --------------------------------------------------
                    // for these values, the trailing bits must be zero
                    // --------------------------------------------------
                    if y[1..] == vec![0u8; y.len() - 1] { return Ok(sval) }
                },
                SpecialValue::UserDefined => return Ok(SpecialValue::UserDefined),
                _ => (),
            }
            Err(_) => (),
        };
        // --------------------------------------------------
        // The default NaN Identifier is a value with all zeros
        // therefore, do not check the trailing bits
        // --------------------------------------------------
        if (y[0] & SpecialValue::PosQuietNan.value()) == *SpecialValue::PosQuietNan.value() { return Ok(SpecialValue::PosQuietNan) }
        if (y[0] & SpecialValue::NegQuietNan.value()) == *SpecialValue::NegQuietNan.value() { return Ok(SpecialValue::NegQuietNan) }
        if (y[0] & SpecialValue::PosSignalNan.value()) == *SpecialValue::PosSignalNan.value() { return Ok(SpecialValue::PosSignalNan) }
        if (y[0] & SpecialValue::NegSignalNan.value()) == *SpecialValue::NegSignalNan.value() { return Ok(SpecialValue::NegSignalNan) }
        // --------------------------------------------------
        // if starts with 0b10XX_XXXX, where any X is set, 
        // then the value is a reserved special value
        // --------------------------------------------------
        if (y[0] & 0b1100_0000) == 0b1000_0000
        && (y[0] & 0b0011_1111 != 0 || y[1..].iter().any(|&byte| byte != 0))
        {
            return Ok(SpecialValue::ReservedSpecial)
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
            return Ok(SpecialValue::ReservedMisbDefined)
        }
        // --------------------------------------------------
        // if all checks fail, return Error
        // --------------------------------------------------
        Err(ImapError::ParseError(format!("Cannot parse special value: {y:?}")))
    }
}

#[cfg(test)]
mod test {
    use super::ImapA;
    use super::ImapB;
    use super::ImapError;

    #[test]
    /// From: https://nsgreg.nga.mil/misb.jsp Misb Standard 1201 v5
    /// Page 22
    fn imap_a_test0() {
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
    /// From: https://nsgreg.nga.mil/misb.jsp Misb Standard 1201 v5
    /// Page 23
    fn imap_b_test0() {
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
    /// From: https://nsgreg.nga.mil/misb.jsp Misb Standard 1201 v5
    /// Page 24
    fn imap_a_test1() {
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
    /// From: https://nsgreg.nga.mil/misb.jsp Misb Standard 1201 v5
    /// Page 24 - 25
    fn imap_b_test1() {
        let example = ImapB::new(0.0, 100.0, 3).unwrap();

        assert_eq!(example.to_imap(0.0).unwrap(), vec![0x00, 0x00, 0x00]);
        assert_eq!(example.from_imap(&vec![0x00, 0x00, 0x00]).unwrap(), 0.0);
        
        assert_eq!(example.to_imap(10.1).unwrap(), vec![0x0A, 0x19, 0x99]);
        assert!(example.from_imap(&vec![0x0A, 0x19, 0x99]).unwrap() - 10.09999 < 1e-6);
        
        assert_eq!(example.to_imap(20.2).unwrap(), vec![0x14, 0x33, 0x33]);
        assert!(example.from_imap(&vec![0x14, 0x33, 0x33]).unwrap() - 20.2 < 1e-6);
        
        assert_eq!(example.to_imap(30.3).unwrap(), vec![0x1E, 0x4C, 0xCC]);
        assert!(example.from_imap(&vec![0x1E, 0x4C, 0xCC]).unwrap() - 30.29999 < 1e-6);
        
        assert_eq!(example.to_imap(40.4).unwrap(), vec![0x28, 0x66, 0x66]);
        assert!(example.from_imap(&vec![0x28, 0x66, 0x66]).unwrap() - 40.39999 < 1e-5);
        
        assert_eq!(example.to_imap(50.5).unwrap(), vec![0x32, 0x80, 0x00]);
        assert_eq!(example.from_imap(&vec![0x32, 0x80, 0x00]).unwrap(), 50.5);
        
        assert_eq!(example.to_imap(60.6).unwrap(), vec![0x3C, 0x99, 0x99]);
        assert!(example.from_imap(&vec![0x3C, 0x99, 0x99]).unwrap() - 60.59999 < 1e-6);
        
        assert_eq!(example.to_imap(70.7).unwrap(), vec![0x46, 0xB3, 0x33]);
        assert!(example.from_imap(&vec![0x46, 0xB3, 0x33]).unwrap() - 70.7 < 1e-6);
        
        assert_eq!(example.to_imap(80.8).unwrap(), vec![0x50, 0xCC, 0xCC]);
        assert!(example.from_imap(&vec![0x50, 0xCC, 0xCC]).unwrap() - 80.79999 < 1e-6);
        
        assert_eq!(example.to_imap(90.9).unwrap(), vec![0x5A, 0xE6, 0x66]);
        assert!(example.from_imap(&vec![0x5A, 0xE6, 0x66]).unwrap() - 90.89999 < 1e-5);

        assert_eq!(example.to_imap(100.0).unwrap(), vec![0x64, 0x00, 0x00]);
        assert_eq!(example.from_imap(&vec![0x64, 0x00, 0x00]).unwrap(), 100.0);
        
        assert_eq!(example.to_imap(f64::NAN).unwrap(), vec![0xD0, 0x00, 0x00]);
        assert!(example.from_imap(&vec![0xD0, 0x00, 0x00]).unwrap().is_nan());
        
        assert_eq!(example.to_imap(f64::INFINITY).unwrap(), vec![0xC8, 0x00, 0x00]);
        assert_eq!(example.from_imap(&vec![0xC8, 0x00, 0x00]).unwrap(), f64::INFINITY);
        
        assert_eq!(example.to_imap(f64::NEG_INFINITY).unwrap(), vec![0xE8, 0x00, 0x00]);
        assert_eq!(example.from_imap(&vec![0xE8, 0x00, 0x00]).unwrap(), f64::NEG_INFINITY);
        
        assert_eq!(example.to_imap(-1.0).unwrap(), vec![0xE0, 0x00, 0x00]);
        assert_eq!(example.from_imap(&vec![0xE0, 0x00, 0x00]).unwrap_err(), ImapError::BelowMinimum);
        
        assert_eq!(example.to_imap(101.0).unwrap(), vec![0xE1, 0x00, 0x00]);
        assert_eq!(example.from_imap(&vec![0xE1, 0x00, 0x00]).unwrap_err(), ImapError::AboveMaximum);
    }

    #[test]
    /// From: https://nsgreg.nga.mil/misb.jsp Misb Standard 1201 v5
    /// Page 25
    fn imap_b_test2() {
        let example = ImapB::new(-9.9, 110.0, 3).unwrap();

        assert_eq!(example.to_imap(-9.9).unwrap(), vec![0x00, 0x00, 0x00]);
        assert!(example.from_imap(&vec![0x00, 0x00, 0x00]).unwrap() - -9.90001 < 1e-6);

        assert_eq!(example.to_imap(0.225).unwrap(), vec![0x0A, 0x20, 0x00]);
        assert!(example.from_imap(&vec![0x0A, 0x20, 0x00]).unwrap() - 0.225 < 1e-6);

        assert_eq!(example.to_imap(10.35).unwrap(), vec![0x14, 0x40, 0x00]);
        assert!(example.from_imap(&vec![0x14, 0x40, 0x00]).unwrap() - 10.34999 < 1e-6);

        assert_eq!(example.to_imap(20.475).unwrap(), vec![0x1E, 0x60, 0x00]);
        assert!(example.from_imap(&vec![0x1E, 0x60, 0x00]).unwrap() - 20.47499 < 1e-6);

        assert_eq!(example.to_imap(30.6).unwrap(), vec![0x28, 0x80, 0x00]);
        assert!(example.from_imap(&vec![0x28, 0x80, 0x00]).unwrap() - 30.59999 < 1e-6);

        assert_eq!(example.to_imap(40.725).unwrap(), vec![0x32, 0xA0, 0x00]);
        assert!(example.from_imap(&vec![0x32, 0xA0, 0x00]).unwrap() - 40.72499 < 1e-6);

        assert_eq!(example.to_imap(50.85).unwrap(), vec![0x3C, 0xC0, 0x00]);
        assert!(example.from_imap(&vec![0x3C, 0xC0, 0x00]).unwrap() - 50.84999 < 1e-6);

        assert_eq!(example.to_imap(60.975).unwrap(), vec![0x46, 0xE0, 0x00]);
        assert!(example.from_imap(&vec![0x46, 0xE0, 0x00]).unwrap() - 60.97499 < 1e-6);

        assert_eq!(example.to_imap(71.1).unwrap(), vec![0x51, 0x00, 0x00]);
        assert!(example.from_imap(&vec![0x51, 0x00, 0x00]).unwrap() - 71.09999 < 1e-6);

        assert_eq!(example.to_imap(81.225).unwrap(), vec![0x5B, 0x20, 0x00]);
        assert!(example.from_imap(&vec![0x5B, 0x20, 0x00]).unwrap() - 81.22499 < 1e-6);

        assert_eq!(example.to_imap(91.35).unwrap(), vec![0x65, 0x40, 0x00]);
        assert!(example.from_imap(&vec![0x65, 0x40, 0x00]).unwrap() - 91.34999 < 1e-6);

        assert_eq!(example.to_imap(101.475).unwrap(), vec![0x6F, 0x60, 0x00]);
        assert!(example.from_imap(&vec![0x6F, 0x60, 0x00]).unwrap() - 101.47499 < 1e-6);

        assert_eq!(example.to_imap(110.0).unwrap(), vec![0x77, 0xE6, 0x67]);
        assert!(example.from_imap(&vec![0x77, 0xE6, 0x67]).unwrap() - 110.0 < 1e-6);

        assert_eq!(example.to_imap(0.0).unwrap(), vec![0x09, 0xE6, 0x67]);
        assert!(example.from_imap(&vec![0x09, 0xE6, 0x67]).unwrap() - 0.0 < 1e-6);

        assert_eq!(example.to_imap(f64::NAN).unwrap(), vec![0xD0, 0x00, 0x00]);
        assert!(example.from_imap(&vec![0xD0, 0x00, 0x00]).unwrap().is_nan());

        assert_eq!(example.to_imap(f64::INFINITY).unwrap(), vec![0xC8, 0x00, 0x00]);
        assert_eq!(example.from_imap(&vec![0xC8, 0x00, 0x00]).unwrap(), f64::INFINITY);

        assert_eq!(example.to_imap(f64::NEG_INFINITY).unwrap(), vec![0xE8, 0x00, 0x00]);
        assert_eq!(example.from_imap(&vec![0xE8, 0x00, 0x00]).unwrap(), f64::NEG_INFINITY);

        assert_eq!(example.to_imap(-100.0).unwrap(), vec![0xE0, 0x00, 0x00]);
        assert_eq!(example.from_imap(&vec![0xE0, 0x00, 0x00]).unwrap_err(), ImapError::BelowMinimum);

        assert_eq!(example.to_imap(121.0).unwrap(), vec![0xE1, 0x00, 0x00]);
        assert_eq!(example.from_imap(&vec![0xE1, 0x00, 0x00]).unwrap_err(), ImapError::AboveMaximum);
    }
}