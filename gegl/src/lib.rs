#![cfg_attr(docsrs, feature(doc_cfg))]

macro_rules! skip_assert_initialized {
    () => {};
}

macro_rules! assert_initialized_main_thread {
    () => {};
}

pub use babl;
pub use ffi;
pub use glib;

mod functions;
pub use functions::*;
mod rectangle;

#[allow(clippy::too_many_arguments)]
#[allow(clippy::let_unit_value)]
#[allow(clippy::let_and_return)]
#[allow(clippy::derived_hash_with_manual_eq)]
mod auto;
pub use auto::functions::*;
pub use auto::*;

pub mod prelude;


