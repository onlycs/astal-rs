// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../../gobject/gir-files
// DO NOT EDIT

use crate::{ffi};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "AstalSlider")]
    pub struct Slider(Object<ffi::AstalSlider, ffi::AstalSliderClass>) @extends gtk::Scale, gtk::Range, gtk::Widget, gobject::InitiallyUnowned, @implements atk::ImplementorIface, gtk::Buildable, gtk::Orientable;

    match fn {
        type_ => || ffi::astal_slider_get_type(),
    }
}

impl Slider {
        pub const NONE: Option<&'static Slider> = None;
    

    #[doc(alias = "astal_slider_new")]
    pub fn new() -> Slider {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::astal_slider_new())
        }
    }
}

impl Default for Slider {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

pub trait SliderExt: IsA<Slider> + 'static {
    #[doc(alias = "astal_slider_get_vertical")]
    #[doc(alias = "get_vertical")]
    fn is_vertical(&self) -> bool {
        unsafe {
            from_glib(ffi::astal_slider_get_vertical(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_slider_set_vertical")]
    fn set_vertical(&self, value: bool) {
        unsafe {
            ffi::astal_slider_set_vertical(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "astal_slider_get_dragging")]
    #[doc(alias = "get_dragging")]
    fn is_dragging(&self) -> bool {
        unsafe {
            from_glib(ffi::astal_slider_get_dragging(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_slider_get_value")]
    #[doc(alias = "get_value")]
    fn value(&self) -> f64 {
        unsafe {
            ffi::astal_slider_get_value(self.as_ref().to_glib_none().0)
        }
    }

    #[doc(alias = "astal_slider_set_value")]
    fn set_value(&self, value: f64) {
        unsafe {
            ffi::astal_slider_set_value(self.as_ref().to_glib_none().0, value);
        }
    }

    #[doc(alias = "astal_slider_get_min")]
    #[doc(alias = "get_min")]
    fn min(&self) -> f64 {
        unsafe {
            ffi::astal_slider_get_min(self.as_ref().to_glib_none().0)
        }
    }

    #[doc(alias = "astal_slider_set_min")]
    fn set_min(&self, value: f64) {
        unsafe {
            ffi::astal_slider_set_min(self.as_ref().to_glib_none().0, value);
        }
    }

    #[doc(alias = "astal_slider_get_max")]
    #[doc(alias = "get_max")]
    fn max(&self) -> f64 {
        unsafe {
            ffi::astal_slider_get_max(self.as_ref().to_glib_none().0)
        }
    }

    #[doc(alias = "astal_slider_set_max")]
    fn set_max(&self, value: f64) {
        unsafe {
            ffi::astal_slider_set_max(self.as_ref().to_glib_none().0, value);
        }
    }

    #[doc(alias = "astal_slider_get_step")]
    #[doc(alias = "get_step")]
    fn step(&self) -> f64 {
        unsafe {
            ffi::astal_slider_get_step(self.as_ref().to_glib_none().0)
        }
    }

    #[doc(alias = "astal_slider_set_step")]
    fn set_step(&self, value: f64) {
        unsafe {
            ffi::astal_slider_set_step(self.as_ref().to_glib_none().0, value);
        }
    }

    fn set_dragging(&self, dragging: bool) {
        ObjectExt::set_property(self.as_ref(),"dragging", dragging)
    }

    #[doc(alias = "dragged")]
    fn connect_dragged<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn dragged_trampoline<P: IsA<Slider>, F: Fn(&P) + 'static>(this: *mut ffi::AstalSlider, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Slider::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"dragged\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(dragged_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "vertical")]
    fn connect_vertical_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_vertical_trampoline<P: IsA<Slider>, F: Fn(&P) + 'static>(this: *mut ffi::AstalSlider, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Slider::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::vertical\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_vertical_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "dragging")]
    fn connect_dragging_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_dragging_trampoline<P: IsA<Slider>, F: Fn(&P) + 'static>(this: *mut ffi::AstalSlider, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Slider::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::dragging\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_dragging_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "value")]
    fn connect_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_trampoline<P: IsA<Slider>, F: Fn(&P) + 'static>(this: *mut ffi::AstalSlider, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Slider::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::value\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_value_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "min")]
    fn connect_min_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_trampoline<P: IsA<Slider>, F: Fn(&P) + 'static>(this: *mut ffi::AstalSlider, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Slider::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::min\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_min_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "max")]
    fn connect_max_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_trampoline<P: IsA<Slider>, F: Fn(&P) + 'static>(this: *mut ffi::AstalSlider, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Slider::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::max\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_max_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "step")]
    fn connect_step_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_step_trampoline<P: IsA<Slider>, F: Fn(&P) + 'static>(this: *mut ffi::AstalSlider, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Slider::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::step\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_step_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<Slider>> SliderExt for O {}
