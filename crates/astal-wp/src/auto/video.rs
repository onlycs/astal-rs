// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../../gir/gir-files
// DO NOT EDIT

use crate::{ffi,Device,Endpoint,Wp};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "AstalWpVideo")]
    pub struct Video(Object<ffi::AstalWpVideo, ffi::AstalWpVideoClass>);

    match fn {
        type_ => || ffi::astal_wp_video_get_type(),
    }
}

impl Video {
    #[doc(alias = "astal_wp_video_new")]
    pub fn new(wp: &Wp) -> Video {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::astal_wp_video_new(wp.to_glib_none().0))
        }
    }

    #[doc(alias = "astal_wp_video_get_device")]
    #[doc(alias = "get_device")]
    pub fn device(&self, id: u32) -> Option<Device> {
        unsafe {
            from_glib_none(ffi::astal_wp_video_get_device(self.to_glib_none().0, id))
        }
    }

    #[doc(alias = "astal_wp_video_get_devices")]
    #[doc(alias = "get_devices")]
    pub fn devices(&self) -> Vec<Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::astal_wp_video_get_devices(self.to_glib_none().0))
        }
    }

    #[doc(alias = "astal_wp_video_get_recorder")]
    #[doc(alias = "get_recorder")]
    pub fn recorder(&self, id: u32) -> Option<Endpoint> {
        unsafe {
            from_glib_none(ffi::astal_wp_video_get_recorder(self.to_glib_none().0, id))
        }
    }

    #[doc(alias = "astal_wp_video_get_recorders")]
    #[doc(alias = "get_recorders")]
    pub fn recorders(&self) -> Vec<Endpoint> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::astal_wp_video_get_recorders(self.to_glib_none().0))
        }
    }

    #[doc(alias = "astal_wp_video_get_sink")]
    #[doc(alias = "get_sink")]
    pub fn sink(&self, id: u32) -> Option<Endpoint> {
        unsafe {
            from_glib_none(ffi::astal_wp_video_get_sink(self.to_glib_none().0, id))
        }
    }

    #[doc(alias = "astal_wp_video_get_sinks")]
    #[doc(alias = "get_sinks")]
    pub fn sinks(&self) -> Vec<Endpoint> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::astal_wp_video_get_sinks(self.to_glib_none().0))
        }
    }

    #[doc(alias = "astal_wp_video_get_source")]
    #[doc(alias = "get_source")]
    pub fn source(&self, id: u32) -> Option<Endpoint> {
        unsafe {
            from_glib_none(ffi::astal_wp_video_get_source(self.to_glib_none().0, id))
        }
    }

    #[doc(alias = "astal_wp_video_get_sources")]
    #[doc(alias = "get_sources")]
    pub fn sources(&self) -> Vec<Endpoint> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::astal_wp_video_get_sources(self.to_glib_none().0))
        }
    }

    #[doc(alias = "astal_wp_video_get_stream")]
    #[doc(alias = "get_stream")]
    pub fn stream(&self, id: u32) -> Option<Endpoint> {
        unsafe {
            from_glib_none(ffi::astal_wp_video_get_stream(self.to_glib_none().0, id))
        }
    }

    #[doc(alias = "astal_wp_video_get_streams")]
    #[doc(alias = "get_streams")]
    pub fn streams(&self) -> Vec<Endpoint> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::astal_wp_video_get_streams(self.to_glib_none().0))
        }
    }

    #[doc(alias = "device-added")]
    pub fn connect_device_added<F: Fn(&Self, &Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn device_added_trampoline<F: Fn(&Video, &Device) + 'static>(this: *mut ffi::AstalWpVideo, object: *mut ffi::AstalWpDevice, f: glib::ffi::gpointer) {
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
        unsafe extern "C" fn device_removed_trampoline<F: Fn(&Video, &Device) + 'static>(this: *mut ffi::AstalWpVideo, object: *mut ffi::AstalWpDevice, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(object))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"device-removed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(device_removed_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "recorder-added")]
    pub fn connect_recorder_added<F: Fn(&Self, &Endpoint) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn recorder_added_trampoline<F: Fn(&Video, &Endpoint) + 'static>(this: *mut ffi::AstalWpVideo, object: *mut ffi::AstalWpEndpoint, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(object))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"recorder-added\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(recorder_added_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "recorder-removed")]
    pub fn connect_recorder_removed<F: Fn(&Self, &Endpoint) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn recorder_removed_trampoline<F: Fn(&Video, &Endpoint) + 'static>(this: *mut ffi::AstalWpVideo, object: *mut ffi::AstalWpEndpoint, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(object))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"recorder-removed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(recorder_removed_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "sink-added")]
    pub fn connect_sink_added<F: Fn(&Self, &Endpoint) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn sink_added_trampoline<F: Fn(&Video, &Endpoint) + 'static>(this: *mut ffi::AstalWpVideo, object: *mut ffi::AstalWpEndpoint, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(object))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"sink-added\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(sink_added_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "sink-removed")]
    pub fn connect_sink_removed<F: Fn(&Self, &Endpoint) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn sink_removed_trampoline<F: Fn(&Video, &Endpoint) + 'static>(this: *mut ffi::AstalWpVideo, object: *mut ffi::AstalWpEndpoint, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(object))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"sink-removed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(sink_removed_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "source-added")]
    pub fn connect_source_added<F: Fn(&Self, &Endpoint) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn source_added_trampoline<F: Fn(&Video, &Endpoint) + 'static>(this: *mut ffi::AstalWpVideo, object: *mut ffi::AstalWpEndpoint, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(object))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"source-added\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(source_added_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "source-removed")]
    pub fn connect_source_removed<F: Fn(&Self, &Endpoint) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn source_removed_trampoline<F: Fn(&Video, &Endpoint) + 'static>(this: *mut ffi::AstalWpVideo, object: *mut ffi::AstalWpEndpoint, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(object))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"source-removed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(source_removed_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "stream-added")]
    pub fn connect_stream_added<F: Fn(&Self, &Endpoint) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn stream_added_trampoline<F: Fn(&Video, &Endpoint) + 'static>(this: *mut ffi::AstalWpVideo, object: *mut ffi::AstalWpEndpoint, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(object))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"stream-added\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(stream_added_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "stream-removed")]
    pub fn connect_stream_removed<F: Fn(&Self, &Endpoint) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn stream_removed_trampoline<F: Fn(&Video, &Endpoint) + 'static>(this: *mut ffi::AstalWpVideo, object: *mut ffi::AstalWpEndpoint, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(object))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"stream-removed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(stream_removed_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "devices")]
    pub fn connect_devices_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_devices_trampoline<F: Fn(&Video) + 'static>(this: *mut ffi::AstalWpVideo, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::devices\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_devices_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "recorders")]
    pub fn connect_recorders_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_recorders_trampoline<F: Fn(&Video) + 'static>(this: *mut ffi::AstalWpVideo, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::recorders\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_recorders_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "sinks")]
    pub fn connect_sinks_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_sinks_trampoline<F: Fn(&Video) + 'static>(this: *mut ffi::AstalWpVideo, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::sinks\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_sinks_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "sources")]
    pub fn connect_sources_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_sources_trampoline<F: Fn(&Video) + 'static>(this: *mut ffi::AstalWpVideo, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::sources\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_sources_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "streams")]
    pub fn connect_streams_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_streams_trampoline<F: Fn(&Video) + 'static>(this: *mut ffi::AstalWpVideo, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::streams\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_streams_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}
