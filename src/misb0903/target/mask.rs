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
//     key(enc = tinyklv::codecs::ber::enc::u8,
//         dec = tinyklv::codecs::ber::dec::u8),
//     len(enc = tinyklv::codecs::ber::enc::ber_length,
//         dec = tinyklv::codecs::ber::dec::ber_length),
//     default(ty = u8, dec = tinyklv::codecs::binary::dec::be_u8),
//     default(ty = u16, dyn = true, dec = tinyklv::codecs::binary::dec::be_u16_lengthed),
//     default(ty = String, dyn = true, dec = tinyklv::codecs::binary::dec::to_string_utf8),
// )]
// pub struct VMask {
//     #[klv(key = 0x01)]
//     /// (Mandatory) At least three unsigned integer numbers specifying the vertices of a
//     /// polygon representing the outline of a target
//     /// 
//     /// A `pixelContour` item is an Array type of three or more points representing the vertices of a
//     /// polygon within a Motion Imagery frame listed in clockwise order. Close the polygon by
//     /// connecting the last point to the first point. Each point is a pixel number with numbering
//     /// commencing with 1, at the top left pixel, proceeding from left to right, top to bottom, then
//     /// encoded using the Length-Value construct of a Variable Length Pack. Note: in the UML of the
//     /// VMTI LS, the closed brackets [ ] indicates an array.
//     pixel_contour: u32,

//     #[klv(key = 0x02)]
//     /// (Mandatory) Describes the area of the frame occupied by a target
//     /// using a run-length encoded bit mask with 1 to indicate
//     /// that a pixel includes a part of the target and 0 to indicate
//     /// otherwise
//     /// 
//     /// The `bitMaskSeries` item is a Series type defining a run length encoding of a bit mask describing
//     /// the pixels which include the target within the Motion Imagery frame. As shown in Figure 19, the
//     /// VMask LS Value consists of multiple pixel-runs. Each pixel-run specifies the starting pixel
//     /// number (i.e., pixel) and the number of pixels in a run (i.e., Run BER).
//     bit_mask_series: u32,
// }

// pub struct BitMaskSeries {

// }