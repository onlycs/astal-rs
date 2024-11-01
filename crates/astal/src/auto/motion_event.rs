// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../../gir/gir-files
// DO NOT EDIT

use crate::{ffi};
use glib::{translate::*};

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MotionEvent(Boxed<ffi::AstalMotionEvent>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::astal_motion_event_get_type(), ptr as *mut _) as *mut ffi::AstalMotionEvent,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::astal_motion_event_get_type(), ptr as *mut _),
        type_ => || ffi::astal_motion_event_get_type(),
    }
}

impl MotionEvent {
    #[doc(alias = "astal_motion_event_init")]
    pub fn init(&mut self, event: &mut gdk::EventMotion) {
        unsafe {
            ffi::astal_motion_event_init(self.to_glib_none_mut().0, event.to_glib_none_mut().0);
        }
    }
}
