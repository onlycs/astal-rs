// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../../gir/gir-files
// DO NOT EDIT

use crate::{ffi,ClosedReason,Notification};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "AstalNotifdNotifd")]
    pub struct Notifd(Object<ffi::AstalNotifdNotifd, ffi::AstalNotifdNotifdClass>);

    match fn {
        type_ => || ffi::astal_notifd_notifd_get_type(),
    }
}

impl Notifd {
        pub const NONE: Option<&'static Notifd> = None;
    

    #[doc(alias = "astal_notifd_notifd_new")]
    pub fn new() -> Notifd {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::astal_notifd_notifd_new())
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`Notifd`] objects.
            ///
            /// This method returns an instance of [`NotifdBuilder`](crate::builders::NotifdBuilder) which can be used to create [`Notifd`] objects.
            pub fn builder() -> NotifdBuilder {
                NotifdBuilder::new()
            }
        

    #[doc(alias = "astal_notifd_notifd_get_default")]
    #[doc(alias = "get_default")]
    #[allow(clippy::should_implement_trait)]    pub fn default() -> Option<Notifd> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::astal_notifd_notifd_get_default())
        }
    }
}

impl Default for Notifd {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`Notifd`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct NotifdBuilder {
            builder: glib::object::ObjectBuilder<'static, Notifd>,
        }

        impl NotifdBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            pub fn ignore_timeout(self, ignore_timeout: bool) -> Self {
                            
                            Self { builder: self.builder.property("ignore-timeout", ignore_timeout), }
                        }

                            pub fn dont_disturb(self, dont_disturb: bool) -> Self {
                            
                            Self { builder: self.builder.property("dont-disturb", dont_disturb), }
                        }

    // rustdoc-stripper-ignore-next
    /// Build the [`Notifd`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Notifd {
assert_initialized_main_thread!();
    self.builder.build() }
}

pub trait NotifdExt: IsA<Notifd> + 'static {
    #[doc(alias = "astal_notifd_notifd_get_notification")]
    #[doc(alias = "get_notification")]
    fn notification(&self, id: u32) -> Option<Notification> {
        unsafe {
            from_glib_full(ffi::astal_notifd_notifd_get_notification(self.as_ref().to_glib_none().0, id))
        }
    }

    #[doc(alias = "astal_notifd_notifd_get_ignore_timeout")]
    #[doc(alias = "get_ignore_timeout")]
    fn ignores_timeout(&self) -> bool {
        unsafe {
            from_glib(ffi::astal_notifd_notifd_get_ignore_timeout(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_notifd_notifd_set_ignore_timeout")]
    fn set_ignore_timeout(&self, value: bool) {
        unsafe {
            ffi::astal_notifd_notifd_set_ignore_timeout(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "astal_notifd_notifd_get_dont_disturb")]
    #[doc(alias = "get_dont_disturb")]
    fn is_dont_disturb(&self) -> bool {
        unsafe {
            from_glib(ffi::astal_notifd_notifd_get_dont_disturb(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_notifd_notifd_set_dont_disturb")]
    fn set_dont_disturb(&self, value: bool) {
        unsafe {
            ffi::astal_notifd_notifd_set_dont_disturb(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "astal_notifd_notifd_get_notifications")]
    #[doc(alias = "get_notifications")]
    fn notifications(&self) -> Vec<Notification> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::astal_notifd_notifd_get_notifications(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "notified")]
    fn connect_notified<F: Fn(&Self, u32, bool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notified_trampoline<P: IsA<Notifd>, F: Fn(&P, u32, bool) + 'static>(this: *mut ffi::AstalNotifdNotifd, id: std::ffi::c_uint, replaced: glib::ffi::gboolean, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Notifd::from_glib_borrow(this).unsafe_cast_ref(), id, from_glib(replaced))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notified\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notified_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "resolved")]
    fn connect_resolved<F: Fn(&Self, u32, ClosedReason) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn resolved_trampoline<P: IsA<Notifd>, F: Fn(&P, u32, ClosedReason) + 'static>(this: *mut ffi::AstalNotifdNotifd, id: std::ffi::c_uint, reason: ffi::AstalNotifdClosedReason, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Notifd::from_glib_borrow(this).unsafe_cast_ref(), id, from_glib(reason))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"resolved\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(resolved_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "ignore-timeout")]
    fn connect_ignore_timeout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ignore_timeout_trampoline<P: IsA<Notifd>, F: Fn(&P) + 'static>(this: *mut ffi::AstalNotifdNotifd, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Notifd::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::ignore-timeout\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_ignore_timeout_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "dont-disturb")]
    fn connect_dont_disturb_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_dont_disturb_trampoline<P: IsA<Notifd>, F: Fn(&P) + 'static>(this: *mut ffi::AstalNotifdNotifd, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Notifd::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::dont-disturb\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_dont_disturb_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "notifications")]
    fn connect_notifications_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_notifications_trampoline<P: IsA<Notifd>, F: Fn(&P) + 'static>(this: *mut ffi::AstalNotifdNotifd, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Notifd::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::notifications\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_notifications_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<Notifd>> NotifdExt for O {}