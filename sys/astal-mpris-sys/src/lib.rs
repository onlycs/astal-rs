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
pub type AstalMprisLoop = c_int;
pub const ASTAL_MPRIS_LOOP_UNSUPPORTED: AstalMprisLoop = 0;
pub const ASTAL_MPRIS_LOOP_NONE: AstalMprisLoop = 1;
pub const ASTAL_MPRIS_LOOP_TRACK: AstalMprisLoop = 2;
pub const ASTAL_MPRIS_LOOP_PLAYLIST: AstalMprisLoop = 3;

pub type AstalMprisPlaybackStatus = c_int;
pub const ASTAL_MPRIS_PLAYBACK_STATUS_PLAYING: AstalMprisPlaybackStatus = 0;
pub const ASTAL_MPRIS_PLAYBACK_STATUS_PAUSED: AstalMprisPlaybackStatus = 1;
pub const ASTAL_MPRIS_PLAYBACK_STATUS_STOPPED: AstalMprisPlaybackStatus = 2;

pub type AstalMprisShuffle = c_int;
pub const ASTAL_MPRIS_SHUFFLE_UNSUPPORTED: AstalMprisShuffle = 0;
pub const ASTAL_MPRIS_SHUFFLE_ON: AstalMprisShuffle = 1;
pub const ASTAL_MPRIS_SHUFFLE_OFF: AstalMprisShuffle = 2;

// Constants
pub const ASTAL_MPRIS_MAJOR_VERSION: c_int = 0;
pub const ASTAL_MPRIS_MINOR_VERSION: c_int = 1;
pub const ASTAL_MPRIS_MICRO_VERSION: c_int = 0;
pub const ASTAL_MPRIS_VERSION: &[u8] = b"0.1.0\0";

// Records
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalMprisMprisClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for AstalMprisMprisClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalMprisMprisClass @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct _AstalMprisMprisPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type AstalMprisMprisPrivate = _AstalMprisMprisPrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalMprisPlayerClass {
    pub parent_class: gobject::GObjectClass,
    pub appeared: Option<unsafe extern "C" fn(*mut AstalMprisPlayer)>,
    pub closed: Option<unsafe extern "C" fn(*mut AstalMprisPlayer)>,
}

impl ::std::fmt::Debug for AstalMprisPlayerClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalMprisPlayerClass @ {self:p}"))
         .field("appeared", &self.appeared)
         .field("closed", &self.closed)
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct _AstalMprisPlayerPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type AstalMprisPlayerPrivate = _AstalMprisPlayerPrivate;

// Classes
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalMprisMpris {
    pub parent_instance: gobject::GObject,
    pub priv_: *mut AstalMprisMprisPrivate,
}

impl ::std::fmt::Debug for AstalMprisMpris {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalMprisMpris @ {self:p}"))
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalMprisPlayer {
    pub parent_instance: gobject::GObject,
    pub priv_: *mut AstalMprisPlayerPrivate,
}

impl ::std::fmt::Debug for AstalMprisPlayer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalMprisPlayer @ {self:p}"))
         .finish()
    }
}

extern "C" {

    //=========================================================================
    // AstalMprisLoop
    //=========================================================================
    pub fn astal_mpris_loop_get_type() -> GType;

    //=========================================================================
    // AstalMprisPlaybackStatus
    //=========================================================================
    pub fn astal_mpris_playback_status_get_type() -> GType;

    //=========================================================================
    // AstalMprisShuffle
    //=========================================================================
    pub fn astal_mpris_shuffle_get_type() -> GType;

    //=========================================================================
    // AstalMprisMpris
    //=========================================================================
    pub fn astal_mpris_mpris_get_type() -> GType;
    pub fn astal_mpris_mpris_get_default() -> *mut AstalMprisMpris;
    pub fn astal_mpris_mpris_new() -> *mut AstalMprisMpris;
    pub fn astal_mpris_mpris_get_players(self_: *mut AstalMprisMpris) -> *mut glib::GList;

    //=========================================================================
    // AstalMprisPlayer
    //=========================================================================
    pub fn astal_mpris_player_get_type() -> GType;
    pub fn astal_mpris_player_raise(self_: *mut AstalMprisPlayer);
    pub fn astal_mpris_player_quit(self_: *mut AstalMprisPlayer);
    pub fn astal_mpris_player_toggle_fullscreen(self_: *mut AstalMprisPlayer);
    pub fn astal_mpris_player_next(self_: *mut AstalMprisPlayer);
    pub fn astal_mpris_player_previous(self_: *mut AstalMprisPlayer);
    pub fn astal_mpris_player_pause(self_: *mut AstalMprisPlayer);
    pub fn astal_mpris_player_play_pause(self_: *mut AstalMprisPlayer);
    pub fn astal_mpris_player_stop(self_: *mut AstalMprisPlayer);
    pub fn astal_mpris_player_play(self_: *mut AstalMprisPlayer);
    pub fn astal_mpris_player_open_uri(self_: *mut AstalMprisPlayer, uri: *const c_char);
    pub fn astal_mpris_player_loop(self_: *mut AstalMprisPlayer);
    pub fn astal_mpris_player_shuffle(self_: *mut AstalMprisPlayer);
    pub fn _astal_mpris_player_get_position(self_: *mut AstalMprisPlayer) -> c_double;
    pub fn astal_mpris_player_new(name: *const c_char) -> *mut AstalMprisPlayer;
    pub fn astal_mpris_player_get_meta(self_: *mut AstalMprisPlayer, key: *const c_char) -> *mut glib::GVariant;
    pub fn astal_mpris_player_try_proxy(self_: *mut AstalMprisPlayer, error: *mut *mut glib::GError);
    pub fn astal_mpris_player_get_bus_name(self_: *mut AstalMprisPlayer) -> *mut c_char;
    pub fn astal_mpris_player_set_bus_name(self_: *mut AstalMprisPlayer, value: *const c_char);
    pub fn astal_mpris_player_get_available(self_: *mut AstalMprisPlayer) -> gboolean;
    pub fn astal_mpris_player_get_can_quit(self_: *mut AstalMprisPlayer) -> gboolean;
    pub fn astal_mpris_player_get_fullscreen(self_: *mut AstalMprisPlayer) -> gboolean;
    pub fn astal_mpris_player_get_can_set_fullscreen(self_: *mut AstalMprisPlayer) -> gboolean;
    pub fn astal_mpris_player_get_can_raise(self_: *mut AstalMprisPlayer) -> gboolean;
    pub fn astal_mpris_player_get_has_track_list(self_: *mut AstalMprisPlayer) -> gboolean;
    pub fn astal_mpris_player_get_identity(self_: *mut AstalMprisPlayer) -> *mut c_char;
    pub fn astal_mpris_player_get_entry(self_: *mut AstalMprisPlayer) -> *mut c_char;
    pub fn astal_mpris_player_get_supported_uri_schemas(self_: *mut AstalMprisPlayer, result_length1: *mut c_int) -> *mut *mut c_char;
    pub fn astal_mpris_player_get_supported_mime_types(self_: *mut AstalMprisPlayer, result_length1: *mut c_int) -> *mut *mut c_char;
    pub fn astal_mpris_player_get_loop_status(self_: *mut AstalMprisPlayer) -> AstalMprisLoop;
    pub fn astal_mpris_player_set_loop_status(self_: *mut AstalMprisPlayer, value: AstalMprisLoop);
    pub fn astal_mpris_player_get_rate(self_: *mut AstalMprisPlayer) -> c_double;
    pub fn astal_mpris_player_set_rate(self_: *mut AstalMprisPlayer, value: c_double);
    pub fn astal_mpris_player_get_shuffle_status(self_: *mut AstalMprisPlayer) -> AstalMprisShuffle;
    pub fn astal_mpris_player_set_shuffle_status(self_: *mut AstalMprisPlayer, value: AstalMprisShuffle);
    pub fn astal_mpris_player_get_volume(self_: *mut AstalMprisPlayer) -> c_double;
    pub fn astal_mpris_player_set_volume(self_: *mut AstalMprisPlayer, value: c_double);
    pub fn astal_mpris_player_get_position(self_: *mut AstalMprisPlayer) -> c_double;
    pub fn astal_mpris_player_set_position(self_: *mut AstalMprisPlayer, value: c_double);
    pub fn astal_mpris_player_get_playback_status(self_: *mut AstalMprisPlayer) -> AstalMprisPlaybackStatus;
    pub fn astal_mpris_player_get_minimum_rate(self_: *mut AstalMprisPlayer) -> c_double;
    pub fn astal_mpris_player_get_maximum_rate(self_: *mut AstalMprisPlayer) -> c_double;
    pub fn astal_mpris_player_get_can_go_next(self_: *mut AstalMprisPlayer) -> gboolean;
    pub fn astal_mpris_player_get_can_go_previous(self_: *mut AstalMprisPlayer) -> gboolean;
    pub fn astal_mpris_player_get_can_play(self_: *mut AstalMprisPlayer) -> gboolean;
    pub fn astal_mpris_player_get_can_pause(self_: *mut AstalMprisPlayer) -> gboolean;
    pub fn astal_mpris_player_get_can_seek(self_: *mut AstalMprisPlayer) -> gboolean;
    pub fn astal_mpris_player_get_can_control(self_: *mut AstalMprisPlayer) -> gboolean;
    pub fn astal_mpris_player_get_metadata(self_: *mut AstalMprisPlayer) -> *mut glib::GHashTable;
    pub fn astal_mpris_player_get_trackid(self_: *mut AstalMprisPlayer) -> *mut c_char;
    pub fn astal_mpris_player_get_length(self_: *mut AstalMprisPlayer) -> c_double;
    pub fn astal_mpris_player_get_art_url(self_: *mut AstalMprisPlayer) -> *mut c_char;
    pub fn astal_mpris_player_get_album(self_: *mut AstalMprisPlayer) -> *mut c_char;
    pub fn astal_mpris_player_get_album_artist(self_: *mut AstalMprisPlayer) -> *mut c_char;
    pub fn astal_mpris_player_get_artist(self_: *mut AstalMprisPlayer) -> *mut c_char;
    pub fn astal_mpris_player_get_lyrics(self_: *mut AstalMprisPlayer) -> *mut c_char;
    pub fn astal_mpris_player_get_title(self_: *mut AstalMprisPlayer) -> *mut c_char;
    pub fn astal_mpris_player_get_composer(self_: *mut AstalMprisPlayer) -> *mut c_char;
    pub fn astal_mpris_player_get_comments(self_: *mut AstalMprisPlayer) -> *mut c_char;
    pub fn astal_mpris_player_get_cover_art(self_: *mut AstalMprisPlayer) -> *mut c_char;

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn astal_mpris_playback_status_from_string(str: *const c_char) -> AstalMprisPlaybackStatus;
    pub fn astal_mpris_playback_status_to_string(self_: AstalMprisPlaybackStatus) -> *mut c_char;
    pub fn astal_mpris_loop_from_string(str: *const c_char) -> AstalMprisLoop;
    pub fn astal_mpris_loop_to_string(self_: AstalMprisLoop) -> *mut c_char;
    pub fn astal_mpris_shuffle_from_bool(b: gboolean) -> AstalMprisShuffle;
    pub fn astal_mpris_shuffle_to_string(self_: AstalMprisShuffle) -> *mut c_char;
    pub fn astal_mpris_get_default() -> *mut AstalMprisMpris;

}
