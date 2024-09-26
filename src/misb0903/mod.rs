// --------------------------------------------------
// tinyklv
// --------------------------------------------------
use tinyklv::Klv;
use tinyklv::prelude::*;

// --------------------------------------------------
// local
// --------------------------------------------------
pub mod ops;
pub mod primitives;
pub use target::Misb0903Target;
pub use ontology::Misb0903Ontology;
pub use algorithm::Misb0903Algorithm;

// --------------------------------------------------
// relative
// --------------------------------------------------
mod target;
mod ontology;
mod algorithm;

#[cfg(any(
    feature = "misb0903-6",
))]
#[derive(Klv, Debug)]
#[klv(
    // ------------------------------------------------
    // confirmed Misb0903 VMTI UL
    //             06 .0E .2B .34 .02 .0B .01 .01 .0E .01 .03 .03 .06 .00 .00 .00
    sentinel = b"\x06\x0E\x2B\x34\x02\x0B\x01\x01\x0E\x01\x03\x03\x06\x00\x00\x00",
    // ------------------------------------------------
    stream = &[u8],
    key(enc = tinyklv::codecs::ber::enc::ber_oid,
        dec = tinyklv::codecs::ber::dec::ber_oid::<u64>),
    len(enc = tinyklv::codecs::ber::enc::ber_length,
        dec = tinyklv::codecs::ber::dec::ber_length),
    default(ty = u8, dyn = true, dec = tinyklv::codecs::binary::dec::be_u8_lengthed),
    default(ty = u16, dyn = true, dec = tinyklv::codecs::binary::dec::be_u16_lengthed),
    default(ty = u32, dyn = true, dec = tinyklv::codecs::binary::dec::be_u32_lengthed),
    default(ty = String, dyn = true, dec = tinyklv::codecs::binary::dec::to_string_utf8),
)]
/// Video Moving Target Indicator Metadata
/// 
/// MISB Standard 0903
/// 
/// For more information, see [Motion Imagery Standards Board (MISB)](https://nsgreg.nga.mil/misb.jsp)
pub struct Misb0903 {
    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x01, dyn = false, dec = tinyklv::codecs::binary::dec::be_u16)]
    /// (Contextual) Detects errors within a standalone VMTI LS
    /// 
    /// `checkSum` -> [`Misb0903::checksum`]
    /// 
    /// The `checkSum` item aids detecting errors in delivery with
    /// standalone-VMTI. Refer to MISB ST 0601 for the checksum algorithm.
    /// Performed over the entire LS, the checksum includes the 16-byte UL
    /// key and 1-byte checksum length. The Value represents the lower
    /// 16-bits of summation.
    /// 
    /// Len: 2
    /// 
    /// Units: None
    pub checksum: Option<u16>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x02, dec = crate::misb0601::ops::to_precision_timestamp)]
    /// (Assumed Optional) Microsecond count from Epoch of 1970
    /// See MISP Time System - MISB ST 0603
    /// 
    /// `precisionTimeStamp` -> [`Misb0903::precision_timestamp`]
    /// 
    /// Defined in MISB ST 0603, the Precision Time Stamp is the number of
    /// microseconds elapsed since the MISP Time System epoch of midnight (00:00:00),
    /// January 1, 1970, and the microsecond count does NOT include leap seconds.
    /// The VMTI LS `precisionTimeStamp` (Item 2) is equal to VMTI-MI-Timestamp.
    /// 
    /// Len: 8
    /// 
    /// Units: Microseconds (μs)
    pub precision_timestamp: Option<chrono::DateTime<chrono::Utc>>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x03)]
    /// (Assumed Optional) Name and/or description of the VMTI system
    /// 
    /// `vmtiSystemName` -> [`Misb0903::vmti_system_name`]
    /// 
    /// The `vmtiSystemName` item is the name or description of the VMTI system
    /// producing the VMTI targets identified as a string of 32 UTF-8 characters.
    /// Note that UTF-8 allows up to four bytes per character; thus, this value
    /// can expand up to 128 bytes maximum. The `vmtiSystemName` is free text.
    /// 
    /// Len: V32
    /// 
    /// Units: None
    pub vmti_system_name: Option<String>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x04)]
    /// (Assumed Optional) Version number of the VMTI Local Set used to
    /// generate the VMTI metadata.
    /// 
    /// `vmtiLsVersionNum` -> [`Misb0903::vmti_ls_version`]
    /// 
    /// The `vmtiLsVersionNum` is the version number of the VMTI LS document
    /// used to generate the VMTI metadata and notifies downstream clients
    /// of the LS version used to encode the VMTI metadata. Values of 1
    /// through 65535 correspond to document revisions 1 through 65535.
    /// 
    /// Len: V2
    /// 
    /// Units: None
    pub vmti_ls_version: Option<u16>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x05)]
    /// (Assumed Optional) Total number of targets in VMTI system's
    /// processing model's target list
    /// 
    /// `totalNumTargetsDetected` -> [`Misb0903::total_num_targets_detected`]
    /// 
    /// The `totalNumTargetsDetected` item is the total number of targets in
    /// the VMTI processing model's target list; this value may be different
    /// than the number of elements in the vTargetSeries. To save bandwidth,
    /// the VMTI system may only report a subset of the VMTI processing
    /// model's target list. Section 6 describes the different scenarios for
    /// generating and reporting target lists. A value of zero represents no
    /// targets detected in the VMTI processing model's list.
    /// 
    /// Len: V3
    /// 
    /// Units: None
    pub total_num_targets_detected: Option<u32>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x06)]
    /// (Mandatory) Number of targets reported following a culling process
    /// 
    /// `numTargetsReported` -> [`Misb0903::num_targets_reported`]
    /// 
    /// The `numTargetsReported` item is the count of a subset of the target list.
    /// Reporting only a subset of the target list improves bandwidth efficiency.
    /// 
    /// Len: V3
    /// 
    /// Units: None
    pub num_targets_reported: u32,

    #[cfg(not(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x07)]
    /// (-) Item deprecated for MISB 0903.6. To implement, please
    /// fill out this section with the appropriate feature flag:
    /// 
    /// ```rust no_run ignore
    /// #[cfg(any(feature = "misb0903-5"))] // do not include `"misb0903-6"`
    /// #[klv(key = 0x07, ...)]
    /// ```
    pub placeholder_key_07: (),

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x08)]
    /// (Assumed Optional) Width of the Motion Imagery frame in pixels
    /// 
    /// `frameWidth` -> [`Misb0903::frame_width`]
    /// 
    /// The `frameWidth` item specifies the width of the
    /// VMTI-MI frame in pixels, which corresponds to the number of pixels
    /// in a row of the image where pixels appear in row-major order. Do not
    /// use a value of zero.
    /// 
    /// Len: V3
    /// 
    /// Units: Pixels
    pub frame_width: Option<u32>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x09)]
    /// (Optional) Height of the Motion Imagery frame in pixels
    /// 
    /// `frameHeight` -> [`Misb0903::frame_height`]
    /// 
    /// The `frameHeight` item specifies the height of the
    /// VMTI-MI frame in pixels, which corresponds to the number of rows of
    /// pixels in the image where pixels appear in row-major order. The
    /// `frameHeight` is not a required value. Do not use a value of zero.
    /// 
    /// Len: V3
    /// 
    /// Units: Pixels
    pub frame_height: Option<u32>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x0A)]
    /// (Assumed Optional) VMTI source sensor (as string). E.g.,
    /// 'EO Nose', 'EO Zoom (DLTV)'
    /// 
    /// `vmtiSourceSensor` -> [`Misb0903::vmti_source_sensor`]
    /// 
    /// The `vmtiSourceSensor` item is a free text identifier for the source of
    /// the VMTI-MI, e.g., 'EO Nose', 'EO Zoom (DLTV)', 'EO Spotter', 'IR Mitsubishi
    /// PtSi Model 500', 'IR InSb Amber Model TBT', 'LYNX SAR Imagery', 'TESAR
    /// Imagery', etc. The `vmtiSourceSensor` identifies the source for systems
    /// where there are multiple sensors. Any change to the VMTI-MI requires updating
    /// this metadata item. The value is a free text string of 128 UTF-8 characters.
    /// UTF-8 allows up to four bytes per character, so this value can expand up to
    /// 512 bytes maximum.
    /// 
    /// Len: V128
    /// 
    /// Units: None
    pub vmti_source_sensor: Option<String>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x0B, dec = ops::to_hvfov)]
    /// (Assumed Optional) Horizontal field of view of imaging sensor input
    /// to VMTI process.
    /// 
    /// `vmtiHorizontalFov` -> [`Misb0903::vmti_hfov`]
    /// 
    /// The `vmtiHorizontalFov` item is the VMTI sensor horizontal field of view (HFOV) of
    /// the source input. ST 0903 requires Item 11 in two cases:
    /// 
    /// 1) standalone-VMTI, or
    /// 2) embedded-VMTI and the VMTI-MI is different from the user-MI.
    /// 
    /// Otherwise, the parent (e.g., ST 0601 LS Item 16) provides the HFOV value.
    /// 
    /// Valid Values: The set of real numbers from 0 to 180 inclusive.
    /// 
    /// Len: 2
    /// 
    /// Units: Degrees (°)
    pub vmti_hfov: Option<f64>,
    
    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x0C, dec = ops::to_hvfov)]
    /// (Assumed Optional) Vertical field of view of imaging sensor input
    /// to VMTI process
    /// 
    /// `vmtiVerticalFov` -> [`Misb0903::vmti_vfov`]
    /// 
    /// The `vmtiVerticalFov` item is the vertical field of view (VFOV) of
    /// the source input. This is a required item in two cases:
    /// 
    /// 1) standalone-VMTI, or
    /// 2) embedded-VMTI and the VMTI-MI is different from the user-MI.
    /// 
    /// Otherwise, the parent (e.g., ST 0601 LS Item 17) provides the VFOV value.
    /// 
    /// Valid Values: The set of real numbers from 0 to 180 inclusive.
    /// 
    /// Len: 2
    /// 
    /// Units: Degrees (°)
    pub vmti_vfov: Option<f64>,

    // #[cfg(any(
    //     feature = "misb0903-6",
    // ))]
    // #[klv(key = 0x0D)]
    // /// (Assumed Optional) A Motion Imagery Identification System (MIIS)
    // /// Core Identifier conformant with MISB ST 1204
    // pub miis_id: Option<Misb1204Miis>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x65, dec = Misb0903Target::decode_vec_vtargets)]
    /// (Mandatory) VTarget Packs ordered as a Series
    /// 
    /// Is "pseudo optional"; if not present, defaults to an empty vector.
    pub v_target_series: Vec<Misb0903Target>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x66, dec = Misb0903Algorithm::repeated)]
    /// (Mandatory) Series of one or more Algorithm LS (Local Set)
    /// 
    /// Is "pseudo optional"; if not present, defaults to an empty vector.
    pub algorithm_series: Vec<Misb0903Algorithm>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x67, dec = Misb0903Ontology::repeated)]
    /// (Mandatory) Series of one or more Ontology LS (Local Set)
    /// 
    /// Is "pseudo optional"; if not present, defaults to an empty vector.
    pub target_series: Vec<Misb0903Ontology>,
}