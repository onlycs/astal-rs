// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../../gir/gir-files
// DO NOT EDIT

use crate::{ffi};

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct HoverEvent(Boxed<ffi::AstalHoverEvent>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::astal_hover_event_get_type(), ptr as *mut _) as *mut ffi::AstalHoverEvent,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::astal_hover_event_get_type(), ptr as *mut _),
        type_ => || ffi::astal_hover_event_get_type(),
    }
}

impl HoverEvent {
    //#[doc(alias = "astal_hover_event_init")]
//#[must_use]
    //pub fn init(event: /*Ignored*/&mut gdk::EventCrossing) -> HoverEvent {
    //    unsafe { TODO: call ffi:astal_hover_event_init() }
    //}
}
