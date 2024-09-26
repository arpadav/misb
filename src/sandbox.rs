mod module_name {
    use tinyklv::Klv;
    use tinyklv::prelude::*;
    use crate::misb0903::ops;
    use crate::misb0903::primitives::*;
    mod mask {}
    mod object {
        use tinyklv::Klv;
        use tinyklv::prelude::*;
        use crate::misb0903::ops;
        #[cfg(any(feature = "misb0903-6"))]
        // #[klv(
        //     stream = &[u8],
        //     key(
        //         enc = tinyklv::codecs::binary::enc::u8,
        //         dec = tinyklv::codecs::binary::dec::u8
        //     ),
        //     len(
        //         enc = tinyklv::codecs::ber::enc::ber_length,
        //         dec = tinyklv::codecs::ber::dec::ber_length
        //     ),
        // )]
        pub struct VObject {
            // #[klv(
            //     key = 0x03,
            //     dyn = true,
            //     dec = tinyklv::codecs::binary::dec::be_u64_lengthed
            // )]
            /// (Assumed Mandatory) Identifier indicating which Ontology in the VMTI's Ontology Series represents
            /// this object
            ///
            /// The `ontologyId` is a reference to one of the ontologies in the VMTI LS ontologySeries. Each item
            /// in the ontologySeries defines either an object label or feature label, see Section 7.3. Each
            /// ontology in the ontologySeries includes an identifier (`ontologyId`) so the value of the VObject LS
            /// `ontologyId` is equal to one of the `ontologyId` values in the ontologySeries. Using the `ontologyId`
            /// saves bandwidth by not duplicating the same information for different VObjects.
            ///
            /// Len: V8
            pub ontology_id: u64,
            // #[klv(key = 0x04, dyn = true, dec = ops::to_confidence)]
            /// (Assumed Mandatory) The amount of confidence in the label of this object
            ///
            /// The confidence item is the measure of "trust" in the labeling of this vObject, ranging from 0.0%
            /// to 100.0%. For example, an object-classifier analyzes a blob of pixels and labels the blob as
            /// representing a vehicle. If the blob closely matches the classifiers criteria for a vehicle the
            /// confidence in the labeling is high (towards 100.0%); alternatively, if the classifier is less sure of
            /// its label the confidence is low (towards 0%). The confidence value is IMAPB(0.0, 100.0, length)
            /// with the length defined by the tag's length value. Increasing the length provides more accuracy.
            /// The minimum length is 1.
            ///
            /// Len: V3
            ///
            /// Units: Percent (%)
            pub confidence: f64,
            // #[klv(key = 0x05, dec = VFeature::repeated)]
            /// (Mandatory) One or more VFeature LS associated with a specific VObject
            ///
            /// Is "pseudo optional"; if not present, defaults to an empty vector.
            pub v_feature_series: Vec<VFeature>,
        }
        #[automatically_derived]
        /// [`VObject`] implementation of [`tinyklv::prelude::Decode`] for [`& [u8]`]
        impl ::tinyklv::prelude::Decode<&[u8]> for VObject {
            fn decode(input: &mut &[u8]) -> ::tinyklv::reexport::winnow::PResult<Self> {
                let checkpoint = input.checkpoint();
                let mut ontology_id: Option<u64> = None;
                let mut confidence: Option<f64> = None;
                let mut v_feature_series: Option<Vec<VFeature>> = None;
                loop {
                    match (
                        tinyklv::codecs::binary::dec::u8,
                        tinyklv::codecs::ber::dec::ber_length,
                    )
                        .parse_next(input)
                    {
                        Ok((key, len)) => {
                            match (key, len) {
                                (0x03, len) => {
                                    ontology_id = tinyklv::codecs::binary::dec::be_u64_lengthed(
                                            len,
                                        )(input)
                                        .ok();
                                }
                                (0x04, len) => {
                                    confidence = ops::to_confidence(len)(input).ok();
                                }
                                (0x05, _) => {
                                    v_feature_series = VFeature::repeated(input).ok();
                                }
                                (_, len) => {
                                    let _ = ::tinyklv::reexport::winnow::token::take::<
                                        usize,
                                        &[u8],
                                        ::tinyklv::reexport::winnow::error::ContextError,
                                    >(len)
                                        .parse_next(input);
                                }
                            }
                        }
                        Err(_) => break,
                    }
                }
                Ok(VObject {
                    ontology_id: ontology_id
                        .ok_or_else(|| {
                            ::tinyklv::reexport::winnow::error::ErrMode::Backtrack(
                                ::tinyklv::reexport::winnow::error::ContextError::new()
                                    .add_context(
                                        input,
                                        &checkpoint,
                                        ::tinyklv::reexport::winnow::error::StrContext::Label(
                                            "`ontology_id` is a required value missing from the `VObject` packet. To prevent this, set this field as optional.",
                                        ),
                                    ),
                            )
                        })?,
                    confidence: confidence
                        .ok_or_else(|| {
                            ::tinyklv::reexport::winnow::error::ErrMode::Backtrack(
                                ::tinyklv::reexport::winnow::error::ContextError::new()
                                    .add_context(
                                        input,
                                        &checkpoint,
                                        ::tinyklv::reexport::winnow::error::StrContext::Label(
                                            "`confidence` is a required value missing from the `VObject` packet. To prevent this, set this field as optional.",
                                        ),
                                    ),
                            )
                        })?,
                    v_feature_series: v_feature_series
                        .ok_or_else(|| {
                            ::tinyklv::reexport::winnow::error::ErrMode::Backtrack(
                                ::tinyklv::reexport::winnow::error::ContextError::new()
                                    .add_context(
                                        input,
                                        &checkpoint,
                                        ::tinyklv::reexport::winnow::error::StrContext::Label(
                                            "`v_feature_series` is a required value missing from the `VObject` packet. To prevent this, set this field as optional.",
                                        ),
                                    ),
                            )
                        })?,
                })
            }
        }
        // #[automatically_derived]
        // impl ::core::fmt::Debug for VObject {
        //     #[inline]
        //     fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        //         ::core::fmt::Formatter::debug_struct_field3_finish(
        //             f,
        //             "VObject",
        //             "ontology_id",
        //             &self.ontology_id,
        //             "confidence",
        //             &self.confidence,
        //             "v_feature_series",
        //             &&self.v_feature_series,
        //         )
        //     }
        // }
        #[cfg(any(feature = "misb0903-6"))]
        // #[klv(
        //     stream = &[u8],
        //     key(
        //         enc = tinyklv::codecs::binary::enc::u8,
        //         dec = tinyklv::codecs::binary::dec::u8
        //     ),
        //     len(
        //         enc = tinyklv::codecs::ber::enc::ber_length,
        //         dec = tinyklv::codecs::ber::dec::ber_length
        //     ),
        // )]
        pub struct VFeature {
            // #[klv(
            //     key = 0x03,
            //     dyn = true,
            //     dec = tinyklv::codecs::binary::dec::be_u64_lengthed
            // )]
            /// (Assumed Mandatory) Identifier indicating which ontology in the VMTI LS's `ontologySeries`
            /// represents this feature
            ///
            /// The `ontologyId` is a reference to one of the ontologies in the VMTI LS `ontologySeries`. Each
            /// ontology in the `ontologySeries` includes an identifier (Id) so the value of the VFeature LS
            /// `ontologyId` is equal to one of the id values in the `ontologySeries`. Using the `ontologyId` saves
            /// bandwidth by not duplicating the same information for different VFeatures.
            ///
            /// Len: V8
            ///
            /// Units: None
            pub ontology_id: u64,
            // #[klv(key = 0x04, dyn = true, dec = ops::to_confidence)]
            /// (Assumed Mandatory) The amount of confidence in the feature's label.
            ///
            /// The confidence item is the measure of "trust" in the feature-classification of this vFeature from
            /// 0.0% to 100.0%. For example, a feature-classifier analyzes an object and "classifies" a subset of
            /// the object's pixels as an antenna. If the pixels closely match the classifier's criteria for an
            /// antenna the confidence in the feature-classification is high (towards 100.0%); alternatively, if the
            /// classifier is less sure of its feature-classification the confidence is low (towards 0%). The
            /// confidence value is IMAPB(0.0, 100.0, length) with the length defined by the item's length
            /// value. Increasing the length provides more accuracy. The minimum length is 1.
            ///
            /// Len: V3
            ///
            /// Units: Percent (%)
            pub confidence: f64,
        }
        #[automatically_derived]
        /// [`VFeature`] implementation of [`tinyklv::prelude::Decode`] for [`& [u8]`]
        impl ::tinyklv::prelude::Decode<&[u8]> for VFeature {
            fn decode(input: &mut &[u8]) -> ::tinyklv::reexport::winnow::PResult<Self> {
                let checkpoint = input.checkpoint();
                let mut ontology_id: Option<u64> = None;
                let mut confidence: Option<f64> = None;
                loop {
                    match (
                        tinyklv::codecs::binary::dec::u8,
                        tinyklv::codecs::ber::dec::ber_length,
                    )
                        .parse_next(input)
                    {
                        Ok((key, len)) => {
                            match (key, len) {
                                (0x03, len) => {
                                    ontology_id = tinyklv::codecs::binary::dec::be_u64_lengthed(
                                            len,
                                        )(input)
                                        .ok();
                                }
                                (0x04, len) => {
                                    confidence = ops::to_confidence(len)(input).ok();
                                }
                                (_, len) => {
                                    let _ = ::tinyklv::reexport::winnow::token::take::<
                                        usize,
                                        &[u8],
                                        ::tinyklv::reexport::winnow::error::ContextError,
                                    >(len)
                                        .parse_next(input);
                                }
                            }
                        }
                        Err(_) => break,
                    }
                }
                Ok(VFeature {
                    ontology_id: ontology_id
                        .ok_or_else(|| {
                            ::tinyklv::reexport::winnow::error::ErrMode::Backtrack(
                                ::tinyklv::reexport::winnow::error::ContextError::new()
                                    .add_context(
                                        input,
                                        &checkpoint,
                                        ::tinyklv::reexport::winnow::error::StrContext::Label(
                                            "`ontology_id` is a required value missing from the `VFeature` packet. To prevent this, set this field as optional.",
                                        ),
                                    ),
                            )
                        })?,
                    confidence: confidence
                        .ok_or_else(|| {
                            ::tinyklv::reexport::winnow::error::ErrMode::Backtrack(
                                ::tinyklv::reexport::winnow::error::ContextError::new()
                                    .add_context(
                                        input,
                                        &checkpoint,
                                        ::tinyklv::reexport::winnow::error::StrContext::Label(
                                            "`confidence` is a required value missing from the `VFeature` packet. To prevent this, set this field as optional.",
                                        ),
                                    ),
                            )
                        })?,
                })
            }
        }
        // #[automatically_derived]
        // impl ::core::fmt::Debug for VFeature {
        //     #[inline]
        //     fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        //         ::core::fmt::Formatter::debug_struct_field2_finish(
        //             f,
        //             "VFeature",
        //             "ontology_id",
        //             &self.ontology_id,
        //             "confidence",
        //             &&self.confidence,
        //         )
        //     }
        // }
    }
    pub use object::VObject;
    #[cfg(any(feature = "misb0903-6"))]
    // #[klv(
    //     stream = &[u8],
    //     sentinel = b"\x06\x0E\x2B\x34\x02\x0B\x01\x01\x0E\x01\x03\x03\x06\x00\x00\x00",
    //     key(
    //         enc = tinyklv::codecs::ber::enc::ber_oid,
    //         dec = tinyklv::codecs::ber::dec::ber_oid::<u64>
    //     ),
    //     len(
    //         enc = tinyklv::codecs::ber::enc::ber_length,
    //         dec = tinyklv::codecs::ber::dec::ber_length
    //     ),
    //     default(ty = u8, dec = tinyklv::codecs::binary::dec::be_u8),
    //     default(
    //         ty = u16,
    //         dyn = true,
    //         dec = tinyklv::codecs::binary::dec::be_u16_lengthed
    //     ),
    //     default(
    //         ty = u32,
    //         dyn = true,
    //         dec = tinyklv::codecs::binary::dec::be_u32_lengthed
    //     ),
    //     default(
    //         ty = u128,
    //         dyn = true,
    //         dec = tinyklv::codecs::binary::dec::be_u128_lengthed
    //     ),
    //     default(
    //         ty = String,
    //         dyn = true,
    //         dec = tinyklv::codecs::binary::dec::to_string_utf8
    //     ),
    //     default(ty = PixelPosition, dyn = true, dec = PixelPosition::decode),
    // )]
    pub struct Misb0903Target {
        #[cfg(any(feature = "misb0903-6"))]
        /// (Mandatory) Mandatory BER-OID encoded target id and first value
        /// in a VTarget Pack
        ///
        /// This value does not have a key.
        ///
        /// Len: V9
        ///
        /// Units: None
        pub target_id: Option<u128>,
        #[cfg(any(feature = "misb0903-6"))]
        // #[klv(key = 0x01)]
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
        #[cfg(any(feature = "misb0903-6"))]
        // #[klv(key = 0x02)]
        /// (Assumed Optional) Position of the top left corner of the target's
        /// bounding box within the Motion Imagery frame as a pixel number
        ///
        /// VTarget Pack Items 2 and 3 define a target's pixel bounding box with two
        /// numbers. The `boundingBoxTopLeft` item is the position of the top left
        /// corner of a target's pixel bounding box using the pixel number
        /// representation
        ///
        /// Len: V6
        ///
        /// Units: Pixel number
        pub bbox_tl: Option<PixelPosition>,
        #[cfg(any(feature = "misb0903-6"))]
        // #[klv(key = 0x03)]
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
        #[cfg(any(feature = "misb0903-6"))]
        // #[klv(key = 0x04)]
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
        #[cfg(any(feature = "misb0903-6"))]
        // #[klv(key = 0x05)]
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
        #[cfg(any(feature = "misb0903-6"))]
        // #[klv(key = 0x06)]
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
        #[cfg(any(feature = "misb0903-6"))]
        // #[klv(key = 0x07)]
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
        #[cfg(any(feature = "misb0903-6"))]
        // #[klv(key = 0x08, dec = Color::decode)]
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
        #[cfg(any(feature = "misb0903-6"))]
        // #[klv(key = 0x09)]
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
        #[cfg(any(feature = "misb0903-6"))]
        // #[klv(key = 0x0A, dec = ops::to_ll_offset)]
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
        #[cfg(any(feature = "misb0903-6"))]
        // #[klv(key = 0x0B, dec = ops::to_ll_offset)]
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
        #[cfg(any(feature = "misb0903-6"))]
        // #[klv(key = 0x0C, dec = ops::to_hae)]
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
        #[cfg(any(feature = "misb0903-6"))]
        // #[klv(key = 0x0D, dec = ops::to_ll_offset)]
        /// (Assumed Optional) Latitude offset for top left corner of target's geospatial bounding box
        ///
        /// The `boundingBoxTopLeftLatOffset` item is the latitude offset for the top left corner of target's
        /// geospatial bounding box from the parent's Frame Center Latitude (e.g., MISB ST 0601 - Item 23)
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
        #[cfg(any(feature = "misb0903-6"))]
        // #[klv(key = 0x0E, dec = ops::to_ll_offset)]
        /// (Assumed Optional) Longitude offset for top left corner of target's geospatial bounding box
        ///
        /// The `boundingBoxTopLeftLonOffset` item is the longitude offset for the top left corner of target's
        /// geospatial bounding box from the parent's Frame Center Longitude (e.g., MISB ST 0601 - Item 24)
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
        #[cfg(any(feature = "misb0903-6"))]
        // #[klv(key = 0x0F, dec = ops::to_ll_offset)]
        /// (Assumed Optional) Latitude offset for bottom right corner of target's geospatial bounding box
        ///
        /// The `boundingBoxBottomRightLatOffset` item is the latitude offset for the bottom right corner of target's
        /// geospatial bounding box from the parent's Frame Center Latitude (e.g., MISB ST 0601 - Item 23)
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
        #[cfg(any(feature = "misb0903-6"))]
        // #[klv(key = 0x10, dec = ops::to_ll_offset)]
        /// (Assumed Optional) Longitude offset for bottom right corner of target's geospatial bounding box
        ///
        /// The `boundingBoxBottomRightLonOffset` item is the longitude offset for the bottom right corner of target's
        /// geospatial bounding box from the parent's Frame Center Longitude (e.g., MISB ST 0601 - Item 24)
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
        #[cfg(any(feature = "misb0903-6"))]
        // #[klv(key = 0x11, dec = Location::decode)]
        /// (Assumed Optional) Location of the target (latitude, longitude, & height above WGS84 Ellipsoid),
        /// with sigma and rho values
        ///
        /// The `targetLocation` item provides detailed geo-positioning information for a target, optionally including
        /// the standard deviation and correlation coefficients. This item is of type [`Location`] which is a Defined
        /// Length Truncation Pack. To specify the geographic coordinates for a target with standalone-VMTI, use
        /// `targetLocation` in lieu of VTarget Pack - Item 10 `targetLocationOffsetLat` and Item 11 `targetLocationOffsetLat`.
        /// However, even when using embedded-VMTI, `targetLocation` is preferred vice offset calculations.
        pub target_location: Option<Location>,
        #[cfg(any(feature = "misb0903-6"))]
        // #[klv(key = 0x12, dec = Location::repeated)]
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
        #[cfg(any(feature = "misb0903-6"))]
        // #[klv(key = 0x13)]
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
        #[cfg(any(feature = "misb0903-6"))]
        // #[klv(key = 0x14)]
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
        #[cfg(any(feature = "misb0903-6"))]
        // #[klv(key = 0x16)]
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
        #[cfg(any(feature = "misb0903-6"))]
        // #[klv(key = 0x17, dec = DetectionStatus::decode)]
        /// (Assumed Optional) Enumeration indicating the current state of VMTI detections for
        /// a given entity (Inactive, ActiveMoving, Dropped, Active-Stopped, Active-Coasting)
        ///
        /// The `detectionStatus` item allows assigning a target a status in detection.
        pub detection_status: Option<DetectionStatus>,
        #[cfg(any(feature = "misb0903-6"))]
        // #[klv(key = 0x6B, dec = VObject::repeated)]
        /// (Mandatory) Series of one or more VObject LS
        ///
        /// The `vObjectSeries` item is a Series (see Figure 18) of one or more VObject LS associated with
        /// a specific target.
        ///
        /// Is "pseudo optional"; if not present, defaults to an empty vector.
        pub v_object_series: Vec<VObject>,
    }
    #[automatically_derived]
    /// [`Misb0903Target`] implementation of [`tinyklv::prelude::Seek`] for [`& [u8]`]
    impl ::tinyklv::prelude::Seek<&[u8]> for Misb0903Target {
        fn seek<'z>(
            input: &mut &'z [u8],
        ) -> ::tinyklv::reexport::winnow::PResult<&'z [u8]> {
            let checkpoint = input.checkpoint();
            let packet_len = match ::winnow::combinator::trace(
                    "",
                    move |input: &mut _| {
                        (
                            b"\x06\x0E\x2B\x34\x02\x0B\x01\x01\x0E\x01\x03\x03\x06\x00\x00\x00"
                                .void(),
                            tinyklv::codecs::ber::dec::ber_length,
                        )
                            .map(|t| { (t.1,) })
                            .parse_next(input)
                    },
                )
                .parse_next(input)
            {
                Ok(x) => x.0 as usize,
                Err(e) => {
                    return Err(
                        e
                            .backtrack()
                            .add_context(
                                input,
                                &checkpoint,
                                ::tinyklv::reexport::winnow::error::StrContext::Label(
                                    "Unable to find recognition sentinal and packet length for initial parsing of `Misb0903Target` packet",
                                ),
                            ),
                    );
                }
            };
            ::tinyklv::reexport::winnow::token::take(packet_len).parse_next(input)
        }
    }
    #[automatically_derived]
    /// [`Misb0903Target`] implementation of [`tinyklv::prelude::Decode`] for [`& [u8]`]
    impl ::tinyklv::prelude::Decode<&[u8]> for Misb0903Target {
        fn decode(input: &mut &[u8]) -> ::tinyklv::reexport::winnow::PResult<Self> {
            let checkpoint = input.checkpoint();
            let mut target_centroid: Option<PixelPosition> = None;
            let mut bbox_tl: Option<PixelPosition> = None;
            let mut bbox_br: Option<PixelPosition> = None;
            let mut target_priority: Option<u8> = None;
            let mut target_confidence_level: Option<u8> = None;
            let mut target_history: Option<u16> = None;
            let mut percentage_of_target_pixels: Option<u8> = None;
            let mut target_color: Option<Color> = None;
            let mut target_intensity: Option<u32> = None;
            let mut target_location_lat_offset: Option<f64> = None;
            let mut target_location_lon_offset: Option<f64> = None;
            let mut target_hae: Option<f64> = None;
            let mut bbox_tl_lat_offset: Option<f64> = None;
            let mut bbox_tl_lon_offset: Option<f64> = None;
            let mut bbox_br_lat_offset: Option<f64> = None;
            let mut bbox_br_lon_offset: Option<f64> = None;
            let mut target_location: Option<Location> = None;
            let mut geospatial_contour_series: Option<Vec<Location>> = None;
            let mut centroid_rows: Option<u32> = None;
            let mut centroid_cols: Option<u32> = None;
            let mut algorithm_id: Option<u32> = None;
            let mut detection_status: Option<DetectionStatus> = None;
            let mut v_object_series: Option<Vec<VObject>> = None;
            loop {
                match (
                    tinyklv::codecs::ber::dec::ber_oid::<u64>,
                    tinyklv::codecs::ber::dec::ber_length,
                )
                    .parse_next(input)
                {
                    Ok((key, len)) => {
                        match (key, len) {
                            (0x01, len) => {
                                target_centroid = PixelPosition::decode(len)(input).ok();
                            }
                            (0x02, len) => {
                                bbox_tl = PixelPosition::decode(len)(input).ok();
                            }
                            (0x03, len) => {
                                bbox_br = PixelPosition::decode(len)(input).ok();
                            }
                            (0x04, _) => {
                                target_priority = tinyklv::codecs::binary::dec::be_u8(input)
                                    .ok();
                            }
                            (0x05, _) => {
                                target_confidence_level = tinyklv::codecs::binary::dec::be_u8(
                                        input,
                                    )
                                    .ok();
                            }
                            (0x06, len) => {
                                target_history = tinyklv::codecs::binary::dec::be_u16_lengthed(
                                        len,
                                    )(input)
                                    .ok();
                            }
                            (0x07, _) => {
                                percentage_of_target_pixels = tinyklv::codecs::binary::dec::be_u8(
                                        input,
                                    )
                                    .ok();
                            }
                            (0x08, _) => target_color = Color::decode(input).ok(),
                            (0x09, len) => {
                                target_intensity = tinyklv::codecs::binary::dec::be_u32_lengthed(
                                        len,
                                    )(input)
                                    .ok();
                            }
                            (0x0A, _) => {
                                target_location_lat_offset = ops::to_ll_offset(input).ok();
                            }
                            (0x0B, _) => {
                                target_location_lon_offset = ops::to_ll_offset(input).ok();
                            }
                            (0x0C, _) => target_hae = ops::to_hae(input).ok(),
                            (0x0D, _) => {
                                bbox_tl_lat_offset = ops::to_ll_offset(input).ok();
                            }
                            (0x0E, _) => {
                                bbox_tl_lon_offset = ops::to_ll_offset(input).ok();
                            }
                            (0x0F, _) => {
                                bbox_br_lat_offset = ops::to_ll_offset(input).ok();
                            }
                            (0x10, _) => {
                                bbox_br_lon_offset = ops::to_ll_offset(input).ok();
                            }
                            (0x11, _) => target_location = Location::decode(input).ok(),
                            (0x12, _) => {
                                geospatial_contour_series = Location::repeated(input).ok();
                            }
                            (0x13, len) => {
                                centroid_rows = tinyklv::codecs::binary::dec::be_u32_lengthed(
                                        len,
                                    )(input)
                                    .ok();
                            }
                            (0x14, len) => {
                                centroid_cols = tinyklv::codecs::binary::dec::be_u32_lengthed(
                                        len,
                                    )(input)
                                    .ok();
                            }
                            (0x16, len) => {
                                algorithm_id = tinyklv::codecs::binary::dec::be_u32_lengthed(
                                        len,
                                    )(input)
                                    .ok();
                            }
                            (0x17, _) => {
                                detection_status = DetectionStatus::decode(input).ok();
                            }
                            (0x6B, _) => v_object_series = VObject::repeated(input).ok(),
                            (_, len) => {
                                let _ = ::tinyklv::reexport::winnow::token::take::<
                                    usize,
                                    &[u8],
                                    ::tinyklv::reexport::winnow::error::ContextError,
                                >(len)
                                    .parse_next(input);
                            }
                        }
                    }
                    Err(_) => break,
                }
            }
            Ok(Misb0903Target {
                target_centroid,
                bbox_tl,
                bbox_br,
                target_priority,
                target_confidence_level,
                target_history,
                percentage_of_target_pixels,
                target_color,
                target_intensity,
                target_location_lat_offset,
                target_location_lon_offset,
                target_hae,
                bbox_tl_lat_offset,
                bbox_tl_lon_offset,
                bbox_br_lat_offset,
                bbox_br_lon_offset,
                target_location,
                geospatial_contour_series,
                centroid_rows,
                centroid_cols,
                algorithm_id,
                detection_status,
                v_object_series: v_object_series
                    .ok_or_else(|| {
                        ::tinyklv::reexport::winnow::error::ErrMode::Backtrack(
                            ::tinyklv::reexport::winnow::error::ContextError::new()
                                .add_context(
                                    input,
                                    &checkpoint,
                                    ::tinyklv::reexport::winnow::error::StrContext::Label(
                                        "`v_object_series` is a required value missing from the `Misb0903Target` packet. To prevent this, set this field as optional.",
                                    ),
                                ),
                        )
                    })?,
                target_id: Option::<u128>::default(),
            })
        }
    }
    // #[automatically_derived]
    // impl ::core::fmt::Debug for Misb0903Target {
    //     #[inline]
    //     fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
    //         let names: &'static _ = &[
    //             "target_id",
    //             "target_centroid",
    //             "bbox_tl",
    //             "bbox_br",
    //             "target_priority",
    //             "target_confidence_level",
    //             "target_history",
    //             "percentage_of_target_pixels",
    //             "target_color",
    //             "target_intensity",
    //             "target_location_lat_offset",
    //             "target_location_lon_offset",
    //             "target_hae",
    //             "bbox_tl_lat_offset",
    //             "bbox_tl_lon_offset",
    //             "bbox_br_lat_offset",
    //             "bbox_br_lon_offset",
    //             "target_location",
    //             "geospatial_contour_series",
    //             "centroid_rows",
    //             "centroid_cols",
    //             "algorithm_id",
    //             "detection_status",
    //             "v_object_series",
    //         ];
    //         let values: &[&dyn ::core::fmt::Debug] = &[
    //             &self.target_id,
    //             &self.target_centroid,
    //             &self.bbox_tl,
    //             &self.bbox_br,
    //             &self.target_priority,
    //             &self.target_confidence_level,
    //             &self.target_history,
    //             &self.percentage_of_target_pixels,
    //             &self.target_color,
    //             &self.target_intensity,
    //             &self.target_location_lat_offset,
    //             &self.target_location_lon_offset,
    //             &self.target_hae,
    //             &self.bbox_tl_lat_offset,
    //             &self.bbox_tl_lon_offset,
    //             &self.bbox_br_lat_offset,
    //             &self.bbox_br_lon_offset,
    //             &self.target_location,
    //             &self.geospatial_contour_series,
    //             &self.centroid_rows,
    //             &self.centroid_cols,
    //             &self.algorithm_id,
    //             &self.detection_status,
    //             &&self.v_object_series,
    //         ];
    //         ::core::fmt::Formatter::debug_struct_fields_finish(
    //             f,
    //             "Misb0903Target",
    //             names,
    //             values,
    //         )
    //     }
    // }
}