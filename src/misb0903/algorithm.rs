// --------------------------------------------------
// tinyklv
// --------------------------------------------------
use tinyklv::Klv;
use tinyklv::prelude::*;

#[cfg(any(
    feature = "misb0903-6",
))]
#[derive(Klv, Debug)]
#[klv(
    // ------------------------------------------------
    // There is no UL - Local Set only
    // ------------------------------------------------
    stream = &[u8],
    key(enc = tinyklv::codecs::binary::enc::u8,
        dec = tinyklv::codecs::binary::dec::u8),
    len(enc = tinyklv::codecs::ber::enc::ber_length,
        dec = tinyklv::codecs::ber::dec::ber_length),
    default(ty = u64, dyn = true, dec = tinyklv::codecs::binary::dec::be_u64_lengthed),
    default(ty = String, dyn = true, dec = tinyklv::codecs::binary::dec::to_string_utf8),
)]
/// MISB 0903 VMTI Algorithm LS (Local Set)
/// 
/// `algorithmSeries` -> [`Vec<Misb0903Algorithm>`]
/// 
/// The Algorithm LS documents attributes of the algorithm used for detection and tracking of
/// targets. The VMTI LS `algorithmSeries` – Item 102 conveys one or more instances of the LS
/// allowing for documenting different algorithms in use within one VMTI LS.
/// 
/// The `algorithmSeries` enables assigning an algorithm LS to a detected target or a generated track,
/// see [`VTarget`](crate::misb0903::target::Misb0903Target) Item 22 and
/// [`VTracker`](crate::misb0903::tracker::Misb0903Tracker) Item 12. 
/// 
/// For more information, see [Motion Imagery Standards Board (MISB)](https://nsgreg.nga.mil/misb.jsp)
pub struct Misb0903Algorithm {
    #[klv(key = 0x01)]
    /// (Mandatory) Identifier for the algorithm used
    /// 
    /// `algorithmId` -> [`Misb0903Algorithm::algorithm_id`]
    /// 
    /// When the VMTI LS contains a Series of Algorithm LS, each element in the Series requires a
    /// unique identifier (`algorithmId`). The `algorithmId` is an integer which identifies a single Algorithm
    /// LS in the Series and is unique among all other elements in the list. Systems may reuse
    /// `algorithmId`s from VMTI LS to VMTI LS (i.e., two sequential VMTI packets) so receivers
    /// should not assume an identifier value is static for an entire VMTI stream. The `algorithmId` does
    /// not need to start with a value of one (1) nor do the `algorithmId`s need to be in any specific order
    /// in an `algorithmSeries`.
    /// 
    /// Len: V8
    /// 
    /// Valid Values: An unsigned integer
    /// 
    /// # Example
    /// 
    /// ```rust
    /// let mut algorithm_id_hex: &[u8] = b"\x09";
    /// let len = 1;
    /// assert_eq!(tinyklv::codecs::binary::dec::be_u64_lengthed(len)(&mut algorithm_id_hex).unwrap(), 9);
    /// ```
    pub algorithm_id: u64,

    #[klv(key = 0x02)]
    /// (Assumed Optional) Name of the algorithm
    /// 
    /// `name` -> [`Misb0903Algorithm::name`]
    /// 
    /// The `name` item is the name assigned to the algorithm by the data producer.
    /// 
    /// Valid Values: Any alphanumeric value in UTF-8
    /// 
    /// # Example
    /// 
    /// ```rust
    /// let name = "k6_yolo_9000_tracker".to_string();
    /// let mut name_hex: &[u8] = b"\x6B\x36\x5F\x79\x6F\x6C\x6F\x5F\x39\x30\x30\x30\x5F\x74\x72\x61\x63\x6B\x65\x72";
    /// assert_eq!(tinyklv::codecs::binary::dec::to_string_utf8(name.len())(&mut name_hex).unwrap(), name);
    /// ```
    pub name: Option<String>,

    #[klv(key = 0x03)]
    /// (Assumed Optional) Version of the algorithm
    /// 
    /// `version` -> [`Misb0903Algorithm::version`]
    /// 
    /// The `version` item is the version of the algorithm. 
    /// 
    /// Valid Values: Any alphanumeric value in UTF-8
    /// 
    /// # Example
    /// 
    /// ```rust
    /// let version = "2.6a".to_string();
    /// let mut version_hex: &[u8] = b"\x32\x2E\x36\x61";
    /// assert_eq!(tinyklv::codecs::binary::dec::to_string_utf8(version.len())(&mut version_hex).unwrap(), version);
    /// ```
    pub version: Option<String>,

    #[klv(key = 0x04)]
    /// (Assumed Optional) Type of algorithm e.g., detector classifier
    /// 
    /// `class` -> [`Misb0903Algorithm::class`]
    /// 
    /// The `class` item is the type of algorithm.
    /// 
    /// Valid Values: Any alphanumeric value in UTF-8
    /// 
    /// # Example
    /// 
    /// ```rust
    /// let class = "kalmann".to_string();
    /// let mut class_hex: &[u8] = b"\x6B\x61\x6C\x6D\x61\x6E\x6E";
    /// assert_eq!(tinyklv::codecs::binary::dec::to_string_utf8(class.len())(&mut class_hex).unwrap(), class);
    /// ```
    pub class: Option<String>,

    #[klv(key = 0x05)]
    /// (Assumed Optional) Number of frames the algorithm operates over
    /// 
    /// `nFrames` -> [`Misb0903Algorithm::n_frames`]
    /// 
    /// The `nFrames` item is the number of frames the algorithm processes when detecting or tracking
    /// the object.
    /// 
    /// Len: V8
    /// 
    /// # Example
    /// 
    /// ```rust
    /// let mut n_frames_hex: &[u8] = b"\x0A";
    /// let len = 1;
    /// assert_eq!(tinyklv::codecs::binary::dec::be_u64_lengthed(len)(&mut n_frames_hex).unwrap(), 10);
    /// ```
    pub n_frames: Option<u64>,
}