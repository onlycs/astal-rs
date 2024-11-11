#[macro_use]
extern crate gir_error;
pub extern crate astal_notifd_sys as ffi;
extern crate gio;
extern crate glib;
extern crate gtk;

#[allow(warnings)]
mod auto;

pub use auto::*;
