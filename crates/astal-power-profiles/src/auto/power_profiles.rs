// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../../gir/gir-files
// DO NOT EDIT

use crate::{ffi,Hold,Profile};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "AstalPowerProfilesPowerProfiles")]
    pub struct PowerProfiles(Object<ffi::AstalPowerProfilesPowerProfiles, ffi::AstalPowerProfilesPowerProfilesClass>);

    match fn {
        type_ => || ffi::astal_power_profiles_power_profiles_get_type(),
    }
}

impl PowerProfiles {
        pub const NONE: Option<&'static PowerProfiles> = None;
    

    #[doc(alias = "astal_power_profiles_power_profiles_new")]
    pub fn new() -> PowerProfiles {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::astal_power_profiles_power_profiles_new())
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`PowerProfiles`] objects.
            ///
            /// This method returns an instance of [`PowerProfilesBuilder`](crate::builders::PowerProfilesBuilder) which can be used to create [`PowerProfiles`] objects.
            pub fn builder() -> PowerProfilesBuilder {
                PowerProfilesBuilder::new()
            }
        

    #[doc(alias = "astal_power_profiles_power_profiles_get_default")]
    #[doc(alias = "get_default")]
    #[allow(clippy::should_implement_trait)]    pub fn default() -> Option<PowerProfiles> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::astal_power_profiles_power_profiles_get_default())
        }
    }
}

impl Default for PowerProfiles {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`PowerProfiles`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct PowerProfilesBuilder {
            builder: glib::object::ObjectBuilder<'static, PowerProfiles>,
        }

        impl PowerProfilesBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            pub fn active_profile(self, active_profile: impl Into<glib::GString>) -> Self {
                            
                            Self { builder: self.builder.property("active-profile", active_profile.into()), }
                        }

    // rustdoc-stripper-ignore-next
    /// Build the [`PowerProfiles`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> PowerProfiles {
assert_initialized_main_thread!();
    self.builder.build() }
}

pub trait PowerProfilesExt: IsA<PowerProfiles> + 'static {
    #[doc(alias = "astal_power_profiles_power_profiles_hold_profile")]
    fn hold_profile(&self, profile: &str, reason: &str, application_id: &str) -> i32 {
        unsafe {
            ffi::astal_power_profiles_power_profiles_hold_profile(self.as_ref().to_glib_none().0, profile.to_glib_none().0, reason.to_glib_none().0, application_id.to_glib_none().0)
        }
    }

    #[doc(alias = "astal_power_profiles_power_profiles_release_profile")]
    fn release_profile(&self, cookie: u32) {
        unsafe {
            ffi::astal_power_profiles_power_profiles_release_profile(self.as_ref().to_glib_none().0, cookie);
        }
    }

    #[doc(alias = "astal_power_profiles_power_profiles_get_active_profile")]
    #[doc(alias = "get_active_profile")]
    fn active_profile(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::astal_power_profiles_power_profiles_get_active_profile(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_power_profiles_power_profiles_set_active_profile")]
    fn set_active_profile(&self, value: &str) {
        unsafe {
            ffi::astal_power_profiles_power_profiles_set_active_profile(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "astal_power_profiles_power_profiles_get_icon_name")]
    #[doc(alias = "get_icon_name")]
    fn icon_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::astal_power_profiles_power_profiles_get_icon_name(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_power_profiles_power_profiles_get_actions")]
    #[doc(alias = "get_actions")]
    fn actions(&self) -> Vec<glib::GString> {
        unsafe {
            let mut result_length1 = std::mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_full_num(ffi::astal_power_profiles_power_profiles_get_actions(self.as_ref().to_glib_none().0, result_length1.as_mut_ptr()), result_length1.assume_init() as _);
            ret
        }
    }

    #[doc(alias = "astal_power_profiles_power_profiles_get_active_profile_holds")]
    #[doc(alias = "get_active_profile_holds")]
    fn active_profile_holds(&self) -> Vec<Hold> {
        unsafe {
            let mut result_length1 = std::mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_full_num(&mut ffi::astal_power_profiles_power_profiles_get_active_profile_holds(self.as_ref().to_glib_none().0, result_length1.as_mut_ptr()) as *mut _, result_length1.assume_init() as _);
            ret
        }
    }

    #[doc(alias = "astal_power_profiles_power_profiles_get_performance_degraded")]
    #[doc(alias = "get_performance_degraded")]
    fn performance_degraded(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::astal_power_profiles_power_profiles_get_performance_degraded(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_power_profiles_power_profiles_get_profiles")]
    #[doc(alias = "get_profiles")]
    fn profiles(&self) -> Vec<Profile> {
        unsafe {
            let mut result_length1 = std::mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_full_num(&mut ffi::astal_power_profiles_power_profiles_get_profiles(self.as_ref().to_glib_none().0, result_length1.as_mut_ptr()) as *mut _, result_length1.assume_init() as _);
            ret
        }
    }

    #[doc(alias = "astal_power_profiles_power_profiles_get_version")]
    #[doc(alias = "get_version")]
    fn version(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::astal_power_profiles_power_profiles_get_version(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "profile-released")]
    fn connect_profile_released<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn profile_released_trampoline<P: IsA<PowerProfiles>, F: Fn(&P, u32) + 'static>(this: *mut ffi::AstalPowerProfilesPowerProfiles, cookie: std::ffi::c_uint, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(PowerProfiles::from_glib_borrow(this).unsafe_cast_ref(), cookie)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"profile-released\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(profile_released_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "active-profile")]
    fn connect_active_profile_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_profile_trampoline<P: IsA<PowerProfiles>, F: Fn(&P) + 'static>(this: *mut ffi::AstalPowerProfilesPowerProfiles, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(PowerProfiles::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::active-profile\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_active_profile_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "icon-name")]
    fn connect_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_name_trampoline<P: IsA<PowerProfiles>, F: Fn(&P) + 'static>(this: *mut ffi::AstalPowerProfilesPowerProfiles, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(PowerProfiles::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::icon-name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_icon_name_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "actions")]
    fn connect_actions_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_actions_trampoline<P: IsA<PowerProfiles>, F: Fn(&P) + 'static>(this: *mut ffi::AstalPowerProfilesPowerProfiles, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(PowerProfiles::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::actions\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_actions_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "performance-degraded")]
    fn connect_performance_degraded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_performance_degraded_trampoline<P: IsA<PowerProfiles>, F: Fn(&P) + 'static>(this: *mut ffi::AstalPowerProfilesPowerProfiles, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(PowerProfiles::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::performance-degraded\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_performance_degraded_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "version")]
    fn connect_version_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_version_trampoline<P: IsA<PowerProfiles>, F: Fn(&P) + 'static>(this: *mut ffi::AstalPowerProfilesPowerProfiles, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(PowerProfiles::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::version\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_version_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<PowerProfiles>> PowerProfilesExt for O {}
