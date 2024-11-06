// Generated by gir (https://github.com/gtk-rs/gir @ 0cdde9fbfd9d)
// from ../../gir/gir-files (@ e6660f4e9430)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(clippy::approx_constant, clippy::type_complexity, clippy::unreadable_literal, clippy::upper_case_acronyms)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use glib_sys as glib;
use gobject_sys as gobject;

#[allow(unused_imports)]
use std::ffi::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong, c_void};
#[allow(unused_imports)]
use libc::{size_t, ssize_t, time_t, off_t, intptr_t, uintptr_t, FILE};
#[cfg(unix)]
#[allow(unused_imports)]
use libc::{dev_t, gid_t, pid_t, socklen_t, uid_t, stat, in6_addr};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Constants
pub const ASTAL_RIVER_MAJOR_VERSION: c_int = 0;
pub const ASTAL_RIVER_MICRO_VERSION: c_int = 0;
pub const ASTAL_RIVER_MINOR_VERSION: c_int = 1;
pub const ASTAL_RIVER_VERSION: &[u8] = b"0.1.0\0";

// Callbacks
pub type AstalRiverCommandCallback = Option<unsafe extern "C" fn(gboolean, *const c_char)>;

// Records
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalRiverOutputClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for AstalRiverOutputClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalRiverOutputClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalRiverRiverClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for AstalRiverRiverClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalRiverRiverClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

// Classes
#[repr(C)]
#[allow(dead_code)]
pub struct AstalRiverOutput {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for AstalRiverOutput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalRiverOutput @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct AstalRiverRiver {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for AstalRiverRiver {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalRiverRiver @ {self:p}"))
         .finish()
    }
}

extern "C" {

    //=========================================================================
    // AstalRiverOutput
    //=========================================================================
    pub fn astal_river_output_get_type() -> GType;
    pub fn astal_river_output_get_focused_tags(self_: *mut AstalRiverOutput) -> c_uint;
    pub fn astal_river_output_get_focused_view(self_: *mut AstalRiverOutput) -> *mut c_char;
    pub fn astal_river_output_get_id(self_: *mut AstalRiverOutput) -> c_uint;
    pub fn astal_river_output_get_layout_name(self_: *mut AstalRiverOutput) -> *mut c_char;
    pub fn astal_river_output_get_name(self_: *mut AstalRiverOutput) -> *mut c_char;
    pub fn astal_river_output_get_occupied_tags(self_: *mut AstalRiverOutput) -> c_uint;
    pub fn astal_river_output_get_urgent_tags(self_: *mut AstalRiverOutput) -> c_uint;
    pub fn astal_river_output_set_focused_tags(self_: *mut AstalRiverOutput, tags: c_uint);

    //=========================================================================
    // AstalRiverRiver
    //=========================================================================
    pub fn astal_river_river_get_type() -> GType;
    pub fn astal_river_river_new() -> *mut AstalRiverRiver;
    pub fn astal_river_river_get_default() -> *mut AstalRiverRiver;
    pub fn astal_river_river_get_focused_output(self_: *mut AstalRiverRiver) -> *mut c_char;
    pub fn astal_river_river_get_focused_view(self_: *mut AstalRiverRiver) -> *mut c_char;
    pub fn astal_river_river_get_mode(self_: *mut AstalRiverRiver) -> *mut c_char;
    pub fn astal_river_river_get_output(self_: *mut AstalRiverRiver, name: *mut c_char) -> *mut AstalRiverOutput;
    pub fn astal_river_river_get_outputs(self_: *mut AstalRiverRiver) -> *mut glib::GList;
    pub fn astal_river_river_run_command_async(self_: *mut AstalRiverRiver, length: c_int, cmd: *mut *const c_char, callback: AstalRiverCommandCallback);

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn astal_river_get_default() -> *mut AstalRiverRiver;

}
