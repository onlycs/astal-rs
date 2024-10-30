// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../../gir/gir-files
// DO NOT EDIT

use crate::{ffi};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "AstalLabel")]
    pub struct Label(Object<ffi::AstalLabel, ffi::AstalLabelClass>) @extends gtk::Label, gtk::Misc, gtk::Widget, gobject::InitiallyUnowned, @implements atk::ImplementorIface, gtk::Buildable;

    match fn {
        type_ => || ffi::astal_label_get_type(),
    }
}

impl Label {
        pub const NONE: Option<&'static Label> = None;
    

    #[doc(alias = "astal_label_new")]
    pub fn new() -> Label {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::astal_label_new())
        }
    }
}

impl Default for Label {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

pub trait LabelExt: IsA<Label> + 'static {
    #[doc(alias = "astal_label_get_truncate")]
    #[doc(alias = "get_truncate")]
    fn must_truncate(&self) -> bool {
        unsafe {
            from_glib(ffi::astal_label_get_truncate(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_label_set_truncate")]
    fn set_truncate(&self, value: bool) {
        unsafe {
            ffi::astal_label_set_truncate(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "astal_label_get_justify_fill")]
    #[doc(alias = "get_justify_fill")]
    fn is_justify_fill(&self) -> bool {
        unsafe {
            from_glib(ffi::astal_label_get_justify_fill(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_label_set_justify_fill")]
    fn set_justify_fill(&self, value: bool) {
        unsafe {
            ffi::astal_label_set_justify_fill(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "truncate")]
    fn connect_truncate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_truncate_trampoline<P: IsA<Label>, F: Fn(&P) + 'static>(this: *mut ffi::AstalLabel, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Label::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::truncate\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_truncate_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "justify-fill")]
    fn connect_justify_fill_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_justify_fill_trampoline<P: IsA<Label>, F: Fn(&P) + 'static>(this: *mut ffi::AstalLabel, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Label::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::justify-fill\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_justify_fill_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<Label>> LabelExt for O {}
