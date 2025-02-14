// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../../gir/gir-files
// DO NOT EDIT

use crate::{ffi,Player};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    #[doc(alias = "AstalMprisMpris")]
    pub struct Mpris(Object<ffi::AstalMprisMpris, ffi::AstalMprisMprisClass>);

    match fn {
        type_ => || ffi::astal_mpris_mpris_get_type(),
    }
}

impl Mpris {
        pub const NONE: Option<&'static Mpris> = None;
    

    #[doc(alias = "astal_mpris_mpris_new")]
    pub fn new() -> Mpris {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::astal_mpris_mpris_new())
        }
    }

    #[doc(alias = "astal_mpris_mpris_get_default")]
    #[doc(alias = "get_default")]
    #[allow(clippy::should_implement_trait)]    pub fn default() -> Option<Mpris> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::astal_mpris_mpris_get_default())
        }
    }
}

impl Default for Mpris {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

pub trait MprisExt: IsA<Mpris> + 'static {
    #[doc(alias = "astal_mpris_mpris_get_players")]
    #[doc(alias = "get_players")]
    fn players(&self) -> Vec<Player> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::astal_mpris_mpris_get_players(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "player-added")]
    fn connect_player_added<F: Fn(&Self, &Player) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn player_added_trampoline<P: IsA<Mpris>, F: Fn(&P, &Player) + 'static>(this: *mut ffi::AstalMprisMpris, player: *mut ffi::AstalMprisPlayer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Mpris::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(player))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"player-added\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(player_added_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "player-closed")]
    fn connect_player_closed<F: Fn(&Self, &Player) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn player_closed_trampoline<P: IsA<Mpris>, F: Fn(&P, &Player) + 'static>(this: *mut ffi::AstalMprisMpris, player: *mut ffi::AstalMprisPlayer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Mpris::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(player))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"player-closed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(player_closed_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[doc(alias = "players")]
    fn connect_players_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_players_trampoline<P: IsA<Mpris>, F: Fn(&P) + 'static>(this: *mut ffi::AstalMprisMpris, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Mpris::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::players\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_players_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<Mpris>> MprisExt for O {}
