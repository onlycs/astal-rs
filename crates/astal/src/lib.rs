#[macro_use]
extern crate gir_error;
pub extern crate astal_sys as ffi;
extern crate atk;
extern crate cairo;
extern crate gdk;
extern crate gio;
extern crate glib as gobject;
extern crate gtk;

mod auto;

pub use auto::*;
