// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../../gir/gir-files
// DO NOT EDIT

use crate::{ffi,Audio,Device,Endpoint,Scale,Video};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "AstalWpWp")]
    pub struct Wp(Object<ffi::AstalWpWp, ffi::AstalWpWpClass>);

    match fn {
        type_ => || ffi::astal_wp_wp_get_type(),
    }
}

impl Wp {
    #[doc(alias = "astal_wp_wp_get_audio")]
    #[doc(alias = "get_audio")]
    pub fn audio(&self) -> Option<Audio> {
        unsafe {
            from_glib_none(ffi::astal_wp_wp_get_audio(self.to_glib_none().0))
        }
    }

    #[doc(alias = "astal_wp_wp_get_default_microphone")]
    #[doc(alias = "get_default_microphone")]
    #[doc(alias = "default-microphone")]
    pub fn default_microphone(&self) -> Option<Endpoint> {
        unsafe {
            from_glib_none(ffi::astal_wp_wp_get_default_microphone(self.to_glib_none().0))
        }
    }

    #[doc(alias = "astal_wp_wp_get_default_speaker")]
    #[doc(alias = "get_default_speaker")]
    #[doc(alias = "default-speaker")]
    pub fn default_speaker(&self) -> Option<Endpoint> {
        unsafe {
            from_glib_none(ffi::astal_wp_wp_get_default_speaker(self.to_glib_none().0))
        }
    }

    #[doc(alias = "astal_wp_wp_get_device")]
    #[doc(alias = "get_device")]
    pub fn device(&self, id: u32) -> Option<Device> {
        unsafe {
            from_glib_none(ffi::astal_wp_wp_get_device(self.to_glib_none().0, id))
        }
    }

    #[doc(alias = "astal_wp_wp_get_devices")]
    #[doc(alias = "get_devices")]
    pub fn devices(&self) -> Vec<Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::astal_wp_wp_get_devices(self.to_glib_none().0))
        }
    }

    #[doc(alias = "astal_wp_wp_get_endpoint")]
    #[doc(alias = "get_endpoint")]
    pub fn endpoint(&self, id: u32) -> Option<Endpoint> {
        unsafe {
            from_glib_none(ffi::astal_wp_wp_get_endpoint(self.to_glib_none().0, id))
        }
    }

    #[doc(alias = "astal_wp_wp_get_endpoints")]
    #[doc(alias = "get_endpoints")]
    pub fn endpoints(&self) -> Vec<Endpoint> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::astal_wp_wp_get_endpoints(self.to_glib_none().0))
        }
    }

    #[doc(alias = "astal_wp_wp_get_scale")]
    #[doc(alias = "get_scale")]
    pub fn scale(&self) -> Scale {
        unsafe {
            from_glib(ffi::astal_wp_wp_get_scale(self.to_glib_none().0))
        }
    }

    #[doc(alias = "astal_wp_wp_get_video")]
    #[doc(alias = "get_video")]
    pub fn video(&self) -> Option<Video> {
        unsafe {
            from_glib_none(ffi::astal_wp_wp_get_video(self.to_glib_none().0))
        }
    }

    #[doc(alias = "astal_wp_wp_set_scale")]
    #[doc(alias = "scale")]
    pub fn set_scale(&self, scale: Scale) {
        unsafe {
            ffi::astal_wp_wp_set_scale(self.to_glib_none().0, scale.into_glib());
        }
    }

    #[doc(alias = "astal_wp_wp_get_default")]
    #[doc(alias = "get_default")]
    #[allow(clippy::should_implement_trait)]    pub fn default() -> Option<Wp> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::astal_wp_wp_get_default())
        }
    }

    #[doc(alias = "device-added")]
    pub fn connect_device_added<F: Fn(&Self, &Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn device_added_trampoline<F: Fn(&Wp, &Device) + 'static>(this: *mut ffi::AstalWpWp, object: *mut ffi::AstalWpDevice, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(object))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"device-added\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(device_added_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "device-removed")]
    pub fn connect_device_removed<F: Fn(&Self, &Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn device_removed_trampoline<F: Fn(&Wp, &Device) + 'static>(this: *mut ffi::AstalWpWp, object: *mut ffi::AstalWpDevice, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(object))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"device-removed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(device_removed_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "endpoint-added")]
    pub fn connect_endpoint_added<F: Fn(&Self, &Endpoint) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn endpoint_added_trampoline<F: Fn(&Wp, &Endpoint) + 'static>(this: *mut ffi::AstalWpWp, object: *mut ffi::AstalWpEndpoint, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(object))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"endpoint-added\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(endpoint_added_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "endpoint-removed")]
    pub fn connect_endpoint_removed<F: Fn(&Self, &Endpoint) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn endpoint_removed_trampoline<F: Fn(&Wp, &Endpoint) + 'static>(this: *mut ffi::AstalWpWp, object: *mut ffi::AstalWpEndpoint, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(object))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"endpoint-removed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(endpoint_removed_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "audio")]
    pub fn connect_audio_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_audio_trampoline<F: Fn(&Wp) + 'static>(this: *mut ffi::AstalWpWp, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::audio\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_audio_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "default-microphone")]
    pub fn connect_default_microphone_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_default_microphone_trampoline<F: Fn(&Wp) + 'static>(this: *mut ffi::AstalWpWp, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::default-microphone\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_default_microphone_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "default-speaker")]
    pub fn connect_default_speaker_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_default_speaker_trampoline<F: Fn(&Wp) + 'static>(this: *mut ffi::AstalWpWp, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::default-speaker\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_default_speaker_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "devices")]
    pub fn connect_devices_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_devices_trampoline<F: Fn(&Wp) + 'static>(this: *mut ffi::AstalWpWp, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::devices\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_devices_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "endpoints")]
    pub fn connect_endpoints_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_endpoints_trampoline<F: Fn(&Wp) + 'static>(this: *mut ffi::AstalWpWp, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::endpoints\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_endpoints_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "scale")]
    pub fn connect_scale_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_scale_trampoline<F: Fn(&Wp) + 'static>(this: *mut ffi::AstalWpWp, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::scale\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_scale_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "video")]
    pub fn connect_video_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_video_trampoline<F: Fn(&Wp) + 'static>(this: *mut ffi::AstalWpWp, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::video\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_video_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}
