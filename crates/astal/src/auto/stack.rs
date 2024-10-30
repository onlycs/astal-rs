// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../../gir/gir-files
// DO NOT EDIT

use crate::{ffi};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "AstalStack")]
    pub struct Stack(Object<ffi::AstalStack, ffi::AstalStackClass>) @extends gtk::Stack, gtk::Container, gtk::Widget, gobject::InitiallyUnowned, @implements atk::ImplementorIface, gtk::Buildable;

    match fn {
        type_ => || ffi::astal_stack_get_type(),
    }
}

impl Stack {
        pub const NONE: Option<&'static Stack> = None;
    

    #[doc(alias = "astal_stack_new")]
    pub fn new() -> Stack {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::astal_stack_new())
        }
    }
}

impl Default for Stack {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

pub trait StackExt: IsA<Stack> + 'static {
    #[doc(alias = "astal_stack_get_shown")]
    #[doc(alias = "get_shown")]
    fn shown(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::astal_stack_get_shown(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_stack_set_shown")]
    fn set_shown(&self, value: &str) {
        unsafe {
            ffi::astal_stack_set_shown(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "astal_stack_get_children")]
    #[doc(alias = "get_children")]
    fn children(&self) -> Vec<gtk::Widget> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::astal_stack_get_children(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_stack_set_children")]
    fn set_children(&self, value: &[gtk::Widget]) {
        unsafe {
            ffi::astal_stack_set_children(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "shown")]
    fn connect_shown_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_shown_trampoline<P: IsA<Stack>, F: Fn(&P) + 'static>(this: *mut ffi::AstalStack, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Stack::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::shown\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_shown_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "children")]
    fn connect_children_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_children_trampoline<P: IsA<Stack>, F: Fn(&P) + 'static>(this: *mut ffi::AstalStack, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Stack::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::children\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_children_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<Stack>> StackExt for O {}
