// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../../gobject/gir-files
// DO NOT EDIT

use crate::{ffi};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "AstalIOApplication")]
    pub struct Application(Interface<ffi::AstalIOApplication, ffi::AstalIOApplicationIface>);

    match fn {
        type_ => || ffi::astal_io_application_get_type(),
    }
}

impl Application {
        pub const NONE: Option<&'static Application> = None;
    
}

pub trait ApplicationExt: IsA<Application> + 'static {
    #[doc(alias = "astal_io_application_quit")]
    fn quit(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let _ = ffi::astal_io_application_quit(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "astal_io_application_inspector")]
    fn inspector(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let _ = ffi::astal_io_application_inspector(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "astal_io_application_toggle_window")]
    fn toggle_window(&self, window: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let _ = ffi::astal_io_application_toggle_window(self.as_ref().to_glib_none().0, window.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "astal_io_application_acquire_socket")]
    fn acquire_socket(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let _ = ffi::astal_io_application_acquire_socket(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "astal_io_application_request")]
    fn request(&self, msg: &str, conn: &impl IsA<gio::SocketConnection>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let _ = ffi::astal_io_application_request(self.as_ref().to_glib_none().0, msg.to_glib_none().0, conn.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "astal_io_application_get_instance_name")]
    #[doc(alias = "get_instance_name")]
    fn instance_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::astal_io_application_get_instance_name(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_io_application_set_instance_name")]
    fn set_instance_name(&self, value: &str) {
        unsafe {
            ffi::astal_io_application_set_instance_name(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "instance-name")]
    fn connect_instance_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_instance_name_trampoline<P: IsA<Application>, F: Fn(&P) + 'static>(this: *mut ffi::AstalIOApplication, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::instance-name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_instance_name_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<Application>> ApplicationExt for O {}
