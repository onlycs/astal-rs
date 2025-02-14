// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../../gir/gir-files
// DO NOT EDIT

use crate::{ffi,Device};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "AstalBluetoothAdapter")]
    pub struct Adapter(Object<ffi::AstalBluetoothAdapter, ffi::AstalBluetoothAdapterClass>);

    match fn {
        type_ => || ffi::astal_bluetooth_adapter_get_type(),
    }
}

impl Adapter {
        pub const NONE: Option<&'static Adapter> = None;
    

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`Adapter`] objects.
            ///
            /// This method returns an instance of [`AdapterBuilder`](crate::builders::AdapterBuilder) which can be used to create [`Adapter`] objects.
            pub fn builder() -> AdapterBuilder {
                AdapterBuilder::new()
            }
        
}

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`Adapter`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct AdapterBuilder {
            builder: glib::object::ObjectBuilder<'static, Adapter>,
        }

        impl AdapterBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            pub fn discoverable(self, discoverable: bool) -> Self {
                            
                            Self { builder: self.builder.property("discoverable", discoverable), }
                        }

                            pub fn pairable(self, pairable: bool) -> Self {
                            
                            Self { builder: self.builder.property("pairable", pairable), }
                        }

                            pub fn powered(self, powered: bool) -> Self {
                            
                            Self { builder: self.builder.property("powered", powered), }
                        }

                            pub fn alias(self, alias: impl Into<glib::GString>) -> Self {
                            
                            Self { builder: self.builder.property("alias", alias.into()), }
                        }

                            pub fn discoverable_timeout(self, discoverable_timeout: u32) -> Self {
                            
                            Self { builder: self.builder.property("discoverable-timeout", discoverable_timeout), }
                        }

                            pub fn pairable_timeout(self, pairable_timeout: u32) -> Self {
                            
                            Self { builder: self.builder.property("pairable-timeout", pairable_timeout), }
                        }

    // rustdoc-stripper-ignore-next
    /// Build the [`Adapter`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Adapter {
assert_initialized_main_thread!();
    self.builder.build() }
}

pub trait AdapterExt: IsA<Adapter> + 'static {
    #[doc(alias = "astal_bluetooth_adapter_remove_device")]
    fn remove_device(&self, device: &impl IsA<Device>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let _ = ffi::astal_bluetooth_adapter_remove_device(self.as_ref().to_glib_none().0, device.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "astal_bluetooth_adapter_start_discovery")]
    fn start_discovery(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let _ = ffi::astal_bluetooth_adapter_start_discovery(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "astal_bluetooth_adapter_stop_discovery")]
    fn stop_discovery(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let _ = ffi::astal_bluetooth_adapter_stop_discovery(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "astal_bluetooth_adapter_get_uuids")]
    #[doc(alias = "get_uuids")]
    fn uuids(&self) -> Vec<glib::GString> {
        unsafe {
            let mut result_length1 = std::mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_full_num(ffi::astal_bluetooth_adapter_get_uuids(self.as_ref().to_glib_none().0, result_length1.as_mut_ptr()), result_length1.assume_init() as _);
            ret
        }
    }

    #[doc(alias = "astal_bluetooth_adapter_get_discovering")]
    #[doc(alias = "get_discovering")]
    fn is_discovering(&self) -> bool {
        unsafe {
            from_glib(ffi::astal_bluetooth_adapter_get_discovering(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_bluetooth_adapter_get_modalias")]
    #[doc(alias = "get_modalias")]
    fn modalias(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::astal_bluetooth_adapter_get_modalias(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_bluetooth_adapter_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::astal_bluetooth_adapter_get_name(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_bluetooth_adapter_get_class")]
    #[doc(alias = "get_class")]
    fn class(&self) -> u32 {
        unsafe {
            ffi::astal_bluetooth_adapter_get_class(self.as_ref().to_glib_none().0)
        }
    }

    #[doc(alias = "astal_bluetooth_adapter_get_address")]
    #[doc(alias = "get_address")]
    fn address(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::astal_bluetooth_adapter_get_address(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_bluetooth_adapter_get_discoverable")]
    #[doc(alias = "get_discoverable")]
    fn is_discoverable(&self) -> bool {
        unsafe {
            from_glib(ffi::astal_bluetooth_adapter_get_discoverable(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_bluetooth_adapter_set_discoverable")]
    fn set_discoverable(&self, value: bool) {
        unsafe {
            ffi::astal_bluetooth_adapter_set_discoverable(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "astal_bluetooth_adapter_get_pairable")]
    #[doc(alias = "get_pairable")]
    fn is_pairable(&self) -> bool {
        unsafe {
            from_glib(ffi::astal_bluetooth_adapter_get_pairable(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_bluetooth_adapter_set_pairable")]
    fn set_pairable(&self, value: bool) {
        unsafe {
            ffi::astal_bluetooth_adapter_set_pairable(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "astal_bluetooth_adapter_get_powered")]
    #[doc(alias = "get_powered")]
    fn is_powered(&self) -> bool {
        unsafe {
            from_glib(ffi::astal_bluetooth_adapter_get_powered(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_bluetooth_adapter_set_powered")]
    fn set_powered(&self, value: bool) {
        unsafe {
            ffi::astal_bluetooth_adapter_set_powered(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "astal_bluetooth_adapter_get_alias")]
    #[doc(alias = "get_alias")]
    fn alias(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::astal_bluetooth_adapter_get_alias(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_bluetooth_adapter_set_alias")]
    fn set_alias(&self, value: &str) {
        unsafe {
            ffi::astal_bluetooth_adapter_set_alias(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "astal_bluetooth_adapter_get_discoverable_timeout")]
    #[doc(alias = "get_discoverable_timeout")]
    fn discoverable_timeout(&self) -> u32 {
        unsafe {
            ffi::astal_bluetooth_adapter_get_discoverable_timeout(self.as_ref().to_glib_none().0)
        }
    }

    #[doc(alias = "astal_bluetooth_adapter_set_discoverable_timeout")]
    fn set_discoverable_timeout(&self, value: u32) {
        unsafe {
            ffi::astal_bluetooth_adapter_set_discoverable_timeout(self.as_ref().to_glib_none().0, value);
        }
    }

    #[doc(alias = "astal_bluetooth_adapter_get_pairable_timeout")]
    #[doc(alias = "get_pairable_timeout")]
    fn pairable_timeout(&self) -> u32 {
        unsafe {
            ffi::astal_bluetooth_adapter_get_pairable_timeout(self.as_ref().to_glib_none().0)
        }
    }

    #[doc(alias = "astal_bluetooth_adapter_set_pairable_timeout")]
    fn set_pairable_timeout(&self, value: u32) {
        unsafe {
            ffi::astal_bluetooth_adapter_set_pairable_timeout(self.as_ref().to_glib_none().0, value);
        }
    }

    #[doc(alias = "uuids")]
    fn connect_uuids_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_uuids_trampoline<P: IsA<Adapter>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothAdapter, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Adapter::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::uuids\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_uuids_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "discovering")]
    fn connect_discovering_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_discovering_trampoline<P: IsA<Adapter>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothAdapter, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Adapter::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::discovering\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_discovering_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "modalias")]
    fn connect_modalias_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_modalias_trampoline<P: IsA<Adapter>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothAdapter, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Adapter::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::modalias\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_modalias_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "name")]
    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P: IsA<Adapter>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothAdapter, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Adapter::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_name_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "class")]
    fn connect_class_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_class_trampoline<P: IsA<Adapter>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothAdapter, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Adapter::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::class\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_class_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "address")]
    fn connect_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_address_trampoline<P: IsA<Adapter>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothAdapter, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Adapter::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::address\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_address_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "discoverable")]
    fn connect_discoverable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_discoverable_trampoline<P: IsA<Adapter>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothAdapter, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Adapter::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::discoverable\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_discoverable_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "pairable")]
    fn connect_pairable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pairable_trampoline<P: IsA<Adapter>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothAdapter, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Adapter::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::pairable\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_pairable_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "powered")]
    fn connect_powered_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_powered_trampoline<P: IsA<Adapter>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothAdapter, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Adapter::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::powered\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_powered_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "alias")]
    fn connect_alias_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_alias_trampoline<P: IsA<Adapter>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothAdapter, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Adapter::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::alias\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_alias_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "discoverable-timeout")]
    fn connect_discoverable_timeout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_discoverable_timeout_trampoline<P: IsA<Adapter>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothAdapter, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Adapter::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::discoverable-timeout\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_discoverable_timeout_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "pairable-timeout")]
    fn connect_pairable_timeout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pairable_timeout_trampoline<P: IsA<Adapter>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBluetoothAdapter, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Adapter::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::pairable-timeout\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_pairable_timeout_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<Adapter>> AdapterExt for O {}
