#![forbid(unsafe_code)]
pub mod misb0102;
pub mod misb0601;
pub mod misb0903;
pub mod misb1201;
pub mod misb1204;

/// Length in bytes of a type
pub trait LengthBytes {
    const LENGTH_BYTES: usize;
}

#[path = "misb0903/target/mod.rs"] mod module_name;
mod sandbox;