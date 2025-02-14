// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../../gir/gir-files
// DO NOT EDIT

use crate::{ffi};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_,pin::Pin};

glib::wrapper! {
    #[doc(alias = "AstalAuthPam")]
    pub struct Pam(Object<ffi::AstalAuthPam, ffi::AstalAuthPamClass>);

    match fn {
        type_ => || ffi::astal_auth_pam_get_type(),
    }
}

impl Pam {
            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`Pam`] objects.
            ///
            /// This method returns an instance of [`PamBuilder`](crate::builders::PamBuilder) which can be used to create [`Pam`] objects.
            pub fn builder() -> PamBuilder {
                PamBuilder::new()
            }
        

    #[doc(alias = "astal_auth_pam_get_service")]
    #[doc(alias = "get_service")]
    pub fn service(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::astal_auth_pam_get_service(self.to_glib_none().0))
        }
    }

    #[doc(alias = "astal_auth_pam_get_username")]
    #[doc(alias = "get_username")]
    pub fn username(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::astal_auth_pam_get_username(self.to_glib_none().0))
        }
    }

    #[doc(alias = "astal_auth_pam_set_service")]
    #[doc(alias = "service")]
    pub fn set_service(&self, service: &str) {
        unsafe {
            ffi::astal_auth_pam_set_service(self.to_glib_none().0, service.to_glib_none().0);
        }
    }

    #[doc(alias = "astal_auth_pam_set_username")]
    #[doc(alias = "username")]
    pub fn set_username(&self, username: &str) {
        unsafe {
            ffi::astal_auth_pam_set_username(self.to_glib_none().0, username.to_glib_none().0);
        }
    }

    #[doc(alias = "astal_auth_pam_start_authenticate")]
    pub fn start_authenticate(&self) -> bool {
        unsafe {
            from_glib(ffi::astal_auth_pam_start_authenticate(self.to_glib_none().0))
        }
    }

    #[doc(alias = "astal_auth_pam_supply_secret")]
    pub fn supply_secret(&self, secret: Option<&str>) {
        unsafe {
            ffi::astal_auth_pam_supply_secret(self.to_glib_none().0, secret.to_glib_none().0);
        }
    }

    #[doc(alias = "astal_auth_pam_authenticate")]
    pub fn authenticate<P: FnOnce(Result<isize, glib::Error>) + 'static>(password: &str, result_callback: P) -> bool {
        assert_initialized_main_thread!();
        
                let main_context = glib::MainContext::ref_thread_default();
                let is_main_context_owner = main_context.is_owner();
                let has_acquired_main_context = (!is_main_context_owner)
                    .then(|| main_context.acquire().ok())
                    .flatten();
                assert!(
                    is_main_context_owner || has_acquired_main_context.is_some(),
                    "Async operations only allowed if the thread is owning the MainContext"
                );
        
        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> = Box_::new(glib::thread_guard::ThreadGuard::new(result_callback));
        unsafe extern "C" fn authenticate_trampoline<P: FnOnce(Result<isize, glib::Error>) + 'static>(_source_object: *mut glib::gobject_ffi::GObject, res: *mut gio::ffi::GAsyncResult, user_data: glib::ffi::gpointer) {
            let mut error = std::ptr::null_mut();
            let ret = ffi::astal_auth_pam_authenticate_finish(res, &mut error);
            let result = if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> = Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = authenticate_trampoline::<P>;
        unsafe {
            from_glib(ffi::astal_auth_pam_authenticate(password.to_glib_none().0, Some(callback), Box_::into_raw(user_data) as *mut _))
        }
    }

    
    pub fn authenticate_future(password: &str) -> Pin<Box_<dyn std::future::Future<Output = Result<isize, glib::Error>> + 'static>> {

        skip_assert_initialized!();
        let password = String::from(password);
        Box_::pin(gio::GioFuture::new(&(), move |_obj, cancellable, send| {
            Self::authenticate(
                &password,
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    #[doc(alias = "auth-error")]
    pub fn connect_auth_error<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn auth_error_trampoline<F: Fn(&Pam, &str) + 'static>(this: *mut ffi::AstalAuthPam, msg: *mut std::ffi::c_char, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &glib::GString::from_glib_borrow(msg))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"auth-error\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(auth_error_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "auth-info")]
    pub fn connect_auth_info<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn auth_info_trampoline<F: Fn(&Pam, &str) + 'static>(this: *mut ffi::AstalAuthPam, msg: *mut std::ffi::c_char, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &glib::GString::from_glib_borrow(msg))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"auth-info\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(auth_info_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "auth-prompt-hidden")]
    pub fn connect_auth_prompt_hidden<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn auth_prompt_hidden_trampoline<F: Fn(&Pam, &str) + 'static>(this: *mut ffi::AstalAuthPam, msg: *mut std::ffi::c_char, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &glib::GString::from_glib_borrow(msg))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"auth-prompt-hidden\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(auth_prompt_hidden_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "auth-prompt-visible")]
    pub fn connect_auth_prompt_visible<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn auth_prompt_visible_trampoline<F: Fn(&Pam, &str) + 'static>(this: *mut ffi::AstalAuthPam, msg: *mut std::ffi::c_char, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &glib::GString::from_glib_borrow(msg))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"auth-prompt-visible\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(auth_prompt_visible_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "fail")]
    pub fn connect_fail<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn fail_trampoline<F: Fn(&Pam, &str) + 'static>(this: *mut ffi::AstalAuthPam, msg: *mut std::ffi::c_char, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &glib::GString::from_glib_borrow(msg))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"fail\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(fail_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "success")]
    pub fn connect_success<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn success_trampoline<F: Fn(&Pam) + 'static>(this: *mut ffi::AstalAuthPam, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"success\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(success_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "service")]
    pub fn connect_service_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_service_trampoline<F: Fn(&Pam) + 'static>(this: *mut ffi::AstalAuthPam, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::service\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_service_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "username")]
    pub fn connect_username_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_username_trampoline<F: Fn(&Pam) + 'static>(this: *mut ffi::AstalAuthPam, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::username\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_username_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`Pam`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct PamBuilder {
            builder: glib::object::ObjectBuilder<'static, Pam>,
        }

        impl PamBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            pub fn service(self, service: impl Into<glib::GString>) -> Self {
                            
                            Self { builder: self.builder.property("service", service.into()), }
                        }

                            pub fn username(self, username: impl Into<glib::GString>) -> Self {
                            
                            Self { builder: self.builder.property("username", username.into()), }
                        }

    // rustdoc-stripper-ignore-next
    /// Build the [`Pam`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Pam {
assert_initialized_main_thread!();
    self.builder.build() }
}
