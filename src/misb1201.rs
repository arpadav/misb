// --------------------------------------------------
// external
// --------------------------------------------------
use thisenum::Const;

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
    T: num_traits::Float,
    T: num_traits::FromPrimitive,
{
    /// The minimum floating point value
    /// 
    /// Also known as a
    min: T,
    /// The maximum floating point value
    /// 
    /// Also known as b
    max: T,
    /// The floating point precision
    /// 
    /// Also known as g
    precision: T,
    /// The [`ImapB`], once the length, in bytes, is known
    imapb: ImapB<T>,
}
/// [`ImapA`] implementation
impl<T> ImapA<T>
where
    T: num_traits::Float,
    T: num_traits::FromPrimitive,
{
    /// Creates a new [`ImapA`]
    pub fn new(min: T, max: T, precision: T) -> Option<Self> {
        match min < max && precision < (max - min) {
            true => {
                match Self::len(&min, &max, &precision) {
                    Some(len) => {
                        let imapb = ImapB::new(min, max, len)?;
                        Some(ImapA { min, max, precision, imapb })
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

    // /// Returns the mapped value
    // pub fn map<O>(&self, x: T) -> O {
    //     self.imapb.map(x)
    // }
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
    T: num_traits::Float,
    T: num_traits::FromPrimitive,
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
    /// Power of 2 adjustment - only used internally
    b_pow: usize,
    /// Power of 2 adjustment - only used internally
    d_pow: usize,
    /// Scaling factor - forward
    s_f: T,
    /// Scaling factor - reverse
    s_r: T,
    /// The zero-point offset
    z_offset: T,
}
/// [`ImapB`] implementation
impl<T> ImapB<T>
where
    T: num_traits::Float,
    T: num_traits::FromPrimitive,
{
    /// Creates a new [`ImapB`]
    pub fn new(min: T, max: T, len: usize) -> Option<Self> {
        match min < max {
            true => {
                let b_pow = Self::calc_b_pow(&min, &max)?;
                let d_pow = Self::calc_d_pow(&len);
                let s_f = Self::calc_s_f(&b_pow, &d_pow)?;
                let s_r = Self::calc_s_r(&b_pow, &d_pow)?;
                let z_offset = Self::calc_z_offset(&min, &max, &s_f);
                Some(ImapB { min, max, len, b_pow, d_pow, s_f, s_r, z_offset })
            },
            false => None
        }
    }

    /// Calculates [`ImapB::b_pow`]
    fn calc_b_pow(min: &T, max: &T) -> Option<usize> {
        (*max - *min).log2().ceil().to_usize()
    }

    /// Calculates [`ImapB::d_pow`]
    fn calc_d_pow(len: &usize) -> usize {
        (*len - 1) * 8
    }

    /// Calculates [`ImapB::s_f`]
    fn calc_s_f(b_pow: &usize, d_pow: &usize) -> Option<T> {
        T::from_usize(2_usize.pow((*d_pow - *b_pow) as u32))
    }

    /// Calculates [`ImapB::s_r`]
    fn calc_s_r(b_pow: &usize, d_pow: &usize) -> Option<T> {
        T::from_usize(2_usize.pow((*b_pow - *d_pow) as u32))
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
    /// * If NaN, return [`Value::PosQuietNan`] or [`Value::NegQuietNan`]
    /// * If infinite, return [`Value::PosInfinity`] or [`Value::NegInfinity`]
    /// * If below minimum, return [`Value::MisbDefined`] with [`MisbSpecialValues::ImapBelowMinimum`]
    /// * If above maximum, return [`Value::MisbDefined`] with [`MisbSpecialValues::ImapAboveMaximum`]
    /// * Otherwise, returns the successfully mapped value
    pub fn to_imap(&self, x: T) -> Option<Vec<u8>> {
        // --------------------------------------------------
        // Rust's floating-point operations and the IEEE-754 standard
        // typically use quiet NaNs for representing invalid results
        // --------------------------------------------------
        if x.is_nan() {
            return Some(match x.is_sign_positive() {
                true => Value::Special(SpecialValue::PosQuietNan).to_imap(self.len),
                false => Value::Special(SpecialValue::NegQuietNan).to_imap(self.len),
            })
        }
        if x.is_infinite() {
            return Some(match x.is_sign_positive() {
                true => Value::Special(SpecialValue::PosInfinity).to_imap(self.len),
                false => Value::Special(SpecialValue::NegInfinity).to_imap(self.len),
            })
        }
        if x < self.min { return Some(Value::Special(SpecialValue::ImapBelowMinimum).to_imap(self.len)) }
        if x > self.max { return Some(Value::Special(SpecialValue::ImapAboveMaximum).to_imap(self.len)) }
        // --------------------------------------------------
        // truncate, convert to usize, then convert to len bytes
        // --------------------------------------------------
        let y = (self.s_f * (x - self.min) + self.z_offset).trunc().to_usize()?;
        Some((0..self.len).map(|i| ((y >> (8 * (self.len - 1 - i))) & 0xFF) as u8).collect())
    }

    /// Maps an IMAP integer value to a floating point value
    /// 
    /// Returns [`None`] when the value fails to be mapped
    /// 
    /// 
    pub fn from_imap(&self, x: Vec<u8>) -> Option<T> {
        if x.len() != self.len { return None }
        // if (x[0] >> 7 & 1) & (x[0] >> 6 & 1) {
        //     return Some(T::nan())
        // }
        None
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

    // /// From an IMAP value
    // pub fn from_imap(x: Vec<u8>, len: usize) -> Option<Self> {
    //     match x.len() == len {
    //         true => Some(Value::Normal(x)),
    //         false => None
    //     }
    // }
}

#[derive(Const)]
#[armtype(u8)]
/// MISB Standard 1201 Special Values
/// 
/// b_n         = 1
/// b_n-1       = Special bit
/// b_n-2       = Sign bit
/// b_n-3       = NaN bit
/// b_n-4       = Any
/// b_n-5 - b_0 = Contextual (either user defined, zero-filled, etc.)
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
    pub fn from_imap(&self, x: &Vec<u8>) -> Option<SpecialValue> {
        // --------------------------------------------------
        // do not include the 3 trailing bits of the most-significant byte
        // --------------------------------------------------
        match Self::try_from(x[0] & 0b1111_1000) {
            Ok(v) => match v {
                SpecialValue::PosInfinity |
                SpecialValue::NegInfinity |
                SpecialValue::ImapBelowMinimum |
                SpecialValue::ImapAboveMaximum => {
                    // --------------------------------------------------
                    // for these values, the trailing bits must be zero
                    // --------------------------------------------------
                    if x[1..] != vec![0u8; x.len() - 1] { return None }
                    return Some(v)
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
        if (x[0] & SpecialValue::PosQuietNan.value()) == *SpecialValue::PosQuietNan.value() { return Some(SpecialValue::PosQuietNan) }
        if (x[0] & SpecialValue::NegQuietNan.value()) == *SpecialValue::NegQuietNan.value() { return Some(SpecialValue::NegQuietNan) }
        if (x[0] & SpecialValue::PosSignalNan.value()) == *SpecialValue::PosSignalNan.value() { return Some(SpecialValue::PosSignalNan) }
        if (x[0] & SpecialValue::NegSignalNan.value()) == *SpecialValue::NegSignalNan.value() { return Some(SpecialValue::NegSignalNan) }
        // --------------------------------------------------
        // if starts with 0b10XX_XXXX, where any X is set, 
        // then the value is a reserved special value
        // --------------------------------------------------
        if (x[0] & 0b1100_0000) == 0b1000_0000
        && (x[0] & 0b0011_1111 != 0 || x[1..].iter().any(|&b| b != 0))
        {
            return Some(SpecialValue::ReservedSpecial)
        }
        // --------------------------------------------------
        // if starts with 0b1110_0XXA, where any X is set,
        // and A is 0 or 1, and the remaining bits are zero
        // then the value is a reserved MISB defined value
        // --------------------------------------------------
        if (x[0] & 0b1111_1000) == 0b1110_0000
        && x[0] & 0b0000_0110 != 0
        && x[1..] == vec![0u8; x.len() - 1]
        {
            return Some(SpecialValue::ReservedMisbDefined)
        }
        // --------------------------------------------------
        // if all checks fail, return None
        // --------------------------------------------------
        None
    }
}


mod test {
    use super::*;
    use rand::Rng;
    use std::time::{
        Instant,
        Duration,
    };

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