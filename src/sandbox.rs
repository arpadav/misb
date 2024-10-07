// use crate::misb0903::*;
// use crate::misb0903::primitives::*;
// use crate::misb0903::target::*;
// use tinyklv::prelude::*;

// #[automatically_derived]
// #[doc = concat!
// (" [`", stringify! (Misb0903Target),
// "`] implementation of [`tinyklv::prelude::Decode`] for [`", stringify!
// (& [u8]), "`]")] impl :: tinyklv :: prelude :: Decode < & [u8] > for
// Misb0903Target
// {
//     fn decode(input : & mut & [u8]) -> :: tinyklv :: reexport :: winnow ::
//     PResult < Self >
//     {
//         let checkpoint = input.checkpoint(); let mut target_centroid : Option
//         < PixelPosition > = None; let mut bbox_tl : Option < PixelPosition > =
//         None; let mut bbox_br : Option < PixelPosition > = None; let mut
//         target_priority : Option < u8 > = None; let mut
//         target_confidence_level : Option < u8 > = None; let mut target_history
//         : Option < u16 > = None; let mut percentage_of_target_pixels : Option
//         < u8 > = None; let mut target_color : Option < Color > = None; let mut
//         target_intensity : Option < u32 > = None; let mut
//         target_location_lat_offset : Option < f64 > = None; let mut
//         target_location_lon_offset : Option < f64 > = None; let mut target_hae
//         : Option < f64 > = None; let mut bbox_tl_lat_offset : Option < f64 > =
//         None; let mut bbox_tl_lon_offset : Option < f64 > = None; let mut
//         bbox_br_lat_offset : Option < f64 > = None; let mut bbox_br_lon_offset
//         : Option < f64 > = None; let mut target_location : Option < Location >
//         = None; let mut geospatial_contour_series : Option < Vec < Location >
//         > = None; let mut centroid_rows : Option < u32 > = None; let mut
//         centroid_cols : Option < u32 > = None; let mut algorithm_id : Option <
//         u32 > = None; let mut detection_status : Option < DetectionStatus > =
//         None; let mut v_object_series : Option < Vec < Misb0903Object > > =
//         None; loop
//         {
//             match
//             (tinyklv :: codecs :: ber :: dec :: ber_oid :: < u64 > , tinyklv
//             :: codecs :: ber :: dec :: ber_length,).parse_next(input)
//             {
//                 Ok((key, len)) => match (key, len)
//                 {
//                     (0x01, len) => target_centroid = PixelPosition ::
//                     decode(len) (input).ok(), (0x02, len) => bbox_tl =
//                     PixelPosition :: decode(len) (input).ok(), (0x03, len) =>
//                     bbox_br = PixelPosition :: decode(len) (input).ok(),
//                     (0x04, len) =>
//                     {
//                         target_priority = tinyklv :: codecs :: binary :: dec ::
//                         be_u8(& mut :: tinyklv :: reexport :: winnow :: token ::
//                         take(len).parse_next(input) ?).ok();
//                     } (0x05, len) =>
//                     {
//                         target_confidence_level = tinyklv :: codecs :: binary :: dec
//                         ::
//                         be_u8(& mut :: tinyklv :: reexport :: winnow :: token ::
//                         take(len).parse_next(input) ?).ok();
//                     } (0x06, len) => target_history = tinyklv :: codecs ::
//                     binary :: dec :: be_u16_lengthed(len) (input).ok(),
//                     (0x07, len) =>
//                     {
//                         percentage_of_target_pixels = tinyklv :: codecs :: binary ::
//                         dec ::
//                         be_u8(& mut :: tinyklv :: reexport :: winnow :: token ::
//                         take(len).parse_next(input) ?).ok();
//                     } (0x08, len) =>
//                     {
//                         target_color = Color ::
//                         decode(& mut :: tinyklv :: reexport :: winnow :: token ::
//                         take(len).parse_next(input) ?).ok();
//                     } (0x09, len) => target_intensity = tinyklv :: codecs ::
//                     binary :: dec :: be_u32_lengthed(len) (input).ok(),
//                     (0x0A, len) =>
//                     {
//                         target_location_lat_offset = ops ::
//                         to_ll_offset(& mut :: tinyklv :: reexport :: winnow :: token
//                         :: take(len).parse_next(input) ?).ok();
//                     } (0x0B, len) =>
//                     {
//                         target_location_lon_offset = ops ::
//                         to_ll_offset(& mut :: tinyklv :: reexport :: winnow :: token
//                         :: take(len).parse_next(input) ?).ok();
//                     } (0x0C, len) =>
//                     {
//                         target_hae = ops ::
//                         to_hae(& mut :: tinyklv :: reexport :: winnow :: token ::
//                         take(len).parse_next(input) ?).ok();
//                     } (0x0D, len) =>
//                     {
//                         bbox_tl_lat_offset = ops ::
//                         to_ll_offset(& mut :: tinyklv :: reexport :: winnow :: token
//                         :: take(len).parse_next(input) ?).ok();
//                     } (0x0E, len) =>
//                     {
//                         bbox_tl_lon_offset = ops ::
//                         to_ll_offset(& mut :: tinyklv :: reexport :: winnow :: token
//                         :: take(len).parse_next(input) ?).ok();
//                     } (0x0F, len) =>
//                     {
//                         bbox_br_lat_offset = ops ::
//                         to_ll_offset(& mut :: tinyklv :: reexport :: winnow :: token
//                         :: take(len).parse_next(input) ?).ok();
//                     } (0x10, len) =>
//                     {
//                         bbox_br_lon_offset = ops ::
//                         to_ll_offset(& mut :: tinyklv :: reexport :: winnow :: token
//                         :: take(len).parse_next(input) ?).ok();
//                     } (0x11, len) =>
//                     {
//                         target_location = Location ::
//                         decode(& mut :: tinyklv :: reexport :: winnow :: token ::
//                         take(len).parse_next(input) ?).ok();
//                     } (0x12, len) =>
//                     {
//                         geospatial_contour_series = Location ::
//                         repeated(& mut :: tinyklv :: reexport :: winnow :: token ::
//                         take(len).parse_next(input) ?).ok();
//                     } (0x13, len) => centroid_rows = tinyklv :: codecs :: binary
//                     :: dec :: be_u32_lengthed(len) (input).ok(), (0x14, len) =>
//                     centroid_cols = tinyklv :: codecs :: binary :: dec ::
//                     be_u32_lengthed(len) (input).ok(), (0x16, len) =>
//                     algorithm_id = tinyklv :: codecs :: binary :: dec ::
//                     be_u32_lengthed(len) (input).ok(), (0x17, len) =>
//                     {
//                         detection_status = DetectionStatus ::
//                         decode(& mut :: tinyklv :: reexport :: winnow :: token ::
//                         take(len).parse_next(input) ?).ok();
//                     } (0x6B, len) =>
//                     {
//                         v_object_series = Misb0903Object ::
//                         repeated(& mut :: tinyklv :: reexport :: winnow :: token ::
//                         take(len).parse_next(input) ?).ok();
//                     } (_, len) =>
//                     {
//                         let _ = :: tinyklv :: reexport :: winnow :: token :: take ::
//                         < usize, & [u8], :: tinyklv :: reexport :: winnow :: error
//                         :: ContextError > (len).parse_next(input);
//                     },
//                 }, Err(_) => break,
//             }
//         }
//         Ok(Misb0903Target
//         {
//             target_centroid, bbox_tl, bbox_br, target_priority,
//             target_confidence_level, target_history,
//             percentage_of_target_pixels, target_color, target_intensity,
//             target_location_lat_offset, target_location_lon_offset,
//             target_hae, bbox_tl_lat_offset, bbox_tl_lon_offset,
//             bbox_br_lat_offset, bbox_br_lon_offset, target_location,
//             geospatial_contour_series, centroid_rows, centroid_cols,
//             algorithm_id, detection_status, v_object_series :
//             v_object_series.ok_or_else(||
//             {
//                 :: tinyklv :: reexport :: winnow :: error :: ErrMode ::
//                 Backtrack(:: tinyklv :: reexport :: winnow :: error ::
//                 ContextError ::
//                 new().add_context(input, & checkpoint, :: tinyklv :: reexport
//                 :: winnow :: error :: StrContext ::
//                 Label(concat!
//                 ("`", stringify! (v_object_series),
//                 "` is a required value missing from the `", stringify!
//                 (Misb0903Target),
//                 "` packet. To prevent this, set this field as optional."))))
//             }) ? , target_id : Option :: < u128 > :: default(),
//         })
//     }
// }