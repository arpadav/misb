// --------------------------------------------------
// tinyklv
// --------------------------------------------------
use tinyklv::prelude::*;

// --------------------------------------------------
// static
// --------------------------------------------------
use std::sync::LazyLock;

use crate::misb1201::ImapB;

/// 2 byte-precision in range [0, 180]
/// 
/// Used for some angles
/// 
/// Units: Degrees (°)
pub(crate) static IMAPB_0_180_2_F64: LazyLock<crate::misb1201::ImapB<f64>> = LazyLock::new(|| {
    crate::misb1201::ImapB::new(0.0, 180.0, 2).unwrap()
});

/// 3 byte-precision in range [-19.2, 19.2]
/// 
/// Used for latitude / longitude offsets
/// 
/// Units: Degrees (°)
pub(crate) static IMAPB_N19P2_19P2_3_F64: LazyLock<crate::misb1201::ImapB<f64>> = LazyLock::new(|| {
    crate::misb1201::ImapB::new(-19.2, 19.2, 3).unwrap()
});

/// 4 byte-precision in range [-90.0, 90.0]
/// 
/// Used for latitude
/// 
/// Units: Degrees (°)
pub(crate) static IMAPB_N90_90_4_F64: LazyLock<crate::misb1201::ImapB<f64>> = LazyLock::new(|| {
    crate::misb1201::ImapB::new(-90.0, 90.0, 4).unwrap()
});

/// 4 byte-precision in range [-180.0, 180.0]
/// 
/// Used for longitude
/// 
/// Units: Degrees (°)
pub(crate) static IMAPB_N180_180_4_F64: LazyLock<crate::misb1201::ImapB<f64>> = LazyLock::new(|| {
    crate::misb1201::ImapB::new(-180.0, 180.0, 4).unwrap()
});

/// 2 byte-precision in range [-900, 900]
/// 
/// Used for velocity / acceleration
/// 
/// Units: Meters per second (m/s) or Meters per second squared (m/s2)
pub(crate) static IMAPB_N900_900_2_F64: LazyLock<crate::misb1201::ImapB<f64>> = LazyLock::new(|| {
    crate::misb1201::ImapB::new(-900.0, 900.0, 2).unwrap()
});

/// 2-byte precision in range [0, 650]
/// 
/// Used for standard-deviation of position
/// 
/// Units: Meters (m)
pub(crate) static IMAPB_0_650_2_F64: LazyLock<crate::misb1201::ImapB<f64>> = LazyLock::new(|| {
    crate::misb1201::ImapB::new(0.0, 650.0, 2).unwrap()
});

/// 2 byte-precision in range [-1, 1]
/// 
/// Used for correlation-coefficients
/// 
/// Units: None
pub(crate) static IMAPB_N1_1_2_F64: LazyLock<crate::misb1201::ImapB<f64>> = LazyLock::new(|| {
    crate::misb1201::ImapB::new(-1.0, 1.0, 2).unwrap()
});

/// 2 byte-precision in range [-900, 19_000]
/// 
/// Used for HAE (height above ellipsoid)
/// 
/// Units: Meters (m)
pub(crate) static IMAPB_N900_19K_2_F64: LazyLock<crate::misb1201::ImapB<f64>> = LazyLock::new(|| {
    crate::misb1201::ImapB::new(-900.0, 19_000.0, 2).unwrap()
});

/// General parser wrapper for [`crate::misb1201::ImapB`]
pub fn imapb_parser<T: crate::misb1201::ImapFloat + 'static> (
    imap: &'static crate::misb1201::ImapB<T>,
    len: usize,
) -> impl Fn(&mut &[u8]) -> winnow::PResult<T> {
    move |input: &mut &[u8]| {
        let checkpoint = input.checkpoint();
        let value = winnow::token::take(len).parse_next(input)?;
        imap.from_imap(value).map_err(|e| tinyklv::err!().add_context(input, &checkpoint, e.into()))
    }
}

/// See: [`crate::misb0903::Misb0903`]
/// 
/// * [`crate::misb0903::Misb0903::vmti_hfov`]
/// * [`crate::misb0903::Misb0903::vmti_vfov`]
pub fn to_hvfov(input: &mut &[u8]) -> winnow::PResult<f64> {
    imapb_parser(&IMAPB_0_180_2_F64, 2).parse_next(input)
}

/// See: [`crate::misb0903::Misb0903`]
/// 
/// * [`crate::misb0903::Misb0903Target::target_location_lat_offset`]
/// * [`crate::misb0903::Misb0903Target::target_location_lon_offset`]
/// * [`crate::misb0903::Misb0903Target::bbox_tl_lat_offset`]
/// * [`crate::misb0903::Misb0903Target::bbox_tl_lon_offset`]
/// * [`crate::misb0903::Misb0903Target::bbox_br_lat_offset`]
/// * [`crate::misb0903::Misb0903Target::bbox_br_lon_offset`]
pub fn to_ll_offset(input: &mut &[u8]) -> winnow::PResult<f64> {
    imapb_parser(&IMAPB_N19P2_19P2_3_F64, 3).parse_next(input)
}

/// See: [`crate::misb0903::Misb0903`]
/// 
/// * [`crate::misb0903::Misb0903Target::target_hae`]
pub fn to_hae(input: &mut &[u8]) -> winnow::PResult<f64> {
    imapb_parser(&IMAPB_N900_19K_2_F64, 2).parse_next(input)
}

/// A dynamically sized [`crate::misb1201::ImapB`] parser for confidence
/// values of length `len`, in the range [0, 100]
pub fn to_confidence(len: usize) -> impl Fn(&mut &[u8]) -> winnow::PResult<f64> {
    move |input: &mut &[u8]| {
        let checkpoint = input.checkpoint();
        let imap = match ImapB::new(0.0_f64, 100.0_f64, len) {
            Ok(x) => x,
            Err(e) => return Err(tinyklv::err!().add_context(input, &checkpoint, e.into())),
        };
        let value = winnow::token::take(len).parse_next(input)?;
        let res = imap.from_imap(value).map_err(|e| tinyklv::err!().add_context(input, &checkpoint, e.into()));
        res
    }
}