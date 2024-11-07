// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../../gir/gir-files
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(clippy::approx_constant, clippy::type_complexity, clippy::unreadable_literal, clippy::upper_case_acronyms)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use gtk_sys as gtk;
use glib_sys as glib;
use gdk_sys as gdk;
use gio_sys as gio;
use gdk_pixbuf_sys as gdk_pixbuf;

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
pub type AstalExclusivity = c_int;
pub const ASTAL_EXCLUSIVITY_NORMAL: AstalExclusivity = 0;
pub const ASTAL_EXCLUSIVITY_EXCLUSIVE: AstalExclusivity = 1;
pub const ASTAL_EXCLUSIVITY_IGNORE: AstalExclusivity = 2;

pub type AstalKeymode = c_int;
pub const ASTAL_KEYMODE_NONE: AstalKeymode = 0;
pub const ASTAL_KEYMODE_EXCLUSIVE: AstalKeymode = 1;
pub const ASTAL_KEYMODE_ON_DEMAND: AstalKeymode = 2;

pub type AstalLayer = c_int;
pub const ASTAL_LAYER_BACKGROUND: AstalLayer = 0;
pub const ASTAL_LAYER_BOTTOM: AstalLayer = 1;
pub const ASTAL_LAYER_TOP: AstalLayer = 2;
pub const ASTAL_LAYER_OVERLAY: AstalLayer = 3;

pub type AstalMouseButton = c_int;
pub const ASTAL_MOUSE_BUTTON_PRIMARY: AstalMouseButton = 1;
pub const ASTAL_MOUSE_BUTTON_MIDDLE: AstalMouseButton = 2;
pub const ASTAL_MOUSE_BUTTON_SECONDARY: AstalMouseButton = 3;
pub const ASTAL_MOUSE_BUTTON_BACK: AstalMouseButton = 4;
pub const ASTAL_MOUSE_BUTTON_FORWARD: AstalMouseButton = 5;

pub type AstalWindowAnchor = c_int;
pub const ASTAL_WINDOW_ANCHOR_NONE: AstalWindowAnchor = 0;
pub const ASTAL_WINDOW_ANCHOR_TOP: AstalWindowAnchor = 2;
pub const ASTAL_WINDOW_ANCHOR_RIGHT: AstalWindowAnchor = 4;
pub const ASTAL_WINDOW_ANCHOR_LEFT: AstalWindowAnchor = 8;
pub const ASTAL_WINDOW_ANCHOR_BOTTOM: AstalWindowAnchor = 16;

// Constants
pub const ASTAL_MAJOR_VERSION: c_int = 3;
pub const ASTAL_MINOR_VERSION: c_int = 0;
pub const ASTAL_MICRO_VERSION: c_int = 0;
pub const ASTAL_VERSION: &[u8] = b"3.0.0\0";

// Records
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalApplicationClass {
    pub parent_class: gtk::GtkApplicationClass,
    pub request: Option<unsafe extern "C" fn(*mut AstalApplication, *const c_char, *mut gio::GSocketConnection)>,
}

impl ::std::fmt::Debug for AstalApplicationClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalApplicationClass @ {self:p}"))
         .field("request", &self.request)
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct _AstalApplicationPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type AstalApplicationPrivate = _AstalApplicationPrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalBoxClass {
    pub parent_class: gtk::GtkBoxClass,
}

impl ::std::fmt::Debug for AstalBoxClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalBoxClass @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct _AstalBoxPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type AstalBoxPrivate = _AstalBoxPrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalButtonClass {
    pub parent_class: gtk::GtkButtonClass,
}

impl ::std::fmt::Debug for AstalButtonClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalButtonClass @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct _AstalButtonPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type AstalButtonPrivate = _AstalButtonPrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalCenterBoxClass {
    pub parent_class: gtk::GtkBoxClass,
}

impl ::std::fmt::Debug for AstalCenterBoxClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalCenterBoxClass @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct _AstalCenterBoxPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type AstalCenterBoxPrivate = _AstalCenterBoxPrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalCircularProgressClass {
    pub parent_class: gtk::GtkBinClass,
}

impl ::std::fmt::Debug for AstalCircularProgressClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalCircularProgressClass @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct _AstalCircularProgressPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type AstalCircularProgressPrivate = _AstalCircularProgressPrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalClickEvent {
    pub release: gboolean,
    pub time: c_uint,
    pub x: c_double,
    pub y: c_double,
    pub modifier: gdk::GdkModifierType,
    pub button: AstalMouseButton,
}

impl ::std::fmt::Debug for AstalClickEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalClickEvent @ {self:p}"))
         .field("release", &self.release)
         .field("time", &self.time)
         .field("x", &self.x)
         .field("y", &self.y)
         .field("modifier", &self.modifier)
         .field("button", &self.button)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalEventBoxClass {
    pub parent_class: gtk::GtkEventBoxClass,
}

impl ::std::fmt::Debug for AstalEventBoxClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalEventBoxClass @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct _AstalEventBoxPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type AstalEventBoxPrivate = _AstalEventBoxPrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalHoverEvent {
    pub lost: gboolean,
    pub time: c_uint,
    pub x: c_double,
    pub y: c_double,
    pub modifier: gdk::GdkModifierType,
    pub mode: gdk::GdkCrossingMode,
    pub detail: gdk::GdkNotifyType,
}

impl ::std::fmt::Debug for AstalHoverEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalHoverEvent @ {self:p}"))
         .field("lost", &self.lost)
         .field("time", &self.time)
         .field("x", &self.x)
         .field("y", &self.y)
         .field("modifier", &self.modifier)
         .field("mode", &self.mode)
         .field("detail", &self.detail)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalIconClass {
    pub parent_class: gtk::GtkImageClass,
}

impl ::std::fmt::Debug for AstalIconClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalIconClass @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct _AstalIconPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type AstalIconPrivate = _AstalIconPrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalLabelClass {
    pub parent_class: gtk::GtkLabelClass,
}

impl ::std::fmt::Debug for AstalLabelClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalLabelClass @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct _AstalLabelPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type AstalLabelPrivate = _AstalLabelPrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalLevelBarClass {
    pub parent_class: gtk::GtkLevelBarClass,
}

impl ::std::fmt::Debug for AstalLevelBarClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalLevelBarClass @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct _AstalLevelBarPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type AstalLevelBarPrivate = _AstalLevelBarPrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalMotionEvent {
    pub time: c_uint,
    pub x: c_double,
    pub y: c_double,
    pub modifier: gdk::GdkModifierType,
}

impl ::std::fmt::Debug for AstalMotionEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalMotionEvent @ {self:p}"))
         .field("time", &self.time)
         .field("x", &self.x)
         .field("y", &self.y)
         .field("modifier", &self.modifier)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalOverlayClass {
    pub parent_class: gtk::GtkOverlayClass,
}

impl ::std::fmt::Debug for AstalOverlayClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalOverlayClass @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct _AstalOverlayPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type AstalOverlayPrivate = _AstalOverlayPrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalScrollEvent {
    pub time: c_uint,
    pub x: c_double,
    pub y: c_double,
    pub modifier: gdk::GdkModifierType,
    pub direction: gdk::GdkScrollDirection,
    pub delta_x: c_double,
    pub delta_y: c_double,
}

impl ::std::fmt::Debug for AstalScrollEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalScrollEvent @ {self:p}"))
         .field("time", &self.time)
         .field("x", &self.x)
         .field("y", &self.y)
         .field("modifier", &self.modifier)
         .field("direction", &self.direction)
         .field("delta_x", &self.delta_x)
         .field("delta_y", &self.delta_y)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalScrollableClass {
    pub parent_class: gtk::GtkScrolledWindowClass,
}

impl ::std::fmt::Debug for AstalScrollableClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalScrollableClass @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct _AstalScrollablePrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type AstalScrollablePrivate = _AstalScrollablePrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalSliderClass {
    pub parent_class: gtk::GtkScaleClass,
}

impl ::std::fmt::Debug for AstalSliderClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalSliderClass @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct _AstalSliderPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type AstalSliderPrivate = _AstalSliderPrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalStackClass {
    pub parent_class: gtk::GtkStackClass,
}

impl ::std::fmt::Debug for AstalStackClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalStackClass @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct _AstalStackPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type AstalStackPrivate = _AstalStackPrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalWindowClass {
    pub parent_class: gtk::GtkWindowClass,
}

impl ::std::fmt::Debug for AstalWindowClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalWindowClass @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct _AstalWindowPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type AstalWindowPrivate = _AstalWindowPrivate;

// Classes
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalApplication {
    pub parent_instance: gtk::GtkApplication,
    pub priv_: *mut AstalApplicationPrivate,
}

impl ::std::fmt::Debug for AstalApplication {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalApplication @ {self:p}"))
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalBox {
    pub parent_instance: gtk::GtkBox,
    pub priv_: *mut AstalBoxPrivate,
}

impl ::std::fmt::Debug for AstalBox {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalBox @ {self:p}"))
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalButton {
    pub parent_instance: gtk::GtkButton,
    pub priv_: *mut AstalButtonPrivate,
}

impl ::std::fmt::Debug for AstalButton {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalButton @ {self:p}"))
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalCenterBox {
    pub parent_instance: gtk::GtkBox,
    pub priv_: *mut AstalCenterBoxPrivate,
}

impl ::std::fmt::Debug for AstalCenterBox {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalCenterBox @ {self:p}"))
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalCircularProgress {
    pub parent_instance: gtk::GtkBin,
    pub priv_: *mut AstalCircularProgressPrivate,
}

impl ::std::fmt::Debug for AstalCircularProgress {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalCircularProgress @ {self:p}"))
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalEventBox {
    pub parent_instance: gtk::GtkEventBox,
    pub priv_: *mut AstalEventBoxPrivate,
}

impl ::std::fmt::Debug for AstalEventBox {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalEventBox @ {self:p}"))
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalIcon {
    pub parent_instance: gtk::GtkImage,
    pub priv_: *mut AstalIconPrivate,
}

impl ::std::fmt::Debug for AstalIcon {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalIcon @ {self:p}"))
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalLabel {
    pub parent_instance: gtk::GtkLabel,
    pub priv_: *mut AstalLabelPrivate,
}

impl ::std::fmt::Debug for AstalLabel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalLabel @ {self:p}"))
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalLevelBar {
    pub parent_instance: gtk::GtkLevelBar,
    pub priv_: *mut AstalLevelBarPrivate,
}

impl ::std::fmt::Debug for AstalLevelBar {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalLevelBar @ {self:p}"))
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalOverlay {
    pub parent_instance: gtk::GtkOverlay,
    pub priv_: *mut AstalOverlayPrivate,
}

impl ::std::fmt::Debug for AstalOverlay {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalOverlay @ {self:p}"))
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalScrollable {
    pub parent_instance: gtk::GtkScrolledWindow,
    pub priv_: *mut AstalScrollablePrivate,
}

impl ::std::fmt::Debug for AstalScrollable {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalScrollable @ {self:p}"))
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalSlider {
    pub parent_instance: gtk::GtkScale,
    pub priv_: *mut AstalSliderPrivate,
}

impl ::std::fmt::Debug for AstalSlider {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalSlider @ {self:p}"))
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalStack {
    pub parent_instance: gtk::GtkStack,
    pub priv_: *mut AstalStackPrivate,
}

impl ::std::fmt::Debug for AstalStack {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalStack @ {self:p}"))
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalWindow {
    pub parent_instance: gtk::GtkWindow,
    pub priv_: *mut AstalWindowPrivate,
}

impl ::std::fmt::Debug for AstalWindow {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalWindow @ {self:p}"))
         .finish()
    }
}

extern "C" {

    //=========================================================================
    // AstalExclusivity
    //=========================================================================
    pub fn astal_exclusivity_get_type() -> GType;

    //=========================================================================
    // AstalKeymode
    //=========================================================================
    pub fn astal_keymode_get_type() -> GType;

    //=========================================================================
    // AstalLayer
    //=========================================================================
    pub fn astal_layer_get_type() -> GType;

    //=========================================================================
    // AstalMouseButton
    //=========================================================================
    pub fn astal_mouse_button_get_type() -> GType;

    //=========================================================================
    // AstalWindowAnchor
    //=========================================================================
    pub fn astal_window_anchor_get_type() -> GType;

    //=========================================================================
    // AstalClickEvent
    //=========================================================================
    pub fn astal_click_event_get_type() -> GType;
    pub fn astal_click_event_init(self_: *mut AstalClickEvent, event: *mut gdk::GdkEventButton);

    //=========================================================================
    // AstalHoverEvent
    //=========================================================================
    pub fn astal_hover_event_get_type() -> GType;
    pub fn astal_hover_event_init(self_: *mut AstalHoverEvent, event: *mut gdk::GdkEventCrossing);

    //=========================================================================
    // AstalMotionEvent
    //=========================================================================
    pub fn astal_motion_event_get_type() -> GType;
    pub fn astal_motion_event_init(self_: *mut AstalMotionEvent, event: *mut gdk::GdkEventMotion);

    //=========================================================================
    // AstalScrollEvent
    //=========================================================================
    pub fn astal_scroll_event_get_type() -> GType;
    pub fn astal_scroll_event_init(self_: *mut AstalScrollEvent, event: *mut gdk::GdkEventScroll);

    //=========================================================================
    // AstalApplication
    //=========================================================================
    pub fn astal_application_get_type() -> GType;
    pub fn astal_application_reset_css(self_: *mut AstalApplication);
    pub fn astal_application_get_window(self_: *mut AstalApplication, name: *const c_char) -> *mut gtk::GtkWindow;
    pub fn astal_application_apply_css(self_: *mut AstalApplication, style: *const c_char, reset: gboolean);
    pub fn astal_application_add_icons(self_: *mut AstalApplication, path: *const c_char);
    pub fn astal_application_request(self_: *mut AstalApplication, msg: *const c_char, conn: *mut gio::GSocketConnection);
    pub fn astal_application_new() -> *mut AstalApplication;
    pub fn astal_application_get_monitors(self_: *mut AstalApplication) -> *mut glib::GList;
    pub fn astal_application_get_windows(self_: *mut AstalApplication) -> *mut glib::GList;
    pub fn astal_application_get_gtk_theme(self_: *mut AstalApplication) -> *mut c_char;
    pub fn astal_application_set_gtk_theme(self_: *mut AstalApplication, value: *const c_char);
    pub fn astal_application_get_icon_theme(self_: *mut AstalApplication) -> *mut c_char;
    pub fn astal_application_set_icon_theme(self_: *mut AstalApplication, value: *const c_char);
    pub fn astal_application_get_cursor_theme(self_: *mut AstalApplication) -> *mut c_char;
    pub fn astal_application_set_cursor_theme(self_: *mut AstalApplication, value: *const c_char);

    //=========================================================================
    // AstalBox
    //=========================================================================
    pub fn astal_box_get_type() -> GType;
    pub fn astal_box_new(vertical: gboolean, children: *mut glib::GList) -> *mut AstalBox;
    pub fn astal_box_get_vertical(self_: *mut AstalBox) -> gboolean;
    pub fn astal_box_set_vertical(self_: *mut AstalBox, value: gboolean);
    pub fn astal_box_get_children(self_: *mut AstalBox) -> *mut glib::GList;
    pub fn astal_box_set_children(self_: *mut AstalBox, value: *mut glib::GList);
    pub fn astal_box_get_child(self_: *mut AstalBox) -> *mut gtk::GtkWidget;
    pub fn astal_box_set_child(self_: *mut AstalBox, value: *mut gtk::GtkWidget);

    //=========================================================================
    // AstalButton
    //=========================================================================
    pub fn astal_button_get_type() -> GType;
    pub fn astal_button_new() -> *mut AstalButton;

    //=========================================================================
    // AstalCenterBox
    //=========================================================================
    pub fn astal_center_box_get_type() -> GType;
    pub fn astal_center_box_new() -> *mut AstalCenterBox;
    pub fn astal_center_box_get_vertical(self_: *mut AstalCenterBox) -> gboolean;
    pub fn astal_center_box_set_vertical(self_: *mut AstalCenterBox, value: gboolean);
    pub fn astal_center_box_get_start_widget(self_: *mut AstalCenterBox) -> *mut gtk::GtkWidget;
    pub fn astal_center_box_set_start_widget(self_: *mut AstalCenterBox, value: *mut gtk::GtkWidget);
    pub fn astal_center_box_get_end_widget(self_: *mut AstalCenterBox) -> *mut gtk::GtkWidget;
    pub fn astal_center_box_set_end_widget(self_: *mut AstalCenterBox, value: *mut gtk::GtkWidget);
    pub fn astal_center_box_get_center_widget(self_: *mut AstalCenterBox) -> *mut gtk::GtkWidget;
    pub fn astal_center_box_set_center_widget(self_: *mut AstalCenterBox, value: *mut gtk::GtkWidget);

    //=========================================================================
    // AstalCircularProgress
    //=========================================================================
    pub fn astal_circular_progress_get_type() -> GType;
    pub fn astal_circular_progress_new() -> *mut AstalCircularProgress;
    pub fn astal_circular_progress_get_start_at(self_: *mut AstalCircularProgress) -> c_double;
    pub fn astal_circular_progress_set_start_at(self_: *mut AstalCircularProgress, value: c_double);
    pub fn astal_circular_progress_get_end_at(self_: *mut AstalCircularProgress) -> c_double;
    pub fn astal_circular_progress_set_end_at(self_: *mut AstalCircularProgress, value: c_double);
    pub fn astal_circular_progress_get_value(self_: *mut AstalCircularProgress) -> c_double;
    pub fn astal_circular_progress_set_value(self_: *mut AstalCircularProgress, value: c_double);
    pub fn astal_circular_progress_get_inverted(self_: *mut AstalCircularProgress) -> gboolean;
    pub fn astal_circular_progress_set_inverted(self_: *mut AstalCircularProgress, value: gboolean);
    pub fn astal_circular_progress_get_rounded(self_: *mut AstalCircularProgress) -> gboolean;
    pub fn astal_circular_progress_set_rounded(self_: *mut AstalCircularProgress, value: gboolean);

    //=========================================================================
    // AstalEventBox
    //=========================================================================
    pub fn astal_event_box_get_type() -> GType;
    pub fn astal_event_box_new() -> *mut AstalEventBox;

    //=========================================================================
    // AstalIcon
    //=========================================================================
    pub fn astal_icon_get_type() -> GType;
    pub fn astal_icon_lookup_icon(icon: *const c_char) -> *mut gtk::GtkIconInfo;
    pub fn astal_icon_new() -> *mut AstalIcon;
    pub fn astal_icon_get_pixbuf(self_: *mut AstalIcon) -> *mut gdk_pixbuf::GdkPixbuf;
    pub fn astal_icon_set_pixbuf(self_: *mut AstalIcon, value: *mut gdk_pixbuf::GdkPixbuf);
    pub fn astal_icon_get_g_icon(self_: *mut AstalIcon) -> *mut gio::GIcon;
    pub fn astal_icon_set_g_icon(self_: *mut AstalIcon, value: *mut gio::GIcon);
    pub fn astal_icon_get_icon(self_: *mut AstalIcon) -> *const c_char;
    pub fn astal_icon_set_icon(self_: *mut AstalIcon, value: *const c_char);

    //=========================================================================
    // AstalLabel
    //=========================================================================
    pub fn astal_label_get_type() -> GType;
    pub fn astal_label_new() -> *mut AstalLabel;
    pub fn astal_label_get_truncate(self_: *mut AstalLabel) -> gboolean;
    pub fn astal_label_set_truncate(self_: *mut AstalLabel, value: gboolean);
    pub fn astal_label_get_justify_fill(self_: *mut AstalLabel) -> gboolean;
    pub fn astal_label_set_justify_fill(self_: *mut AstalLabel, value: gboolean);

    //=========================================================================
    // AstalLevelBar
    //=========================================================================
    pub fn astal_level_bar_get_type() -> GType;
    pub fn astal_level_bar_new() -> *mut AstalLevelBar;
    pub fn astal_level_bar_get_vertical(self_: *mut AstalLevelBar) -> gboolean;
    pub fn astal_level_bar_set_vertical(self_: *mut AstalLevelBar, value: gboolean);

    //=========================================================================
    // AstalOverlay
    //=========================================================================
    pub fn astal_overlay_get_type() -> GType;
    pub fn astal_overlay_add_overlay(self_: *mut AstalOverlay, widget: *mut gtk::GtkWidget);
    pub fn astal_overlay_new() -> *mut AstalOverlay;
    pub fn astal_overlay_get_pass_through(self_: *mut AstalOverlay) -> gboolean;
    pub fn astal_overlay_set_pass_through(self_: *mut AstalOverlay, value: gboolean);
    pub fn astal_overlay_get_overlay(self_: *mut AstalOverlay) -> *mut gtk::GtkWidget;
    pub fn astal_overlay_set_overlay(self_: *mut AstalOverlay, value: *mut gtk::GtkWidget);
    pub fn astal_overlay_get_overlays(self_: *mut AstalOverlay) -> *mut glib::GList;
    pub fn astal_overlay_set_overlays(self_: *mut AstalOverlay, value: *mut glib::GList);
    pub fn astal_overlay_get_child(self_: *mut AstalOverlay) -> *mut gtk::GtkWidget;
    pub fn astal_overlay_set_child(self_: *mut AstalOverlay, value: *mut gtk::GtkWidget);

    //=========================================================================
    // AstalScrollable
    //=========================================================================
    pub fn astal_scrollable_get_type() -> GType;
    pub fn astal_scrollable_get_child(self_: *mut AstalScrollable) -> *mut gtk::GtkWidget;
    pub fn astal_scrollable_new() -> *mut AstalScrollable;
    pub fn astal_scrollable_get_hscroll(self_: *mut AstalScrollable) -> gtk::GtkPolicyType;
    pub fn astal_scrollable_set_hscroll(self_: *mut AstalScrollable, value: gtk::GtkPolicyType);
    pub fn astal_scrollable_get_vscroll(self_: *mut AstalScrollable) -> gtk::GtkPolicyType;
    pub fn astal_scrollable_set_vscroll(self_: *mut AstalScrollable, value: gtk::GtkPolicyType);

    //=========================================================================
    // AstalSlider
    //=========================================================================
    pub fn astal_slider_get_type() -> GType;
    pub fn astal_slider_new() -> *mut AstalSlider;
    pub fn astal_slider_get_vertical(self_: *mut AstalSlider) -> gboolean;
    pub fn astal_slider_set_vertical(self_: *mut AstalSlider, value: gboolean);
    pub fn astal_slider_get_dragging(self_: *mut AstalSlider) -> gboolean;
    pub fn astal_slider_get_value(self_: *mut AstalSlider) -> c_double;
    pub fn astal_slider_set_value(self_: *mut AstalSlider, value: c_double);
    pub fn astal_slider_get_min(self_: *mut AstalSlider) -> c_double;
    pub fn astal_slider_set_min(self_: *mut AstalSlider, value: c_double);
    pub fn astal_slider_get_max(self_: *mut AstalSlider) -> c_double;
    pub fn astal_slider_set_max(self_: *mut AstalSlider, value: c_double);
    pub fn astal_slider_get_step(self_: *mut AstalSlider) -> c_double;
    pub fn astal_slider_set_step(self_: *mut AstalSlider, value: c_double);

    //=========================================================================
    // AstalStack
    //=========================================================================
    pub fn astal_stack_get_type() -> GType;
    pub fn astal_stack_new() -> *mut AstalStack;
    pub fn astal_stack_get_shown(self_: *mut AstalStack) -> *const c_char;
    pub fn astal_stack_set_shown(self_: *mut AstalStack, value: *const c_char);
    pub fn astal_stack_get_children(self_: *mut AstalStack) -> *mut glib::GList;
    pub fn astal_stack_set_children(self_: *mut AstalStack, value: *mut glib::GList);

    //=========================================================================
    // AstalWindow
    //=========================================================================
    pub fn astal_window_get_type() -> GType;
    pub fn astal_window_new() -> *mut AstalWindow;
    pub fn astal_window_get_inhibit(self_: *mut AstalWindow) -> gboolean;
    pub fn astal_window_set_inhibit(self_: *mut AstalWindow, value: gboolean);
    pub fn astal_window_get_namespace(self_: *mut AstalWindow) -> *const c_char;
    pub fn astal_window_set_namespace(self_: *mut AstalWindow, value: *const c_char);
    pub fn astal_window_get_anchor(self_: *mut AstalWindow) -> AstalWindowAnchor;
    pub fn astal_window_set_anchor(self_: *mut AstalWindow, value: AstalWindowAnchor);
    pub fn astal_window_get_exclusivity(self_: *mut AstalWindow) -> AstalExclusivity;
    pub fn astal_window_set_exclusivity(self_: *mut AstalWindow, value: AstalExclusivity);
    pub fn astal_window_get_layer(self_: *mut AstalWindow) -> AstalLayer;
    pub fn astal_window_set_layer(self_: *mut AstalWindow, value: AstalLayer);
    pub fn astal_window_get_keymode(self_: *mut AstalWindow) -> AstalKeymode;
    pub fn astal_window_set_keymode(self_: *mut AstalWindow, value: AstalKeymode);
    pub fn astal_window_get_gdkmonitor(self_: *mut AstalWindow) -> *mut gdk::GdkMonitor;
    pub fn astal_window_set_gdkmonitor(self_: *mut AstalWindow, value: *mut gdk::GdkMonitor);
    pub fn astal_window_get_margin_top(self_: *mut AstalWindow) -> c_int;
    pub fn astal_window_set_margin_top(self_: *mut AstalWindow, value: c_int);
    pub fn astal_window_get_margin_bottom(self_: *mut AstalWindow) -> c_int;
    pub fn astal_window_set_margin_bottom(self_: *mut AstalWindow, value: c_int);
    pub fn astal_window_get_margin_left(self_: *mut AstalWindow) -> c_int;
    pub fn astal_window_set_margin_left(self_: *mut AstalWindow, value: c_int);
    pub fn astal_window_get_margin_right(self_: *mut AstalWindow) -> c_int;
    pub fn astal_window_set_margin_right(self_: *mut AstalWindow, value: c_int);
    pub fn astal_window_set_margin(self_: *mut AstalWindow, value: c_int);
    pub fn astal_window_get_monitor(self_: *mut AstalWindow) -> c_int;
    pub fn astal_window_set_monitor(self_: *mut AstalWindow, value: c_int);

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn astal_widget_set_css(widget: *mut gtk::GtkWidget, css: *const c_char);
    pub fn astal_widget_get_css(widget: *mut gtk::GtkWidget) -> *mut c_char;
    pub fn astal_widget_set_class_names(widget: *mut gtk::GtkWidget, class_names: *mut *mut c_char, class_names_length1: c_int);
    pub fn astal_widget_get_class_names(widget: *mut gtk::GtkWidget) -> *mut glib::GList;
    pub fn astal_widget_toggle_class_name(widget: *mut gtk::GtkWidget, class_name: *const c_char, condition: gboolean);
    pub fn astal_widget_set_cursor(widget: *mut gtk::GtkWidget, cursor: *const c_char);
    pub fn astal_widget_get_cursor(widget: *mut gtk::GtkWidget) -> *mut c_char;
    pub fn astal_widget_set_click_through(widget: *mut gtk::GtkWidget, click_through: gboolean);
    pub fn astal_widget_get_click_through(widget: *mut gtk::GtkWidget) -> gboolean;

}
