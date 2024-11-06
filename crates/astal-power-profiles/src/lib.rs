#[macro_use]
extern crate gir_error;
extern crate astal_power_profiles_sys as ffi;
extern crate gio;
extern crate glib;
extern crate gtk;

#[allow(warnings)]
mod auto;

pub use auto::*;
