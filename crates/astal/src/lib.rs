#[macro_use]
extern crate gir_error;
extern crate atk;
extern crate bitflags;
extern crate gdk;
extern crate gio;
extern crate glib as gobject;
extern crate gtk;
extern crate libc;

pub use auto::*;
use ffi;
mod auto;

pub mod functions {
    pub use super::auto::functions::*;
}
