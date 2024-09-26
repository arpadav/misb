// --------------------------------------------------
// tinyklv
// --------------------------------------------------
// use tinyklv::Klv;
use tinyklv::prelude::*;

// --------------------------------------------------
// external
// --------------------------------------------------
use thisenum::Const;

// --------------------------------------------------
// local
// --------------------------------------------------
use crate::misb0903::ops;

#[derive(Debug)]
/// A pixel position
/// 
/// Defaults to a pixel number, but once a frame
/// width is provided, it is converted to a row and column
/// 
/// This is not intended to go in reverse. It is solely
/// for initialization purposes.
/// 
/// For usage examples, please see: 
/// 
/// * [`crate::misb0903::Misb0903Target::target_centroid`]
/// * [`crate::misb0903::Misb0903Target::bbox_tl`]
/// * [`crate::misb0903::Misb0903Target::bbox_br`]
pub struct PixelPosition {
    pub num: u32,
    pub rc: Option<(u32, u32)>,
    pub width: Option<f32>
}
/// [`PixelPosition`] implementation
impl PixelPosition {
    #[inline(always)]
    /// Creates a new [`PixelPosition`]
    pub(crate) fn new(num: u32) -> Self {
        Self { num, rc: None, width: None }
    }

    /// Converts a pixel number to a row and column
    /// 
    /// This is used internally, once a `frame_width` is provided
    /// in the MISB 0903 stream.
    pub(crate) fn to_rc(&mut self, width: f32) -> (u32, u32) {
        // --------------------------------------------------
        // return self if already set and equal
        // --------------------------------------------------
        // the unwrap is safe because `rc` is always set when
        // `width` is set
        // --------------------------------------------------
        if let Some(self_width) = self.width {
            if self_width == width { return self.rc.unwrap(); }
        }
        // --------------------------------------------------
        // otherwise, calculate
        // --------------------------------------------------
        self.width = Some(width);
        let row = ((self.num as f32 / width)).floor() as u32 + 1;
        let col = ((self.num - (row - 1)) as f32 * width) as u32;
        self.rc = Some((row, col));
        (row, col)
    }

    #[inline(always)]
    /// Decodes a pixel number from a stream
    pub fn decode(len: usize) -> impl Fn(&mut &[u8]) -> winnow::PResult<Self> {
        move |input| Ok(PixelPosition::new(tinyklv::binary::dec::be_u32_lengthed(len)(input)?))
    }
}

#[derive(Debug)]
/// RGB color
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
/// [`Color`] implementation of [`std::fmt::Display`]
impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(r: {}, g: {}, b: {})", self.red, self.green, self.blue)
    }
}
/// [`Color`] implementation of [`tinyklv::prelude::Decode`]
impl tinyklv::prelude::Decode<&[u8]> for Color {
    fn decode(input: &mut &[u8]) -> winnow::PResult<Self> {
        let (red, green, blue) = (
            tinyklv::codecs::binary::dec::be_u8,
            tinyklv::codecs::binary::dec::be_u8,
            tinyklv::codecs::binary::dec::be_u8,
        ).parse_next(input)?;
        Ok(Self { red, green, blue })
    }
}

#[derive(Debug)]
/// A location
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub hae: f64,
    pub measurements: EnuMeasurements,
}
/// [`Location`] implementation of [`crate::LengthBytes`]
impl crate::LengthBytes for Location {
    const LENGTH_BYTES: usize = 10 + EnuMeasurements::LENGTH_BYTES;
}
/// [`Location`] implementation of [`tinyklv::prelude::Decode`]
impl tinyklv::prelude::Decode<&[u8]> for Location {
    fn decode(input: &mut &[u8]) -> winnow::PResult<Self> {
        let (
            latitude,
            longitude,
            hae,
            measurements,
        ) = (
            ops::imapb_parser(&ops::IMAPB_N90_90_4_F64, 4),
            ops::imapb_parser(&ops::IMAPB_N180_180_4_F64, 4),
            ops::imapb_parser(&ops::IMAPB_N900_19K_2_F64, 2),
            EnuMeasurements::decode,
        ).parse_next(input)?;
        Ok(Self { latitude, longitude, hae, measurements })
    }
}

#[derive(Debug)]
/// Motion
pub enum Motion {
    /// The first-order derivative of position
    /// 
    /// Units: Meters per second (m/s)
    Velocity(MotionValues),
    /// The second-order derivative of position
    /// 
    /// Units: Meters per second squared (m/s^2)
    Acceleration(MotionValues),
}
/// [`Motion`] implementation
impl Motion {
    /// Decodes a velocity
    /// 
    /// See: [`crate::misb0903::Motion`]
    pub fn decode_velocity(input: &mut &[u8]) -> winnow::PResult<Self> {
        Ok(Motion::Velocity(MotionValues::decode(input)?))
    }
    /// Decodes an acceleration
    /// 
    /// See: [`crate::misb0903::Motion`]
    pub fn decode_acceleration(input: &mut &[u8]) -> winnow::PResult<Self> {
        Ok(Motion::Acceleration(MotionValues::decode(input)?))
    }
}

#[derive(Debug)]
/// Motion values, which describes any N'th order derivative
/// of position
/// 
/// See: [`crate::misb0903::primitives::Motion`]
pub struct MotionValues {
    pub east: f64,
    pub north: f64,
    pub up: f64,
    pub measurements: EnuMeasurements,
}
/// [`MotionValues`] implementation of [`crate::LengthBytes`]
impl crate::LengthBytes for MotionValues {
    const LENGTH_BYTES: usize = 6 + EnuMeasurements::LENGTH_BYTES;
}
/// [`MotionValues`] implementation of [`tinyklv::prelude::Decode`]
impl tinyklv::prelude::Decode<&[u8]> for MotionValues {
    fn decode(input: &mut &[u8]) -> winnow::PResult<Self> {
        let (
            east,
            north,
            up,
            measurements,
        ) = (
            ops::imapb_parser(&ops::IMAPB_N900_900_2_F64, 2),
            ops::imapb_parser(&ops::IMAPB_N900_900_2_F64, 2),
            ops::imapb_parser(&ops::IMAPB_N900_900_2_F64, 2),
            EnuMeasurements::decode,
        ).parse_next(input)?;
        Ok(Self { east, north, up, measurements })
    }
}

#[derive(Debug)]
/// Measurements
/// 
/// This includes the standard-deviations and correlation-coefficients
/// of an ENU coordinate system
/// 
/// Unit agnostic. Can be used for any N'th order temporal derivative of position.
/// 
/// This includes position, velocity, acceleration, etc.
pub struct EnuMeasurements {
    pub sig_east: f64,
    pub sig_north: f64,
    pub sig_up: f64,
    pub rho_east_north: f64,
    pub rho_east_up: f64,
    pub rho_north_up: f64,
}
/// [`EnuMeasurements`] implementation of [`crate::LengthBytes`]
impl crate::LengthBytes for EnuMeasurements {
    const LENGTH_BYTES: usize = 12;
}
/// [`EnuMeasurements`] implementation of [`tinyklv::prelude::Decode`]
impl tinyklv::prelude::Decode<&[u8]> for EnuMeasurements {
    fn decode(input: &mut &[u8]) -> winnow::PResult<Self> {
        let (
            sig_east,
            sig_north,
            sig_up,
            rho_east_north,
            rho_east_up,
            rho_north_up,
        ) = (
            ops::imapb_parser(&ops::IMAPB_0_650_2_F64, 2),
            ops::imapb_parser(&ops::IMAPB_0_650_2_F64, 2),
            ops::imapb_parser(&ops::IMAPB_0_650_2_F64, 2),
            ops::imapb_parser(&ops::IMAPB_N1_1_2_F64, 2),
            ops::imapb_parser(&ops::IMAPB_N1_1_2_F64, 2),
            ops::imapb_parser(&ops::IMAPB_N1_1_2_F64, 2),
        ).parse_next(input)?;
        Ok(Self { sig_east, sig_north, sig_up, rho_east_north, rho_east_up, rho_north_up })
    }
}

#[derive(Const)]
#[armtype(u8)]
/// Detection Status
/// 
/// See 7.2 of MISB 0903.6
pub enum DetectionStatus {
    #[value = 0]
    /// The target's coasting time has expired.
    /// The target may be reacquired in future detections.
    Inactive,

    #[value = 1]
    /// The target is in motion.
    /// System maintains visual detection of moving target.
    ActiveMoving,

    #[value = 2]
    /// The system does not detect target and determines it is no longer visible.
    /// The system will not reuse the target id.
    Dropped,

    #[value = 3]
    /// The target is stationary.
    /// System maintains visual detection of stationary targets.
    ActiveStopped,

    #[value = 4]
    /// The system is waiting for the target's coasting time to expire.
    /// The target may be reacquired in future detections.
    ActiveCoasting,
}
/// [`DetectionStatus`] implementation of [`tinyklv::prelude::Decode`]
impl tinyklv::prelude::Decode<&[u8]> for DetectionStatus {
    fn decode(input: &mut &[u8]) -> winnow::PResult<Self> {
        Self::try_from(tinyklv::dec::binary::be_u8.parse_next(input)?).map_err(|_| tinyklv::err!())
    }
}
/// [`DetectionStatus`] implementation of [`tinyklv::prelude::Encode`]
impl tinyklv::prelude::Encode<u8, Vec<u8>> for DetectionStatus {
    fn encode(&self) -> Vec<u8> {
        return vec![*self.value()]
    }
}