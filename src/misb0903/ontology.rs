// // --------------------------------------------------
// // tinyklv
// // --------------------------------------------------
// use tinyklv::Klv;
// use tinyklv::prelude::*;

// #[cfg(any(
//     feature = "misb0903-6",
// ))]
// #[derive(Klv, Debug)]
// #[klv(
//     stream = &[u8],
//     // sentinel = b"",
//     key(enc = tinyklv::codecs::ber::enc::ber_oid,
//         dec = tinyklv::codecs::ber::dec::ber_oid::<u64>),
//     len(enc = tinyklv::codecs::ber::enc::ber_length,
//         dec = tinyklv::codecs::ber::dec::ber_length),
//     default(ty = u32, dyn = true, dec = tinyklv::codecs::binary::dec::be_u32_lengthed),
//     default(ty = String, dyn = true, dec = tinyklv::codecs::binary::dec::to_string_utf8),
// )]
// /// Video Moving Target Indicator Metadata
// /// 
// /// MISB Standard 0903
// /// 
// /// For more information, see [Motion Imagery Standards Board (MISB)](https://nsgreg.nga.mil/misb.jsp)
// pub struct Misb0903Ontology {
//     #[klv(key = 0x01)]
//     /// (Contextual) Identifier for the ontology used
//     /// 
//     /// Len: V8
//     /// 
//     /// Units: None
//     pub ontology_id: u32,

//     #[klv(key = 0x02)]
//     pub parent_id: u32,

//     #[klv(key = 0x03)]
//     pub onology_iri: String,

//     #[klv(key = 0x04)]
//     pub entity_iri: String,

//     #[klv(key = 0x05)]
//     pub version_iri: String,

//     #[klv(key = 0x06)]
//     pub label: String,
// }