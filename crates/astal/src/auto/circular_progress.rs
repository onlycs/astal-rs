// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../../gobject/gir-files
// DO NOT EDIT

use crate::{ffi};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "AstalCircularProgress")]
    pub struct CircularProgress(Object<ffi::AstalCircularProgress, ffi::AstalCircularProgressClass>) @extends gtk::Bin, gtk::Container, gtk::Widget, gobject::InitiallyUnowned, @implements atk::ImplementorIface, gtk::Buildable;

    match fn {
        type_ => || ffi::astal_circular_progress_get_type(),
    }
}

impl CircularProgress {
        pub const NONE: Option<&'static CircularProgress> = None;
    

    #[doc(alias = "astal_circular_progress_new")]
    pub fn new() -> CircularProgress {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::astal_circular_progress_new())
        }
    }
}

impl Default for CircularProgress {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

pub trait CircularProgressExt: IsA<CircularProgress> + 'static {
    #[doc(alias = "astal_circular_progress_get_start_at")]
    #[doc(alias = "get_start_at")]
    fn start_at(&self) -> f64 {
        unsafe {
            ffi::astal_circular_progress_get_start_at(self.as_ref().to_glib_none().0)
        }
    }

    #[doc(alias = "astal_circular_progress_set_start_at")]
    fn set_start_at(&self, value: f64) {
        unsafe {
            ffi::astal_circular_progress_set_start_at(self.as_ref().to_glib_none().0, value);
        }
    }

    #[doc(alias = "astal_circular_progress_get_end_at")]
    #[doc(alias = "get_end_at")]
    fn end_at(&self) -> f64 {
        unsafe {
            ffi::astal_circular_progress_get_end_at(self.as_ref().to_glib_none().0)
        }
    }

    #[doc(alias = "astal_circular_progress_set_end_at")]
    fn set_end_at(&self, value: f64) {
        unsafe {
            ffi::astal_circular_progress_set_end_at(self.as_ref().to_glib_none().0, value);
        }
    }

    #[doc(alias = "astal_circular_progress_get_value")]
    #[doc(alias = "get_value")]
    fn value(&self) -> f64 {
        unsafe {
            ffi::astal_circular_progress_get_value(self.as_ref().to_glib_none().0)
        }
    }

    #[doc(alias = "astal_circular_progress_set_value")]
    fn set_value(&self, value: f64) {
        unsafe {
            ffi::astal_circular_progress_set_value(self.as_ref().to_glib_none().0, value);
        }
    }

    #[doc(alias = "astal_circular_progress_get_inverted")]
    #[doc(alias = "get_inverted")]
    fn is_inverted(&self) -> bool {
        unsafe {
            from_glib(ffi::astal_circular_progress_get_inverted(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_circular_progress_set_inverted")]
    fn set_inverted(&self, value: bool) {
        unsafe {
            ffi::astal_circular_progress_set_inverted(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "astal_circular_progress_get_rounded")]
    #[doc(alias = "get_rounded")]
    fn is_rounded(&self) -> bool {
        unsafe {
            from_glib(ffi::astal_circular_progress_get_rounded(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_circular_progress_set_rounded")]
    fn set_rounded(&self, value: bool) {
        unsafe {
            ffi::astal_circular_progress_set_rounded(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "start-at")]
    fn connect_start_at_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_start_at_trampoline<P: IsA<CircularProgress>, F: Fn(&P) + 'static>(this: *mut ffi::AstalCircularProgress, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(CircularProgress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::start-at\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_start_at_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "end-at")]
    fn connect_end_at_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_end_at_trampoline<P: IsA<CircularProgress>, F: Fn(&P) + 'static>(this: *mut ffi::AstalCircularProgress, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(CircularProgress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::end-at\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_end_at_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "value")]
    fn connect_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_trampoline<P: IsA<CircularProgress>, F: Fn(&P) + 'static>(this: *mut ffi::AstalCircularProgress, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(CircularProgress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::value\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_value_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "inverted")]
    fn connect_inverted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_inverted_trampoline<P: IsA<CircularProgress>, F: Fn(&P) + 'static>(this: *mut ffi::AstalCircularProgress, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(CircularProgress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::inverted\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_inverted_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "rounded")]
    fn connect_rounded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_rounded_trampoline<P: IsA<CircularProgress>, F: Fn(&P) + 'static>(this: *mut ffi::AstalCircularProgress, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(CircularProgress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::rounded\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_rounded_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<CircularProgress>> CircularProgressExt for O {}
