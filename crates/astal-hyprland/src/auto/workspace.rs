// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../../gir/gir-files
// DO NOT EDIT

use crate::{ffi,Client,Monitor};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "AstalHyprlandWorkspace")]
    pub struct Workspace(Object<ffi::AstalHyprlandWorkspace, ffi::AstalHyprlandWorkspaceClass>);

    match fn {
        type_ => || ffi::astal_hyprland_workspace_get_type(),
    }
}

impl Workspace {
        pub const NONE: Option<&'static Workspace> = None;
    

    #[doc(alias = "astal_hyprland_workspace_new_dummy")]
    pub fn dummy(id: i32, monitor: Option<&impl IsA<Monitor>>) -> Workspace {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::astal_hyprland_workspace_new_dummy(id, monitor.map(|p| p.as_ref()).to_glib_none().0))
        }
    }

    #[doc(alias = "astal_hyprland_workspace_new")]
    pub fn new() -> Workspace {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::astal_hyprland_workspace_new())
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`Workspace`] objects.
            ///
            /// This method returns an instance of [`WorkspaceBuilder`](crate::builders::WorkspaceBuilder) which can be used to create [`Workspace`] objects.
            pub fn builder() -> WorkspaceBuilder {
                WorkspaceBuilder::new()
            }
        
}

impl Default for Workspace {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`Workspace`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct WorkspaceBuilder {
            builder: glib::object::ObjectBuilder<'static, Workspace>,
        }

        impl WorkspaceBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            pub fn id(self, id: i32) -> Self {
                            
                            Self { builder: self.builder.property("id", id), }
                        }

                            pub fn name(self, name: impl Into<glib::GString>) -> Self {
                            
                            Self { builder: self.builder.property("name", name.into()), }
                        }

                            pub fn monitor(self, monitor: &impl IsA<Monitor>) -> Self {
                            
                            Self { builder: self.builder.property("monitor", monitor.clone().upcast()), }
                        }

                            pub fn has_fullscreen(self, has_fullscreen: bool) -> Self {
                            
                            Self { builder: self.builder.property("has-fullscreen", has_fullscreen), }
                        }

                            pub fn last_client(self, last_client: &impl IsA<Client>) -> Self {
                            
                            Self { builder: self.builder.property("last-client", last_client.clone().upcast()), }
                        }

    // rustdoc-stripper-ignore-next
    /// Build the [`Workspace`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Workspace {
assert_initialized_main_thread!();
    self.builder.build() }
}

pub trait WorkspaceExt: IsA<Workspace> + 'static {
    #[doc(alias = "astal_hyprland_workspace_focus")]
    fn focus(&self) {
        unsafe {
            ffi::astal_hyprland_workspace_focus(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "astal_hyprland_workspace_move_to")]
    fn move_to(&self, m: &impl IsA<Monitor>) {
        unsafe {
            ffi::astal_hyprland_workspace_move_to(self.as_ref().to_glib_none().0, m.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "astal_hyprland_workspace_get_id")]
    #[doc(alias = "get_id")]
    fn id(&self) -> i32 {
        unsafe {
            ffi::astal_hyprland_workspace_get_id(self.as_ref().to_glib_none().0)
        }
    }

    #[doc(alias = "astal_hyprland_workspace_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::astal_hyprland_workspace_get_name(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_hyprland_workspace_get_monitor")]
    #[doc(alias = "get_monitor")]
    fn monitor(&self) -> Option<Monitor> {
        unsafe {
            from_glib_none(ffi::astal_hyprland_workspace_get_monitor(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_hyprland_workspace_get_clients")]
    #[doc(alias = "get_clients")]
    fn clients(&self) -> Vec<Client> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::astal_hyprland_workspace_get_clients(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_hyprland_workspace_get_has_fullscreen")]
    #[doc(alias = "get_has_fullscreen")]
    fn has_fullscreen(&self) -> bool {
        unsafe {
            from_glib(ffi::astal_hyprland_workspace_get_has_fullscreen(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "astal_hyprland_workspace_get_last_client")]
    #[doc(alias = "get_last_client")]
    fn last_client(&self) -> Option<Client> {
        unsafe {
            from_glib_none(ffi::astal_hyprland_workspace_get_last_client(self.as_ref().to_glib_none().0))
        }
    }

    fn set_id(&self, id: i32) {
        ObjectExt::set_property(self.as_ref(),"id", id)
    }

    fn set_name(&self, name: Option<&str>) {
        ObjectExt::set_property(self.as_ref(),"name", name)
    }

    fn set_monitor<P: IsA<Monitor>>(&self, monitor: Option<&P>) {
        ObjectExt::set_property(self.as_ref(),"monitor", monitor)
    }

    #[doc(alias = "has-fullscreen")]
    fn set_has_fullscreen(&self, has_fullscreen: bool) {
        ObjectExt::set_property(self.as_ref(),"has-fullscreen", has_fullscreen)
    }

    #[doc(alias = "last-client")]
    fn set_last_client<P: IsA<Client>>(&self, last_client: Option<&P>) {
        ObjectExt::set_property(self.as_ref(),"last-client", last_client)
    }

    #[doc(alias = "removed")]
    fn connect_removed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn removed_trampoline<P: IsA<Workspace>, F: Fn(&P) + 'static>(this: *mut ffi::AstalHyprlandWorkspace, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Workspace::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"removed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(removed_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "id")]
    fn connect_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_id_trampoline<P: IsA<Workspace>, F: Fn(&P) + 'static>(this: *mut ffi::AstalHyprlandWorkspace, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Workspace::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::id\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_id_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "name")]
    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P: IsA<Workspace>, F: Fn(&P) + 'static>(this: *mut ffi::AstalHyprlandWorkspace, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Workspace::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_name_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "monitor")]
    fn connect_monitor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_monitor_trampoline<P: IsA<Workspace>, F: Fn(&P) + 'static>(this: *mut ffi::AstalHyprlandWorkspace, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Workspace::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::monitor\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_monitor_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "clients")]
    fn connect_clients_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_clients_trampoline<P: IsA<Workspace>, F: Fn(&P) + 'static>(this: *mut ffi::AstalHyprlandWorkspace, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Workspace::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::clients\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_clients_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "has-fullscreen")]
    fn connect_has_fullscreen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_fullscreen_trampoline<P: IsA<Workspace>, F: Fn(&P) + 'static>(this: *mut ffi::AstalHyprlandWorkspace, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Workspace::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::has-fullscreen\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_has_fullscreen_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "last-client")]
    fn connect_last_client_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_last_client_trampoline<P: IsA<Workspace>, F: Fn(&P) + 'static>(this: *mut ffi::AstalHyprlandWorkspace, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Workspace::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::last-client\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_last_client_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<Workspace>> WorkspaceExt for O {}
