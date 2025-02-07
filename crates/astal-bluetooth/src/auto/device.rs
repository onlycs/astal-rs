// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../../gir/gir-files
// DO NOT EDIT

use crate::{ffi};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "AstalBluetoothDevice")]
    pub struct Device(Object<ffi::AstalBluetoothDevice, ffi::AstalBluetoothDeviceClass>);

    match fn {
        type_ => || ffi::astal_bluetooth_device_get_type(),
    }
}

impl Device {
        pub const NONE: Option<&'static Device> = None;
    

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`Device`] objects.
            ///
            /// This method returns an instance of [`DeviceBuilder`](crate::builders::DeviceBuilder) which can be used to create [`Device`] objects.
            pub fn builder() -> DeviceBuilder {
                DeviceBuilder::new()
            }
        
}

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`Device`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct DeviceBuilder {
            builder: glib::object::ObjectBuilder<'static, Device>,
        }

        impl DeviceBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            pub fn connecting(self, connecting: bool) -> Self {
                            
                            Self { builder: self.builder.property("connecting", connecting), }
                        }

                            pub fn blocked(self, blocked: bool) -> Self {
                            
                            Self { builder: self.builder.property("blocked", blocked), }
                        }

                            pub fn trusted(self, trusted: bool) -> Self {
                            
                            Self { builder: self.builder.property("trusted", trusted), }
                        }

                            pub fn alias(self, alias: impl Into<glib::GString>) -> Self {
                            
                            Self { builder: self.builder.property("alias", alias.into()), }
                        }

    // rustdoc-stripper-ignore-next
    /// Build the [`Device`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Device {
assert_initialized_main_thread!();
    self.builder.build() }
}

pub trait DeviceExt: IsA<Device> + 'static {
    //#[doc(alias = "astal_bluetooth_device_connect_device")]
    //fn connect_device(&self, _callback_: AsyncReadyCallback) {
    //    unsafe { TODO: call ffi:astal_bluetooth_device_connect_device() }
    //}

    //#[doc(alias = "astal_bluetooth_device_disconnect_device")]
    //fn disconnect_device(&self, _callback_: AsyncReadyCallback) {
    //    unsafe { TODO: call ffi:astal_bluetooth_device_disconnect_device() }
    //}

    #[doc(alias = "astal_bluetooth_device_connect_profile")]
    fn connect_profile(&self, uuid: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let _ = ffi::astal_bluetooth_device_connect_profile(self.as_ref().to_glib_none().0, uuid.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "astal_bluetooth_device_disconnect_profile")]
    fn disconnect_profile(&self, uuid: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let _ = ffi::astal_bluetooth_device_disconnect_profile(self.as_ref().to_glib_none().0, uuid.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "astal_bluetooth_device_pair")]
    fn pair(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let _ = ffi::astal_bluetooth_device_pair(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "astal_bluetooth_device_cancel_pairing")]
    fn cancel_pairing(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let _ = ffi::astal_bluetooth_device_cancel_pairing(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "astal_bluetooth_device_get_uuids")]
    #[doc(alias = "get_uuids")]
    fn uuids(&self) -> Vec<glib::GString> {
        unsafe {
            let mut result_length1 = std::mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_full_num(ffi::astal_bluetooth_device_get_uuids(self.as_ref().to_glib_none().0, result_length1.as_mut_ptr()), result_length1.assume_init() as _);
            ret
        }
    }

    #[doc(alias = "astal_bluetooth_device_get_connected")]
    #[doc(alias = "get_connected")]
    fn is_connected(&self) -> bool {
        unsafe {
            from_glib(ffi::astal_bluetooth_device_get_connected(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_bluetooth_device_get_legacy_pairing")]
    #[doc(alias = "get_legacy_pairing")]
    fn is_legacy_pairing(&self) -> bool {
        unsafe {
            from_glib(ffi::astal_bluetooth_device_get_legacy_pairing(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_bluetooth_device_get_paired")]
    #[doc(alias = "get_paired")]
    fn is_paired(&self) -> bool {
        unsafe {
            from_glib(ffi::astal_bluetooth_device_get_paired(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_bluetooth_device_get_rssi")]
    #[doc(alias = "get_rssi")]
    fn rssi(&self) -> i16 {
        unsafe {
            ffi::astal_bluetooth_device_get_rssi(self.as_ref().to_glib_none().0)
        }
    }

    #[doc(alias = "astal_bluetooth_device_get_adapter")]
    #[doc(alias = "get_adapter")]
    fn adapter(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::astal_bluetooth_device_get_adapter(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_bluetooth_device_get_address")]
    #[doc(alias = "get_address")]
    fn address(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::astal_bluetooth_device_get_address(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_bluetooth_device_get_icon")]
    #[doc(alias = "get_icon")]
    fn icon(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::astal_bluetooth_device_get_icon(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_bluetooth_device_get_modalias")]
    #[doc(alias = "get_modalias")]
    fn modalias(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::astal_bluetooth_device_get_modalias(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_bluetooth_device_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::astal_bluetooth_device_get_name(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_bluetooth_device_get_appearance")]
    #[doc(alias = "get_appearance")]
    fn appearance(&self) -> u16 {
        unsafe {
            ffi::astal_bluetooth_device_get_appearance(self.as_ref().to_glib_none().0)
        }
    }

    #[doc(alias = "astal_bluetooth_device_get_class")]
    #[doc(alias = "get_class")]
    fn class(&self) -> u32 {
        unsafe {
            ffi::astal_bluetooth_device_get_class(self.as_ref().to_glib_none().0)
        }
    }

    #[doc(alias = "astal_bluetooth_device_get_connecting")]
    #[doc(alias = "get_connecting")]
    fn is_connecting(&self) -> bool {
        unsafe {
            from_glib(ffi::astal_bluetooth_device_get_connecting(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_bluetooth_device_get_blocked")]
    #[doc(alias = "get_blocked")]
    fn is_blocked(&self) -> bool {
        unsafe {
            from_glib(ffi::astal_bluetooth_device_get_blocked(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_bluetooth_device_set_blocked")]
    fn set_blocked(&self, value: bool) {
        unsafe {
            ffi::astal_bluetooth_device_set_blocked(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "astal_bluetooth_device_get_trusted")]
    #[doc(alias = "get_trusted")]
    fn is_trusted(&self) -> bool {
        unsafe {
            from_glib(ffi::astal_bluetooth_device_get_trusted(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_bluetooth_device_set_trusted")]
    fn set_trusted(&self, value: bool) {
        unsafe {
            ffi::astal_bluetooth_device_set_trusted(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "astal_bluetooth_device_get_battery_percentage")]
    #[doc(alias = "get_battery_percentage")]
    fn battery_percentage(&self) -> f64 {
        unsafe {
            ffi::astal_bluetooth_device_get_battery_percentage(self.as_ref().to_glib_none().0)
        }
    }

    #[doc(alias = "astal_bluetooth_device_get_alias")]
    #[doc(alias = "get_alias")]
    fn alias(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::astal_bluetooth_device_get_alias(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_bluetooth_device_set_alias")]
    fn set_alias(&self, value: &str) {
        unsafe {
            ffi::astal_bluetooth_device_set_alias(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_connecting(&self, connecting: bool) {
        ObjectExt::set_property(self.as_ref(),"connecting", connecting)
    }

    #[doc(alias = "uuids")]
    fn connect_uuids_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_uuids_trampoline<P: IsA<Device>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothDevice, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Device::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::uuids\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_uuids_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "connected")]
    fn connect_connected_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_connected_trampoline<P: IsA<Device>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothDevice, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Device::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::connected\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_connected_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "legacy-pairing")]
    fn connect_legacy_pairing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_legacy_pairing_trampoline<P: IsA<Device>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothDevice, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Device::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::legacy-pairing\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_legacy_pairing_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "paired")]
    fn connect_paired_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_paired_trampoline<P: IsA<Device>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothDevice, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Device::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::paired\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_paired_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "rssi")]
    fn connect_rssi_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_rssi_trampoline<P: IsA<Device>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothDevice, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Device::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::rssi\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_rssi_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "adapter")]
    fn connect_adapter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_adapter_trampoline<P: IsA<Device>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothDevice, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Device::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::adapter\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_adapter_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "address")]
    fn connect_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_address_trampoline<P: IsA<Device>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothDevice, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Device::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::address\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_address_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "icon")]
    fn connect_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_trampoline<P: IsA<Device>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothDevice, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Device::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::icon\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_icon_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "modalias")]
    fn connect_modalias_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_modalias_trampoline<P: IsA<Device>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothDevice, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Device::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::modalias\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_modalias_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "name")]
    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P: IsA<Device>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothDevice, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Device::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_name_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "appearance")]
    fn connect_appearance_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_appearance_trampoline<P: IsA<Device>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothDevice, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Device::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::appearance\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_appearance_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "class")]
    fn connect_class_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_class_trampoline<P: IsA<Device>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothDevice, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Device::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::class\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_class_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "connecting")]
    fn connect_connecting_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_connecting_trampoline<P: IsA<Device>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothDevice, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Device::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::connecting\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_connecting_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "blocked")]
    fn connect_blocked_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_blocked_trampoline<P: IsA<Device>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothDevice, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Device::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::blocked\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_blocked_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "trusted")]
    fn connect_trusted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_trusted_trampoline<P: IsA<Device>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothDevice, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Device::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::trusted\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_trusted_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "battery-percentage")]
    fn connect_battery_percentage_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_battery_percentage_trampoline<P: IsA<Device>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothDevice, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Device::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::battery-percentage\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_battery_percentage_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "alias")]
    fn connect_alias_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_alias_trampoline<P: IsA<Device>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothDevice, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Device::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::alias\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_alias_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<Device>> DeviceExt for O {}
