// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../../gir/gir-files
// DO NOT EDIT

use crate::{ffi};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "AstalBox")]
    pub struct Box(Object<ffi::AstalBox, ffi::AstalBoxClass>) @extends gtk::Box, gtk::Container, gtk::Widget, gobject::InitiallyUnowned, @implements atk::ImplementorIface, gtk::Buildable, gtk::Orientable;

    match fn {
        type_ => || ffi::astal_box_get_type(),
    }
}

impl Box {
        pub const NONE: Option<&'static Box> = None;
    

    #[doc(alias = "astal_box_new")]
    pub fn new(vertical: bool, children: &[gtk::Widget]) -> Box {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::astal_box_new(vertical.into_glib(), children.to_glib_none().0))
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`Box`] objects.
            ///
            /// This method returns an instance of [`BoxBuilder`](crate::builders::BoxBuilder) which can be used to create [`Box`] objects.
            pub fn builder() -> BoxBuilder {
                BoxBuilder::new()
            }
        
}

impl Default for Box {
                     fn default() -> Self {
                         glib::object::Object::new::<Self>()
                     }
                 }

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`Box`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct BoxBuilder {
            builder: glib::object::ObjectBuilder<'static, Box>,
        }

        impl BoxBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            pub fn vertical(self, vertical: bool) -> Self {
                            
                            Self { builder: self.builder.property("vertical", vertical), }
                        }

                            pub fn children(self, children: &[gtk::Widget]) -> Self {
                            
                    use gobject::{
                        gobject_ffi::{g_value_init, g_value_set_pointer, G_TYPE_POINTER},
                        Type, Value
                    };
                    use std::boxed::Box as Box_;
                    
                    let boxed = Box_::<[gtk::Widget]>::from(children);
                    let leak = Box_::leak(boxed) as *mut _;
                
                    let children = Value::from_type(Type::POINTER);
                    let raw = children.as_ptr();

                    unsafe {
                        g_value_init(raw, G_TYPE_POINTER);
                        g_value_set_pointer(raw, leak as *mut _);
                    }
                    
                            Self { builder: self.builder.property("children", children), }
                        }

                            pub fn child(self, child: &impl IsA<gtk::Widget>) -> Self {
                            
                            Self { builder: self.builder.property("child", child.clone().upcast()), }
                        }

                            pub fn baseline_position(self, baseline_position: gtk::BaselinePosition) -> Self {
                            
                            Self { builder: self.builder.property("baseline-position", baseline_position), }
                        }

                            pub fn homogeneous(self, homogeneous: bool) -> Self {
                            
                            Self { builder: self.builder.property("homogeneous", homogeneous), }
                        }

                            pub fn spacing(self, spacing: i32) -> Self {
                            
                            Self { builder: self.builder.property("spacing", spacing), }
                        }

                            pub fn border_width(self, border_width: u32) -> Self {
                            
                            Self { builder: self.builder.property("border-width", border_width), }
                        }

                            pub fn resize_mode(self, resize_mode: gtk::ResizeMode) -> Self {
                            
                            Self { builder: self.builder.property("resize-mode", resize_mode), }
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
    /// Build the [`Box`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Box {
assert_initialized_main_thread!();
    self.builder.build() }
}

pub trait BoxExt: IsA<Box> + 'static {
    #[doc(alias = "astal_box_get_vertical")]
    #[doc(alias = "get_vertical")]
    fn is_vertical(&self) -> bool {
        unsafe {
            from_glib(ffi::astal_box_get_vertical(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_box_set_vertical")]
    fn set_vertical(&self, value: bool) {
        unsafe {
            ffi::astal_box_set_vertical(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "astal_box_get_children")]
    #[doc(alias = "get_children")]
    fn children(&self) -> Vec<gtk::Widget> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::astal_box_get_children(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_box_set_children")]
    fn set_children(&self, value: &[gtk::Widget]) {
        unsafe {
            ffi::astal_box_set_children(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "astal_box_get_child")]
    #[doc(alias = "get_child")]
    fn child(&self) -> Option<gtk::Widget> {
        unsafe {
            from_glib_full(ffi::astal_box_get_child(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_box_set_child")]
    fn set_child(&self, value: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::astal_box_set_child(self.as_ref().to_glib_none().0, value.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "vertical")]
    fn connect_vertical_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_vertical_trampoline<P: IsA<Box>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBox, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Box::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::vertical\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_vertical_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "children")]
    fn connect_children_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_children_trampoline<P: IsA<Box>, F: Fn(&P) + 'static>(this: *mut ffi::AstalBox, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Box::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::children\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_children_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<Box>> BoxExt for O {}
