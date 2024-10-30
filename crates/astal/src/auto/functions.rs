// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../../gir/gir-files
// DO NOT EDIT

use crate::{ffi};
use glib::{prelude::*,translate::*};


#[doc(alias = "astal_widget_set_css")]
pub fn widget_set_css(widget: &impl IsA<gtk::Widget>, css: &str) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::astal_widget_set_css(widget.as_ref().to_glib_none().0, css.to_glib_none().0);
    }
}

#[doc(alias = "astal_widget_get_css")]
pub fn widget_get_css(widget: &impl IsA<gtk::Widget>) -> Option<glib::GString> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::astal_widget_get_css(widget.as_ref().to_glib_none().0))
    }
}

#[doc(alias = "astal_widget_set_class_names")]
pub fn widget_set_class_names(widget: &impl IsA<gtk::Widget>, class_names: &[&str]) {
    assert_initialized_main_thread!();
    let class_names_length1 = class_names.len() as _;
    unsafe {
        ffi::astal_widget_set_class_names(widget.as_ref().to_glib_none().0, class_names.to_glib_none().0, class_names_length1);
    }
}

#[doc(alias = "astal_widget_get_class_names")]
pub fn widget_get_class_names(widget: &impl IsA<gtk::Widget>) -> Vec<glib::GString> {
    assert_initialized_main_thread!();
    unsafe {
        FromGlibPtrContainer::from_glib_container(ffi::astal_widget_get_class_names(widget.as_ref().to_glib_none().0))
    }
}

#[doc(alias = "astal_widget_toggle_class_name")]
pub fn widget_toggle_class_name(widget: &impl IsA<gtk::Widget>, class_name: &str, condition: bool) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::astal_widget_toggle_class_name(widget.as_ref().to_glib_none().0, class_name.to_glib_none().0, condition.into_glib());
    }
}

#[doc(alias = "astal_widget_set_cursor")]
pub fn widget_set_cursor(widget: &impl IsA<gtk::Widget>, cursor: &str) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::astal_widget_set_cursor(widget.as_ref().to_glib_none().0, cursor.to_glib_none().0);
    }
}

#[doc(alias = "astal_widget_get_cursor")]
pub fn widget_get_cursor(widget: &impl IsA<gtk::Widget>) -> Option<glib::GString> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::astal_widget_get_cursor(widget.as_ref().to_glib_none().0))
    }
}

#[doc(alias = "astal_widget_set_click_through")]
pub fn widget_set_click_through(widget: &impl IsA<gtk::Widget>, click_through: bool) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::astal_widget_set_click_through(widget.as_ref().to_glib_none().0, click_through.into_glib());
    }
}

#[doc(alias = "astal_widget_get_click_through")]
pub fn widget_get_click_through(widget: &impl IsA<gtk::Widget>) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::astal_widget_get_click_through(widget.as_ref().to_glib_none().0))
    }
}
