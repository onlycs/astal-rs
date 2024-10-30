#[macro_use]
extern crate gir_error;
extern crate atk;
extern crate bitflags;
extern crate ffi;
extern crate gdk;
extern crate gio;
extern crate glib as gobject;
extern crate gtk;
extern crate libc;

mod auto;

pub use auto::*;

pub mod functions {
    pub use super::auto::functions::*;
}

pub mod traits {
    pub use super::auto::traits::*;
}
