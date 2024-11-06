#[macro_use]
extern crate gir_error;
extern crate astal_tray_sys as ffi;
extern crate gdk_pixbuf;
extern crate gio;
extern crate glib;
extern crate gtk;

#[allow(warnings)]
mod auto;

pub use auto::*;
