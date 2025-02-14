// Generated by gir (https://github.com/gtk-rs/gir @ 26e721588f2f)
// from ../../gir/gir-files (@ 29f90b13f748+)
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
pub const ASTAL_POWER_PROFILES_MAJOR_VERSION: c_int = 0;
pub const ASTAL_POWER_PROFILES_MINOR_VERSION: c_int = 1;
pub const ASTAL_POWER_PROFILES_MICRO_VERSION: c_int = 0;
pub const ASTAL_POWER_PROFILES_VERSION: &[u8] = b"0.1.0\0";

// Records
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalPowerProfilesHold {
    pub application_id: *mut c_char,
    pub profile: *mut c_char,
    pub reason: *mut c_char,
}

impl ::std::fmt::Debug for AstalPowerProfilesHold {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalPowerProfilesHold @ {self:p}"))
         .field("application_id", &self.application_id)
         .field("profile", &self.profile)
         .field("reason", &self.reason)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalPowerProfilesPowerProfilesClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for AstalPowerProfilesPowerProfilesClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalPowerProfilesPowerProfilesClass @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct _AstalPowerProfilesPowerProfilesPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type AstalPowerProfilesPowerProfilesPrivate = _AstalPowerProfilesPowerProfilesPrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalPowerProfilesProfile {
    pub profile: *mut c_char,
    pub cpu_driver: *mut c_char,
    pub platform_driver: *mut c_char,
    pub driver: *mut c_char,
}

impl ::std::fmt::Debug for AstalPowerProfilesProfile {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalPowerProfilesProfile @ {self:p}"))
         .field("profile", &self.profile)
         .field("cpu_driver", &self.cpu_driver)
         .field("platform_driver", &self.platform_driver)
         .field("driver", &self.driver)
         .finish()
    }
}

// Classes
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalPowerProfilesPowerProfiles {
    pub parent_instance: gobject::GObject,
    pub priv_: *mut AstalPowerProfilesPowerProfilesPrivate,
}

impl ::std::fmt::Debug for AstalPowerProfilesPowerProfiles {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalPowerProfilesPowerProfiles @ {self:p}"))
         .finish()
    }
}

extern "C" {

    //=========================================================================
    // AstalPowerProfilesHold
    //=========================================================================
    pub fn astal_power_profiles_hold_get_type() -> GType;

    //=========================================================================
    // AstalPowerProfilesProfile
    //=========================================================================
    pub fn astal_power_profiles_profile_get_type() -> GType;

    //=========================================================================
    // AstalPowerProfilesPowerProfiles
    //=========================================================================
    pub fn astal_power_profiles_power_profiles_get_type() -> GType;
    pub fn astal_power_profiles_power_profiles_get_default() -> *mut AstalPowerProfilesPowerProfiles;
    pub fn astal_power_profiles_power_profiles_hold_profile(self_: *mut AstalPowerProfilesPowerProfiles, profile: *const c_char, reason: *const c_char, application_id: *const c_char) -> c_int;
    pub fn astal_power_profiles_power_profiles_release_profile(self_: *mut AstalPowerProfilesPowerProfiles, cookie: c_uint);
    pub fn astal_power_profiles_power_profiles_new() -> *mut AstalPowerProfilesPowerProfiles;
    pub fn astal_power_profiles_power_profiles_get_active_profile(self_: *mut AstalPowerProfilesPowerProfiles) -> *mut c_char;
    pub fn astal_power_profiles_power_profiles_set_active_profile(self_: *mut AstalPowerProfilesPowerProfiles, value: *const c_char);
    pub fn astal_power_profiles_power_profiles_get_icon_name(self_: *mut AstalPowerProfilesPowerProfiles) -> *mut c_char;
    pub fn astal_power_profiles_power_profiles_get_actions(self_: *mut AstalPowerProfilesPowerProfiles, result_length1: *mut c_int) -> *mut *mut c_char;
    pub fn astal_power_profiles_power_profiles_get_active_profile_holds(self_: *mut AstalPowerProfilesPowerProfiles, result_length1: *mut c_int) -> *mut AstalPowerProfilesHold;
    pub fn astal_power_profiles_power_profiles_get_performance_degraded(self_: *mut AstalPowerProfilesPowerProfiles) -> *mut c_char;
    pub fn astal_power_profiles_power_profiles_get_profiles(self_: *mut AstalPowerProfilesPowerProfiles, result_length1: *mut c_int) -> *mut AstalPowerProfilesProfile;
    pub fn astal_power_profiles_power_profiles_get_version(self_: *mut AstalPowerProfilesPowerProfiles) -> *mut c_char;

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn astal_power_profiles_get_default() -> *mut AstalPowerProfilesPowerProfiles;

}
