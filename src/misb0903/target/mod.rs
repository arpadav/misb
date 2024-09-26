// --------------------------------------------------
// tinyklv
// --------------------------------------------------
use tinyklv::Klv;
use tinyklv::prelude::*;

// --------------------------------------------------
// local
// --------------------------------------------------
use crate::misb0903::ops;
use crate::misb0903::primitives::*;

// --------------------------------------------------
// relative
// --------------------------------------------------
mod mask;
mod object;
pub use object::Misb0903Object;

#[cfg(any(
    feature = "misb0903-6",
))]
#[derive(Klv, Debug)]
#[klv(
    stream = &[u8],
    key(enc = tinyklv::codecs::ber::enc::ber_oid,
        dec = tinyklv::codecs::ber::dec::ber_oid::<u64>),
    len(enc = tinyklv::codecs::ber::enc::ber_length,
        dec = tinyklv::codecs::ber::dec::ber_length),
    default(ty = u8, dec = tinyklv::codecs::binary::dec::be_u8),
    default(ty = u16, dyn = true, dec = tinyklv::codecs::binary::dec::be_u16_lengthed),
    default(ty = u32, dyn = true, dec = tinyklv::codecs::binary::dec::be_u32_lengthed),
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
    /// ***This value does not have a key.***
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
    #[klv(key = 0x11, dec = Location::decode)]
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
    /// The `geospatialContourSeries` item is of type `BoundarySeries`, which provides detailed geopositioning
    /// information for the contour around the target. An arbitrary number of vertices defines the contour.
    /// Each vertex is an element of type [`Location`]. The [`Location`] type captures geopositioning data about
    /// a specific location on or near the surface of the Earth. Typical geospatial contours include boxes defined
    /// by two or four vertices, although other contours are possible.
    /// 
    /// Use a `geospatialContourSeries` instead of
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

    // #[cfg(any(
    //     feature = "misb0903-6",
    // ))]
    // #[klv(key = 0x65)]
    // /// (Assumed Optional) Local Set to include a mask for delineating the perimeter
    // /// of the target
    // /// 
    // /// LONG_DESCRIPTION
    // pub v_mask: (),

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

    // #[cfg(any(
    //     feature = "misb0903-6",
    // ))]
    // #[klv(key = 0x68)]
    // /// (Assumed Optional) DESCRIPTION
    // /// 
    // /// LONG_DESCRIPTION
    // pub v_tracker: (),

    // #[cfg(any(
    //     feature = "misb0903-6",
    // ))]
    // #[klv(key = 0x69)]
    // /// (Assumed Optional) DESCRIPTION
    // /// 
    // /// LONG_DESCRIPTION
    // pub v_chip: (),

    // #[cfg(any(
    //     feature = "misb0903-6",
    // ))]
    // #[klv(key = 0x6A)]
    // /// (Assumed Optional) DESCRIPTION
    // /// 
    // /// LONG_DESCRIPTION
    // pub v_chip_series: (),

    #[cfg(any(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x6B, dec = Misb0903Object::repeated)]
    /// (Mandatory) Series of one or more VObject LS
    /// 
    /// The `vObjectSeries` item is a Series (see Figure 18) of one or more VObject LS associated with
    /// a specific target.
    /// 
    /// Is "pseudo optional"; if not present, defaults to an empty vector.
    pub v_object_series: Vec<Misb0903Object>,
}
impl Misb0903Target {
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
    pub fn decode_vtarget(input: &mut &[u8]) -> winnow::PResult<Self> {
        let target_id = tinyklv::codecs::ber::dec::ber_oid::<u128>.parse_next(input).ok();
        let mut output = Self::decode.parse_next(input)?;
        output.target_id = target_id;
        Ok(output)
    }

    /// 
    pub fn decode_vec_vtargets(input: &mut &[u8]) -> winnow::PResult<Vec<Self>> {
        winnow::combinator::repeat(0.., Self::decode_vtarget).parse_next(input)
    }
}

// pub struct Misb0903Algorithm {}
// pub struct Misb0601Ontology {}

// pub struct VTracker {
//     pub track_id: Option<String>,
//     pub placeholder_key_02: (),
//     // pub first_observation_time: Option<DateTime<Utc>>,
//     // pub last_observation_time: Option<DateTime<Utc>>,
//     pub track_boundary_series: Option<Vec<Location>>,
//     pub placeholder_key_06: (),
//     pub confidence_level: Option<u8>,
//     pub placeholder_key_08: (),
//     pub track_history_series: Option<Vec<()>>,
//     pub velocity: Option<Motion>,
//     pub acceleration: Option<Motion>,
//     pub algorithm_id: Option<u32>,
// }