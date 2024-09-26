// --------------------------------------------------
// tinyklv
// --------------------------------------------------
use tinyklv::Klv;
use tinyklv::prelude::*;

// --------------------------------------------------
// local
// --------------------------------------------------
use crate::misb0903::ops;

#[cfg(any(
    feature = "misb0903-6",
))]
#[derive(Klv, Debug)]
#[klv(
    stream = &[u8],
    key(enc = tinyklv::codecs::binary::enc::u8,
        dec = tinyklv::codecs::binary::dec::u8),
    len(enc = tinyklv::codecs::ber::enc::ber_length,
        dec = tinyklv::codecs::ber::dec::ber_length),
)]
pub struct Misb0903Object {
    #[cfg(not(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x01)]
    /// (-) Item deprecated for MISB 0903.6. To implement, please
    /// fill out this section with the appropriate feature flag:
    /// 
    /// ```rust no_run ignore
    /// #[cfg(any(feature = "misb0903-5"))] // do not include `"misb0903-6"`
    /// #[klv(key = 0x01, ...)]
    /// ```
    pub placeholder_key_01: (),

    #[cfg(not(
        feature = "misb0903-6",
    ))]
    #[klv(key = 0x02)]
    /// (-) Item deprecated for MISB 0903.6. To implement, please
    /// fill out this section with the appropriate feature flag:
    /// 
    /// ```rust no_run ignore
    /// #[cfg(any(feature = "misb0903-5"))] // do not include `"misb0903-6"`
    /// #[klv(key = 0x02, ...)]
    /// ```
    pub placeholder_key_02: (),

    #[klv(key = 0x03, dyn = true, dec = tinyklv::codecs::binary::dec::be_u64_lengthed)]
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

    #[klv(key = 0x04, dyn = true, dec = ops::to_confidence)]
    /// (Assumed Mandatory) The amount of confidence in the label of this object
    /// 
    /// The confidence item is the measure of "trust" in the labeling of this vObject, ranging from 0.0%
    /// to 100.0%. For example, an object-classifier analyzes a blob of pixels and labels the blob as
    /// representing a vehicle. If the blob closely matches the classifiers criteria for a vehicle the
    /// confidence in the labeling is high (towards 100.0%); alternatively, if the classifier is less sure of
    /// its label the confidence is low (towards 0%). The confidence value is IMAPB(0.0, 100.0, length)
    /// with the length defined by the tag’s length value. Increasing the length provides more accuracy.
    /// The minimum length is 1.
    /// 
    /// Len: V3
    /// 
    /// Units: Percent (%)
    pub confidence: f64,

    #[klv(key = 0x05, dec = Misb0903Feature::repeated)]
    /// (Mandatory) One or more VFeature LS associated with a specific VObject
    /// 
    /// Is "pseudo optional"; if not present, defaults to an empty vector.
    pub v_feature_series: Vec<Misb0903Feature>,
}

#[cfg(any(
    feature = "misb0903-6",
))]
#[derive(Klv, Debug)]
#[klv(
    stream = &[u8],
    key(enc = tinyklv::codecs::binary::enc::u8,
        dec = tinyklv::codecs::binary::dec::u8),
    len(enc = tinyklv::codecs::ber::enc::ber_length,
        dec = tinyklv::codecs::ber::dec::ber_length),
)]
pub struct Misb0903Feature {
    #[klv(key = 0x01)]
    #[cfg(not(
        feature = "misb0903-6",
    ))]
    /// (-) Item deprecated for MISB 0903.6. To implement, please
    /// fill out this section with the appropriate feature flag:
    /// 
    /// ```rust no_run ignore
    /// #[cfg(any(feature = "misb0903-5"))] // do not include `"misb0903-6"`
    /// #[klv(key = 0x01, ...)]
    /// ```
    pub placeholder_key_01: (),

    #[klv(key = 0x02)]
    #[cfg(not(
        feature = "misb0903-6",
    ))]
    /// (-) Item deprecated for MISB 0903.6. To implement, please
    /// fill out this section with the appropriate feature flag:
    /// 
    /// ```rust no_run ignore
    /// #[cfg(any(feature = "misb0903-5"))] // do not include `"misb0903-6"`
    /// #[klv(key = 0x02, ...)]
    /// ```
    pub placeholder_key_02: (),

    #[klv(key = 0x03, dyn = true, dec = tinyklv::codecs::binary::dec::be_u64_lengthed)]
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
    
    #[klv(key = 0x04, dyn = true, dec = ops::to_confidence)]
    /// (Assumed Mandatory) The amount of confidence in the feature's label.
    /// 
    /// The confidence item is the measure of "trust" in the feature-classification of this vFeature from
    /// 0.0% to 100.0%. For example, a feature-classifier analyzes an object and "classifies" a subset of
    /// the object’s pixels as an antenna. If the pixels closely match the classifier’s criteria for an
    /// antenna the confidence in the feature-classification is high (towards 100.0%); alternatively, if the
    /// classifier is less sure of its feature-classification the confidence is low (towards 0%). The
    /// confidence value is IMAPB(0.0, 100.0, length) with the length defined by the item’s length
    /// value. Increasing the length provides more accuracy. The minimum length is 1.
    /// 
    /// Len: V3
    /// 
    /// Units: Percent (%)
    pub confidence: f64,
}