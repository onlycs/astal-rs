// Generated by gir (https://github.com/gtk-rs/gir @ 0cdde9fbfd9d)
// from ../../gir/gir-files (@ 611247c05351+)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(clippy::approx_constant, clippy::type_complexity, clippy::unreadable_literal, clippy::upper_case_acronyms)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use glib_sys as glib;
use gio_sys as gio;
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
pub const ASTAL_BLUETOOTH_MAJOR_VERSION: c_int = 0;
pub const ASTAL_BLUETOOTH_MINOR_VERSION: c_int = 1;
pub const ASTAL_BLUETOOTH_MICRO_VERSION: c_int = 0;
pub const ASTAL_BLUETOOTH_VERSION: &[u8] = b"0.1.0\0";

// Records
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalBluetoothAdapterClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for AstalBluetoothAdapterClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalBluetoothAdapterClass @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct _AstalBluetoothAdapterPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type AstalBluetoothAdapterPrivate = _AstalBluetoothAdapterPrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalBluetoothBluetoothClass {
    pub parent_class: gobject::GObjectClass,
    pub device_added: Option<unsafe extern "C" fn(*mut AstalBluetoothBluetooth, *mut AstalBluetoothDevice)>,
    pub device_removed: Option<unsafe extern "C" fn(*mut AstalBluetoothBluetooth, *mut AstalBluetoothDevice)>,
    pub adapter_added: Option<unsafe extern "C" fn(*mut AstalBluetoothBluetooth, *mut AstalBluetoothAdapter)>,
    pub adapter_removed: Option<unsafe extern "C" fn(*mut AstalBluetoothBluetooth, *mut AstalBluetoothAdapter)>,
}

impl ::std::fmt::Debug for AstalBluetoothBluetoothClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalBluetoothBluetoothClass @ {self:p}"))
         .field("device_added", &self.device_added)
         .field("device_removed", &self.device_removed)
         .field("adapter_added", &self.adapter_added)
         .field("adapter_removed", &self.adapter_removed)
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct _AstalBluetoothBluetoothPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type AstalBluetoothBluetoothPrivate = _AstalBluetoothBluetoothPrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalBluetoothDeviceClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for AstalBluetoothDeviceClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalBluetoothDeviceClass @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct _AstalBluetoothDevicePrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type AstalBluetoothDevicePrivate = _AstalBluetoothDevicePrivate;

// Classes
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalBluetoothAdapter {
    pub parent_instance: gobject::GObject,
    pub priv_: *mut AstalBluetoothAdapterPrivate,
}

impl ::std::fmt::Debug for AstalBluetoothAdapter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalBluetoothAdapter @ {self:p}"))
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalBluetoothBluetooth {
    pub parent_instance: gobject::GObject,
    pub priv_: *mut AstalBluetoothBluetoothPrivate,
}

impl ::std::fmt::Debug for AstalBluetoothBluetooth {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalBluetoothBluetooth @ {self:p}"))
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalBluetoothDevice {
    pub parent_instance: gobject::GObject,
    pub priv_: *mut AstalBluetoothDevicePrivate,
}

impl ::std::fmt::Debug for AstalBluetoothDevice {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalBluetoothDevice @ {self:p}"))
         .finish()
    }
}

extern "C" {

    //=========================================================================
    // AstalBluetoothAdapter
    //=========================================================================
    pub fn astal_bluetooth_adapter_get_type() -> GType;
    pub fn astal_bluetooth_adapter_remove_device(self_: *mut AstalBluetoothAdapter, device: *mut AstalBluetoothDevice, error: *mut *mut glib::GError);
    pub fn astal_bluetooth_adapter_start_discovery(self_: *mut AstalBluetoothAdapter, error: *mut *mut glib::GError);
    pub fn astal_bluetooth_adapter_stop_discovery(self_: *mut AstalBluetoothAdapter, error: *mut *mut glib::GError);
    pub fn astal_bluetooth_adapter_get_uuids(self_: *mut AstalBluetoothAdapter, result_length1: *mut c_int) -> *mut *mut c_char;
    pub fn astal_bluetooth_adapter_get_discovering(self_: *mut AstalBluetoothAdapter) -> gboolean;
    pub fn astal_bluetooth_adapter_get_modalias(self_: *mut AstalBluetoothAdapter) -> *mut c_char;
    pub fn astal_bluetooth_adapter_get_name(self_: *mut AstalBluetoothAdapter) -> *mut c_char;
    pub fn astal_bluetooth_adapter_get_class(self_: *mut AstalBluetoothAdapter) -> c_uint;
    pub fn astal_bluetooth_adapter_get_address(self_: *mut AstalBluetoothAdapter) -> *mut c_char;
    pub fn astal_bluetooth_adapter_get_discoverable(self_: *mut AstalBluetoothAdapter) -> gboolean;
    pub fn astal_bluetooth_adapter_set_discoverable(self_: *mut AstalBluetoothAdapter, value: gboolean);
    pub fn astal_bluetooth_adapter_get_pairable(self_: *mut AstalBluetoothAdapter) -> gboolean;
    pub fn astal_bluetooth_adapter_set_pairable(self_: *mut AstalBluetoothAdapter, value: gboolean);
    pub fn astal_bluetooth_adapter_get_powered(self_: *mut AstalBluetoothAdapter) -> gboolean;
    pub fn astal_bluetooth_adapter_set_powered(self_: *mut AstalBluetoothAdapter, value: gboolean);
    pub fn astal_bluetooth_adapter_get_alias(self_: *mut AstalBluetoothAdapter) -> *mut c_char;
    pub fn astal_bluetooth_adapter_set_alias(self_: *mut AstalBluetoothAdapter, value: *const c_char);
    pub fn astal_bluetooth_adapter_get_discoverable_timeout(self_: *mut AstalBluetoothAdapter) -> c_uint;
    pub fn astal_bluetooth_adapter_set_discoverable_timeout(self_: *mut AstalBluetoothAdapter, value: c_uint);
    pub fn astal_bluetooth_adapter_get_pairable_timeout(self_: *mut AstalBluetoothAdapter) -> c_uint;
    pub fn astal_bluetooth_adapter_set_pairable_timeout(self_: *mut AstalBluetoothAdapter, value: c_uint);

    //=========================================================================
    // AstalBluetoothBluetooth
    //=========================================================================
    pub fn astal_bluetooth_bluetooth_get_type() -> GType;
    pub fn astal_bluetooth_bluetooth_get_default() -> *mut AstalBluetoothBluetooth;
    pub fn astal_bluetooth_bluetooth_toggle(self_: *mut AstalBluetoothBluetooth);
    pub fn astal_bluetooth_bluetooth_new() -> *mut AstalBluetoothBluetooth;
    pub fn astal_bluetooth_bluetooth_get_is_powered(self_: *mut AstalBluetoothBluetooth) -> gboolean;
    pub fn astal_bluetooth_bluetooth_get_is_connected(self_: *mut AstalBluetoothBluetooth) -> gboolean;
    pub fn astal_bluetooth_bluetooth_get_adapter(self_: *mut AstalBluetoothBluetooth) -> *mut AstalBluetoothAdapter;
    pub fn astal_bluetooth_bluetooth_get_adapters(self_: *mut AstalBluetoothBluetooth) -> *mut glib::GList;
    pub fn astal_bluetooth_bluetooth_get_devices(self_: *mut AstalBluetoothBluetooth) -> *mut glib::GList;

    //=========================================================================
    // AstalBluetoothDevice
    //=========================================================================
    pub fn astal_bluetooth_device_get_type() -> GType;
    pub fn astal_bluetooth_device_connect_device(self_: *mut AstalBluetoothDevice, _callback_: gio::GAsyncReadyCallback, _callback__target: *mut c_void);
    pub fn astal_bluetooth_device_connect_device_finish(self_: *mut AstalBluetoothDevice, _res_: *mut gio::GAsyncResult, error: *mut *mut glib::GError);
    pub fn astal_bluetooth_device_disconnect_device(self_: *mut AstalBluetoothDevice, _callback_: gio::GAsyncReadyCallback, _callback__target: *mut c_void);
    pub fn astal_bluetooth_device_disconnect_device_finish(self_: *mut AstalBluetoothDevice, _res_: *mut gio::GAsyncResult, error: *mut *mut glib::GError);
    pub fn astal_bluetooth_device_connect_profile(self_: *mut AstalBluetoothDevice, uuid: *const c_char, error: *mut *mut glib::GError);
    pub fn astal_bluetooth_device_disconnect_profile(self_: *mut AstalBluetoothDevice, uuid: *const c_char, error: *mut *mut glib::GError);
    pub fn astal_bluetooth_device_pair(self_: *mut AstalBluetoothDevice, error: *mut *mut glib::GError);
    pub fn astal_bluetooth_device_cancel_pairing(self_: *mut AstalBluetoothDevice, error: *mut *mut glib::GError);
    pub fn astal_bluetooth_device_get_uuids(self_: *mut AstalBluetoothDevice, result_length1: *mut c_int) -> *mut *mut c_char;
    pub fn astal_bluetooth_device_get_connected(self_: *mut AstalBluetoothDevice) -> gboolean;
    pub fn astal_bluetooth_device_get_legacy_pairing(self_: *mut AstalBluetoothDevice) -> gboolean;
    pub fn astal_bluetooth_device_get_paired(self_: *mut AstalBluetoothDevice) -> gboolean;
    pub fn astal_bluetooth_device_get_rssi(self_: *mut AstalBluetoothDevice) -> i16;
    pub fn astal_bluetooth_device_get_adapter(self_: *mut AstalBluetoothDevice) -> *mut c_char;
    pub fn astal_bluetooth_device_get_address(self_: *mut AstalBluetoothDevice) -> *mut c_char;
    pub fn astal_bluetooth_device_get_icon(self_: *mut AstalBluetoothDevice) -> *mut c_char;
    pub fn astal_bluetooth_device_get_modalias(self_: *mut AstalBluetoothDevice) -> *mut c_char;
    pub fn astal_bluetooth_device_get_name(self_: *mut AstalBluetoothDevice) -> *mut c_char;
    pub fn astal_bluetooth_device_get_appearance(self_: *mut AstalBluetoothDevice) -> u16;
    pub fn astal_bluetooth_device_get_class(self_: *mut AstalBluetoothDevice) -> u32;
    pub fn astal_bluetooth_device_get_connecting(self_: *mut AstalBluetoothDevice) -> gboolean;
    pub fn astal_bluetooth_device_get_blocked(self_: *mut AstalBluetoothDevice) -> gboolean;
    pub fn astal_bluetooth_device_set_blocked(self_: *mut AstalBluetoothDevice, value: gboolean);
    pub fn astal_bluetooth_device_get_trusted(self_: *mut AstalBluetoothDevice) -> gboolean;
    pub fn astal_bluetooth_device_set_trusted(self_: *mut AstalBluetoothDevice, value: gboolean);
    pub fn astal_bluetooth_device_get_alias(self_: *mut AstalBluetoothDevice) -> *mut c_char;
    pub fn astal_bluetooth_device_set_alias(self_: *mut AstalBluetoothDevice, value: *const c_char);

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn astal_bluetooth_get_default() -> *mut AstalBluetoothBluetooth;

}
