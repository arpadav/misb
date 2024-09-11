// --------------------------------------------------
// tinyklv
// --------------------------------------------------
use tinyklv::prelude::*;

// --------------------------------------------------
// static
// --------------------------------------------------
use std::sync::LazyLock;
/// 2 byte-precision in range [0, 180]
/// 
/// Used for some angles
/// 
/// Units: Degrees (°)
static IMAPB_0_180_2_F64: LazyLock<crate::misb1201::ImapB<f64>> = LazyLock::new(|| {
    crate::misb1201::ImapB::new(0.0, 180.0, 2).unwrap()
});

/// See: [`crate::misb0903::Misb0903`]
/// 
/// * [`crate::misb0903::Misb0903::vmti_hfov`]
/// * [`crate::misb0903::Misb0903::vmti_vfov`]
pub fn to_hvfov(input: &mut &[u8]) -> winnow::PResult<f64> {
    let checkpoint = input.checkpoint();
    IMAPB_0_180_2_F64
        .from_imap(input)
        .map_err(|e| tinyklv::err!().add_context(input, &checkpoint, e.into()))
}

pub fn to_row_col(input: &mut &[u8], len: usize) -> winnow::PResult<(u32, u32)> {
    let pixel_value = tinyklv::binary::dec::be_u32_lengthed(input, len)?;
    let frame_width = 100.0;
    let row = ((pixel_value as f32 / frame_width)).floor() as u32 + 1;
    let col = ((pixel_value - (row - 1)) as f32 * frame_width) as u32;
    Ok((row, col))
}