#[macro_use]
extern crate gir_error;
pub extern crate astal_battery_sys as ffi;
extern crate gio;
extern crate glib;
extern crate gtk;

#[allow(warnings)]
mod auto;

pub use auto::*;
