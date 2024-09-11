// --------------------------------------------------
// tinyklv
// --------------------------------------------------
use tinyklv::Klv;
use tinyklv::prelude::*;
// --------------------------------------------------
// external
// --------------------------------------------------
use thisenum::Const;

// --------------------------------------------------
// local
// --------------------------------------------------
pub mod ops;

#[cfg(any(
    feature = "misb0903-6",
))]
#[derive(Klv, Debug)]
#[klv(
    stream = &[u8],
    sentinel = b"\x06\x0E\x2B\x34\x02\x0B\x01\x01\x0E\x01\x03\x03\x06\x00\x00\x00",
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
    // pub miis_id: Option<String>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x65, dec = Misb0903Target::decode_all_vtargets)]
    /// (Mandatory) VTarget Packs ordered as a Series
    /// 
    /// Is "pseudo optional"; if not present, defaults to an empty vector.
    pub v_target_series: Vec<Misb0903Target>

    // #[cfg(any(
    //     feature = "misb0903-6",
    // ))]
    // #[klv(key = 0x66, dec = Misb0903Algorithm::decode)]
    // /// (Mandatory) Series of one or more Algorithm LS (Local Set)
    // /// 
    // /// Is "pseudo optional"; if not present, defaults to an empty vector.
    // pub algorithm_series: Vec<Misb0903Algorithm>

    // #[cfg(any(
    //     feature = "misb0903-6",
    // ))]
    // #[klv(key = 0x67)]
    // /// (Mandatory) Series of one or more Ontology LS (Local Set)
    // /// 
    // /// Is "pseudo optional"; if not present, defaults to an empty vector.
    // pub target_series: Vec<Misb0601Ontology>
}

#[cfg(any(
    feature = "misb0903-6",
))]
#[derive(Klv, Debug)]
#[klv(
    stream = &[u8],
    sentinel = b"\x06\x0E\x2B\x34\x02\x0B\x01\x01\x0E\x01\x03\x03\x06\x00\x00\x00",
    key(enc = tinyklv::codecs::ber::enc::ber_oid,
        dec = tinyklv::codecs::ber::dec::ber_oid::<u64>),
    len(enc = tinyklv::codecs::ber::enc::ber_length,
        dec = tinyklv::codecs::ber::dec::ber_length),
    default(ty = u8, dec = tinyklv::codecs::binary::dec::be_u8),
    default(ty = u16, dyn = true, dec = tinyklv::codecs::binary::dec::be_u16_lengthed),
    default(ty = u32, dyn = true, dec = tinyklv::codecs::binary::dec::be_u32_lengthed),
    default(ty = u128, dyn = true, dec = tinyklv::codecs::binary::dec::be_u128_lengthed),
    default(ty = String, dyn = true, dec = tinyklv::codecs::binary::dec::to_string_utf8),
    default(ty = PixelPosition, dyn = true, dec = PixelPosition::decode),
)]
pub struct Misb0903Target {
    #[cfg(any(
        feature = "misb0903-6",
    ))]
    /// (Mandatory) Mandatory BER-OID encoded target id and first value
    /// in a VTarget Pack
    /// 
    /// This value does not have a key.
    /// 
    /// Len: V9
    /// 
    /// Units: None
    pub target_id: Option<u128>,
    
    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x01)]
    /// (Assumed Optional) Defines the position of the target within the Motion
    /// Imagery frame as a pixel number
    /// 
    /// The `targetCentroid` item specifies the position of a target centroid
    /// within a frame as a pixel number.
    /// 
    /// Valid values: All integer values from 1 to 0xFFFFFFFFFFFF (281,474,976,710,655).
    /// 
    /// Len: V6
    /// 
    /// Units: Pixel number
    pub target_centroid: Option<PixelPosition>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x02)]
    /// (Assumed Optional) Position of the top left corner of the target's
    /// bounding box within the Motion Imagery frame as a pixel number
    /// 
    /// VTarget Pack Items 2 and 3 define a target’s pixel bounding box with two
    /// numbers. The `boundingBoxTopLeft` item is the position of the top left
    /// corner of a target's pixel bounding box using the pixel number
    /// representation
    ///
    /// Len: V6
    /// 
    /// Units: Pixel number
    pub bbox_tl: Option<PixelPosition>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x03)]
    /// (Assumed Optional) Position of the bottom right corner of the target's
    /// bounding box within the Motion Imagery frame as a pixel number
    /// 
    /// The `boundingBoxBottomRight` item specifies the position of the bottom
    /// right corner of the target's pixel bounding box within the frame using
    /// the pixel number representation
    /// 
    /// Len: V6
    /// 
    /// Units: Pixel number
    pub bbox_br: Option<PixelPosition>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x04)]
    /// (Assumed Optional) Priority or validity of target based on criteria
    /// within the VMTI system
    /// 
    /// The `targetPriority` item provides systems downstream a means to intelligently
    /// cull targets for a given frame as VMTI processors may generate thousands
    /// of hits.
    /// 
    /// 1 - Highest priority
    /// 255 - Lowest priority
    /// 
    /// Valid values: [1, 255]
    ///
    /// Len: 1
    /// 
    /// Units: None
    pub target_priority: Option<u8>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x05)]
    /// (Assumed Optional) Confidence level of target based on criteria within
    /// the VMTI system
    /// 
    /// The `targetConfidenceLevel` item expresses the confidence level as a percentage
    /// based on criteria within the VMTI system. Target(s) with the highest confidence
    /// may not have the highest priority value. The potential is to send the highest
    /// confidence targets in limited bandwidth scenarios. Multiple targets may have
    /// the same confidence level. The range is 0 to 100, where 100 percent is the
    /// highest confidence. A confidence level of 0 percent indicates no confidence that
    /// a detection is a potential target. A target detected with a high confidence may
    /// be a low priority target.
    /// 
    /// Valid values: [0, 100]
    /// 
    /// Len: 1
    /// 
    /// Units: Percentage (%)
    pub target_confidence_level: Option<u8>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x06)]
    /// The `targetHistory` is the number of times (i.e., frames) the system detects the
    /// same target with the same targetId. The `targetHistory` can indicate target
    /// persistence i.e., the number of previous detections of the same target and may
    /// provide useful context when a target reappears after no detection for a significant
    /// time. There is no requirement that detections be in consecutive frames.
    /// 
    /// Valid values: [0, 65535], where 0 denotes the target as a new detection
    /// 
    /// Len: V2
    /// 
    /// Units: Frame number
    pub target_history: Option<u16>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x07)]
    /// (Assumed Optional) Ratio of the target's pixels to the number of pixels in
    /// the target's pixel bounding box (multiplied by 100)
    /// 
    /// The `percentageOfTargetPixels` item specifies the ratio of the target pixels
    /// to the size of the bounding box, multiplied by 100. The range is 1 to 100,
    /// where 100 signifies the target completely fills the bounding box.
    /// 
    /// Valid values: [1, 100]
    /// 
    /// Len: 1
    /// 
    /// Units: Percentage (%)
    pub percentage_of_target_pixels: Option<u8>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x08, dec = Color::decode)]
    /// (Assumed Optional) Dominant color of the target
    /// 
    /// The `targetColor` item is the dominant color of the target expressed using RGB
    /// color values, with general mapping of any multispectral dataset to an RGB value.
    /// VMTI systems may compute the dominant color by any desired method, for example
    /// averaging all the pixels, by bands, in the bounding box. The `targetColor`'s
    /// primary use is when transmitting metadata in the absence of the underlying Motion
    /// Imagery. Represent the RGB color value as: first byte = Red, second byte = Green,
    /// third byte = Blue.
    /// 
    /// Valid values: ([0, 255], [0, 255], [0, 255])
    /// 
    /// Len: 3
    /// 
    /// Units: 8-bit RGB
    pub target_color: Option<Color>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x09)]
    /// (Assumed Optional) Dominant intensity of the target
    /// 
    /// The `targetIntensity` item is the dominant intensity of the target with dynamic range
    /// up to 24 bits. The `targetIntensity` provides a relative measure of how the different
    /// targets compare with each other. The intensity value comes directly from the source
    /// imagery and knowledge of the specific bit-range or status (e.g., gain adjusted) is
    /// unknown. VMTI systems may compute the dominant intensity of a target by any desired
    /// method, for example using the maximum intensity in the target bounding box or averaging
    /// all the intensities in the bounding box.
    /// 
    /// The primary use of the `targetIntensity` is for infrared (IR) systems; for non-IR
    /// systems use the `targetColor` item (Item 8). The intensity value meaning (i.e., White-Hot,
    /// or Black-Hot) is consistent with IR Polarity specified in the parent set (e.g., MISB ST 0601),
    /// if present; if the IR Polarity is unknown, assume White Hot.
    /// 
    /// Primarily, for use when transmitting metadata in the absence of the underlying Motion Imagery
    /// 
    /// Len: V3
    /// 
    /// Units: None
    pub target_intensity: Option<u32>,
    
    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x0A, dec = ops::to_ll_offset)]
    /// (Assumed Optional) Latitude offset for target from frame center latitude (used with
    /// embedded-VMTI)
    /// 
    /// The `targetLocationOffsetLat` item is the latitude offset for the target from the
    /// parent's Frame Center Latitude (e.g., MISB ST 0601 Item 23) based on the WGS84 ellipsoid.
    /// This item has meaning only when embedding the VMTI LS in ST 0601 LS. The `targetLocationOffsetLat`
    /// adds to the Frame Center Latitude to determine the latitude of the target. Both data items
    /// need to be in decimal representation prior to their addition to determine the actual measured
    /// or calculated Motion Imagery target location.
    /// 
    /// The `targetLocationOffsetLat` has a real earth coordinate represented by a latitude-longitude
    /// pair.
    /// 
    /// Valid values: [-19.2, 19.2]
    /// 
    /// Len: 3
    /// 
    /// Units: Degrees (°)
    pub target_location_lat_offset: Option<f64>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x0B, dec = ops::to_ll_offset)]
    /// (Assumed Optional) DESCRIPTION
    /// 
    /// The `targetLocationOffsetLon` item is the longitude offset for the target from
    /// parent's Frame Center Longitude (e.g., MISB ST 0601 - Item 24) based on the WGS84 ellipsoid.
    /// This item has meaning only when embedding the VMTI LS in ST 0601 LS. The `targetLocationOffsetLon`
    /// adds to the Frame Center Longitude to determine the longitude of the target. Both data items
    /// need to be in decimal representation prior to their addition to determine the actual measured
    /// or calculated Motion Imagery target location.
    /// 
    /// The `targetLocationOffsetLon` has a real earth coordinate represented by a latitude-longitude
    /// pair.
    /// 
    /// Valid values: [-19.2, 19.2]
    /// 
    /// Len: 3
    /// 
    /// Units: Degrees (°)
    pub target_location_lon_offset: Option<f64>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x0C, dec = ops::to_hae)]
    /// (Assumed Optional) Height of target in meters above WGS84 Ellipsoid
    /// 
    /// The `targetHae` item is the height of the target expressed as height in meters above the WGS84
    /// ellipsoid (HAE).
    /// 
    /// Valid values: [-900.0, 19_000.0]
    /// 
    /// Len: 2
    /// 
    /// Units: Meters (m)
    pub target_hae: Option<f64>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x0D, dec = ops::to_ll_offset)]
    /// (Assumed Optional) Latitude offset for top left corner of target's geospatial bounding box
    /// 
    /// The `boundingBoxTopLeftLatOffset` item is the latitude offset for the top left corner of target's
    /// geospatial bounding box from the parent’s Frame Center Latitude (e.g., MISB ST 0601 - Item 23)
    /// based on the WGS84 ellipsoid. The `boundingBoxTopLeftLatOffset` adds to the Frame Center Latitude
    /// to determine the latitude of the top left corner of the target's geospatial bounding box. Both
    /// data items need to be in decimal representation prior to their addition to determine the actual
    /// measured or calculated Motion Imagery target location.
    /// 
    /// Valid values: [-19.2, 19.2]
    /// 
    /// Len: 3
    /// 
    /// Units: Degrees (°)
    pub bbox_tl_lat_offset: Option<f64>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x0E, dec = ops::to_ll_offset)]
    /// (Assumed Optional) Longitude offset for top left corner of target's geospatial bounding box
    /// 
    /// The `boundingBoxTopLeftLonOffset` item is the longitude offset for the top left corner of target's
    /// geospatial bounding box from the parent’s Frame Center Longitude (e.g., MISB ST 0601 - Item 24)
    /// based on the WGS84 ellipsoid. The `boundingBoxTopLeftLonOffset` adds to the Frame Center Longitude
    /// to determine the longitude of the top left corner of the target's geospatial bounding box. Both
    /// data items need to be in decimal representation prior to their addition to determine the actual
    /// measured or calculated Motion Imagery target location.
    /// 
    /// Valid values: [-19.2, 19.2]
    /// 
    /// Len: 3
    /// 
    /// Units: Degrees (°)
    pub bbox_tl_lon_offset: Option<f64>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x0F, dec = ops::to_ll_offset)]
    /// (Assumed Optional) Latitude offset for bottom right corner of target's geospatial bounding box
    /// 
    /// The `boundingBoxBottomRightLatOffset` item is the latitude offset for the bottom right corner of target's
    /// geospatial bounding box from the parent’s Frame Center Latitude (e.g., MISB ST 0601 - Item 23)
    /// based on the WGS84 ellipsoid. The `boundingBoxBottomRightLatOffset` adds to the Frame Center Latitude
    /// to determine the latitude of the bottom right corner of the target's geospatial bounding box. Both
    /// data items need to be in decimal representation prior to their addition to determine the actual
    /// measured or calculated Motion Imagery target location.
    /// 
    /// Valid values: [-19.2, 19.2]
    /// 
    /// Len: 3
    /// 
    /// Units: Degrees (°)
    pub bbox_br_lat_offset: Option<f64>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x10, dec = ops::to_ll_offset)]
    /// (Assumed Optional) Longitude offset for bottom right corner of target's geospatial bounding box
    /// 
    /// The `boundingBoxBottomRightLonOffset` item is the longitude offset for the bottom right corner of target's
    /// geospatial bounding box from the parent’s Frame Center Longitude (e.g., MISB ST 0601 - Item 24)
    /// based on the WGS84 ellipsoid. The `boundingBoxBottomRightLonOffset` adds to the Frame Center Longitude
    /// to determine the longitude of the bottom right corner of the target's geospatial bounding box. Both
    /// data items need to be in decimal representation prior to their addition to determine the actual
    /// measured or calculated Motion Imagery target location.
    /// 
    /// Valid values: [-19.2, 19.2]
    /// 
    /// Len: 3
    /// 
    /// Units: Degrees (°)
    pub bbox_br_lon_offset: Option<f64>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x11, dyn = true, dec = Location::decode)]
    /// (Assumed Optional) Location of the target (latitude, longitude, & height above WGS84 Ellipsoid),
    /// with sigma and rho values
    /// 
    /// The `targetLocation` item provides detailed geo-positioning information for a target, optionally including
    /// the standard deviation and correlation coefficients. This item is of type [`Location`] which is a Defined
    /// Length Truncation Pack. To specify the geographic coordinates for a target with standalone-VMTI, use
    /// `targetLocation` in lieu of VTarget Pack - Item 10 `targetLocationOffsetLat` and Item 11 `targetLocationOffsetLat`.
    /// However, even when using embedded-VMTI, `targetLocation` is preferred vice offset calculations.
    pub target_location: Option<Location>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x12, dec = Location::repeated)]
    /// (Assumed Optional) Geospatial boundary encompassing the target
    /// 
    /// The `geospatialContourSeries` item is of type [`BoundarySeries`], which provides detailed geopositioning
    /// information for the contour around the target. An arbitrary number of vertices defines the contour.
    /// Each vertex is an element of type [`Location`]. The [`Location`] type captures geopositioning data about
    /// a specific location on or near the surface of the Earth. Typical geospatial contours include boxes defined
    /// by two or four vertices, although other contours are possible. Use a `geospatialContourSeries` instead of
    /// a target's geospatial bounding box (Items 13 through 16) when accuracy and correlation information is
    /// available and needed. Such information aids fusion with other moving object indicators, such as, radar
    /// based GMTI, to support track identification and tracking.
    pub geospatial_contour_series: Option<Vec<Location>>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x13)]
    /// (Assumed Optional) Specifies the row in pixels of the target centroid within the Motion Imagery frame
    /// 
    /// The `centroidPixRow` item specifies the row of the target centroid within the Motion Imagery
    /// frame in pixels. Numbering commences from 1, denoting the top row. The `centroidPixRow` may
    /// be used with VTarget Pack `centroidPixCol` - Item 20 to provide an alternate method to specify
    /// VTarget Pack `targetCentroid` – Item 1, the pixel location of the target centroid. If present, the
    /// `centroidPixCol` - Item 20 must also be present.
    /// 
    /// Valid values: [1, 2^32 - 1]
    /// 
    /// Len: V4
    /// 
    /// Units: None
    pub centroid_rows: Option<u32>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x14)]
    /// (Assumed Optional) Specifies the column in pixels of the target centroid within the Motion Imagery frame
    /// 
    /// The `centroidPixCol` item specifies the column of the target centroid within the Motion Imagery
    /// frame in pixels. Numbering commences from 1, denoting the left column. May be used with
    /// VTarget Pack `centroidPixRow` - Item 19 to provide an alternate method to specify VTarget Pack
    /// targetCentroid – Item 1, the pixel location of the target centroid. If present, the `centroidPixRow` -
    /// Item 19 must also be present.
    /// 
    /// Valid values: [1, 2^32 - 1]
    /// 
    /// Len: V4
    /// 
    /// Units: None
    pub centroid_cols: Option<u32>,

    #[cfg(not(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x15)]
    /// (-) Item deprecated for MISB 0903.6. To implement, please
    /// fill out this section with the appropriate feature flag:
    /// 
    /// ```rust no_run ignore
    /// #[cfg(any(feature = "misb0903-5"))] // do not include `"misb0903-6"`
    /// #[klv(key = 0x15, ...)]
    /// ```
    pub placeholder_key_15: (),

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x16)]
    /// (Assumed Optional) Identifier indicating which algorithm in Algorithm Series detected
    /// this target
    /// 
    /// The `algorithmId` item refers to one of the algorithm ids in the VMTI LS `algorithmSeries` Item
    /// 102, which lists all the algorithms a VMTI LS uses. Each algorithm in the series includes an
    /// identifier (`algorithmId`). The `algorithmId` value equals one of the Id values in the
    /// `algorithmSeries`.
    /// 
    /// Len: V3 
    /// 
    /// Units: None
    pub algorithm_id: Option<u32>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x17, dec = DetectionStatus::decode)]
    /// (Assumed Optional) Enumeration indicating the current state of VMTI detections for
    /// a given entity (Inactive, ActiveMoving, Dropped, Active-Stopped, Active-Coasting)
    /// 
    /// The `detectionStatus` item allows assigning a target a status in detection.
    pub detection_status: Option<DetectionStatus>,

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x65)]
    /// (Assumed Optional) DESCRIPTION
    /// 
    /// LONG_DESCRIPTION
    /// 
    /// Valid values: 
    /// 
    /// Len:
    /// 
    /// Units:
    pub v_mask: (),

    #[cfg(not(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x66)]
    /// (-) Item deprecated for MISB 0903.6. To implement, please
    /// fill out this section with the appropriate feature flag:
    /// 
    /// ```rust no_run ignore
    /// #[cfg(any(feature = "misb0903-5"))] // do not include `"misb0903-6"`
    /// #[klv(key = 0x66, ...)]
    /// ```
    pub placeholder_key_66: (),

    #[cfg(not(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x67)]
    /// (-) Item deprecated for MISB 0903.6. To implement, please
    /// fill out this section with the appropriate feature flag:
    /// 
    /// ```rust no_run ignore
    /// #[cfg(any(feature = "misb0903-5"))] // do not include `"misb0903-6"`
    /// #[klv(key = 0x67, ...)]
    /// ```
    pub placeholder_key_67: (),

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x68)]
    /// (Assumed Optional) DESCRIPTION
    /// 
    /// LONG_DESCRIPTION
    pub v_tracker: (),

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x69)]
    /// (Assumed Optional) DESCRIPTION
    /// 
    /// LONG_DESCRIPTION
    pub v_chip: (),

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x6A)]
    /// (Assumed Optional) DESCRIPTION
    /// 
    /// LONG_DESCRIPTION
    pub v_chip_series: (),

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x6B)]
    /// (Assumed Optional) DESCRIPTION
    /// 
    /// LONG_DESCRIPTION
    pub v_object_series: (),
}
impl Misb0903Target {
    pub fn decode_all_vtargets(input: &mut &[u8]) -> winnow::PResult<Vec<Self>> {
        todo!()
    }

    /// For MISB 0903, the target id is the first item in the VTarget Pack
    /// and is not preceded by a key.
    /// 
    /// Meaning, when the key for the [`Misb0903Target`], specified
    /// by the `key` field in [`Misb0903`] (`0x65`), is located within the
    /// input stream, then the length of the entire VTarget Pack is returned.
    /// Intuitively, each element in the VTarget Pack will be a series of
    /// keys and values. However, this is not the case for the first value:
    /// [`Misb0903Target::target_id`], which is not preceded by a key.
    /// 
    /// See the standard documentation for more details.
    pub fn decode_vtarget_item(input: &mut &[u8]) -> winnow::PResult<Self> {
        // let target_id = tinyklv::codecs::ber::dec::ber_oid::<u128>.parse_next(input).ok();
        // let mut output = Self::decode.parse_next(input)?;
        // output.target_id = target_id;
        // Ok(output)
        todo!()
    }
}

// pub struct Misb0903Algorithm {}
// pub struct Misb0601Ontology {}

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
/// * [`Misb0903Target::bbox_tl`]
/// * [`Misb0903Target::bbox_br`]
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
    fn decode(input: &mut &[u8], len: usize) -> winnow::PResult<Self> {
        Ok(PixelPosition::new(tinyklv::binary::dec::be_u32_lengthed(input, len)?))
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
    latitude: f64,
    longitude: f64,
    hae: f64,
    measurements: EnuMeasurements,
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
/// 
/// See: [`crate::misb0903::Motion`]
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
    fn decode_velocity(input: &mut &[u8]) -> winnow::PResult<Self> {
        Ok(Motion::Velocity(MotionValues::decode(input)?))
    }
    /// Decodes an acceleration
    /// 
    /// See: [`crate::misb0903::Motion`]
    fn decode_acceleration(input: &mut &[u8]) -> winnow::PResult<Self> {
        Ok(Motion::Acceleration(MotionValues::decode(input)?))
    }
}

#[derive(Debug)]
/// Motion values, which describes any N'th order derivative
/// of position
/// 
/// See: [`crate::misb0903::Motion`]
pub struct MotionValues {
    east: f64,
    north: f64,
    up: f64,
    measurements: EnuMeasurements,
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
    sig_east: f64,
    sig_north: f64,
    sig_up: f64,
    rho_east_north: f64,
    rho_east_up: f64,
    rho_north_up: f64,
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
impl tinyklv::prelude::Encode<Vec<u8>> for DetectionStatus {
    fn encode(&self) -> Vec<u8> {
        return vec![*self.value()]
    }
}