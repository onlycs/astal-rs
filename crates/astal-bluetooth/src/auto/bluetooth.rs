// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../../gir/gir-files
// DO NOT EDIT

use crate::{ffi,Adapter,Device};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "AstalBluetoothBluetooth")]
    pub struct Bluetooth(Object<ffi::AstalBluetoothBluetooth, ffi::AstalBluetoothBluetoothClass>);

    match fn {
        type_ => || ffi::astal_bluetooth_bluetooth_get_type(),
    }
}

impl Bluetooth {
        pub const NONE: Option<&'static Bluetooth> = None;
    

    #[doc(alias = "astal_bluetooth_bluetooth_new")]
    pub fn new() -> Bluetooth {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::astal_bluetooth_bluetooth_new())
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`Bluetooth`] objects.
            ///
            /// This method returns an instance of [`BluetoothBuilder`](crate::builders::BluetoothBuilder) which can be used to create [`Bluetooth`] objects.
            pub fn builder() -> BluetoothBuilder {
                BluetoothBuilder::new()
            }
        

    #[doc(alias = "astal_bluetooth_bluetooth_get_default")]
    #[doc(alias = "get_default")]
    #[allow(clippy::should_implement_trait)]    pub fn default() -> Option<Bluetooth> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::astal_bluetooth_bluetooth_get_default())
        }
    }
}

impl Default for Bluetooth {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`Bluetooth`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct BluetoothBuilder {
            builder: glib::object::ObjectBuilder<'static, Bluetooth>,
        }

        impl BluetoothBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            pub fn is_powered(self, is_powered: bool) -> Self {
                            
                            Self { builder: self.builder.property("is-powered", is_powered), }
                        }

                            pub fn is_connected(self, is_connected: bool) -> Self {
                            
                            Self { builder: self.builder.property("is-connected", is_connected), }
                        }

    // rustdoc-stripper-ignore-next
    /// Build the [`Bluetooth`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Bluetooth {
assert_initialized_main_thread!();
    self.builder.build() }
}

pub trait BluetoothExt: IsA<Bluetooth> + 'static {
    #[doc(alias = "astal_bluetooth_bluetooth_toggle")]
    fn toggle(&self) {
        unsafe {
            ffi::astal_bluetooth_bluetooth_toggle(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "astal_bluetooth_bluetooth_get_is_powered")]
    #[doc(alias = "get_is_powered")]
    fn is_powered(&self) -> bool {
        unsafe {
            from_glib(ffi::astal_bluetooth_bluetooth_get_is_powered(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_bluetooth_bluetooth_get_is_connected")]
    #[doc(alias = "get_is_connected")]
    fn is_connected(&self) -> bool {
        unsafe {
            from_glib(ffi::astal_bluetooth_bluetooth_get_is_connected(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_bluetooth_bluetooth_get_adapter")]
    #[doc(alias = "get_adapter")]
    fn adapter(&self) -> Option<Adapter> {
        unsafe {
            from_glib_none(ffi::astal_bluetooth_bluetooth_get_adapter(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_bluetooth_bluetooth_get_adapters")]
    #[doc(alias = "get_adapters")]
    fn adapters(&self) -> Vec<Adapter> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::astal_bluetooth_bluetooth_get_adapters(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_bluetooth_bluetooth_get_devices")]
    #[doc(alias = "get_devices")]
    fn devices(&self) -> Vec<Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::astal_bluetooth_bluetooth_get_devices(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "is-powered")]
    fn set_is_powered(&self, is_powered: bool) {
        ObjectExt::set_property(self.as_ref(),"is-powered", is_powered)
    }

    #[doc(alias = "is-connected")]
    fn set_is_connected(&self, is_connected: bool) {
        ObjectExt::set_property(self.as_ref(),"is-connected", is_connected)
    }

    #[doc(alias = "device-added")]
    fn connect_device_added<F: Fn(&Self, &Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn device_added_trampoline<P: IsA<Bluetooth>, F: Fn(&P, &Device) + 'static>(this: *mut ffi::AstalBluetoothBluetooth, device: *mut ffi::AstalBluetoothDevice, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Bluetooth::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(device))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"device-added\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(device_added_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "device-removed")]
    fn connect_device_removed<F: Fn(&Self, &Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn device_removed_trampoline<P: IsA<Bluetooth>, F: Fn(&P, &Device) + 'static>(this: *mut ffi::AstalBluetoothBluetooth, device: *mut ffi::AstalBluetoothDevice, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Bluetooth::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(device))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"device-removed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(device_removed_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "adapter-added")]
    fn connect_adapter_added<F: Fn(&Self, &Adapter) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn adapter_added_trampoline<P: IsA<Bluetooth>, F: Fn(&P, &Adapter) + 'static>(this: *mut ffi::AstalBluetoothBluetooth, adapter: *mut ffi::AstalBluetoothAdapter, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Bluetooth::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(adapter))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"adapter-added\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(adapter_added_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "adapter-removed")]
    fn connect_adapter_removed<F: Fn(&Self, &Adapter) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn adapter_removed_trampoline<P: IsA<Bluetooth>, F: Fn(&P, &Adapter) + 'static>(this: *mut ffi::AstalBluetoothBluetooth, adapter: *mut ffi::AstalBluetoothAdapter, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Bluetooth::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(adapter))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"adapter-removed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(adapter_removed_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "is-powered")]
    fn connect_is_powered_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_powered_trampoline<P: IsA<Bluetooth>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothBluetooth, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Bluetooth::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::is-powered\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_is_powered_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "is-connected")]
    fn connect_is_connected_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_connected_trampoline<P: IsA<Bluetooth>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothBluetooth, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Bluetooth::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::is-connected\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_is_connected_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "adapter")]
    fn connect_adapter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_adapter_trampoline<P: IsA<Bluetooth>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothBluetooth, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Bluetooth::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::adapter\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_adapter_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "adapters")]
    fn connect_adapters_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_adapters_trampoline<P: IsA<Bluetooth>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothBluetooth, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Bluetooth::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::adapters\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_adapters_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "devices")]
    fn connect_devices_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_devices_trampoline<P: IsA<Bluetooth>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothBluetooth, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Bluetooth::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::devices\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_devices_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<Bluetooth>> BluetoothExt for O {}