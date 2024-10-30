pub use auto::*;
use ffi;
mod auto;

#[macro_use]
extern crate gir_error;

pub mod functions {
    pub use super::auto::functions::*;
}

pub mod traits {
    pub use super::auto::traits::*;
}
