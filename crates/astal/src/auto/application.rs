// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../../gir/gir-files
// DO NOT EDIT

use crate::{ffi};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

#[cfg(feature = "gio_v2_28")]
#[cfg_attr(docsrs, doc(cfg(feature = "gio_v2_28")))]
glib::wrapper! {
    #[doc(alias = "AstalApplication")]
    pub struct Application(Object<ffi::AstalApplication, ffi::AstalApplicationClass>) @extends gtk::Application, gio::Application, @implements gio::ActionGroup, gio::ActionMap, astal_io::Application;

    match fn {
        type_ => || ffi::astal_application_get_type(),
    }
}

#[cfg(not(any(feature = "gio_v2_28")))]
#[cfg(feature = "gio_v2_32")]
glib::wrapper! {
    #[doc(alias = "AstalApplication")]
    pub struct Application(Object<ffi::AstalApplication, ffi::AstalApplicationClass>) @extends gtk::Application, @implements gio::ActionGroup, gio::ActionMap, astal_io::Application;

    match fn {
        type_ => || ffi::astal_application_get_type(),
    }
}

#[cfg(not(any(feature = "gio_v2_32")))]
glib::wrapper! {
    #[doc(alias = "AstalApplication")]
    pub struct Application(Object<ffi::AstalApplication, ffi::AstalApplicationClass>) @extends gtk::Application, @implements gio::ActionGroup, astal_io::Application;

    match fn {
        type_ => || ffi::astal_application_get_type(),
    }
}

impl Application {
        pub const NONE: Option<&'static Application> = None;
    

    #[doc(alias = "astal_application_new")]
    pub fn new() -> Application {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::astal_application_new())
        }
    }
}

impl Default for Application {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

pub trait ApplicationExt: IsA<Application> + 'static {
    #[doc(alias = "astal_application_reset_css")]
    fn reset_css(&self) {
        unsafe {
            ffi::astal_application_reset_css(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "astal_application_get_window")]
    #[doc(alias = "get_window")]
    fn window(&self, name: &str) -> Option<gtk::Window> {
        unsafe {
            from_glib_full(ffi::astal_application_get_window(self.as_ref().to_glib_none().0, name.to_glib_none().0))
        }
    }

    #[doc(alias = "astal_application_apply_css")]
    fn apply_css(&self, style: &str, reset: bool) {
        unsafe {
            ffi::astal_application_apply_css(self.as_ref().to_glib_none().0, style.to_glib_none().0, reset.into_glib());
        }
    }

    #[doc(alias = "astal_application_add_icons")]
    fn add_icons(&self, path: Option<&str>) {
        unsafe {
            ffi::astal_application_add_icons(self.as_ref().to_glib_none().0, path.to_glib_none().0);
        }
    }

    #[doc(alias = "astal_application_request")]
    fn request(&self, msg: &str, conn: &impl IsA<gio::SocketConnection>) {
        unsafe {
            ffi::astal_application_request(self.as_ref().to_glib_none().0, msg.to_glib_none().0, conn.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "astal_application_get_monitors")]
    #[doc(alias = "get_monitors")]
    fn monitors(&self) -> Vec<gdk::Monitor> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::astal_application_get_monitors(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_application_get_windows")]
    #[doc(alias = "get_windows")]
    fn windows(&self) -> Vec<gtk::Window> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::astal_application_get_windows(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_application_get_gtk_theme")]
    #[doc(alias = "get_gtk_theme")]
    fn gtk_theme(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::astal_application_get_gtk_theme(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_application_set_gtk_theme")]
    fn set_gtk_theme(&self, value: &str) {
        unsafe {
            ffi::astal_application_set_gtk_theme(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "astal_application_get_icon_theme")]
    #[doc(alias = "get_icon_theme")]
    fn icon_theme(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::astal_application_get_icon_theme(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_application_set_icon_theme")]
    fn set_icon_theme(&self, value: &str) {
        unsafe {
            ffi::astal_application_set_icon_theme(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "astal_application_get_cursor_theme")]
    #[doc(alias = "get_cursor_theme")]
    fn cursor_theme(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::astal_application_get_cursor_theme(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_application_set_cursor_theme")]
    fn set_cursor_theme(&self, value: &str) {
        unsafe {
            ffi::astal_application_set_cursor_theme(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "monitor-added")]
    fn connect_monitor_added<F: Fn(&Self, &gdk::Monitor) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn monitor_added_trampoline<P: IsA<Application>, F: Fn(&P, &gdk::Monitor) + 'static>(this: *mut ffi::AstalApplication, monitor: *mut gdk::ffi::GdkMonitor, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Application::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(monitor))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"monitor-added\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(monitor_added_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "monitor-removed")]
    fn connect_monitor_removed<F: Fn(&Self, &gdk::Monitor) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn monitor_removed_trampoline<P: IsA<Application>, F: Fn(&P, &gdk::Monitor) + 'static>(this: *mut ffi::AstalApplication, monitor: *mut gdk::ffi::GdkMonitor, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Application::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(monitor))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"monitor-removed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(monitor_removed_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "window-toggled")]
    fn connect_window_toggled<F: Fn(&Self, &gtk::Window) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn window_toggled_trampoline<P: IsA<Application>, F: Fn(&P, &gtk::Window) + 'static>(this: *mut ffi::AstalApplication, window: *mut gtk::ffi::GtkWindow, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Application::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(window))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"window-toggled\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(window_toggled_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "monitors")]
    fn connect_monitors_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_monitors_trampoline<P: IsA<Application>, F: Fn(&P) + 'static>(this: *mut ffi::AstalApplication, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::monitors\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_monitors_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "windows")]
    fn connect_windows_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_windows_trampoline<P: IsA<Application>, F: Fn(&P) + 'static>(this: *mut ffi::AstalApplication, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::windows\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_windows_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "gtk-theme")]
    fn connect_gtk_theme_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_gtk_theme_trampoline<P: IsA<Application>, F: Fn(&P) + 'static>(this: *mut ffi::AstalApplication, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::gtk-theme\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_gtk_theme_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "icon-theme")]
    fn connect_icon_theme_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_theme_trampoline<P: IsA<Application>, F: Fn(&P) + 'static>(this: *mut ffi::AstalApplication, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::icon-theme\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_icon_theme_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "cursor-theme")]
    fn connect_cursor_theme_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_cursor_theme_trampoline<P: IsA<Application>, F: Fn(&P) + 'static>(this: *mut ffi::AstalApplication, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::cursor-theme\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_cursor_theme_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<Application>> ApplicationExt for O {}
