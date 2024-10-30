// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../../gobject/gir-files
// DO NOT EDIT

use crate::{ffi,Application};
use glib::{prelude::*,translate::*};


#[doc(alias = "astal_io_acquire_socket")]
pub fn acquire_socket(app: &impl IsA<Application>) -> Result<(gio::SocketService, glib::GString), glib::Error> {
    skip_assert_initialized!();
    unsafe {
        let mut sock = std::ptr::null_mut();
        let mut error = std::ptr::null_mut();
        let ret = ffi::astal_io_acquire_socket(app.as_ref().to_glib_none().0, &mut sock, &mut error);
        if error.is_null() { Ok((from_glib_full(ret), from_glib_full(sock))) } else { Err(from_glib_full(error)) }
    }
}

#[doc(alias = "astal_io_get_instances")]
#[doc(alias = "get_instances")]
pub fn instances() -> Vec<glib::GString> {
    assert_initialized_main_thread!();
    unsafe {
        FromGlibPtrContainer::from_glib_full(ffi::astal_io_get_instances())
    }
}

#[doc(alias = "astal_io_quit_instance")]
pub fn quit_instance(instance: &str) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::astal_io_quit_instance(instance.to_glib_none().0);
    }
}

#[doc(alias = "astal_io_open_inspector")]
pub fn open_inspector(instance: &str) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::astal_io_open_inspector(instance.to_glib_none().0);
    }
}

#[doc(alias = "astal_io_toggle_window_by_name")]
pub fn toggle_window_by_name(instance: &str, window: &str) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::astal_io_toggle_window_by_name(instance.to_glib_none().0, window.to_glib_none().0);
    }
}

#[doc(alias = "astal_io_send_message")]
pub fn send_message(instance: &str, msg: &str) -> Option<glib::GString> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::astal_io_send_message(instance.to_glib_none().0, msg.to_glib_none().0))
    }
}

//#[doc(alias = "astal_io_read_sock")]
//pub fn read_sock(conn: &impl IsA<gio::SocketConnection>, _callback_: AsyncReadyCallback) {
//    unsafe { TODO: call ffi:astal_io_read_sock() }
//}

//#[doc(alias = "astal_io_write_sock")]
//pub fn write_sock(conn: &impl IsA<gio::SocketConnection>, response: &str, _callback_: AsyncReadyCallback) {
//    unsafe { TODO: call ffi:astal_io_write_sock() }
//}

#[doc(alias = "astal_io_read_file")]
pub fn read_file(path: &str) -> Option<glib::GString> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::astal_io_read_file(path.to_glib_none().0))
    }
}

//#[doc(alias = "astal_io_read_file_async")]
//pub fn read_file_async(path: &str, _callback_: AsyncReadyCallback) {
//    unsafe { TODO: call ffi:astal_io_read_file_async() }
//}

#[doc(alias = "astal_io_write_file")]
pub fn write_file(path: &str, content: &str) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::astal_io_write_file(path.to_glib_none().0, content.to_glib_none().0);
    }
}

//#[doc(alias = "astal_io_write_file_async")]
//pub fn write_file_async(path: &str, content: &str, _callback_: AsyncReadyCallback) {
//    unsafe { TODO: call ffi:astal_io_write_file_async() }
//}

#[doc(alias = "astal_io_monitor_file")]
pub fn monitor_file(path: &str, callback: &glib::Closure) -> Option<gio::FileMonitor> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::astal_io_monitor_file(path.to_glib_none().0, callback.to_glib_none().0))
    }
}