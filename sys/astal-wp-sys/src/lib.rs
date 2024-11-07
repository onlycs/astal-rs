// Generated by gir (https://github.com/gtk-rs/gir @ 0cdde9fbfd9d)
// from ../../gir/gir-files (@ e6660f4e9430+)
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

// Enums
pub type AstalWpDeviceType = c_int;
pub const ASTAL_WP_DEVICE_TYPE_AUDIO: AstalWpDeviceType = 0;
pub const ASTAL_WP_DEVICE_TYPE_VIDEO: AstalWpDeviceType = 1;

pub type AstalWpMediaClass = c_int;
pub const ASTAL_WP_MEDIA_CLASS_AUDIO_MICROPHONE: AstalWpMediaClass = 0;
pub const ASTAL_WP_MEDIA_CLASS_AUDIO_SPEAKER: AstalWpMediaClass = 1;
pub const ASTAL_WP_MEDIA_CLASS_AUDIO_RECORDER: AstalWpMediaClass = 2;
pub const ASTAL_WP_MEDIA_CLASS_AUDIO_STREAM: AstalWpMediaClass = 3;
pub const ASTAL_WP_MEDIA_CLASS_VIDEO_SOURCE: AstalWpMediaClass = 4;
pub const ASTAL_WP_MEDIA_CLASS_VIDEO_SINK: AstalWpMediaClass = 5;
pub const ASTAL_WP_MEDIA_CLASS_VIDEO_RECORDER: AstalWpMediaClass = 6;
pub const ASTAL_WP_MEDIA_CLASS_VIDEO_STREAM: AstalWpMediaClass = 7;

pub type AstalWpScale = c_int;
pub const ASTAL_WP_SCALE_LINEAR: AstalWpScale = 0;
pub const ASTAL_WP_SCALE_CUBIC: AstalWpScale = 1;

// Constants
pub const ASTAL_WP_MAJOR_VERSION: c_int = 0;
pub const ASTAL_WP_MICRO_VERSION: c_int = 0;
pub const ASTAL_WP_MINOR_VERSION: c_int = 1;
pub const ASTAL_WP_VERSION: &[u8] = b"0.1.0\0";

// Records
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalWpAudioClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for AstalWpAudioClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalWpAudioClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalWpDeviceClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for AstalWpDeviceClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalWpDeviceClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalWpEndpointClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for AstalWpEndpointClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalWpEndpointClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalWpProfileClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for AstalWpProfileClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalWpProfileClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalWpVideoClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for AstalWpVideoClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalWpVideoClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalWpWpClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for AstalWpWpClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalWpWpClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

// Classes
#[repr(C)]
#[allow(dead_code)]
pub struct AstalWpAudio {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for AstalWpAudio {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalWpAudio @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct AstalWpDevice {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for AstalWpDevice {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalWpDevice @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct AstalWpEndpoint {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for AstalWpEndpoint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalWpEndpoint @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct AstalWpProfile {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for AstalWpProfile {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalWpProfile @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct AstalWpVideo {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for AstalWpVideo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalWpVideo @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct AstalWpWp {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for AstalWpWp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalWpWp @ {self:p}"))
         .finish()
    }
}

extern "C" {

    //=========================================================================
    // AstalWpAudio
    //=========================================================================
    pub fn astal_wp_audio_get_type() -> GType;
    pub fn astal_wp_audio_new(wp: *mut AstalWpWp) -> *mut AstalWpAudio;
    pub fn astal_wp_audio_get_default_microphone(self_: *mut AstalWpAudio) -> *mut AstalWpEndpoint;
    pub fn astal_wp_audio_get_default_speaker(self_: *mut AstalWpAudio) -> *mut AstalWpEndpoint;
    pub fn astal_wp_audio_get_device(self_: *mut AstalWpAudio, id: c_uint) -> *mut AstalWpDevice;
    pub fn astal_wp_audio_get_devices(self_: *mut AstalWpAudio) -> *mut glib::GList;
    pub fn astal_wp_audio_get_endpoint(self_: *mut AstalWpAudio, id: c_uint) -> *mut AstalWpEndpoint;
    pub fn astal_wp_audio_get_microphone(self_: *mut AstalWpAudio, id: c_uint) -> *mut AstalWpEndpoint;
    pub fn astal_wp_audio_get_microphones(self_: *mut AstalWpAudio) -> *mut glib::GList;
    pub fn astal_wp_audio_get_recorder(self_: *mut AstalWpAudio, id: c_uint) -> *mut AstalWpEndpoint;
    pub fn astal_wp_audio_get_recorders(self_: *mut AstalWpAudio) -> *mut glib::GList;
    pub fn astal_wp_audio_get_speaker(self_: *mut AstalWpAudio, id: c_uint) -> *mut AstalWpEndpoint;
    pub fn astal_wp_audio_get_speakers(self_: *mut AstalWpAudio) -> *mut glib::GList;
    pub fn astal_wp_audio_get_stream(self_: *mut AstalWpAudio, id: c_uint) -> *mut AstalWpEndpoint;
    pub fn astal_wp_audio_get_streams(self_: *mut AstalWpAudio) -> *mut glib::GList;

    //=========================================================================
    // AstalWpDevice
    //=========================================================================
    pub fn astal_wp_device_get_type() -> GType;
    pub fn astal_wp_device_get_active_profile(self_: *mut AstalWpDevice) -> c_int;
    pub fn astal_wp_device_get_description(self_: *mut AstalWpDevice) -> *const c_char;
    pub fn astal_wp_device_get_device_type(self_: *mut AstalWpDevice) -> AstalWpDeviceType;
    pub fn astal_wp_device_get_icon(self_: *mut AstalWpDevice) -> *const c_char;
    pub fn astal_wp_device_get_id(self_: *mut AstalWpDevice) -> c_uint;
    pub fn astal_wp_device_get_profile(self_: *mut AstalWpDevice, id: c_int) -> *mut AstalWpProfile;
    pub fn astal_wp_device_get_profiles(self_: *mut AstalWpDevice) -> *mut glib::GList;
    pub fn astal_wp_device_set_active_profile(self_: *mut AstalWpDevice, profile_id: c_int);

    //=========================================================================
    // AstalWpEndpoint
    //=========================================================================
    pub fn astal_wp_endpoint_get_type() -> GType;
    pub fn astal_wp_endpoint_get_description(self_: *mut AstalWpEndpoint) -> *const c_char;
    pub fn astal_wp_endpoint_get_icon(self_: *mut AstalWpEndpoint) -> *const c_char;
    pub fn astal_wp_endpoint_get_id(self_: *mut AstalWpEndpoint) -> c_uint;
    pub fn astal_wp_endpoint_get_is_default(self_: *mut AstalWpEndpoint) -> gboolean;
    pub fn astal_wp_endpoint_get_lock_channels(self_: *mut AstalWpEndpoint) -> gboolean;
    pub fn astal_wp_endpoint_get_media_class(self_: *mut AstalWpEndpoint) -> AstalWpMediaClass;
    pub fn astal_wp_endpoint_get_mute(self_: *mut AstalWpEndpoint) -> gboolean;
    pub fn astal_wp_endpoint_get_name(self_: *mut AstalWpEndpoint) -> *const c_char;
    pub fn astal_wp_endpoint_get_volume(self_: *mut AstalWpEndpoint) -> c_double;
    pub fn astal_wp_endpoint_get_volume_icon(self_: *mut AstalWpEndpoint) -> *const c_char;
    pub fn astal_wp_endpoint_set_is_default(self_: *mut AstalWpEndpoint, is_default: gboolean);
    pub fn astal_wp_endpoint_set_lock_channels(self_: *mut AstalWpEndpoint, lock_channels: gboolean);
    pub fn astal_wp_endpoint_set_mute(self_: *mut AstalWpEndpoint, mute: gboolean);
    pub fn astal_wp_endpoint_set_volume(self_: *mut AstalWpEndpoint, volume: c_double);

    //=========================================================================
    // AstalWpProfile
    //=========================================================================
    pub fn astal_wp_profile_get_type() -> GType;
    pub fn astal_wp_profile_get_description(self_: *mut AstalWpProfile) -> *const c_char;
    pub fn astal_wp_profile_get_index(self_: *mut AstalWpProfile) -> c_int;

    //=========================================================================
    // AstalWpVideo
    //=========================================================================
    pub fn astal_wp_video_get_type() -> GType;
    pub fn astal_wp_video_new(wp: *mut AstalWpWp) -> *mut AstalWpVideo;
    pub fn astal_wp_video_get_device(self_: *mut AstalWpVideo, id: c_uint) -> *mut AstalWpDevice;
    pub fn astal_wp_video_get_devices(self_: *mut AstalWpVideo) -> *mut glib::GList;
    pub fn astal_wp_video_get_recorder(self_: *mut AstalWpVideo, id: c_uint) -> *mut AstalWpEndpoint;
    pub fn astal_wp_video_get_recorders(self_: *mut AstalWpVideo) -> *mut glib::GList;
    pub fn astal_wp_video_get_sink(self_: *mut AstalWpVideo, id: c_uint) -> *mut AstalWpEndpoint;
    pub fn astal_wp_video_get_sinks(self_: *mut AstalWpVideo) -> *mut glib::GList;
    pub fn astal_wp_video_get_source(self_: *mut AstalWpVideo, id: c_uint) -> *mut AstalWpEndpoint;
    pub fn astal_wp_video_get_sources(self_: *mut AstalWpVideo) -> *mut glib::GList;
    pub fn astal_wp_video_get_stream(self_: *mut AstalWpVideo, id: c_uint) -> *mut AstalWpEndpoint;
    pub fn astal_wp_video_get_streams(self_: *mut AstalWpVideo) -> *mut glib::GList;

    //=========================================================================
    // AstalWpWp
    //=========================================================================
    pub fn astal_wp_wp_get_type() -> GType;
    pub fn astal_wp_wp_get_default() -> *mut AstalWpWp;
    pub fn astal_wp_wp_get_audio(self_: *mut AstalWpWp) -> *mut AstalWpAudio;
    pub fn astal_wp_wp_get_default_microphone(self_: *mut AstalWpWp) -> *mut AstalWpEndpoint;
    pub fn astal_wp_wp_get_default_speaker(self_: *mut AstalWpWp) -> *mut AstalWpEndpoint;
    pub fn astal_wp_wp_get_device(self_: *mut AstalWpWp, id: c_uint) -> *mut AstalWpDevice;
    pub fn astal_wp_wp_get_devices(self_: *mut AstalWpWp) -> *mut glib::GList;
    pub fn astal_wp_wp_get_endpoint(self_: *mut AstalWpWp, id: c_uint) -> *mut AstalWpEndpoint;
    pub fn astal_wp_wp_get_endpoints(self_: *mut AstalWpWp) -> *mut glib::GList;
    pub fn astal_wp_wp_get_scale(self_: *mut AstalWpWp) -> AstalWpScale;
    pub fn astal_wp_wp_get_video(self_: *mut AstalWpWp) -> *mut AstalWpVideo;
    pub fn astal_wp_wp_set_scale(self_: *mut AstalWpWp, scale: AstalWpScale);

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn astal_wp_get_default() -> *mut AstalWpWp;

}
