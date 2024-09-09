#![allow(dead_code)]
#![allow(non_upper_case_globals)]
// --------------------------------------------------
// external
// --------------------------------------------------
use tinyklv::prelude::*;

// --------------------------------------------------
// constants
// --------------------------------------------------
// it is known that some values are repeated
// --------------------------------------------------
// encoding values
// --------------------------------------------------
pub const SFT_2_LAT: f64 = 4294967294.0 / 180.0;
pub const SFT_2_LON: f64 = 4294967294.0 / 360.0;
pub const SFT_2_PLATFORM_HEADING: f32 = 65535.0 / 360.0;
pub const SFT_2_PLATFORM_PITCH: f32 = 65534.0 / 40.0;
pub const SFT_2_PLATFORM_ROLL: f32 = 65534.0 / 100.0;
pub const SFT_2_SENSOR_TRUE_ALT_P1: f32 = 65535.0 / 19900.0;
pub const SFT_2_SENSOR_HVFOV: f32 = 65535.0 / 180.0;
pub const SFT_2_SENSOR_REL_AZM_RLL_ANGLE: f64 = 4294967295.0 / 360.0;
pub const SFT_2_SENSOR_REL_ELV_ANGLE: f64 = 4294967294.0 / 360.0;
pub const SFT_2_SLANT_RANGE: f64 = 4294967295.0 / 5_000_000.0;
pub const SFT_2_TARGET_WIDTH: f32 = 65535.0 / 10_000.0;
pub const SFT_2_OFFSET_LL: f32 = 65534.0 / 0.15;
pub const SFT_2_WIND_DIRECTION: f32 = 65535.0 / 360.0;
pub const SFT_2_WIND_SPEED: f32 = 255.0 / 100.0;
pub const SFT_2_MBAR_PRESSURE: f32 = 65535.0 / 5000.0;
pub const SFT_2_ERROR_ESTIMATE: f32 = 65535.0 / 4095.0;
pub const SFT_2_PLATFORM_VERT_SPEED: f32 = 65534.0 / 360.0;
// --------------------------------------------------
// decoding values
// --------------------------------------------------
pub const KLV_2_LAT: f64 = 180.0 / 4294967294.0;
pub const KLV_2_LON: f64 = 360.0 / 4294967294.0;
pub const KLV_2_PLATFORM_HEADING: f32 = 360.0 / 65535.0;
pub const KLV_2_PLATFORM_PITCH: f32 = 40.0 / 65534.0;
pub const KLV_2_PLATFORM_ROLL: f32 = 100.0 / 65534.0;
pub const KLV_2_SENSOR_TRUE_ALT_P1: f32 = 19900.0 / 65535.0;
pub const KLV_2_SENSOR_HVFOV: f32 = 180.0 / 65535.0;
pub const KLV_2_SENSOR_REL_AZM_RLL_ANGLE: f64 = 360.0 / 4294967295.0;
pub const KLV_2_SENSOR_REL_ELV_ANGLE: f64 = 360.0 / 4294967294.0;
pub const KLV_2_SLANT_RANGE: f64 = 5_000_000.0 / 4294967295.0;
pub const KLV_2_TARGET_WIDTH: f32 = 10_000.0 / 65535.0;
pub const KLV_2_OFFSET_LL: f32 = 0.15 / 65534.0;
pub const KLV_2_WIND_DIRECTION: f32 = 360.0 / 65535.0;
pub const KLV_2_WIND_SPEED: f32 = 100.0 / 255.0;
pub const KLV_2_MBAR_PRESSURE: f32 = 5000.0 / 65535.0;
pub const KLV_2_ERROR_ESTIMATE: f32 = 4095.0 / 65535.0;
pub const KLV_2_PLATFORM_VERT_SPEED: f32 = 360.0 / 65534.0;
// --------------------------------------------------
// both encoding and decoding / misc
// --------------------------------------------------
pub const SENSOR_TRUE_ALT_OFFSET_P2: f32 = 900.0;

#[inline(always)]
#[cfg(feature = "misb0601-19")]
/// Represents the number of MICROSECONDS elapsed since midnight
/// (00:00:00), January 1, 1970, not including leap seconds.
/// 
/// # Example
/// 
/// ```
/// use chrono::TimeZone;
/// use tinyklv::prelude::*;
/// use misb::misb0601::ops::to_precision_timestamp;
/// let mut val1: &[u8] = &(0x0004_59F4_A6AA_4AA8 as u64).to_be_bytes();
/// let mut val2: &[u8] = &(0x0003_8244_30F6_CE40 as u64).to_be_bytes();
/// let result1 = to_precision_timestamp(&mut val1);
/// let result2 = to_precision_timestamp(&mut val2);
/// assert_eq!(result1, Ok(chrono::Utc.with_ymd_and_hms(2008, 10, 24, 0, 13, 29).unwrap() + chrono::Duration::milliseconds(913)));
/// assert_eq!(result2, Ok(chrono::Utc.with_ymd_and_hms(2001,  4, 19, 4, 25, 21).unwrap()));
/// ```
pub fn to_precision_timestamp(input: &mut &[u8]) -> winnow::PResult<chrono::DateTime<chrono::Utc>> {
    let checkpoint = input.checkpoint();
    // time in microseconds
    let ts = winnow::binary::be_u64.parse_next(input)?; 
    // time in seconds, time in nanoseconds
    let (ts, tns) = (ts / 1_000_000, (ts % 1_000_000) * 1_000);
    // convert to UTC
    match chrono::DateTime::from_timestamp(ts as i64, tns as u32) {
        Some(dt) => Ok(dt),
        None => Err(tinyklv::err!().add_context(
            input,
            &checkpoint,
            winnow::error::StrContext::Label("Invalid timestamp")
        )),
    }
}

#[inline(always)]
#[cfg(feature = "misb0601-19")]
/// Represents the number of MICROSECONDS elapsed since midnight
/// (00:00:00), January 1, 1970, not including leap seconds.
/// 
/// # Example
/// 
/// ```
/// use chrono::TimeZone;
/// use tinyklv::prelude::*;
/// use misb::misb0601::ops::from_precision_timestamp;
/// let val1 = chrono::Utc.with_ymd_and_hms(2008, 10, 24, 0, 13, 29).unwrap() + chrono::Duration::milliseconds(913);
/// let val2 = chrono::Utc.with_ymd_and_hms(2001,  4, 19, 4, 25, 21).unwrap();
/// let result1 = from_precision_timestamp(val1);
/// let result2 = from_precision_timestamp(val2);
/// assert_eq!(result1, [0x00, 0x04, 0x59, 0xF4, 0xA6, 0xAA, 0x4A, 0xA8]);
/// assert_eq!(result2, [0x00, 0x03, 0x82, 0x44, 0x30, 0xF6, 0xCE, 0x40]);
/// ```
pub const fn from_precision_timestamp(input: chrono::DateTime<chrono::Utc>) -> [u8; 8] {
    let seconds = input.timestamp() as u64; // seconds since epoch
    let nanoseconds = input.timestamp_subsec_nanos() as u64; // nanoseconds part
    (seconds * 1_000_000 + nanoseconds / 1_000).to_be_bytes()
}

#[inline(always)]
#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601`]
/// 
/// * [`crate::misb0601::Misb0601::sensor_latitude`]
/// * [`crate::misb0601::Misb0601::frame_center_latitude`]
pub fn to_lat(input: &mut &[u8]) -> winnow::PResult<f64> {
    let value = tinyklv::codecs::binary::dec::be_i32.parse_next(input)?;
    if value as u32 == 0x8000_0000 { return Err(tinyklv::err!()) } // "Reserved" - keep for backwards compatibility
    Ok((value as f64) * KLV_2_LAT)
}

#[inline(always)]
#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601`]
/// 
/// * [`crate::misb0601::Misb0601::sensor_latitude`]
/// * [`crate::misb0601::Misb0601::frame_center_latitude`]
/// 
/// This encoder cannot be `const fn` due to floating point arithmetic
pub fn from_lat(input: f64) -> [u8; 4] {
    let output = input * SFT_2_LAT;
    (output as i32).to_be_bytes()
}

#[inline(always)]
#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601`]
/// 
/// * [`crate::misb0601::Misb0601::sensor_longitude`]
/// * [`crate::misb0601::Misb0601::frame_center_longitude`]
pub fn to_lon(input: &mut &[u8]) -> winnow::PResult<f64> {
    let value = tinyklv::codecs::binary::dec::be_i32.parse_next(input)?;
    if value as u32 == 0x8000_0000 { return Err(tinyklv::err!()) } // "Reserved" - keep for backwards compatibility
    Ok((value as f64) * KLV_2_LON)
}

#[inline(always)]
#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601`]
/// 
/// * [`crate::misb0601::Misb0601::sensor_longitude`]
/// * [`crate::misb0601::Misb0601::frame_center_longitude`]
///
/// This encoder cannot be `const fn` due to floating point arithmetic
pub fn from_lon(input: f64) -> [u8; 4] {
    let output = input * SFT_2_LON;
    (output as i32).to_be_bytes()
}

#[inline(always)]
#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601`]
/// 
/// * [`crate::misb0601::Misb0601::sensor_true_altitude`]
/// * [`crate::misb0601::Misb0601::frame_center_elevation`]
pub fn to_alt(input: &mut &[u8]) -> winnow::PResult<f32> {
    let value = tinyklv::codecs::binary::dec::be_u16.parse_next(input)?;
    Ok((value as f32 * KLV_2_SENSOR_TRUE_ALT_P1) - SENSOR_TRUE_ALT_OFFSET_P2)
}

#[inline(always)]
#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601`]
/// 
/// * [`crate::misb0601::Misb0601::sensor_true_altitude`]
/// * [`crate::misb0601::Misb0601::frame_center_elevation`]
/// 
/// This encoder cannot be `const fn` due to floating point arithmetic
pub fn from_alt(input: f32) -> [u8; 2] {
    let output = (input + SENSOR_TRUE_ALT_OFFSET_P2) / KLV_2_SENSOR_TRUE_ALT_P1;
    (output as u16).to_be_bytes()
}

#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601::platform_heading_angle`]
pub const to_platform_heading_angle: fn(&mut &[u8]) -> winnow::PResult<f32> = tinyklv::scale!(
    tinyklv::codecs::binary::dec::be_u16,
    f32,
    KLV_2_PLATFORM_HEADING
);

#[inline(always)]
#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601::platform_heading_angle`]
/// 
/// This encoder cannot be `const fn` due to floating point arithmetic
pub fn from_platform_heading_angle(input: f32) -> [u8; 2] {
    let output = input * SFT_2_PLATFORM_HEADING;
    (output as u16).to_be_bytes()
}

#[inline(always)]
#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601::platform_pitch_angle`]
pub fn to_platform_pitch_angle(input: &mut &[u8]) -> winnow::PResult<f32> {
    let value = tinyklv::codecs::binary::dec::be_i16.parse_next(input)?;
    if value as u32 == 0x8000 { return Err(tinyklv::err!()) } // "Out of Range" - keep for backwards compatibility
    Ok((value as f32) * KLV_2_PLATFORM_PITCH)
}

#[inline(always)]
#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601::platform_pitch_angle`]
/// 
/// This encoder cannot be `const fn` due to floating point arithmetic
pub fn from_platform_pitch_angle(input: f32) -> [u8; 2] {
    let output = input * SFT_2_PLATFORM_PITCH;
    (output as i16).to_be_bytes()
}

#[inline(always)]
#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601::platform_roll_angle`]
pub fn to_platform_roll_angle(input: &mut &[u8]) -> winnow::PResult<f32> {
    let value = tinyklv::codecs::binary::dec::be_i16.parse_next(input)?;
    if value as u32 == 0x8000 { return Err(tinyklv::err!()) } // "Out of Range" - keep for backwards compatibility
    Ok((value as f32) * KLV_2_PLATFORM_ROLL)
}

#[inline(always)]
#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601::platform_roll_angle`]
/// 
/// This encoder cannot be `const fn` due to floating point arithmetic
pub fn from_platform_roll_angle(input: f32) -> [u8; 2] {
    let output = input * SFT_2_PLATFORM_ROLL;
    (output as i16).to_be_bytes()
}

#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601`]
/// 
/// * [`crate::misb0601::Misb0601::sensor_hfov`]
/// * [`crate::misb0601::Misb0601::sensor_vfov`]
pub const to_sensor_hvfov: fn(&mut &[u8]) -> winnow::PResult<f32> = tinyklv::scale!(
    tinyklv::codecs::binary::dec::be_u16,
    f32,
    KLV_2_SENSOR_HVFOV
);

#[inline(always)]
#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601`]
/// 
/// * [`crate::misb0601::Misb0601::sensor_hfov`]
/// * [`crate::misb0601::Misb0601::sensor_vfov`]
/// 
/// This encoder cannot be `const fn` due to floating point arithmetic
pub fn from_sensor_hvfov(input: f32) -> [u8; 2] {
    let output = input * SFT_2_SENSOR_HVFOV;
    (output as u16).to_be_bytes()
}

#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601::sensor_relative_azimuth_angle`]
/// 
/// Same as [`to_sensor_relative_roll_angle`]
pub const to_sensor_relative_azimuth_angle: fn(&mut &[u8]) -> winnow::PResult<f64> = tinyklv::scale!(
    tinyklv::codecs::binary::dec::be_u32,
    f64,
    KLV_2_SENSOR_REL_AZM_RLL_ANGLE
);

#[inline(always)]
#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601::sensor_relative_azimuth_angle`]
/// 
/// This encoder cannot be `const fn` due to floating point arithmetic
pub fn from_sensor_relative_azimuth_angle(input: f64) -> [u8; 4] {
    let output = input * SFT_2_SENSOR_REL_AZM_RLL_ANGLE;
    (output as u32).to_be_bytes()
}

#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601::sensor_relative_elevation_angle`]
pub const to_sensor_relative_elevation_angle: fn(&mut &[u8]) -> winnow::PResult<f64> = tinyklv::scale!(
    tinyklv::codecs::binary::dec::be_i32,
    f64,
    KLV_2_SENSOR_REL_ELV_ANGLE
);

#[inline(always)]
#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601::sensor_relative_elevation_angle`]
/// 
/// This encoder cannot be `const fn` due to floating point arithmetic
pub fn from_sensor_relative_elevation_angle(input: f64) -> [u8; 4] {
    let output = input * SFT_2_SENSOR_REL_ELV_ANGLE;
    (output as i32).to_be_bytes()
}

#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601::sensor_relative_roll_angle`]
/// 
/// Same as [`to_sensor_relative_azimuth_angle`]
pub const to_sensor_relative_roll_angle: fn(&mut &[u8]) -> winnow::PResult<f64> = tinyklv::scale!(
    tinyklv::codecs::binary::dec::be_u32,
    f64,
    KLV_2_SENSOR_REL_AZM_RLL_ANGLE
);

#[inline(always)]
#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601::sensor_relative_roll_angle`]
/// 
/// This encoder cannot be `const fn` due to floating point arithmetic
pub fn from_sensor_relative_roll_angle(input: f64) -> [u8; 4] {
    let output = input * SFT_2_SENSOR_REL_AZM_RLL_ANGLE;
    (output as u32).to_be_bytes()
}

#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601::slant_range`]
pub const to_slant_range: fn(&mut &[u8]) -> winnow::PResult<f64> = tinyklv::scale!(
    tinyklv::codecs::binary::dec::be_u32,
    f64,
    KLV_2_SLANT_RANGE
);

#[inline(always)]
#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601::slant_range`]
/// 
/// This encoder cannot be `const fn` due to floating point arithmetic
pub fn from_slant_range(input: f64) -> [u8; 4] {
    let output = input * SFT_2_SLANT_RANGE;
    (output as u32).to_be_bytes()
}

#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601::target_width`]
pub const to_target_width: fn(&mut &[u8]) -> winnow::PResult<f32> = tinyklv::scale!(
    tinyklv::codecs::binary::dec::be_u16,
    f32,
    KLV_2_TARGET_WIDTH
);

#[inline(always)]
#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601::target_width`]
/// 
/// This encoder cannot be `const fn` due to floating point arithmetic
pub fn from_target_width(input: f32) -> [u8; 2] {
    let output = input * SFT_2_TARGET_WIDTH;
    (output as u16).to_be_bytes()
}

#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601`]
/// 
/// * [`crate::misb0601::Misb0601::offset_corner_lat_p1`]
/// * [`crate::misb0601::Misb0601::offset_corner_lon_p1`]
/// * [`crate::misb0601::Misb0601::offset_corner_lat_p2`]
/// * [`crate::misb0601::Misb0601::offset_corner_lon_p2`]
/// * [`crate::misb0601::Misb0601::offset_corner_lat_p3`]
/// * [`crate::misb0601::Misb0601::offset_corner_lon_p3`]
/// * [`crate::misb0601::Misb0601::offset_corner_lat_p4`]
/// * [`crate::misb0601::Misb0601::offset_corner_lon_p4`]
pub const to_offset_ll: fn(&mut &[u8]) -> winnow::PResult<f32> = tinyklv::scale!(
    tinyklv::codecs::binary::dec::be_i16,
    f32,
    KLV_2_OFFSET_LL,
);

#[inline(always)]
#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601`]
/// 
/// * [`crate::misb0601::Misb0601::offset_corner_lat_p1`]
/// * [`crate::misb0601::Misb0601::offset_corner_lon_p1`]
/// * [`crate::misb0601::Misb0601::offset_corner_lat_p2`]
/// * [`crate::misb0601::Misb0601::offset_corner_lon_p2`]
/// * [`crate::misb0601::Misb0601::offset_corner_lat_p3`]
/// * [`crate::misb0601::Misb0601::offset_corner_lon_p3`]
/// * [`crate::misb0601::Misb0601::offset_corner_lat_p4`]
/// * [`crate::misb0601::Misb0601::offset_corner_lon_p4`]
/// 
/// This encoder cannot be `const fn` due to floating point arithmetic
pub fn from_offset_ll(input: f32) -> [u8; 2] {
    let output = input * SFT_2_OFFSET_LL;
    (output as i16).to_be_bytes()
}

#[inline(always)]
#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601::icing_detected`]
pub fn to_icing_detected(input: &mut &[u8]) -> winnow::PResult<crate::misb0601::Icing> {
    match tinyklv::codecs::binary::dec::be_u8.parse_next(input)? {
        0 => Ok(crate::misb0601::Icing::DetectorOff),
        1 => Ok(crate::misb0601::Icing::NoIcingDetected),
        2 => Ok(crate::misb0601::Icing::IcingDetected),
        _ => Err(tinyklv::err!()),
    }
}

#[inline(always)]
#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601::icing_detected`]
/// 
/// This encoder cannot be `const fn` due to floating point arithmetic
pub fn from_icing_detected(input: crate::misb0601::Icing) -> u8 {
    match input {
        crate::misb0601::Icing::DetectorOff => 0,
        crate::misb0601::Icing::NoIcingDetected => 1,
        crate::misb0601::Icing::IcingDetected => 2,
    }
}

#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601::wind_direction`]
pub const to_wind_direction: fn(&mut &[u8]) -> winnow::PResult<f32> = tinyklv::scale!(
    tinyklv::codecs::binary::dec::be_u16,
    f32,
    KLV_2_WIND_DIRECTION
);

#[inline(always)]
#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601::wind_direction`]
/// 
/// This encoder cannot be `const fn` due to floating point arithmetic
pub fn from_wind_direction(input: f32) -> [u8; 2] {
    let output = input * SFT_2_WIND_DIRECTION;
    (output as i16).to_be_bytes()
}

#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601::wind_speed`]
pub const to_wind_speed: fn(&mut &[u8]) -> winnow::PResult<f32> = tinyklv::scale!(
    tinyklv::codecs::binary::dec::be_u8,
    f32,
    KLV_2_WIND_SPEED,
);

#[inline(always)]
#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601::wind_speed`]
/// 
/// This encoder cannot be `const fn` due to floating point arithmetic
pub fn from_wind_speed(input: f32) -> [u8; 2] {
    let output = input * SFT_2_WIND_SPEED;
    (output as i16).to_be_bytes()
}

#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601`]
/// 
/// * [`crate::misb0601::Misb0601::static_pressure`]
/// * [`crate::misb0601::Misb0601::differential_pressure`]
pub const to_mbar_pressure: fn(&mut &[u8]) -> winnow::PResult<f32> = tinyklv::scale!(
    tinyklv::codecs::binary::dec::be_u16,
    f32,
    KLV_2_MBAR_PRESSURE,
);

#[inline(always)]
#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601`]
/// 
/// * [`crate::misb0601::Misb0601::static_pressure`]
/// * [`crate::misb0601::Misb0601::differential_pressure`]
/// 
/// This encoder cannot be `const fn` due to floating point arithmetic
pub fn from_mbar_pressure(input: f32) -> [u8; 2] {
    let output = input * SFT_2_MBAR_PRESSURE;
    (output as i16).to_be_bytes()
}

#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601`]
/// 
/// * [`crate::misb0601::Misb0601::target_track_gate_width`]
/// * [`crate::misb0601::Misb0601::target_track_gate_height`]
pub const to_target_track_gate_hw: fn(&mut &[u8]) -> winnow::PResult<u16> = tinyklv::scale!(
    tinyklv::codecs::binary::dec::be_u8,
    u16,
    2,
);

#[inline(always)]
#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601`]
/// 
/// * [`crate::misb0601::Misb0601::target_track_gate_width`]
/// * [`crate::misb0601::Misb0601::target_track_gate_height`]
pub const fn from_target_track_gate_hw(input: u16) -> [u8; 2] {
    (input >> 1).to_be_bytes()
}

#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601`]
/// 
/// * [`crate::misb0601::Misb0601::target_error_estimate_ce90`]
/// * [`crate::misb0601::Misb0601::target_error_estimate_le90`]
pub const to_error_estimate: fn(&mut &[u8]) -> winnow::PResult<f32> = tinyklv::scale!(
    tinyklv::codecs::binary::dec::be_u16,
    f32,
    KLV_2_ERROR_ESTIMATE,
);

#[inline(always)]
#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601`]
/// 
/// * [`crate::misb0601::Misb0601::target_error_estimate_ce90`]
/// * [`crate::misb0601::Misb0601::target_error_estimate_le90`]
/// 
/// This encoder cannot be `const fn` due to floating point arithmetic
pub fn from_error_estimate(input: f32) -> [u8; 2] {
    let output = input * SFT_2_ERROR_ESTIMATE;
    (output as u16).to_be_bytes()
}

#[inline(always)]
#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601::platform_vertical_speed`]
pub fn to_platform_vertical_speed(input: &mut &[u8]) -> winnow::PResult<f32> {
    let value = tinyklv::codecs::binary::dec::be_i16.parse_next(input)?;
    if value as u32 == 0x8000 { return Err(tinyklv::err!()) } // "Out of Range" - keep for backwards compatibility
    Ok((value as f32) * KLV_2_PLATFORM_VERT_SPEED)
}

#[inline(always)]
#[cfg(feature = "misb0601-19")]
/// See [`crate::misb0601::Misb0601::platform_vertical_speed`]
/// 
/// This encoder cannot be `const fn` due to floating point arithmetic
pub fn from_platform_vertical_speed(input: f32) -> [u8; 2] {
    let output = input * SFT_2_PLATFORM_VERT_SPEED;
    (output as i16).to_be_bytes()
}