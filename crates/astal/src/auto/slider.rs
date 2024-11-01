// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../../gir/gir-files
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

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`Slider`] objects.
            ///
            /// This method returns an instance of [`SliderBuilder`](crate::builders::SliderBuilder) which can be used to create [`Slider`] objects.
            pub fn builder() -> SliderBuilder {
                SliderBuilder::new()
            }
        
}

impl Default for Slider {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`Slider`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct SliderBuilder {
            builder: glib::object::ObjectBuilder<'static, Slider>,
        }

        impl SliderBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            pub fn vertical(self, vertical: bool) -> Self {
                            
                            Self { builder: self.builder.property("vertical", vertical), }
                        }

                            pub fn dragging(self, dragging: bool) -> Self {
                            
                            Self { builder: self.builder.property("dragging", dragging), }
                        }

                            pub fn value(self, value: f64) -> Self {
                            
                            Self { builder: self.builder.property("value", value), }
                        }

                            pub fn min(self, min: f64) -> Self {
                            
                            Self { builder: self.builder.property("min", min), }
                        }

                            pub fn max(self, max: f64) -> Self {
                            
                            Self { builder: self.builder.property("max", max), }
                        }

                            pub fn step(self, step: f64) -> Self {
                            
                            Self { builder: self.builder.property("step", step), }
                        }

                            pub fn digits(self, digits: i32) -> Self {
                            
                            Self { builder: self.builder.property("digits", digits), }
                        }

                            pub fn draw_value(self, draw_value: bool) -> Self {
                            
                            Self { builder: self.builder.property("draw-value", draw_value), }
                        }

                            pub fn has_origin(self, has_origin: bool) -> Self {
                            
                            Self { builder: self.builder.property("has-origin", has_origin), }
                        }

                            pub fn value_pos(self, value_pos: gtk::PositionType) -> Self {
                            
                            Self { builder: self.builder.property("value-pos", value_pos), }
                        }

                            pub fn adjustment(self, adjustment: &impl IsA<gtk::Adjustment>) -> Self {
                            
                            Self { builder: self.builder.property("adjustment", adjustment.clone().upcast()), }
                        }

                            #[cfg(feature = "gtk_v2_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v2_12")))]
    pub fn fill_level(self, fill_level: f64) -> Self {
                            
                            Self { builder: self.builder.property("fill-level", fill_level), }
                        }

                            pub fn inverted(self, inverted: bool) -> Self {
                            
                            Self { builder: self.builder.property("inverted", inverted), }
                        }

                            pub fn lower_stepper_sensitivity(self, lower_stepper_sensitivity: gtk::SensitivityType) -> Self {
                            
                            Self { builder: self.builder.property("lower-stepper-sensitivity", lower_stepper_sensitivity), }
                        }

                            #[cfg(feature = "gtk_v2_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v2_12")))]
    pub fn restrict_to_fill_level(self, restrict_to_fill_level: bool) -> Self {
                            
                            Self { builder: self.builder.property("restrict-to-fill-level", restrict_to_fill_level), }
                        }

                            #[cfg(feature = "gtk_v2_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v2_24")))]
    pub fn round_digits(self, round_digits: i32) -> Self {
                            
                            Self { builder: self.builder.property("round-digits", round_digits), }
                        }

                            #[cfg(feature = "gtk_v2_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v2_12")))]
    pub fn show_fill_level(self, show_fill_level: bool) -> Self {
                            
                            Self { builder: self.builder.property("show-fill-level", show_fill_level), }
                        }

                            pub fn upper_stepper_sensitivity(self, upper_stepper_sensitivity: gtk::SensitivityType) -> Self {
                            
                            Self { builder: self.builder.property("upper-stepper-sensitivity", upper_stepper_sensitivity), }
                        }

                            pub fn app_paintable(self, app_paintable: bool) -> Self {
                            
                            Self { builder: self.builder.property("app-paintable", app_paintable), }
                        }

                            pub fn can_default(self, can_default: bool) -> Self {
                            
                            Self { builder: self.builder.property("can-default", can_default), }
                        }

                            pub fn can_focus(self, can_focus: bool) -> Self {
                            
                            Self { builder: self.builder.property("can-focus", can_focus), }
                        }

                            #[cfg(feature = "gtk_v2_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v2_18")))]
    #[cfg_attr(feature = "v3_14", deprecated = "Since 3.14")]
    pub fn double_buffered(self, double_buffered: bool) -> Self {
                            
                            Self { builder: self.builder.property("double-buffered", double_buffered), }
                        }

                            pub fn events(self, events: gdk::EventMask) -> Self {
                            
                            Self { builder: self.builder.property("events", events), }
                        }

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    pub fn expand(self, expand: bool) -> Self {
                            
                            Self { builder: self.builder.property("expand", expand), }
                        }

                            #[cfg(feature = "gtk_v3_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3_20")))]
    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
                            
                            Self { builder: self.builder.property("focus-on-click", focus_on_click), }
                        }

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    pub fn halign(self, halign: gtk::Align) -> Self {
                            
                            Self { builder: self.builder.property("halign", halign), }
                        }

                            pub fn has_default(self, has_default: bool) -> Self {
                            
                            Self { builder: self.builder.property("has-default", has_default), }
                        }

                            pub fn has_focus(self, has_focus: bool) -> Self {
                            
                            Self { builder: self.builder.property("has-focus", has_focus), }
                        }

                            #[cfg(feature = "gtk_v2_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v2_12")))]
    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
                            
                            Self { builder: self.builder.property("has-tooltip", has_tooltip), }
                        }

                            pub fn height_request(self, height_request: i32) -> Self {
                            
                            Self { builder: self.builder.property("height-request", height_request), }
                        }

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    pub fn hexpand(self, hexpand: bool) -> Self {
                            
                            Self { builder: self.builder.property("hexpand", hexpand), }
                        }

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
                            
                            Self { builder: self.builder.property("hexpand-set", hexpand_set), }
                        }

                            pub fn is_focus(self, is_focus: bool) -> Self {
                            
                            Self { builder: self.builder.property("is-focus", is_focus), }
                        }

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    pub fn margin(self, margin: i32) -> Self {
                            
                            Self { builder: self.builder.property("margin", margin), }
                        }

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
                            
                            Self { builder: self.builder.property("margin-bottom", margin_bottom), }
                        }

                            #[cfg(feature = "gtk_v3_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3_12")))]
    pub fn margin_end(self, margin_end: i32) -> Self {
                            
                            Self { builder: self.builder.property("margin-end", margin_end), }
                        }

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    #[cfg_attr(feature = "v3_12", deprecated = "Since 3.12")]
    pub fn margin_left(self, margin_left: i32) -> Self {
                            
                            Self { builder: self.builder.property("margin-left", margin_left), }
                        }

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    #[cfg_attr(feature = "v3_12", deprecated = "Since 3.12")]
    pub fn margin_right(self, margin_right: i32) -> Self {
                            
                            Self { builder: self.builder.property("margin-right", margin_right), }
                        }

                            #[cfg(feature = "gtk_v3_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3_12")))]
    pub fn margin_start(self, margin_start: i32) -> Self {
                            
                            Self { builder: self.builder.property("margin-start", margin_start), }
                        }

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    pub fn margin_top(self, margin_top: i32) -> Self {
                            
                            Self { builder: self.builder.property("margin-top", margin_top), }
                        }

                            pub fn name(self, name: impl Into<glib::GString>) -> Self {
                            
                            Self { builder: self.builder.property("name", name.into()), }
                        }

                            pub fn no_show_all(self, no_show_all: bool) -> Self {
                            
                            Self { builder: self.builder.property("no-show-all", no_show_all), }
                        }

                            #[cfg(feature = "gtk_v3_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3_8")))]
    pub fn opacity(self, opacity: f64) -> Self {
                            
                            Self { builder: self.builder.property("opacity", opacity), }
                        }

                            pub fn parent(self, parent: &impl IsA<gtk::Container>) -> Self {
                            
                            Self { builder: self.builder.property("parent", parent.clone().upcast()), }
                        }

                            pub fn receives_default(self, receives_default: bool) -> Self {
                            
                            Self { builder: self.builder.property("receives-default", receives_default), }
                        }

                            pub fn sensitive(self, sensitive: bool) -> Self {
                            
                            Self { builder: self.builder.property("sensitive", sensitive), }
                        }

                            pub fn style(self, style: &impl IsA<gtk::Style>) -> Self {
                            
                            Self { builder: self.builder.property("style", style.clone().upcast()), }
                        }

                            #[cfg(feature = "gtk_v2_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v2_12")))]
    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
                            
                            Self { builder: self.builder.property("tooltip-markup", tooltip_markup.into()), }
                        }

                            #[cfg(feature = "gtk_v2_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v2_12")))]
    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
                            
                            Self { builder: self.builder.property("tooltip-text", tooltip_text.into()), }
                        }

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    pub fn valign(self, valign: gtk::Align) -> Self {
                            
                            Self { builder: self.builder.property("valign", valign), }
                        }

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    pub fn vexpand(self, vexpand: bool) -> Self {
                            
                            Self { builder: self.builder.property("vexpand", vexpand), }
                        }

                            #[cfg(feature = "gtk_v3")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v3")))]
    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
                            
                            Self { builder: self.builder.property("vexpand-set", vexpand_set), }
                        }

                            pub fn visible(self, visible: bool) -> Self {
                            
                            Self { builder: self.builder.property("visible", visible), }
                        }

                            pub fn width_request(self, width_request: i32) -> Self {
                            
                            Self { builder: self.builder.property("width-request", width_request), }
                        }

                            #[cfg(feature = "gtk_v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v2_16")))]
    pub fn orientation(self, orientation: gtk::Orientation) -> Self {
                            
                            Self { builder: self.builder.property("orientation", orientation), }
                        }

    // rustdoc-stripper-ignore-next
    /// Build the [`Slider`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Slider {
assert_initialized_main_thread!();
    self.builder.build() }
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
