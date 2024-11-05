// Generated by gir (https://github.com/gtk-rs/gir @ 88259bc5f36f+)
// from ../../gir/gir-files (@ 1783d05ebac3+)
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
pub type AstalBatteryBatteryLevel = c_int;
pub const ASTAL_BATTERY_BATTERY_LEVEL_UNKNOWN: AstalBatteryBatteryLevel = 0;
pub const ASTAL_BATTERY_BATTERY_LEVEL_NONE: AstalBatteryBatteryLevel = 1;
pub const ASTAL_BATTERY_BATTERY_LEVEL_LOW: AstalBatteryBatteryLevel = 2;
pub const ASTAL_BATTERY_BATTERY_LEVEL_CRITICIAL: AstalBatteryBatteryLevel = 3;
pub const ASTAL_BATTERY_BATTERY_LEVEL_NORMAL: AstalBatteryBatteryLevel = 4;
pub const ASTAL_BATTERY_BATTERY_LEVEL_HIGH: AstalBatteryBatteryLevel = 5;
pub const ASTAL_BATTERY_BATTERY_LEVEL_FULL: AstalBatteryBatteryLevel = 6;

pub type AstalBatteryState = c_int;
pub const ASTAL_BATTERY_STATE_UNKNOWN: AstalBatteryState = 0;
pub const ASTAL_BATTERY_STATE_CHARGING: AstalBatteryState = 1;
pub const ASTAL_BATTERY_STATE_DISCHARGING: AstalBatteryState = 2;
pub const ASTAL_BATTERY_STATE_EMPTY: AstalBatteryState = 3;
pub const ASTAL_BATTERY_STATE_FULLY_CHARGED: AstalBatteryState = 4;
pub const ASTAL_BATTERY_STATE_PENDING_CHARGE: AstalBatteryState = 5;
pub const ASTAL_BATTERY_STATE_PENDING_DISCHARGE: AstalBatteryState = 6;

pub type AstalBatteryTechnology = c_int;
pub const ASTAL_BATTERY_TECHNOLOGY_UNKNOWN: AstalBatteryTechnology = 0;
pub const ASTAL_BATTERY_TECHNOLOGY_LITHIUM_ION: AstalBatteryTechnology = 1;
pub const ASTAL_BATTERY_TECHNOLOGY_LITHIUM_POLYMER: AstalBatteryTechnology = 2;
pub const ASTAL_BATTERY_TECHNOLOGY_LITHIUM_IRON_PHOSPHATE: AstalBatteryTechnology = 3;
pub const ASTAL_BATTERY_TECHNOLOGY_LEAD_ACID: AstalBatteryTechnology = 4;
pub const ASTAL_BATTERY_TECHNOLOGY_NICKEL_CADMIUM: AstalBatteryTechnology = 5;
pub const ASTAL_BATTERY_TECHNOLOGY_NICKEL_METAL_HYDRIDE: AstalBatteryTechnology = 6;

pub type AstalBatteryType = c_int;
pub const ASTAL_BATTERY_TYPE_UNKNOWN: AstalBatteryType = 0;
pub const ASTAL_BATTERY_TYPE_LINE_POWER: AstalBatteryType = 1;
pub const ASTAL_BATTERY_TYPE_BATTERY: AstalBatteryType = 2;
pub const ASTAL_BATTERY_TYPE_UPS: AstalBatteryType = 3;
pub const ASTAL_BATTERY_TYPE_MONITOR: AstalBatteryType = 4;
pub const ASTAL_BATTERY_TYPE_MOUSE: AstalBatteryType = 5;
pub const ASTAL_BATTERY_TYPE_KEYBOARD: AstalBatteryType = 6;
pub const ASTAL_BATTERY_TYPE_PDA: AstalBatteryType = 7;
pub const ASTAL_BATTERY_TYPE_PHONE: AstalBatteryType = 8;
pub const ASTAL_BATTERY_TYPE_MEDIA_PLAYER: AstalBatteryType = 9;
pub const ASTAL_BATTERY_TYPE_TABLET: AstalBatteryType = 10;
pub const ASTAL_BATTERY_TYPE_COMPUTER: AstalBatteryType = 11;
pub const ASTAL_BATTERY_TYPE_GAMING_INPUT: AstalBatteryType = 12;
pub const ASTAL_BATTERY_TYPE_PEN: AstalBatteryType = 13;
pub const ASTAL_BATTERY_TYPE_TOUCHPAD: AstalBatteryType = 14;
pub const ASTAL_BATTERY_TYPE_MODEM: AstalBatteryType = 15;
pub const ASTAL_BATTERY_TYPE_NETWORK: AstalBatteryType = 16;
pub const ASTAL_BATTERY_TYPE_HEADSET: AstalBatteryType = 17;
pub const ASTAL_BATTERY_TYPE_SPEAKERS: AstalBatteryType = 18;
pub const ASTAL_BATTERY_TYPE_HEADPHONES: AstalBatteryType = 19;
pub const ASTAL_BATTERY_TYPE_VIDEO: AstalBatteryType = 20;
pub const ASTAL_BATTERY_TYPE_OTHER_AUDIO: AstalBatteryType = 21;
pub const ASTAL_BATTERY_TYPE_REMOVE_CONTROL: AstalBatteryType = 22;
pub const ASTAL_BATTERY_TYPE_PRINTER: AstalBatteryType = 23;
pub const ASTAL_BATTERY_TYPE_SCANNER: AstalBatteryType = 24;
pub const ASTAL_BATTERY_TYPE_CAMERA: AstalBatteryType = 25;
pub const ASTAL_BATTERY_TYPE_WEARABLE: AstalBatteryType = 26;
pub const ASTAL_BATTERY_TYPE_TOY: AstalBatteryType = 27;
pub const ASTAL_BATTERY_TYPE_BLUETOOTH_GENERIC: AstalBatteryType = 28;

pub type AstalBatteryWarningLevel = c_int;
pub const ASTAL_BATTERY_WARNING_LEVEL_UNKNOWN: AstalBatteryWarningLevel = 0;
pub const ASTAL_BATTERY_WARNING_LEVEL_NONE: AstalBatteryWarningLevel = 1;
pub const ASTAL_BATTERY_WARNING_LEVEL_DISCHARGING: AstalBatteryWarningLevel = 2;
pub const ASTAL_BATTERY_WARNING_LEVEL_LOW: AstalBatteryWarningLevel = 3;
pub const ASTAL_BATTERY_WARNING_LEVEL_CRITICIAL: AstalBatteryWarningLevel = 4;
pub const ASTAL_BATTERY_WARNING_LEVEL_ACTION: AstalBatteryWarningLevel = 5;

// Constants
pub const ASTAL_BATTERY_MAJOR_VERSION: c_int = 0;
pub const ASTAL_BATTERY_MINOR_VERSION: c_int = 1;
pub const ASTAL_BATTERY_MICRO_VERSION: c_int = 0;
pub const ASTAL_BATTERY_VERSION: &[u8] = b"0.1.0\0";

// Records
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalBatteryDeviceClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for AstalBatteryDeviceClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalBatteryDeviceClass @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct _AstalBatteryDevicePrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type AstalBatteryDevicePrivate = _AstalBatteryDevicePrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalBatteryHistoryDataPoint {
    pub time: u32,
    pub value: c_double,
    pub state: u32,
}

impl ::std::fmt::Debug for AstalBatteryHistoryDataPoint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalBatteryHistoryDataPoint @ {self:p}"))
         .field("time", &self.time)
         .field("value", &self.value)
         .field("state", &self.state)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalBatteryIUPowerDeviceIface {
    pub parent_iface: gobject::GTypeInterface,
    pub get_history: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice, *const c_char, u32, u32, *mut c_int, *mut *mut glib::GError) -> *mut AstalBatteryHistoryDataPoint>,
    pub get_statistics: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice, *const c_char, *mut c_int, *mut *mut glib::GError) -> *mut AstalBatteryStatisticsDataPoint>,
    pub refresh: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice, *mut *mut glib::GError)>,
    pub get_Type: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice) -> c_uint>,
    pub get_native_path: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice) -> *mut c_char>,
    pub get_vendor: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice) -> *mut c_char>,
    pub get_model: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice) -> *mut c_char>,
    pub get_serial: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice) -> *mut c_char>,
    pub get_update_time: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice) -> u64>,
    pub get_power_supply: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice) -> gboolean>,
    pub get_has_history: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice) -> gboolean>,
    pub get_has_statistics: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice) -> gboolean>,
    pub get_online: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice) -> gboolean>,
    pub get_energy: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice) -> c_double>,
    pub get_energy_empty: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice) -> c_double>,
    pub get_energy_full: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice) -> c_double>,
    pub get_energy_full_design: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice) -> c_double>,
    pub get_energy_rate: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice) -> c_double>,
    pub get_voltage: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice) -> c_double>,
    pub get_charge_cycles: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice) -> i32>,
    pub get_luminosity: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice) -> c_double>,
    pub get_time_to_empty: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice) -> i64>,
    pub get_time_to_full: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice) -> i64>,
    pub get_percentage: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice) -> c_double>,
    pub get_temperature: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice) -> c_double>,
    pub get_is_present: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice) -> gboolean>,
    pub get_state: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice) -> c_uint>,
    pub get_is_rechargable: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice) -> gboolean>,
    pub get_capacity: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice) -> c_double>,
    pub get_technology: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice) -> c_uint>,
    pub get_warning_level: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice) -> u32>,
    pub get_battery_level: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice) -> u32>,
    pub get_icon_name: Option<unsafe extern "C" fn(*mut AstalBatteryIUPowerDevice) -> *mut c_char>,
}

impl ::std::fmt::Debug for AstalBatteryIUPowerDeviceIface {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalBatteryIUPowerDeviceIface @ {self:p}"))
         .field("get_history", &self.get_history)
         .field("get_statistics", &self.get_statistics)
         .field("refresh", &self.refresh)
         .field("get_Type", &self.get_Type)
         .field("get_native_path", &self.get_native_path)
         .field("get_vendor", &self.get_vendor)
         .field("get_model", &self.get_model)
         .field("get_serial", &self.get_serial)
         .field("get_update_time", &self.get_update_time)
         .field("get_power_supply", &self.get_power_supply)
         .field("get_has_history", &self.get_has_history)
         .field("get_has_statistics", &self.get_has_statistics)
         .field("get_online", &self.get_online)
         .field("get_energy", &self.get_energy)
         .field("get_energy_empty", &self.get_energy_empty)
         .field("get_energy_full", &self.get_energy_full)
         .field("get_energy_full_design", &self.get_energy_full_design)
         .field("get_energy_rate", &self.get_energy_rate)
         .field("get_voltage", &self.get_voltage)
         .field("get_charge_cycles", &self.get_charge_cycles)
         .field("get_luminosity", &self.get_luminosity)
         .field("get_time_to_empty", &self.get_time_to_empty)
         .field("get_time_to_full", &self.get_time_to_full)
         .field("get_percentage", &self.get_percentage)
         .field("get_temperature", &self.get_temperature)
         .field("get_is_present", &self.get_is_present)
         .field("get_state", &self.get_state)
         .field("get_is_rechargable", &self.get_is_rechargable)
         .field("get_capacity", &self.get_capacity)
         .field("get_technology", &self.get_technology)
         .field("get_warning_level", &self.get_warning_level)
         .field("get_battery_level", &self.get_battery_level)
         .field("get_icon_name", &self.get_icon_name)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalBatteryStatisticsDataPoint {
    pub value: c_double,
    pub accuracy: c_double,
}

impl ::std::fmt::Debug for AstalBatteryStatisticsDataPoint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalBatteryStatisticsDataPoint @ {self:p}"))
         .field("value", &self.value)
         .field("accuracy", &self.accuracy)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalBatteryUPowerClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for AstalBatteryUPowerClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalBatteryUPowerClass @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct _AstalBatteryUPowerPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type AstalBatteryUPowerPrivate = _AstalBatteryUPowerPrivate;

// Classes
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalBatteryDevice {
    pub parent_instance: gobject::GObject,
    pub priv_: *mut AstalBatteryDevicePrivate,
}

impl ::std::fmt::Debug for AstalBatteryDevice {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalBatteryDevice @ {self:p}"))
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AstalBatteryUPower {
    pub parent_instance: gobject::GObject,
    pub priv_: *mut AstalBatteryUPowerPrivate,
}

impl ::std::fmt::Debug for AstalBatteryUPower {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("AstalBatteryUPower @ {self:p}"))
         .finish()
    }
}

// Interfaces
#[repr(C)]
#[allow(dead_code)]
pub struct AstalBatteryIUPowerDevice {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for AstalBatteryIUPowerDevice {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "AstalBatteryIUPowerDevice @ {self:p}")
    }
}


extern "C" {

    //=========================================================================
    // AstalBatteryBatteryLevel
    //=========================================================================
    pub fn astal_battery_battery_level_get_type() -> GType;

    //=========================================================================
    // AstalBatteryState
    //=========================================================================
    pub fn astal_battery_state_get_type() -> GType;

    //=========================================================================
    // AstalBatteryTechnology
    //=========================================================================
    pub fn astal_battery_technology_get_type() -> GType;

    //=========================================================================
    // AstalBatteryType
    //=========================================================================
    pub fn astal_battery_type_get_type() -> GType;

    //=========================================================================
    // AstalBatteryWarningLevel
    //=========================================================================
    pub fn astal_battery_warning_level_get_type() -> GType;

    //=========================================================================
    // AstalBatteryHistoryDataPoint
    //=========================================================================
    pub fn astal_battery_history_data_point_get_type() -> GType;

    //=========================================================================
    // AstalBatteryStatisticsDataPoint
    //=========================================================================
    pub fn astal_battery_statistics_data_point_get_type() -> GType;

    //=========================================================================
    // AstalBatteryDevice
    //=========================================================================
    pub fn astal_battery_device_get_type() -> GType;
    pub fn astal_battery_device_get_default() -> *mut AstalBatteryDevice;
    pub fn astal_battery_device_new(path: *const c_char, error: *mut *mut glib::GError) -> *mut AstalBatteryDevice;
    pub fn astal_battery_device_sync(self_: *mut AstalBatteryDevice);
    pub fn astal_battery_device_get_device_type(self_: *mut AstalBatteryDevice) -> AstalBatteryType;
    pub fn astal_battery_device_get_native_path(self_: *mut AstalBatteryDevice) -> *mut c_char;
    pub fn astal_battery_device_get_vendor(self_: *mut AstalBatteryDevice) -> *mut c_char;
    pub fn astal_battery_device_get_model(self_: *mut AstalBatteryDevice) -> *mut c_char;
    pub fn astal_battery_device_get_serial(self_: *mut AstalBatteryDevice) -> *mut c_char;
    pub fn astal_battery_device_get_update_time(self_: *mut AstalBatteryDevice) -> u64;
    pub fn astal_battery_device_get_power_supply(self_: *mut AstalBatteryDevice) -> gboolean;
    pub fn astal_battery_device_get_has_history(self_: *mut AstalBatteryDevice) -> gboolean;
    pub fn astal_battery_device_get_has_statistics(self_: *mut AstalBatteryDevice) -> gboolean;
    pub fn astal_battery_device_get_online(self_: *mut AstalBatteryDevice) -> gboolean;
    pub fn astal_battery_device_get_energy(self_: *mut AstalBatteryDevice) -> c_double;
    pub fn astal_battery_device_get_energy_empty(self_: *mut AstalBatteryDevice) -> c_double;
    pub fn astal_battery_device_get_energy_full(self_: *mut AstalBatteryDevice) -> c_double;
    pub fn astal_battery_device_get_energy_full_design(self_: *mut AstalBatteryDevice) -> c_double;
    pub fn astal_battery_device_get_energy_rate(self_: *mut AstalBatteryDevice) -> c_double;
    pub fn astal_battery_device_get_voltage(self_: *mut AstalBatteryDevice) -> c_double;
    pub fn astal_battery_device_get_charge_cycles(self_: *mut AstalBatteryDevice) -> c_int;
    pub fn astal_battery_device_get_luminosity(self_: *mut AstalBatteryDevice) -> c_double;
    pub fn astal_battery_device_get_time_to_empty(self_: *mut AstalBatteryDevice) -> i64;
    pub fn astal_battery_device_get_time_to_full(self_: *mut AstalBatteryDevice) -> i64;
    pub fn astal_battery_device_get_percentage(self_: *mut AstalBatteryDevice) -> c_double;
    pub fn astal_battery_device_get_temperature(self_: *mut AstalBatteryDevice) -> c_double;
    pub fn astal_battery_device_get_is_present(self_: *mut AstalBatteryDevice) -> gboolean;
    pub fn astal_battery_device_get_state(self_: *mut AstalBatteryDevice) -> AstalBatteryState;
    pub fn astal_battery_device_get_is_rechargable(self_: *mut AstalBatteryDevice) -> gboolean;
    pub fn astal_battery_device_get_capacity(self_: *mut AstalBatteryDevice) -> c_double;
    pub fn astal_battery_device_get_technology(self_: *mut AstalBatteryDevice) -> AstalBatteryTechnology;
    pub fn astal_battery_device_get_warning_level(self_: *mut AstalBatteryDevice) -> AstalBatteryWarningLevel;
    pub fn astal_battery_device_get_battery_level(self_: *mut AstalBatteryDevice) -> AstalBatteryBatteryLevel;
    pub fn astal_battery_device_get_icon_name(self_: *mut AstalBatteryDevice) -> *mut c_char;
    pub fn astal_battery_device_get_charging(self_: *mut AstalBatteryDevice) -> gboolean;
    pub fn astal_battery_device_get_is_battery(self_: *mut AstalBatteryDevice) -> gboolean;
    pub fn astal_battery_device_get_battery_icon_name(self_: *mut AstalBatteryDevice) -> *const c_char;
    pub fn astal_battery_device_get_device_type_name(self_: *mut AstalBatteryDevice) -> *const c_char;
    pub fn astal_battery_device_get_device_type_icon(self_: *mut AstalBatteryDevice) -> *const c_char;

    //=========================================================================
    // AstalBatteryUPower
    //=========================================================================
    pub fn astal_battery_upower_get_type() -> GType;
    pub fn astal_battery_upower_new() -> *mut AstalBatteryUPower;
    pub fn astal_battery_upower_get_devices(self_: *mut AstalBatteryUPower) -> *mut glib::GList;
    pub fn astal_battery_upower_get_display_device(self_: *mut AstalBatteryUPower) -> *mut AstalBatteryDevice;
    pub fn astal_battery_upower_get_daemon_version(self_: *mut AstalBatteryUPower) -> *mut c_char;
    pub fn astal_battery_upower_get_on_battery(self_: *mut AstalBatteryUPower) -> gboolean;
    pub fn astal_battery_upower_get_lid_is_closed(self_: *mut AstalBatteryUPower) -> gboolean;
    pub fn astal_battery_upower_get_lis_is_present(self_: *mut AstalBatteryUPower) -> gboolean;
    pub fn astal_battery_upower_get_critical_action(self_: *mut AstalBatteryUPower) -> *mut c_char;

    //=========================================================================
    // AstalBatteryIUPowerDevice
    //=========================================================================
    pub fn astal_battery_iu_power_device_get_type() -> GType;
    pub fn astal_battery_iu_power_device_get_history(self_: *mut AstalBatteryIUPowerDevice, type_: *const c_char, timespan: u32, resolution: u32, result_length1: *mut c_int, error: *mut *mut glib::GError) -> *mut AstalBatteryHistoryDataPoint;
    pub fn astal_battery_iu_power_device_get_statistics(self_: *mut AstalBatteryIUPowerDevice, type_: *const c_char, result_length1: *mut c_int, error: *mut *mut glib::GError) -> *mut AstalBatteryStatisticsDataPoint;
    pub fn astal_battery_iu_power_device_refresh(self_: *mut AstalBatteryIUPowerDevice, error: *mut *mut glib::GError);
    pub fn astal_battery_iu_power_device_get_Type(self_: *mut AstalBatteryIUPowerDevice) -> c_uint;
    pub fn astal_battery_iu_power_device_get_native_path(self_: *mut AstalBatteryIUPowerDevice) -> *mut c_char;
    pub fn astal_battery_iu_power_device_get_vendor(self_: *mut AstalBatteryIUPowerDevice) -> *mut c_char;
    pub fn astal_battery_iu_power_device_get_model(self_: *mut AstalBatteryIUPowerDevice) -> *mut c_char;
    pub fn astal_battery_iu_power_device_get_serial(self_: *mut AstalBatteryIUPowerDevice) -> *mut c_char;
    pub fn astal_battery_iu_power_device_get_update_time(self_: *mut AstalBatteryIUPowerDevice) -> u64;
    pub fn astal_battery_iu_power_device_get_power_supply(self_: *mut AstalBatteryIUPowerDevice) -> gboolean;
    pub fn astal_battery_iu_power_device_get_has_history(self_: *mut AstalBatteryIUPowerDevice) -> gboolean;
    pub fn astal_battery_iu_power_device_get_has_statistics(self_: *mut AstalBatteryIUPowerDevice) -> gboolean;
    pub fn astal_battery_iu_power_device_get_online(self_: *mut AstalBatteryIUPowerDevice) -> gboolean;
    pub fn astal_battery_iu_power_device_get_energy(self_: *mut AstalBatteryIUPowerDevice) -> c_double;
    pub fn astal_battery_iu_power_device_get_energy_empty(self_: *mut AstalBatteryIUPowerDevice) -> c_double;
    pub fn astal_battery_iu_power_device_get_energy_full(self_: *mut AstalBatteryIUPowerDevice) -> c_double;
    pub fn astal_battery_iu_power_device_get_energy_full_design(self_: *mut AstalBatteryIUPowerDevice) -> c_double;
    pub fn astal_battery_iu_power_device_get_energy_rate(self_: *mut AstalBatteryIUPowerDevice) -> c_double;
    pub fn astal_battery_iu_power_device_get_voltage(self_: *mut AstalBatteryIUPowerDevice) -> c_double;
    pub fn astal_battery_iu_power_device_get_charge_cycles(self_: *mut AstalBatteryIUPowerDevice) -> i32;
    pub fn astal_battery_iu_power_device_get_luminosity(self_: *mut AstalBatteryIUPowerDevice) -> c_double;
    pub fn astal_battery_iu_power_device_get_time_to_empty(self_: *mut AstalBatteryIUPowerDevice) -> i64;
    pub fn astal_battery_iu_power_device_get_time_to_full(self_: *mut AstalBatteryIUPowerDevice) -> i64;
    pub fn astal_battery_iu_power_device_get_percentage(self_: *mut AstalBatteryIUPowerDevice) -> c_double;
    pub fn astal_battery_iu_power_device_get_temperature(self_: *mut AstalBatteryIUPowerDevice) -> c_double;
    pub fn astal_battery_iu_power_device_get_is_present(self_: *mut AstalBatteryIUPowerDevice) -> gboolean;
    pub fn astal_battery_iu_power_device_get_state(self_: *mut AstalBatteryIUPowerDevice) -> c_uint;
    pub fn astal_battery_iu_power_device_get_is_rechargable(self_: *mut AstalBatteryIUPowerDevice) -> gboolean;
    pub fn astal_battery_iu_power_device_get_capacity(self_: *mut AstalBatteryIUPowerDevice) -> c_double;
    pub fn astal_battery_iu_power_device_get_technology(self_: *mut AstalBatteryIUPowerDevice) -> c_uint;
    pub fn astal_battery_iu_power_device_get_warning_level(self_: *mut AstalBatteryIUPowerDevice) -> u32;
    pub fn astal_battery_iu_power_device_get_battery_level(self_: *mut AstalBatteryIUPowerDevice) -> u32;
    pub fn astal_battery_iu_power_device_get_icon_name(self_: *mut AstalBatteryIUPowerDevice) -> *mut c_char;

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn astal_battery_type_get_icon_name(self_: AstalBatteryType) -> *mut c_char;
    pub fn astal_battery_type_get_name(self_: AstalBatteryType) -> *const c_char;
    pub fn astal_battery_get_default() -> *mut AstalBatteryDevice;

}
