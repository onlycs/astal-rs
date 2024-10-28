// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../../gobject/gir-files
// DO NOT EDIT

use crate::{ffi,ClickEvent,HoverEvent,MotionEvent,ScrollEvent};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "AstalEventBox")]
    pub struct EventBox(Object<ffi::AstalEventBox, ffi::AstalEventBoxClass>) @extends gtk::EventBox, gtk::Bin, gtk::Container, gtk::Widget, gobject::InitiallyUnowned, @implements atk::ImplementorIface, gtk::Buildable;

    match fn {
        type_ => || ffi::astal_event_box_get_type(),
    }
}

impl EventBox {
        pub const NONE: Option<&'static EventBox> = None;
    

    #[doc(alias = "astal_event_box_new")]
    pub fn new() -> EventBox {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::astal_event_box_new())
        }
    }
}

impl Default for EventBox {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

pub trait EventBoxExt: IsA<EventBox> + 'static {
    #[doc(alias = "hover")]
    fn connect_hover<F: Fn(&Self, &HoverEvent) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn hover_trampoline<P: IsA<EventBox>, F: Fn(&P, &HoverEvent) + 'static>(this: *mut ffi::AstalEventBox, event: *mut ffi::AstalHoverEvent, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(EventBox::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(event))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"hover\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(hover_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "hover-lost")]
    fn connect_hover_lost<F: Fn(&Self, &HoverEvent) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn hover_lost_trampoline<P: IsA<EventBox>, F: Fn(&P, &HoverEvent) + 'static>(this: *mut ffi::AstalEventBox, event: *mut ffi::AstalHoverEvent, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(EventBox::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(event))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"hover-lost\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(hover_lost_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "click")]
    fn connect_click<F: Fn(&Self, &ClickEvent) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn click_trampoline<P: IsA<EventBox>, F: Fn(&P, &ClickEvent) + 'static>(this: *mut ffi::AstalEventBox, event: *mut ffi::AstalClickEvent, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(EventBox::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(event))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"click\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(click_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "click-release")]
    fn connect_click_release<F: Fn(&Self, &ClickEvent) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn click_release_trampoline<P: IsA<EventBox>, F: Fn(&P, &ClickEvent) + 'static>(this: *mut ffi::AstalEventBox, event: *mut ffi::AstalClickEvent, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(EventBox::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(event))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"click-release\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(click_release_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "scroll")]
    fn connect_scroll<F: Fn(&Self, &ScrollEvent) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn scroll_trampoline<P: IsA<EventBox>, F: Fn(&P, &ScrollEvent) + 'static>(this: *mut ffi::AstalEventBox, event: *mut ffi::AstalScrollEvent, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(EventBox::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(event))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"scroll\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(scroll_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "motion")]
    fn connect_motion<F: Fn(&Self, &MotionEvent) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn motion_trampoline<P: IsA<EventBox>, F: Fn(&P, &MotionEvent) + 'static>(this: *mut ffi::AstalEventBox, event: *mut ffi::AstalMotionEvent, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(EventBox::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(event))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"motion\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(motion_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<EventBox>> EventBoxExt for O {}
